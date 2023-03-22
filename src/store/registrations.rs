use {
    super::StoreError,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
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
    pub client_id: String,
    /// The registered tags
    pub tags: Vec<String>,
    /// The registered relay_url
    pub relay_url: String,
}

#[async_trait]
pub trait RegistrationStore: 'static + std::fmt::Debug + Send + Sync {
    async fn upsert_registration(
        &self,
        client_id: &str,
        tags: Vec<&str>,
        relay_url: &str,
    ) -> Result<(), StoreError>;
    async fn get_registration(&self, client_id: &str) -> Result<Registration, StoreError>;
}
