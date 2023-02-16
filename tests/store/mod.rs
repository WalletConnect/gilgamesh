use {
    crate::context::StoreContext,
    core::time,
    gilgamesh::stores::messages::MessagesPersistentStorage,
    test_context::test_context,
};

const TEST_TOPIC: &str = "123456789";
const TEST_QUERY_SIZE: usize = 3;

#[test_context(StoreContext)]
#[tokio::test]
async fn test_get_messages(ctx: &mut StoreContext) {
    fill_store(ctx, TEST_TOPIC, 20).await;
    futures::join!(
        test_after_no_origin(ctx),
        test_after_origin(ctx),
        test_after_origin_overflow(ctx),
        test_before_no_origin(ctx),
        test_before_origin(ctx),
        test_before_origin_overflow(ctx),
    );
}

async fn test_after_no_origin(ctx: &StoreContext) {
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

async fn test_after_origin(ctx: &StoreContext) {
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

async fn test_after_origin_overflow(ctx: &StoreContext) {
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

async fn test_before_no_origin(ctx: &StoreContext) {
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

async fn test_before_origin(ctx: &StoreContext) {
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

async fn test_before_origin_overflow(ctx: &StoreContext) {
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

async fn fill_store(ctx: &StoreContext, topic: &str, size: i32) {
    for id in 1..(size + 1) {
        ctx.storage
            .store
            .upsert_message(topic, &id.to_string())
            .await
            .unwrap();
        std::thread::sleep(time::Duration::from_millis(2));
    }
}
