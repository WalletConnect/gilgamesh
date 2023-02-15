extern crate core;

// mod env;
// mod providers;
mod context;
mod functional;
mod store;

pub type ErrorResult<T> = Result<T, TestError>;

#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error(transparent)]
    Elapsed(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    RustHttpStarter(#[from] gilgamesh::error::Error),
}
