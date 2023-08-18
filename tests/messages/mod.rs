use {
    crate::{context::ServerContext, get_client_jwt, TEST_RELAY_URL},
    archive::{
        handlers::{
            get_messages::{Direction, GetMessagesResponse},
            save_message::ArchivePayload,
        },
        store::{messages::Message, registrations::Registration},
    },
    axum::http,
    chrono::Utc,
    std::sync::Arc,
    test_context::test_context,
};

const TEST_METHOD: &str = "publish";
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
            method: Arc::from(TEST_METHOD),
            client_id: Arc::from(TEST_CLIENT_ID),
            message_id: Arc::from(TEST_MESSAGE_ID),
            topic: Arc::from(TEST_TOPIC),
            message: Arc::from(TEST_MESSAGE),
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

    assert_eq!(response.topic.as_ref(), TEST_TOPIC);
    assert_eq!(response.direction, Direction::Forward);
    assert_eq!(response.next_id.unwrap().as_ref(), "after");

    assert_eq!(response.messages.len(), 1);
    assert_eq!(response.messages[0].client_id.as_ref(), TEST_CLIENT_ID);
    assert_eq!(response.messages[0].topic.as_ref(), TEST_TOPIC);
    assert_eq!(response.messages[0].message_id.as_ref(), TEST_MESSAGE_ID);
    assert_eq!(response.messages[0].message.as_ref(), TEST_MESSAGE);
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
            method: Arc::from(TEST_METHOD),
            client_id: Arc::from(TEST_CLIENT_ID),
            message_id: Arc::from(TEST_MESSAGE_ID),
            topic: Arc::from(TEST_TOPIC),
            message: Arc::from(TEST_MESSAGE),
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

    assert_eq!(response.topic.as_ref(), TEST_TOPIC);
    assert_eq!(response.direction, Direction::Forward);
    assert_eq!(response.next_id.unwrap().as_ref(), "after");

    assert_eq!(response.messages.len(), 1);
    assert_eq!(response.messages[0].client_id.as_ref(), TEST_CLIENT_ID);
    assert_eq!(response.messages[0].topic.as_ref(), TEST_TOPIC);
    assert_eq!(response.messages[0].message_id.as_ref(), TEST_MESSAGE_ID);
    assert_eq!(response.messages[0].message.as_ref(), TEST_MESSAGE);
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
            method: Arc::from(TEST_METHOD),
            client_id: Arc::from(TEST_CLIENT_ID),
            message_id: Arc::from(TEST_MESSAGE_ID),
            topic: Arc::from(TEST_TOPIC),
            message: Arc::from(TEST_MESSAGE),
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

    assert_eq!(response.topic.as_ref(), TEST_TOPIC);
    assert_eq!(response.direction, Direction::Backward);
    assert_eq!(response.next_id.unwrap().as_ref(), "before");

    assert_eq!(response.messages.len(), 1);
    assert_eq!(response.messages[0].client_id.as_ref(), TEST_CLIENT_ID);
    assert_eq!(response.messages[0].topic.as_ref(), TEST_TOPIC);
    assert_eq!(response.messages[0].message_id.as_ref(), TEST_MESSAGE_ID);
    assert_eq!(response.messages[0].message.as_ref(), TEST_MESSAGE);
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_save_message_saved(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_client_jwt();

    let tags = vec![Arc::from("4000"), Arc::from("5***")];
    let registration = Registration {
        id: None,
        client_id: client_id.clone().into_value(),
        tags: tags.clone(),
        relay_url: Arc::from(TEST_RELAY_URL),
    };

    ctx.server
        .registration_store
        .registrations
        .insert(client_id.to_string(), registration)
        .await;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("http://{}/messages", ctx.server.public_addr))
        .json(&ArchivePayload {
            method: Arc::from(TEST_METHOD),
            client_id: client_id.clone().into_value(),
            message_id: Arc::from(format!("{TEST_MESSAGE_ID}-1")),
            topic: Arc::from(TEST_TOPIC),
            tag: 4000,
            message: Arc::from(TEST_MESSAGE),
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
        .json(&ArchivePayload {
            method: Arc::from(TEST_METHOD),
            client_id: client_id.clone().into_value(),
            message_id: Arc::from(format!("{TEST_MESSAGE_ID}-2")),
            topic: Arc::from(TEST_TOPIC),
            tag: 5123,
            message: Arc::from(TEST_MESSAGE),
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
    assert_eq!(msg.client_id.as_ref(), client_id.to_string());
    assert_eq!(msg.topic.as_ref(), TEST_TOPIC);
    assert_eq!(msg.message_id.as_ref(), format!("{TEST_MESSAGE_ID}-1"));
    assert_eq!(msg.message.as_ref(), TEST_MESSAGE);

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
    assert_eq!(msg.client_id.as_ref(), client_id.to_string());
    assert_eq!(msg.topic.as_ref(), TEST_TOPIC);
    assert_eq!(msg.message_id.as_ref(), format!("{TEST_MESSAGE_ID}-2"));
    assert_eq!(msg.message.as_ref(), TEST_MESSAGE);
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_save_message_filtered_out(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_client_jwt();

    let tags = vec![Arc::from("4000"), Arc::from("5***")];
    let registration = Registration {
        id: None,
        client_id: client_id.clone().into_value(),
        tags: tags.clone(),
        relay_url: Arc::from(TEST_RELAY_URL),
    };

    ctx.server
        .registration_store
        .registrations
        .insert(client_id.to_string(), registration)
        .await;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("http://{}/messages", ctx.server.public_addr))
        .json(&ArchivePayload {
            method: Arc::from(TEST_METHOD),
            client_id: client_id.clone().into_value(),
            message_id: Arc::from(format!("{TEST_MESSAGE_ID}-1")),
            topic: Arc::from(TEST_TOPIC),
            tag: 4123,
            message: Arc::from(TEST_MESSAGE),
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
        .json(&ArchivePayload {
            method: Arc::from(TEST_METHOD),
            client_id: client_id.clone().into_value(),
            message_id: Arc::from(format!("{TEST_MESSAGE_ID}-2")),
            topic: Arc::from(TEST_TOPIC),
            tag: 5123,
            message: Arc::from(TEST_MESSAGE),
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
    assert_eq!(msg.client_id, client_id.into_value());
    assert_eq!(msg.topic.as_ref(), TEST_TOPIC);
    assert_eq!(msg.message_id.as_ref(), format!("{TEST_MESSAGE_ID}-2"));
    assert_eq!(msg.message.as_ref(), TEST_MESSAGE);
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_save_message_no_registration(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_client_jwt();

    let tags = vec![Arc::from("4000"), Arc::from("5***")];
    let registration = Registration {
        id: None,
        client_id: client_id.clone().into_value(),
        tags: tags.clone(),
        relay_url: Arc::from(TEST_RELAY_URL),
    };

    ctx.server
        .registration_store
        .registrations
        .insert(client_id.to_string(), registration)
        .await;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("http://{}/messages", ctx.server.public_addr))
        .json(&ArchivePayload {
            method: Arc::from(TEST_METHOD),
            client_id: Arc::from(TEST_CLIENT_ID),
            message_id: Arc::from(TEST_MESSAGE_ID),
            topic: Arc::from(TEST_TOPIC),
            tag: 4000,
            message: Arc::from(TEST_MESSAGE),
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

    let msg = ctx
        .server
        .message_store
        .test_get(client_id.value(), TEST_TOPIC, TEST_MESSAGE_ID)
        .await;
    assert!(msg.is_none());
}
