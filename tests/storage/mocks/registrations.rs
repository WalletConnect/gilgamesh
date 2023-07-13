use {
    archive::store::{
        registrations::{Registration, RegistrationStore},
        StoreError,
    },
    async_trait::async_trait,
    moka::future::Cache,
    std::{fmt::Debug, sync::Arc},
};

#[derive(Debug)]
pub struct MockRegistrationStore {
    pub registrations: Cache<String, Registration>,
}

impl MockRegistrationStore {
    pub fn new() -> Self {
        Self {
            registrations: Cache::builder().build(),
        }
    }
}

#[async_trait]
impl RegistrationStore for MockRegistrationStore {
    async fn upsert_registration(
        &self,
        client_id: &str,
        tags: Vec<&str>,
        relay_url: &str,
    ) -> Result<(), StoreError> {
        let reg = Registration {
            id: None,
            client_id: Arc::from(client_id),
            tags: tags.iter().map(|s| Arc::from(s.to_string())).collect(),
            relay_url: Arc::from(relay_url),
        };

        self.registrations.insert(client_id.to_string(), reg).await;
        Ok(())
    }

    async fn get_registration(&self, client_id: &str) -> Result<Registration, StoreError> {
        self.registrations
            .get(client_id)
            .ok_or(StoreError::NotFound(
                "registration".to_string(),
                client_id.to_string(),
            ))
    }
}
