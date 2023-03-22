use {
    crate::context::StoreContext,
    ::function_name::named,
    gilgamesh::store::messages::MessagesStore,
    std::time,
    test_context::test_context,
};

const TEST_CLIENT_ID: &str = "12345";
const TEST_QUERY_SIZE: usize = 3;

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[named]
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_after_no_origin(ctx: &StoreContext) {
    let topic = function_name!();
    fill_store(ctx, TEST_CLIENT_ID, topic, 20).await;

    let result = ctx
        .storage
        .store
        .get_messages_after(TEST_CLIENT_ID, topic, None, TEST_QUERY_SIZE)
        .await
        .unwrap();

    assert_eq!(
        result.messages.len(),
        TEST_QUERY_SIZE,
        "check result length"
    );

    for n in 0..TEST_QUERY_SIZE {
        assert_eq!(
            result.messages.get(n).unwrap().message_id,
            (1 + n).to_string(),
            "check that item #{} equals expected {}",
            n,
            1 + n
        )
    }

    assert_eq!(result.next_id, Some("4".to_string()), "Check next_id");
}

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[named]
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_after_origin(ctx: &StoreContext) {
    let topic = function_name!();
    fill_store(ctx, TEST_CLIENT_ID, topic, 20).await;

    let origin = 4;

    let result = ctx
        .storage
        .store
        .get_messages_after(
            TEST_CLIENT_ID,
            topic,
            Some(&origin.to_string()),
            TEST_QUERY_SIZE,
        )
        .await
        .unwrap();

    assert_eq!(
        result.messages.len(),
        TEST_QUERY_SIZE,
        "check result length"
    );

    for n in 0..TEST_QUERY_SIZE {
        assert_eq!(
            result.messages.get(n).unwrap().message_id,
            (origin + n).to_string(),
            "check that item #{} equals expected {}",
            n,
            origin + n
        )
    }

    assert_eq!(
        result.next_id,
        Some((origin + TEST_QUERY_SIZE).to_string()),
        "Check next_id"
    );
}

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[named]
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_after_origin_overflow(ctx: &StoreContext) {
    let topic = function_name!();
    fill_store(ctx, TEST_CLIENT_ID, topic, 20).await;

    let origin = 19;

    let result = ctx
        .storage
        .store
        .get_messages_after(
            TEST_CLIENT_ID,
            topic,
            Some(&origin.to_string()),
            TEST_QUERY_SIZE,
        )
        .await
        .unwrap();

    assert_eq!(result.messages.len(), 2, "check result length");

    for n in 0..2 {
        assert_eq!(
            result.messages.get(n).unwrap().message_id,
            (origin + n).to_string(),
            "check that item #{} equals expected {}",
            n,
            origin + n
        )
    }

    assert_eq!(result.next_id, None, "Check next_id");
}

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[named]
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_before_no_origin(ctx: &StoreContext) {
    let topic = function_name!();
    fill_store(ctx, TEST_CLIENT_ID, topic, 20).await;

    let result = ctx
        .storage
        .store
        .get_messages_before(TEST_CLIENT_ID, topic, None, TEST_QUERY_SIZE)
        .await
        .unwrap();

    assert_eq!(
        result.messages.len(),
        TEST_QUERY_SIZE,
        "check result length"
    );

    for n in 0..TEST_QUERY_SIZE {
        assert_eq!(
            result.messages.get(n).unwrap().message_id,
            (20 - n).to_string(),
            "check that item #{} equals expected {}",
            n,
            20 - n
        )
    }

    assert_eq!(result.next_id, Some("17".to_string()), "Check next_id");
}

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[named]
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_before_origin(ctx: &StoreContext) {
    let topic = function_name!();
    fill_store(ctx, TEST_CLIENT_ID, topic, 20).await;

    let origin = 16;

    let result = ctx
        .storage
        .store
        .get_messages_before(
            TEST_CLIENT_ID,
            topic,
            Some(&origin.to_string()),
            TEST_QUERY_SIZE,
        )
        .await
        .unwrap();

    assert_eq!(
        result.messages.len(),
        TEST_QUERY_SIZE,
        "check result length"
    );

    for n in 0..TEST_QUERY_SIZE {
        assert_eq!(
            result.messages.get(n).unwrap().message_id,
            (origin - n).to_string(),
            "check that item #{} equals expected {}",
            n,
            origin - n
        )
    }

    assert_eq!(
        result.next_id,
        Some((origin - TEST_QUERY_SIZE).to_string()),
        "Check next_id"
    );
}

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[named]
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_before_origin_overflow(ctx: &StoreContext) {
    let topic = function_name!();
    fill_store(ctx, TEST_CLIENT_ID, topic, 20).await;

    let origin = 2;

    let result = ctx
        .storage
        .store
        .get_messages_before(
            TEST_CLIENT_ID,
            topic,
            Some(&origin.to_string()),
            TEST_QUERY_SIZE,
        )
        .await
        .unwrap();

    assert_eq!(result.messages.len(), 2, "check result length");

    for n in 0..2 {
        assert_eq!(
            result.messages.get(n).unwrap().message_id,
            (origin - n).to_string(),
            "check that item #{} equals expected {}",
            n,
            origin - n
        )
    }

    assert_eq!(result.next_id, None, "Check next_id");
}

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[named]
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_multi_topic(ctx: &StoreContext) {
    const NUM_TOPICS: usize = 5;
    const QUERY_SIZE: usize = 2;

    for t in 0..NUM_TOPICS {
        let topic = format!("{}-{}", function_name!(), t + 1);
        fill_store(ctx, TEST_CLIENT_ID, topic.as_str(), QUERY_SIZE as i32).await;
    }

    for t in 0..NUM_TOPICS {
        let topic = format!("{}-{}", function_name!(), t + 1);

        let result = ctx
            .storage
            .store
            .get_messages_after(TEST_CLIENT_ID, topic.as_str(), None, QUERY_SIZE)
            .await
            .unwrap();

        assert_eq!(
            result.messages.len(),
            QUERY_SIZE,
            "check result length for topic {}",
            t + 1
        );

        for n in 0..QUERY_SIZE {
            let message = result.messages.get(n).unwrap();
            assert_eq!(
                message.message_id,
                (1 + n).to_string(),
                "check that item #{}/{} equals expected {}",
                t + 1,
                n + 1,
                n + 1
            )
        }
    }
}

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[named]
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_multi_clients(ctx: &StoreContext) {
    const NUM_CLIENTS: usize = 5;
    const QUERY_SIZE: usize = 2;

    let topic = function_name!();

    for t in 0..NUM_CLIENTS {
        let client_id = format!("{}-{}", TEST_CLIENT_ID, t + 1);
        fill_store(ctx, client_id.as_str(), topic, QUERY_SIZE as i32).await;
    }

    for t in 0..NUM_CLIENTS {
        let client_id = format!("{}-{}", TEST_CLIENT_ID, t + 1);

        let result = ctx
            .storage
            .store
            .get_messages_after(client_id.as_str(), topic, None, QUERY_SIZE)
            .await
            .unwrap();

        assert_eq!(
            result.messages.len(),
            QUERY_SIZE,
            "check result length for topic {}",
            t + 1
        );

        for n in 0..QUERY_SIZE {
            let message = result.messages.get(n).unwrap();
            assert_eq!(
                message.message_id,
                (1 + n).to_string(),
                "check that item #{}/{} equals expected {}",
                t + 1,
                n + 1,
                n + 1
            )
        }
    }
}

async fn fill_store(ctx: &StoreContext, client_id: &str, topic: &str, size: i32) {
    for id in 1..(size + 1) {
        ctx.storage
            .store
            .upsert_message(client_id, topic, &id.to_string(), id.to_string().as_str())
            .await
            .unwrap();
        std::thread::sleep(time::Duration::from_millis(2));
    }
}
