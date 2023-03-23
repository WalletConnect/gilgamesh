use {
    crate::context::StoreContext,
    gilgamesh::store::{
        registrations::{Registration, RegistrationStore},
        StoreError,
    },
    std::sync::Arc,
    test_context::test_context,
};

const TEST_CLIENT_ID: &str = "12345";
const TEST_RELAY_URL: &str = "https:://test.relay.walletconnect.com";

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_registration(ctx: &StoreContext) {
    const TAGS: [&str; 2] = ["1234", "5678"];
    ctx.storage
        .store
        .upsert_registration(TEST_CLIENT_ID, Vec::from(TAGS), TEST_RELAY_URL)
        .await
        .unwrap();

    let registration = ctx
        .storage
        .store
        .get_registration(TEST_CLIENT_ID)
        .await
        .unwrap();
    assert_eq!(registration.client_id.as_ref(), TEST_CLIENT_ID);
    assert_eq!(registration.relay_url.as_ref(), TEST_RELAY_URL);

    let tags: Vec<&str> = registration.tags.iter().map(Arc::as_ref).collect();
    assert_eq!(tags, TAGS);
}

// NOTE: Requires the dev MongoDB container (see `ops/docker-compose.yml`).
#[test_context(StoreContext)]
#[tokio::test]
#[cfg_attr(not(feature = "storage-tests"), ignore)]
async fn test_registration_not_found(ctx: &StoreContext) {
    let client_id = format!("{TEST_CLIENT_ID}-not-found");

    let res: Result<Registration, StoreError> =
        ctx.storage.store.get_registration(client_id.as_str()).await;

    match res {
        Ok(r) => panic!("Expected error, got: {r:?}"),
        Err(StoreError::NotFound(err_reason, err_client_id)) => {
            assert_eq!(err_reason, "registration".to_string());
            assert_eq!(err_client_id, client_id);
        }
        Err(e) => panic!("Expected `StoreError::NotFound` error, got: {e:?}"),
    }
}
