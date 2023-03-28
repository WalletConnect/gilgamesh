pub mod messages;
pub mod mongo;
pub mod registrations;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// Not found error, params are entity name and identifier
    #[error("Cannot find {0} with specified identifier {1}")]
    NotFound(String, String),

    #[error(transparent)]
    Database(#[from] wither::WitherError),
}
