extern crate core;

use relay_rpc::{
    auth::{
        ed25519_dalek::Keypair,
        rand::{rngs::StdRng, SeedableRng},
    },
    domain::{ClientId, DecodedClientId},
};

mod context;
mod messages;
mod metrics;
mod registration;
mod simple;
mod storage;

const TEST_RELAY_URL: &str = "https://history.walletconnect.com";

pub type ErrorResult<T> = Result<T, TestError>;

#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error(transparent)]
    Elapsed(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    Gilgamesh(#[from] gilgamesh::error::Error),
}

fn get_client_jwt() -> (String, ClientId) {
    let mut rng = StdRng::from_entropy();
    let keypair = Keypair::generate(&mut rng);

    let random_client_id = DecodedClientId(*keypair.public_key().as_bytes());
    let client_id = ClientId::from(random_client_id);

    let jwt = relay_rpc::auth::AuthToken::new(client_id.value().clone())
        .aud(TEST_RELAY_URL.to_string())
        .as_jwt(&keypair)
        .unwrap()
        .to_string();

    (jwt, client_id)
}
