use cloud_sdk_azure_mock::{AzureMockConfig, AzureMockServer};
use serde_json::Value;

const BEARER: &str = "Bearer mock-token";
const SUB_ID: &str = "00000000-0000-0000-0000-000000000000";

fn test_config() -> AzureMockConfig {
    AzureMockConfig::from_toml(
        r#"
[server]
port = 0

[subscriptions.default]
id = "00000000-0000-0000-0000-000000000000"
display_name = "Mock Subscription"
tenant_id = "00000000-0000-0000-0000-000000000001"
state = "Enabled"

[[subscriptions.default.resource_groups]]
name = "test-rg"
location = "eastus"
"#,
    )
    .unwrap()
}

async fn start_server() -> (reqwest::Client, String) {
    let handle = AzureMockServer::from_config(test_config())
        .start_on_random_port()
        .await
        .unwrap();
    let base = handle.url();
    let client = reqwest::Client::new();
    std::mem::forget(handle);
    (client, base)
}

fn auth() -> (reqwest::header::HeaderName, reqwest::header::HeaderValue) {
    (
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_static(BEARER),
    )
}

// ── Storage Account ARM tests ──────────────────────────────────────────

#[tokio::test]
async fn create_storage_account() {
    let (client, base) = start_server().await;

    let resp = client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg/providers/Microsoft.Storage/storageAccounts/mystorage?api-version=2023-05-01"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "location": "eastus",
            "kind": "StorageV2",
            "sku": { "name": "Standard_LRS", "tier": "Standard" }
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 201);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["name"], "mystorage");
    assert_eq!(body["kind"], "StorageV2");
    assert_eq!(body["sku"]["name"], "Standard_LRS");
    assert_eq!(body["type"], "Microsoft.Storage/storageAccounts");
    assert_eq!(body["properties"]["provisioningState"], "Succeeded");
}

#[tokio::test]
async fn get_storage_account() {
    let (client, base) = start_server().await;

    // Create
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg/providers/Microsoft.Storage/storageAccounts/gettest?api-version=2023-05-01"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "location": "eastus",
            "kind": "StorageV2",
            "sku": { "name": "Standard_LRS" }
        }))
        .send()
        .await
        .unwrap();

    // Get
    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg/providers/Microsoft.Storage/storageAccounts/gettest?api-version=2023-05-01"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["name"], "gettest");
}

#[tokio::test]
async fn list_storage_accounts() {
    let (client, base) = start_server().await;

    for name in &["sa1", "sa2"] {
        client
            .put(format!(
                "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg/providers/Microsoft.Storage/storageAccounts/{name}?api-version=2023-05-01"
            ))
            .header(auth().0, auth().1)
            .json(&serde_json::json!({
                "location": "eastus",
                "kind": "StorageV2",
                "sku": { "name": "Standard_LRS" }
            }))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg/providers/Microsoft.Storage/storageAccounts?api-version=2023-05-01"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["value"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn delete_storage_account() {
    let (client, base) = start_server().await;

    // Create
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg/providers/Microsoft.Storage/storageAccounts/delsa?api-version=2023-05-01"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "location": "eastus",
            "kind": "StorageV2",
            "sku": { "name": "Standard_LRS" }
        }))
        .send()
        .await
        .unwrap();

    // Delete
    let resp = client
        .delete(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg/providers/Microsoft.Storage/storageAccounts/delsa?api-version=2023-05-01"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify gone
    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg/providers/Microsoft.Storage/storageAccounts/delsa?api-version=2023-05-01"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

// ── Blob data plane tests ──────────────────────────────────────────────

/// Helper: create a storage account via ARM, then return the client + base.
async fn setup_with_storage_account() -> (reqwest::Client, String) {
    let (client, base) = start_server().await;

    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg/providers/Microsoft.Storage/storageAccounts/blobacct?api-version=2023-05-01"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "location": "eastus",
            "kind": "StorageV2",
            "sku": { "name": "Standard_LRS" }
        }))
        .send()
        .await
        .unwrap();

    (client, base)
}

#[tokio::test]
async fn create_and_list_containers() {
    let (client, base) = setup_with_storage_account().await;

    // Create container
    let resp = client
        .put(format!("{base}/blobacct/mycontainer"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);

    // List containers
    let resp = client.get(format!("{base}/blobacct")).send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let containers = body["containers"].as_array().unwrap();
    assert_eq!(containers.len(), 1);
    assert_eq!(containers[0]["name"], "mycontainer");
}

#[tokio::test]
async fn blob_put_get_delete() {
    let (client, base) = setup_with_storage_account().await;

    // Create container
    client
        .put(format!("{base}/blobacct/data"))
        .send()
        .await
        .unwrap();

    // Put blob
    let resp = client
        .put(format!("{base}/blobacct/data/hello.txt"))
        .header("content-type", "text/plain")
        .body("hello world")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);

    // Get blob
    let resp = client
        .get(format!("{base}/blobacct/data/hello.txt"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "text/plain");
    let body = resp.text().await.unwrap();
    assert_eq!(body, "hello world");

    // Head blob (properties)
    let resp = client
        .head(format!("{base}/blobacct/data/hello.txt"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-length"], "11");

    // Delete blob
    let resp = client
        .delete(format!("{base}/blobacct/data/hello.txt"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 202);

    // Verify gone
    let resp = client
        .get(format!("{base}/blobacct/data/hello.txt"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn list_blobs() {
    let (client, base) = setup_with_storage_account().await;

    // Create container + blobs
    client
        .put(format!("{base}/blobacct/files"))
        .send()
        .await
        .unwrap();

    for name in &["a.txt", "b.txt", "c.txt"] {
        client
            .put(format!("{base}/blobacct/files/{name}"))
            .body("data")
            .send()
            .await
            .unwrap();
    }

    // List
    let resp = client
        .get(format!("{base}/blobacct/files"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["blobs"].as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn delete_container() {
    let (client, base) = setup_with_storage_account().await;

    client
        .put(format!("{base}/blobacct/temp"))
        .send()
        .await
        .unwrap();

    let resp = client
        .delete(format!("{base}/blobacct/temp"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 202);

    // List should be empty
    let resp = client.get(format!("{base}/blobacct")).send().await.unwrap();
    let body: Value = resp.json().await.unwrap();
    assert!(body["containers"].as_array().unwrap().is_empty());
}
