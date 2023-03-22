use {
    crate::context::ServerContext,
    gilgamesh::handlers::get_messages::GetMessagesBody,
    relay_rpc::{
        auth::{
            ed25519_dalek::Keypair,
            rand::{rngs::StdRng, SeedableRng},
        },
        domain::{ClientId, DecodedClientId},
    },
    test_context::test_context,
};

#[test_context(ServerContext)]
#[tokio::test]
async fn test_get_message(ctx: &mut ServerContext) {
    let mut rng = StdRng::from_entropy();
    let keypair = Keypair::generate(&mut rng);

    let random_client_id = DecodedClientId(*keypair.public_key().as_bytes());
    let client_id = ClientId::from(random_client_id);

    let jwt = relay_rpc::auth::AuthToken::new(client_id.value().clone())
        .aud(format!(
            "http://127.0.0.1:{}",
            ctx.server.public_addr.port()
        ))
        .as_jwt(&keypair)
        .unwrap()
        .to_string();

    let payload = GetMessagesBody {
        topic: "".to_string(),
        origin_id: None,
        message_count: Default::default(),
        direction: None,
    };

    // Register client
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/messages", ctx.server.public_addr))
        .json(&payload)
        .header("Authorization", jwt.clone())
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    // let body = reqwest::get(format!("http://{}/health", ctx.server.public_addr))
    //     .await
    //     .expect("Failed to call /health")
    //     .status();
    // assert!(body.is_success());
}
