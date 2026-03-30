use cloud_sdk_core::models::resource::CreateResourceGroupParams;
use cloud_sdk_core::services::ResourceManagerService;
use cloud_sdk_core::services::storage::{
    AccountSasParameters, CreateStorageAccountParams, ServiceSasParameters, StorageService,
    StorageSku,
};
use cloud_sdk_test::TestHarness;
use std::collections::HashMap;

fn storage_params() -> CreateStorageAccountParams {
    CreateStorageAccountParams {
        location: "eastus".to_string(),
        kind: "StorageV2".to_string(),
        sku: StorageSku {
            name: "Standard_LRS".to_string(),
            tier: Some("Standard".to_string()),
        },
        tags: HashMap::new(),
        properties: None,
        identity: None,
        extended_location: None,
    }
}

async fn setup() -> TestHarness {
    let harness = TestHarness::start().await.unwrap();
    harness
        .provider()
        .resource_manager()
        .create_resource_group(
            "test-rg",
            CreateResourceGroupParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
            },
        )
        .await
        .unwrap();
    harness
}

async fn setup_with_account() -> TestHarness {
    let harness = setup().await;
    harness
        .provider()
        .storage()
        .create_storage_account("test-rg", "testacct", storage_params())
        .await
        .unwrap();
    harness
}

// ── Management plane (extended) ───────────────────────────────────────

#[tokio::test]
async fn update_storage_account() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    let patch = serde_json::json!({
        "tags": { "env": "staging", "team": "platform" }
    });
    let updated = storage
        .update_storage_account("test-rg", "testacct", patch)
        .await
        .unwrap();

    assert_eq!(updated.name, "testacct");
    assert_eq!(updated.tags.get("env").map(|s| s.as_str()), Some("staging"));
    assert_eq!(
        updated.tags.get("team").map(|s| s.as_str()),
        Some("platform")
    );
}

#[tokio::test]
async fn list_all_storage_accounts() {
    let harness = setup().await;
    let storage = harness.provider().storage();

    // Create accounts in the same resource group
    storage
        .create_storage_account("test-rg", "sa1", storage_params())
        .await
        .unwrap();
    storage
        .create_storage_account("test-rg", "sa2", storage_params())
        .await
        .unwrap();

    let page = storage.list_all_storage_accounts().await.unwrap();
    assert_eq!(page.value.len(), 2);

    let names: Vec<&str> = page.value.iter().map(|sa| sa.name.as_str()).collect();
    assert!(names.contains(&"sa1"));
    assert!(names.contains(&"sa2"));
}

#[tokio::test]
async fn check_name_availability_available() {
    let harness = setup().await;
    let storage = harness.provider().storage();

    let result = storage
        .check_name_availability("brandnewname")
        .await
        .unwrap();
    assert_eq!(result.name_available, Some(true));
}

#[tokio::test]
async fn check_name_availability_taken() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    let result = storage.check_name_availability("testacct").await.unwrap();
    assert_eq!(result.name_available, Some(false));
    assert!(result.reason.is_some());
}

#[tokio::test]
async fn list_keys() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    let result = storage.list_keys("test-rg", "testacct").await.unwrap();
    let keys = result.keys.unwrap();
    assert_eq!(keys.len(), 2);
    assert!(keys[0].key_name.is_some());
    assert!(keys[0].value.is_some());
}

#[tokio::test]
async fn regenerate_key() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    let result = storage
        .regenerate_key("test-rg", "testacct", "key1")
        .await
        .unwrap();

    let keys = result.keys.unwrap();
    assert_eq!(keys.len(), 2);
    // The regenerated key1 should have its value changed
    let key1 = keys
        .iter()
        .find(|k| k.key_name.as_deref() == Some("key1"))
        .unwrap();
    assert!(key1.value.as_ref().unwrap().starts_with("regen"));
}

#[tokio::test]
async fn list_account_sas() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    let params = AccountSasParameters {
        signed_services: "bfqt".to_string(),
        signed_resource_types: "sco".to_string(),
        signed_permission: "rwdlacup".to_string(),
        signed_expiry: "2099-01-01T00:00:00Z".to_string(),
        signed_start: None,
        signed_ip: None,
        signed_protocol: None,
        key_to_sign: None,
    };

    let result = storage
        .list_account_sas("test-rg", "testacct", params)
        .await
        .unwrap();
    assert!(result.account_sas_token.is_some());
    assert!(
        result
            .account_sas_token
            .unwrap()
            .contains("mock-sas-testacct")
    );
}

#[tokio::test]
async fn list_service_sas() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    let params = ServiceSasParameters {
        canonicalized_resource: "/blob/testacct/mycontainer".to_string(),
        signed_resource: "c".to_string(),
        signed_permission: "rwdl".to_string(),
        signed_expiry: "2099-01-01T00:00:00Z".to_string(),
        signed_start: None,
        signed_ip: None,
        signed_protocol: None,
        key_to_sign: None,
    };

    let result = storage
        .list_service_sas("test-rg", "testacct", params)
        .await
        .unwrap();
    assert!(result.service_sas_token.is_some());
    assert!(
        result
            .service_sas_token
            .unwrap()
            .contains("mock-service-sas-testacct")
    );
}

#[tokio::test]
async fn revoke_user_delegation_keys() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    // Should succeed without error
    storage
        .revoke_user_delegation_keys("test-rg", "testacct")
        .await
        .unwrap();
}

// ── Data plane (extended) ─────────────────────────────────────────────

async fn setup_with_blob() -> TestHarness {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    storage.create_container("testacct", "data").await.unwrap();
    storage
        .put_blob(
            "testacct",
            "data",
            "file.txt",
            bytes::Bytes::from("hello world"),
            Some("text/plain"),
        )
        .await
        .unwrap();
    harness
}

#[tokio::test]
async fn set_and_get_blob_metadata() {
    let harness = setup_with_blob().await;
    let storage = harness.provider().storage();

    let mut metadata = HashMap::new();
    metadata.insert("author".to_string(), "alice".to_string());
    metadata.insert("version".to_string(), "3".to_string());

    storage
        .set_blob_metadata("testacct", "data", "file.txt", metadata.clone())
        .await
        .unwrap();

    let fetched = storage
        .get_blob_metadata("testacct", "data", "file.txt")
        .await
        .unwrap();

    assert_eq!(fetched.get("author").map(|s| s.as_str()), Some("alice"));
    assert_eq!(fetched.get("version").map(|s| s.as_str()), Some("3"));
}

#[tokio::test]
async fn set_and_get_blob_tags() {
    let harness = setup_with_blob().await;
    let storage = harness.provider().storage();

    let mut tags = HashMap::new();
    tags.insert("project".to_string(), "cloud-sdk".to_string());
    tags.insert("priority".to_string(), "high".to_string());

    storage
        .set_blob_tags("testacct", "data", "file.txt", tags.clone())
        .await
        .unwrap();

    let fetched = storage
        .get_blob_tags("testacct", "data", "file.txt")
        .await
        .unwrap();

    assert_eq!(
        fetched.get("project").map(|s| s.as_str()),
        Some("cloud-sdk")
    );
    assert_eq!(fetched.get("priority").map(|s| s.as_str()), Some("high"));
}

#[tokio::test]
async fn copy_blob() {
    let harness = setup_with_blob().await;
    let storage = harness.provider().storage();

    // Create destination container
    storage
        .create_container("testacct", "backup")
        .await
        .unwrap();

    // Build source URL
    let source_url = format!("{}/testacct/data/file.txt", harness.base_url());

    let copy_id = storage
        .copy_blob("testacct", "backup", "file-copy.txt", &source_url)
        .await
        .unwrap();

    assert!(!copy_id.is_empty());

    // Verify copy exists
    let data = storage
        .get_blob("testacct", "backup", "file-copy.txt")
        .await
        .unwrap();
    assert_eq!(data.as_ref(), b"hello world");
}

#[tokio::test]
async fn set_blob_tier() {
    let harness = setup_with_blob().await;
    let storage = harness.provider().storage();

    storage
        .set_blob_tier("testacct", "data", "file.txt", "Cool")
        .await
        .unwrap();

    // Verify via blob properties
    let props = storage
        .get_blob_properties("testacct", "data", "file.txt")
        .await
        .unwrap();
    assert_eq!(props.access_tier.as_deref(), Some("Cool"));
}

#[tokio::test]
async fn set_container_metadata() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    storage
        .create_container("testacct", "mycontainer")
        .await
        .unwrap();

    let mut metadata = HashMap::new();
    metadata.insert("department".to_string(), "engineering".to_string());

    storage
        .set_container_metadata("testacct", "mycontainer", metadata)
        .await
        .unwrap();

    // Verify by listing containers and checking metadata
    let containers = storage.list_containers("testacct").await.unwrap();
    let container = containers.iter().find(|c| c.name == "mycontainer").unwrap();
    assert_eq!(
        container.metadata.get("department").map(|s| s.as_str()),
        Some("engineering")
    );
}
