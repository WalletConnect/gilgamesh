use {
    crate::{
        config::Configuration,
        store::{
            messages::{Message, MessagesStore, StoreMessages},
            registrations::{Registration, RegistrationStore},
            StoreError,
        },
    },
    async_trait::async_trait,
    chrono::Utc,
    futures::TryStreamExt,
    wither::{
        bson::{self, doc, Document},
        mongodb::{
            options::{ClientOptions, FindOneAndUpdateOptions, FindOptions},
            Client,
            Database,
        },
        Model,
    },
};

#[derive(Debug, Clone)]
pub struct MongoStore {
    db: Database,
}

impl MongoStore {
    pub async fn new(config: &Configuration) -> anyhow::Result<Self> {
        let url = &config.mongo_address;

        let client_options = ClientOptions::parse(url).await?;
        let client = Client::with_options(client_options)?;
        let db = client.default_database().ok_or_else(|| {
            anyhow::anyhow!("no default database specified in the connection URL")
        })?;

        Message::sync(&db).await?;
        Registration::sync(&db).await?;

        Ok(Self { db })
    }

    async fn get_message_timestamp(
        &self,
        topic: &str,
        message_id: &str,
    ) -> Result<bson::DateTime, StoreError> {
        let filter = doc! {
            "topic": &topic,
            "message_id": message_id,
        };

        let cursor = Message::find(&self.db, filter, None).await?;
        let origin: Vec<Message> = cursor.try_collect().await?;
        let origin = origin.first().ok_or(StoreError::NotFound(
            topic.to_string(),
            message_id.to_string(),
        ))?;

        Ok(origin.timestamp)
    }

    async fn get_messages(
        &self,
        client_id: &str,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
        comparator: &str,
        sort_order: i32,
    ) -> Result<StoreMessages, StoreError> {
        let filter: Result<Document, StoreError> = match origin {
            None => Ok(doc! {
                "client_id": &client_id,
                "topic": &topic,
            }),
            Some(origin) => {
                let ts = self.get_message_timestamp(topic, origin).await?;
                Ok(doc! {
                    "client_id": &client_id,
                    "topic": &topic,
                    "ts": { comparator: ts }
                })
            }
        };
        let filter = filter?;

        let message_count: i64 = message_count as i64;
        let limit = -(message_count + 1);
        let options = FindOptions::builder()
            .sort(doc! {"ts": sort_order})
            .limit(limit)
            .build();

        let cursor = Message::find(&self.db, filter, options).await?;

        let mut messages: Vec<Message> = cursor.try_collect().await?;

        if messages.len() > message_count as usize {
            let next_id = messages.pop().map(|message| message.message_id);
            return Ok(StoreMessages { messages, next_id });
        }

        Ok(StoreMessages {
            messages,
            next_id: None,
        })
    }
}

#[async_trait]
impl MessagesStore for MongoStore {
    async fn upsert_message(
        &self,
        method: &str,
        client_id: &str,
        topic: &str,
        message_id: &str,
        message: &str,
    ) -> Result<(), StoreError> {
        let filter = doc! {
            "client_id": &client_id,
            "topic": &topic,
            "message_id": &message_id,
        };

        let update = doc! {
            "$set": {
                "ts": Utc::now(),
                "method": &method,
                "client_id": &client_id,
                "topic": &topic,
                "message_id": &message_id,
                "message": &message,
            }
        };

        let option = FindOneAndUpdateOptions::builder().upsert(true).build();

        match Message::find_one_and_update(&self.db, filter, update, option).await? {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    async fn get_messages_after(
        &self,
        client_id: &str,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<StoreMessages, StoreError> {
        self.get_messages(client_id, topic, origin, message_count, "$gte", 1)
            .await
    }

    async fn get_messages_before(
        &self,
        client_id: &str,
        topic: &str,
        origin: Option<&str>,
        message_count: usize,
    ) -> Result<StoreMessages, StoreError> {
        self.get_messages(client_id, topic, origin, message_count, "$lte", -1)
            .await
    }
}

#[async_trait]
impl RegistrationStore for MongoStore {
    async fn upsert_registration(
        &self,
        client_id: &str,
        tags: Vec<&str>,
        relay_url: &str,
    ) -> Result<(), StoreError> {
        let filter = doc! {
            "client_id": &client_id,
        };

        let update = doc! {
            "$set": {
                "client_id": &client_id,
                "tags": tags,
                "relay_url": &relay_url,
            }
        };

        let option = FindOneAndUpdateOptions::builder().upsert(true).build();

        match Registration::find_one_and_update(&self.db, filter, update, option).await? {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    async fn get_registration(&self, client_id: &str) -> Result<Registration, StoreError> {
        let filter = doc! {
            "client_id": &client_id,
        };

        let registration = Registration::find_one(&self.db, filter, None).await?;
        registration.ok_or(StoreError::NotFound(
            "registration".to_string(),
            client_id.to_string(),
        ))
    }
}
