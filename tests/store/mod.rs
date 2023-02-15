use {
    crate::context::StoreContext,
    core::time,
    gilgamesh::stores::messages::MessagesPersistentStorage,
    std::thread,
    test_context::test_context,
};

const TEST_TOPIC: &str = "1234";
const TEST_QUERY_SIZE: usize = 3;

#[test_context(StoreContext)]
#[tokio::test]
async fn after_no_origin(ctx: &mut StoreContext) {
    fill_store(ctx, 20).await;

    let result = ctx
        .storage
        .store
        .get_messages_after(TEST_TOPIC, None, TEST_QUERY_SIZE)
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

#[test_context(StoreContext)]
#[tokio::test]
async fn after_origin(ctx: &mut StoreContext) {
    fill_store(ctx, 20).await;

    let origin = 4;

    let result = ctx
        .storage
        .store
        .get_messages_after(TEST_TOPIC, Some(&origin.to_string()), TEST_QUERY_SIZE)
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

#[test_context(StoreContext)]
#[tokio::test]
async fn after_origin_overflow(ctx: &mut StoreContext) {
    fill_store(ctx, 20).await;

    let origin = 19;

    let result = ctx
        .storage
        .store
        .get_messages_after(TEST_TOPIC, Some(&origin.to_string()), TEST_QUERY_SIZE)
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

#[test_context(StoreContext)]
#[tokio::test]
async fn before_no_origin(ctx: &mut StoreContext) {
    fill_store(ctx, 20).await;

    let result = ctx
        .storage
        .store
        .get_messages_before(TEST_TOPIC, None, TEST_QUERY_SIZE)
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

#[test_context(StoreContext)]
#[tokio::test]
async fn before_origin(ctx: &mut StoreContext) {
    fill_store(ctx, 20).await;

    let origin = 16;

    let result = ctx
        .storage
        .store
        .get_messages_before(TEST_TOPIC, Some(&origin.to_string()), TEST_QUERY_SIZE)
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

#[test_context(StoreContext)]
#[tokio::test]
async fn before_origin_overflow(ctx: &mut StoreContext) {
    fill_store(ctx, 20).await;

    let origin = 2;

    let result = ctx
        .storage
        .store
        .get_messages_before(TEST_TOPIC, Some(&origin.to_string()), TEST_QUERY_SIZE)
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

async fn fill_store(ctx: &mut StoreContext, size: i32) {
    for id in 1..(size + 1) {
        ctx.storage
            .store
            .upsert_message(TEST_TOPIC, &id.to_string())
            .await
            .unwrap();
        thread::sleep(time::Duration::from_millis(2));
    }
}
