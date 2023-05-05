use {
    super::StoreError,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    wither::{
        bson::{doc, oid::ObjectId},
        Model,
    },
};

#[derive(Clone, Debug, Model, Serialize, Deserialize, PartialEq, Eq)]
#[model(
    collection_name = "Registrations",
    index(keys = r#"doc!{"client_id": 1}"#, options = r#"doc!{"unique": true}"#)
)]
pub struct Registration {
    /// MongoDB's default `_id` field.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The 'client_id' of the registration.
    pub client_id: Arc<str>,
    /// The registered tags
    pub tags: Vec<Arc<str>>,
    /// The registered relay_url
    pub relay_url: Arc<str>,
}

#[async_trait]
pub trait RegistrationStore: 'static + Send + Sync {
    async fn upsert_registration(
        &self,
        client_id: &str,
        tags: Vec<&str>,
        relay_url: &str,
    ) -> Result<(), StoreError>;
    async fn get_registration(&self, client_id: &str) -> Result<Registration, StoreError>;
}
