use {
    crate::{context::ServerContext, get_client_jwt, TEST_RELAY_URL},
    axum::http,
    chrono::Utc,
    gilgamesh::{
        handlers::{
            get_messages::{Direction, GetMessagesResponse},
            save_message::HistoryPayload,
        },
        store::{messages::Message, registrations::Registration},
    },
    test_context::test_context,
};

const TEST_CLIENT_ID: &str = "12345";
const TEST_MESSAGE_ID: &str = "67890";
const TEST_TOPIC: &str = "test-topic";
const TEST_MESSAGE: &str = "test-message";

#[test_context(ServerContext)]
#[tokio::test]
async fn test_get_message_no_origin_no_count_no_direction(ctx: &mut ServerContext) {
    let (jwt, _) = get_client_jwt();

    ctx.server
        .message_store
        .test_add(Message {
            id: None,
            timestamp: Utc::now().into(),
            client_id: TEST_CLIENT_ID.to_string(),
            message_id: TEST_MESSAGE_ID.to_string(),
            topic: TEST_TOPIC.to_string(),
            message: TEST_MESSAGE.to_string(),
        })
        .await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/messages", ctx.server.public_addr))
        .query(&[("topic", TEST_TOPIC)])
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

    let response: GetMessagesResponse = response.json().await.unwrap();

    assert_eq!(response.topic, TEST_TOPIC);
    assert_eq!(response.direction, Direction::Forward);
    assert_eq!(response.next_id, Some("after".to_string()));

    assert_eq!(response.messages.len(), 1);
    assert_eq!(response.messages[0].client_id, TEST_CLIENT_ID);
    assert_eq!(response.messages[0].topic, TEST_TOPIC);
    assert_eq!(response.messages[0].message_id, TEST_MESSAGE_ID);
    assert_eq!(response.messages[0].message, TEST_MESSAGE);
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_get_message_origin_count_forward(ctx: &mut ServerContext) {
    let (jwt, _) = get_client_jwt();

    ctx.server
        .message_store
        .test_add(Message {
            id: None,
            timestamp: Utc::now().into(),
            client_id: TEST_CLIENT_ID.to_string(),
            message_id: TEST_MESSAGE_ID.to_string(),
            topic: TEST_TOPIC.to_string(),
            message: TEST_MESSAGE.to_string(),
        })
        .await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/messages", ctx.server.public_addr))
        .query(&[
            ("topic", TEST_TOPIC),
            ("originId", "1"),
            ("messageCount", "2"),
            ("direction", "forward"),
        ])
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

    let response: GetMessagesResponse = response.json().await.unwrap();

    assert_eq!(response.topic, TEST_TOPIC);
    assert_eq!(response.direction, Direction::Forward);
    assert_eq!(response.next_id, Some("after".to_string()));

    assert_eq!(response.messages.len(), 1);
    assert_eq!(response.messages[0].client_id, TEST_CLIENT_ID);
    assert_eq!(response.messages[0].topic, TEST_TOPIC);
    assert_eq!(response.messages[0].message_id, TEST_MESSAGE_ID);
    assert_eq!(response.messages[0].message, TEST_MESSAGE);
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_get_message_origin_count_backward(ctx: &mut ServerContext) {
    let (jwt, _) = get_client_jwt();

    ctx.server
        .message_store
        .test_add(Message {
            id: None,
            timestamp: Utc::now().into(),
            client_id: TEST_CLIENT_ID.to_string(),
            message_id: TEST_MESSAGE_ID.to_string(),
            topic: TEST_TOPIC.to_string(),
            message: TEST_MESSAGE.to_string(),
        })
        .await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/messages", ctx.server.public_addr))
        .query(&[
            ("topic", TEST_TOPIC),
            ("originId", "1"),
            ("messageCount", "2"),
            ("direction", "backward"),
        ])
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

    let response: GetMessagesResponse = response.json().await.unwrap();

    assert_eq!(response.topic, TEST_TOPIC);
    assert_eq!(response.direction, Direction::Backward);
    assert_eq!(response.next_id, Some("before".to_string()));

    assert_eq!(response.messages.len(), 1);
    assert_eq!(response.messages[0].client_id, TEST_CLIENT_ID);
    assert_eq!(response.messages[0].topic, TEST_TOPIC);
    assert_eq!(response.messages[0].message_id, TEST_MESSAGE_ID);
    assert_eq!(response.messages[0].message, TEST_MESSAGE);
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_save_message_saved(ctx: &mut ServerContext) {
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
        .post(format!("http://{}/messages", ctx.server.public_addr))
        .json(&HistoryPayload {
            client_id: client_id.to_string(),
            message_id: format!("{TEST_MESSAGE_ID}-1"),
            topic: TEST_TOPIC.to_string(),
            tag: 4000,
            message: TEST_MESSAGE.to_string(),
        })
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

    let response = client
        .post(format!("http://{}/messages", ctx.server.public_addr))
        .json(&HistoryPayload {
            client_id: client_id.to_string(),
            message_id: format!("{TEST_MESSAGE_ID}-2"),
            topic: TEST_TOPIC.to_string(),
            tag: 5123,
            message: TEST_MESSAGE.to_string(),
        })
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

    let _all = ctx.server.message_store.test_get_keys();
    let _client_id = client_id.to_string();

    let msg = ctx
        .server
        .message_store
        .test_get(
            client_id.value(),
            TEST_TOPIC,
            format!("{TEST_MESSAGE_ID}-1").as_str(),
        )
        .await
        .unwrap();
    assert_eq!(msg.client_id, client_id.to_string());
    assert_eq!(msg.topic, TEST_TOPIC);
    assert_eq!(msg.message_id, format!("{TEST_MESSAGE_ID}-1"));
    assert_eq!(msg.message, TEST_MESSAGE);

    let msg = ctx
        .server
        .message_store
        .test_get(
            client_id.value(),
            TEST_TOPIC,
            format!("{TEST_MESSAGE_ID}-2").as_str(),
        )
        .await
        .unwrap();
    assert_eq!(msg.client_id, client_id.to_string());
    assert_eq!(msg.topic, TEST_TOPIC);
    assert_eq!(msg.message_id, format!("{TEST_MESSAGE_ID}-2"));
    assert_eq!(msg.message, TEST_MESSAGE);
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_save_message_filtered_out(ctx: &mut ServerContext) {
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
        .post(format!("http://{}/messages", ctx.server.public_addr))
        .json(&HistoryPayload {
            client_id: client_id.to_string(),
            message_id: format!("{TEST_MESSAGE_ID}-1"),
            topic: TEST_TOPIC.to_string(),
            tag: 4123,
            message: TEST_MESSAGE.to_string(),
        })
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

    let response = client
        .post(format!("http://{}/messages", ctx.server.public_addr))
        .json(&HistoryPayload {
            client_id: client_id.to_string(),
            message_id: format!("{TEST_MESSAGE_ID}-2"),
            topic: TEST_TOPIC.to_string(),
            tag: 5123,
            message: TEST_MESSAGE.to_string(),
        })
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

    let _all = ctx.server.message_store.test_get_keys();
    let _client_id = client_id.to_string();

    let msg = ctx
        .server
        .message_store
        .test_get(
            client_id.value(),
            TEST_TOPIC,
            format!("{TEST_MESSAGE_ID}-1").as_str(),
        )
        .await;
    assert!(msg.is_none());

    let msg = ctx
        .server
        .message_store
        .test_get(
            client_id.value(),
            TEST_TOPIC,
            format!("{TEST_MESSAGE_ID}-2").as_str(),
        )
        .await
        .unwrap();
    assert_eq!(msg.client_id, client_id.to_string());
    assert_eq!(msg.topic, TEST_TOPIC);
    assert_eq!(msg.message_id, format!("{TEST_MESSAGE_ID}-2"));
    assert_eq!(msg.message, TEST_MESSAGE);
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_save_message_no_registration(ctx: &mut ServerContext) {
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
        .post(format!("http://{}/messages", ctx.server.public_addr))
        .json(&HistoryPayload {
            client_id: TEST_CLIENT_ID.to_string(),
            message_id: TEST_MESSAGE_ID.to_string(),
            topic: TEST_TOPIC.to_string(),
            tag: 4000,
            message: TEST_MESSAGE.to_string(),
        })
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

    let _all = ctx.server.message_store.test_get_keys();
    let _client_id = client_id.to_string();

    let msg = ctx
        .server
        .message_store
        .test_get(client_id.value(), TEST_TOPIC, TEST_MESSAGE_ID)
        .await;
    assert!(msg.is_none());
}
