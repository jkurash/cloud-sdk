use cloud_sdk_core::models::resource::CreateResourceGroupParams;
use cloud_sdk_core::services::ResourceManagerService;
use cloud_sdk_core::services::storage::{CreateStorageAccountParams, StorageService, StorageSku};
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
    // Create a resource group first
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

// ── Storage Account ARM tests ──────────────────────────────────────────

#[tokio::test]
async fn create_and_get_storage_account() {
    let harness = setup().await;
    let storage = harness.provider().storage();

    let sa = storage
        .create_storage_account("test-rg", "mystorage", storage_params())
        .await
        .unwrap();

    assert_eq!(sa.name, "mystorage");
    assert_eq!(sa.kind, "StorageV2");
    assert_eq!(sa.sku.name, "Standard_LRS");

    let fetched = storage
        .get_storage_account("test-rg", "mystorage")
        .await
        .unwrap();
    assert_eq!(fetched.name, "mystorage");
}

#[tokio::test]
async fn list_storage_accounts() {
    let harness = setup().await;
    let storage = harness.provider().storage();

    storage
        .create_storage_account("test-rg", "sa1", storage_params())
        .await
        .unwrap();
    storage
        .create_storage_account("test-rg", "sa2", storage_params())
        .await
        .unwrap();

    let page = storage.list_storage_accounts("test-rg").await.unwrap();
    assert_eq!(page.value.len(), 2);
}

#[tokio::test]
async fn delete_storage_account() {
    let harness = setup().await;
    let storage = harness.provider().storage();

    storage
        .create_storage_account("test-rg", "delsa", storage_params())
        .await
        .unwrap();

    storage
        .delete_storage_account("test-rg", "delsa")
        .await
        .unwrap();

    let result = storage.get_storage_account("test-rg", "delsa").await;
    assert!(result.is_err());
}

// ── Blob data plane tests ──────────────────────────────────────────────

async fn setup_with_account() -> TestHarness {
    let harness = setup().await;
    harness
        .provider()
        .storage()
        .create_storage_account("test-rg", "blobacct", storage_params())
        .await
        .unwrap();
    harness
}

#[tokio::test]
async fn container_lifecycle() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    // Create container
    storage
        .create_container("blobacct", "mycontainer")
        .await
        .unwrap();

    // List
    let containers = storage.list_containers("blobacct").await.unwrap();
    assert_eq!(containers.len(), 1);
    assert_eq!(containers[0].name, "mycontainer");

    // Delete
    storage
        .delete_container("blobacct", "mycontainer")
        .await
        .unwrap();

    let containers = storage.list_containers("blobacct").await.unwrap();
    assert!(containers.is_empty());
}

#[tokio::test]
async fn blob_put_get_delete() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    storage.create_container("blobacct", "data").await.unwrap();

    // Put
    let data = bytes::Bytes::from("hello from the SDK");
    storage
        .put_blob(
            "blobacct",
            "data",
            "greeting.txt",
            data.clone(),
            Some("text/plain"),
        )
        .await
        .unwrap();

    // Get
    let fetched = storage
        .get_blob("blobacct", "data", "greeting.txt")
        .await
        .unwrap();
    assert_eq!(fetched, data);

    // List
    let blobs = storage.list_blobs("blobacct", "data").await.unwrap();
    assert_eq!(blobs.len(), 1);
    assert_eq!(blobs[0].name, "greeting.txt");
    assert_eq!(blobs[0].content_length, 18);

    // Delete
    storage
        .delete_blob("blobacct", "data", "greeting.txt")
        .await
        .unwrap();

    let blobs = storage.list_blobs("blobacct", "data").await.unwrap();
    assert!(blobs.is_empty());
}

#[tokio::test]
async fn blob_properties() {
    let harness = setup_with_account().await;
    let storage = harness.provider().storage();

    storage.create_container("blobacct", "props").await.unwrap();

    storage
        .put_blob(
            "blobacct",
            "props",
            "doc.pdf",
            bytes::Bytes::from(vec![0u8; 1024]),
            Some("application/pdf"),
        )
        .await
        .unwrap();

    let props = storage
        .get_blob_properties("blobacct", "props", "doc.pdf")
        .await
        .unwrap();

    assert_eq!(props.name, "doc.pdf");
    assert_eq!(props.content_length, 1024);
    assert_eq!(props.content_type.as_deref(), Some("application/pdf"));
}
