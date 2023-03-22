use {
    crate::{context::ServerContext, get_client_jwt, TEST_RELAY_URL},
    axum::http,
    gilgamesh::{handlers::register::RegisterPayload, store::registrations::Registration},
    test_context::test_context,
};

#[test_context(ServerContext)]
#[tokio::test]
async fn test_register(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_client_jwt();

    let payload = RegisterPayload {
        tags: vec!["4000".to_string(), "5***".to_string()],
        relay_url: TEST_RELAY_URL.to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/register", ctx.server.public_addr))
        .json(&payload)
        .header(http::header::AUTHORIZATION, format!("Bearer {jwt}"))
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful: {:?} - {:?}",
        response.status(),
        response.text().await
    );

    assert!(ctx
        .server
        .registration_store
        .registrations
        .get(client_id.value().as_ref())
        .is_some())
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_get_registration(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_client_jwt();

    let tags = vec!["4000".to_string(), "5***".to_string()];
    let registration = Registration {
        id: None,
        client_id: client_id.to_string(),
        tags: tags.clone(),
        relay_url: TEST_RELAY_URL.to_string(),
    };

    ctx.server
        .registration_store
        .registrations
        .insert(client_id.value().to_string(), registration)
        .await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/register", ctx.server.public_addr))
        .header(http::header::AUTHORIZATION, format!("Bearer {jwt}"))
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_success(),
        "Response was not successful: {:?} - {:?}",
        response.status(),
        response.text().await
    );

    assert!(response
        .headers()
        .contains_key("Access-Control-Allow-Origin"));
    let allowed_origins = response
        .headers()
        .get("Access-Control-Allow-Origin")
        .unwrap();
    assert_eq!(allowed_origins.to_str().unwrap(), "*");

    let payload: RegisterPayload = response.json().await.unwrap();
    assert_eq!(payload.tags, tags);
    assert_eq!(payload.relay_url, TEST_RELAY_URL);
}
