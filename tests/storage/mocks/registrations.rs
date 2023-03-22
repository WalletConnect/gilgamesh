use {
    async_trait::async_trait,
    gilgamesh::store::{
        registrations::{Registration, RegistrationStore},
        StoreError,
    },
    moka::future::Cache,
    std::fmt::Debug,
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
            client_id: client_id.to_string(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            relay_url: relay_url.to_string(),
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
