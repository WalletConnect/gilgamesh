use {
    crate::{context::ServerContext, get_client_jwt, get_invalid_client_jwt, TEST_RELAY_URL},
    axum::http,
    gilgamesh::{handlers::register::RegisterPayload, store::registrations::Registration},
    std::sync::Arc,
    test_context::test_context,
};

#[test_context(ServerContext)]
#[tokio::test]
async fn test_register_invalid_jwt(ctx: &mut ServerContext) {
    let (jwt, _) = get_invalid_client_jwt();

    let payload = RegisterPayload {
        tags: Some(vec![Arc::from("4000"), Arc::from("5***")]),
        append_tags: None,
        remove_tags: None,
        relay_url: Arc::from(TEST_RELAY_URL),
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
        response.status().is_client_error(),
        "Response was not successful: {:?} - {:?}",
        response.status(),
        response.text().await
    );
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_register_new(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_client_jwt();

    let payload = RegisterPayload {
        tags: Some(vec![Arc::from("4000"), Arc::from("5***")]),
        append_tags: None,
        remove_tags: None,
        relay_url: Arc::from(TEST_RELAY_URL),
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
async fn test_register(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_client_jwt();
    let relay_url: Arc<str> = Arc::from(TEST_RELAY_URL);

    struct TestCase {
        name: &'static str,
        start: Vec<Arc<str>>,
        overwrite: Option<Vec<Arc<str>>>,
        append: Option<Vec<Arc<str>>>,
        remove: Option<Vec<Arc<str>>>,
        expected: Vec<Arc<str>>,
    }

    let tests = vec![
        TestCase {
            name: "Overwrite",
            start: vec![Arc::from("4000")],
            overwrite: Some(vec![Arc::from("4001"), Arc::from("4002")]),
            append: None,
            remove: None,
            expected: vec![Arc::from("4001"), Arc::from("4002")],
        },
        TestCase {
            name: "Update, Add tags",
            start: vec![Arc::from("4000")],
            overwrite: None,
            append: Some(vec![Arc::from("4001"), Arc::from("4002")]),
            remove: None,
            expected: vec![Arc::from("4000"), Arc::from("4001"), Arc::from("4002")],
        },
        TestCase {
            name: "Update, Add existing tags",
            start: vec![Arc::from("4000")],
            overwrite: None,
            append: Some(vec![Arc::from("4000"), Arc::from("4001")]),
            remove: None,
            expected: vec![Arc::from("4000"), Arc::from("4001")],
        },
        TestCase {
            name: "Update, Remove tags",
            start: vec![Arc::from("4000"), Arc::from("4001"), Arc::from("4002")],
            overwrite: None,
            append: None,
            remove: Some(vec![Arc::from("4001"), Arc::from("4002")]),
            expected: vec![Arc::from("4000")],
        },
        TestCase {
            name: "Update, Remove missing tags",
            start: vec![Arc::from("4000"), Arc::from("4001"), Arc::from("4002")],
            overwrite: None,
            append: None,
            remove: Some(vec![Arc::from("5000"), Arc::from("4001")]),
            expected: vec![Arc::from("4000"), Arc::from("4002")],
        },
        TestCase {
            name: "Overwrite + Update, Update has remove tag from overwrite",
            start: vec![Arc::from("4000")],
            overwrite: Some(vec![Arc::from("5000")]),
            append: Some(vec![Arc::from("4001"), Arc::from("4002")]),
            remove: Some(vec![Arc::from("5000")]),
            expected: vec![Arc::from("5000")],
        },
        TestCase {
            name: "Empty",
            start: vec![Arc::from("4000")],
            overwrite: None,
            append: None,
            remove: None,
            expected: vec![Arc::from("4000")],
        },
    ];

    for test in tests.iter() {
        ctx.server
            .registration_store
            .registrations
            .insert(client_id.to_string(), Registration {
                id: None,
                client_id: client_id.clone().into_value(),
                tags: test.start.clone(),
                relay_url: relay_url.clone(),
            })
            .await;

        let payload = RegisterPayload {
            tags: test.overwrite.clone(),
            append_tags: test.append.clone(),
            remove_tags: test.remove.clone(),
            relay_url: relay_url.clone(),
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
            "{:?} - Response was not successful: {:?} - {:?}",
            test.name,
            response.status(),
            response.text().await
        );

        let registration = ctx
            .server
            .registration_store
            .registrations
            .get(client_id.value().as_ref());

        assert!(
            registration.is_some(),
            "{:?} - Registration was not found in store",
            test.name
        );

        let mut registration = registration.unwrap();
        registration.tags.sort();
        assert_eq!(
            registration.tags,
            test.expected.clone(),
            "{:?} - Tags did not match expected",
            test.name
        );
    }
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_register_update_bad_update(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_client_jwt();

    let payload = RegisterPayload {
        tags: None,
        append_tags: Some(vec![Arc::from("5000")]),
        remove_tags: Some(vec![Arc::from("5000")]),
        relay_url: Arc::from(TEST_RELAY_URL),
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
        response.status().is_client_error(),
        "Response status was invalid: {:?} - {:?}",
        response.status(),
        response.text().await
    );

    let registration = ctx
        .server
        .registration_store
        .registrations
        .get(client_id.value().as_ref());

    assert!(
        registration.is_none(),
        "Registration was found in store when it should not"
    );
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_register_update_bad_update_with_overwrite(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_client_jwt();

    let tags = vec![Arc::from("4000")];
    let payload = RegisterPayload {
        tags: Some(tags.clone()),
        append_tags: Some(vec![Arc::from("5000")]),
        remove_tags: Some(vec![Arc::from("5000")]),
        relay_url: Arc::from(TEST_RELAY_URL),
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
        "Response was unsuccessful: {:?} - {:?}",
        response.status(),
        response.text().await
    );

    assert!(ctx
        .server
        .registration_store
        .registrations
        .get(client_id.value().as_ref())
        .is_some());

    let registration = ctx
        .server
        .registration_store
        .registrations
        .get(client_id.value().as_ref());

    assert!(
        registration.is_some(),
        "Registration was not found in store"
    );

    assert_eq!(
        registration.unwrap().tags,
        tags.clone(),
        "Tags did not match expected"
    );
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_get_registration(ctx: &mut ServerContext) {
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
    assert_eq!(payload.tags.unwrap(), tags);
    assert_eq!(payload.relay_url.as_ref(), TEST_RELAY_URL);
}

#[test_context(ServerContext)]
#[tokio::test]
async fn test_get_registration_invalid_jwt(ctx: &mut ServerContext) {
    let (jwt, client_id) = get_invalid_client_jwt();

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
        .get(format!("http://{}/register", ctx.server.public_addr))
        .header(http::header::AUTHORIZATION, format!("Bearer {jwt}"))
        .send()
        .await
        .expect("Call failed");

    assert!(
        response.status().is_client_error(),
        "Response was not successful: {:?} - {:?}",
        response.status(),
        response.text().await
    );
}
