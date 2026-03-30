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

// ── Container extended operations ─────────────────────────────────────

#[tokio::test]
async fn container_properties_head() {
    let (client, base) = setup_with_storage_account().await;

    // Create container
    client
        .put(format!("{base}/blobacct/propcontainer"))
        .send()
        .await
        .unwrap();

    // HEAD → get properties
    let resp = client
        .head(format!("{base}/blobacct/propcontainer"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["x-ms-lease-status"], "unlocked");
    assert_eq!(resp.headers()["x-ms-lease-state"], "available");
    assert_eq!(resp.headers()["x-ms-has-immutability-policy"], "false");
    assert_eq!(resp.headers()["x-ms-has-legal-hold"], "false");
    assert!(resp.headers().contains_key("etag"));
    assert!(resp.headers().contains_key("last-modified"));
}

#[tokio::test]
async fn container_metadata_set_get() {
    let (client, base) = setup_with_storage_account().await;

    // Create container
    client
        .put(format!("{base}/blobacct/metacontainer"))
        .send()
        .await
        .unwrap();

    // Set metadata via PUT with comp=metadata
    let resp = client
        .put(format!("{base}/blobacct/metacontainer?comp=metadata"))
        .header("x-ms-meta-project", "cloud-sdk")
        .header("x-ms-meta-env", "test")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Get metadata via GET with comp=metadata
    let resp = client
        .get(format!("{base}/blobacct/metacontainer?comp=metadata"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["x-ms-meta-project"], "cloud-sdk");
    assert_eq!(resp.headers()["x-ms-meta-env"], "test");
}

#[tokio::test]
async fn container_head_not_found() {
    let (client, base) = setup_with_storage_account().await;

    let resp = client
        .head(format!("{base}/blobacct/nonexistent"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

// ── Blob metadata/properties/tags ─────────────────────────────────────

#[tokio::test]
async fn blob_metadata_set_get() {
    let (client, base) = setup_with_storage_account().await;

    // Setup: container + blob
    client
        .put(format!("{base}/blobacct/metafiles"))
        .send()
        .await
        .unwrap();
    client
        .put(format!("{base}/blobacct/metafiles/doc.txt"))
        .header("content-type", "text/plain")
        .body("hello")
        .send()
        .await
        .unwrap();

    // Set blob metadata
    let resp = client
        .put(format!("{base}/blobacct/metafiles/doc.txt?comp=metadata"))
        .header("x-ms-meta-author", "alice")
        .header("x-ms-meta-version", "1")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Get blob metadata
    let resp = client
        .get(format!("{base}/blobacct/metafiles/doc.txt?comp=metadata"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["x-ms-meta-author"], "alice");
    assert_eq!(resp.headers()["x-ms-meta-version"], "1");
}

#[tokio::test]
async fn blob_tags_set_get() {
    let (client, base) = setup_with_storage_account().await;

    // Setup
    client
        .put(format!("{base}/blobacct/tagfiles"))
        .send()
        .await
        .unwrap();
    client
        .put(format!("{base}/blobacct/tagfiles/data.bin"))
        .body("binary data")
        .send()
        .await
        .unwrap();

    // Set tags (BlobTags format)
    let resp = client
        .put(format!("{base}/blobacct/tagfiles/data.bin?comp=tags"))
        .json(&serde_json::json!({
            "blobTagSet": [
                { "key": "department", "value": "engineering" },
                { "key": "tier", "value": "hot" }
            ]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 204);

    // Get tags
    let resp = client
        .get(format!("{base}/blobacct/tagfiles/data.bin?comp=tags"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let tag_set = body["blobTagSet"].as_array().unwrap();
    assert_eq!(tag_set.len(), 2);

    // Verify tag values are present (order may vary)
    let tags: Vec<(&str, &str)> = tag_set
        .iter()
        .map(|t| (t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
        .collect();
    assert!(tags.contains(&("department", "engineering")));
    assert!(tags.contains(&("tier", "hot")));
}

#[tokio::test]
async fn blob_properties_set_via_comp() {
    let (client, base) = setup_with_storage_account().await;

    // Setup
    client
        .put(format!("{base}/blobacct/propsfiles"))
        .send()
        .await
        .unwrap();
    client
        .put(format!("{base}/blobacct/propsfiles/report.pdf"))
        .header("content-type", "application/octet-stream")
        .body("fake pdf")
        .send()
        .await
        .unwrap();

    // Set properties via comp=properties
    let resp = client
        .put(format!(
            "{base}/blobacct/propsfiles/report.pdf?comp=properties"
        ))
        .header("x-ms-blob-content-type", "application/pdf")
        .header("x-ms-blob-cache-control", "max-age=3600")
        .header("x-ms-blob-content-disposition", "attachment")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify via HEAD
    let resp = client
        .head(format!("{base}/blobacct/propsfiles/report.pdf"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "application/pdf");
    assert_eq!(resp.headers()["x-ms-blob-cache-control"], "max-age=3600");
    assert_eq!(
        resp.headers()["x-ms-blob-content-disposition"],
        "attachment"
    );
}

#[tokio::test]
async fn blob_head_enriched_properties() {
    let (client, base) = setup_with_storage_account().await;

    // Setup
    client
        .put(format!("{base}/blobacct/richprops"))
        .send()
        .await
        .unwrap();
    client
        .put(format!("{base}/blobacct/richprops/file.dat"))
        .header("content-type", "text/plain")
        .body("some content")
        .send()
        .await
        .unwrap();

    // HEAD should return enriched properties
    let resp = client
        .head(format!("{base}/blobacct/richprops/file.dat"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "text/plain");
    assert_eq!(resp.headers()["content-length"], "12");
    assert_eq!(resp.headers()["x-ms-blob-type"], "BlockBlob");
    assert_eq!(resp.headers()["x-ms-access-tier"], "Hot");
    assert_eq!(resp.headers()["x-ms-lease-status"], "unlocked");
    assert_eq!(resp.headers()["x-ms-lease-state"], "available");
    assert_eq!(resp.headers()["x-ms-server-encrypted"], "true");
    assert!(resp.headers().contains_key("etag"));
    assert!(resp.headers().contains_key("last-modified"));
    assert!(resp.headers().contains_key("x-ms-creation-time"));
}

#[tokio::test]
async fn blob_metadata_not_found() {
    let (client, base) = setup_with_storage_account().await;

    client
        .put(format!("{base}/blobacct/emptycontainer"))
        .send()
        .await
        .unwrap();

    let resp = client
        .get(format!(
            "{base}/blobacct/emptycontainer/missing.txt?comp=metadata"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn blob_tags_not_found() {
    let (client, base) = setup_with_storage_account().await;

    client
        .put(format!("{base}/blobacct/emptycontainer2"))
        .send()
        .await
        .unwrap();

    let resp = client
        .get(format!(
            "{base}/blobacct/emptycontainer2/missing.txt?comp=tags"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn container_list_shows_enriched_fields() {
    let (client, base) = setup_with_storage_account().await;

    // Create container
    client
        .put(format!("{base}/blobacct/enriched"))
        .send()
        .await
        .unwrap();

    // List containers should show enriched fields
    let resp = client.get(format!("{base}/blobacct")).send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let containers = body["containers"].as_array().unwrap();
    assert_eq!(containers.len(), 1);
    let c = &containers[0];
    assert_eq!(c["name"], "enriched");
    assert!(c["etag"].is_string());
    assert_eq!(c["leaseStatus"], "unlocked");
    assert_eq!(c["leaseState"], "available");
    assert_eq!(c["hasImmutabilityPolicy"], false);
    assert_eq!(c["hasLegalHold"], false);
}

// ── Copy Blob ─────────────────────────────────────────────────────────

#[tokio::test]
async fn copy_blob_success() {
    let (client, base) = setup_with_storage_account().await;

    // Create source container + blob
    client
        .put(format!("{base}/blobacct/source"))
        .send()
        .await
        .unwrap();
    client
        .put(format!("{base}/blobacct/source/original.txt"))
        .header("content-type", "text/plain")
        .body("copy me please")
        .send()
        .await
        .unwrap();

    // Create destination container
    client
        .put(format!("{base}/blobacct/dest"))
        .send()
        .await
        .unwrap();

    // Copy blob via x-ms-copy-source header
    let resp = client
        .put(format!("{base}/blobacct/dest/copied.txt"))
        .header(
            "x-ms-copy-source",
            format!("{base}/blobacct/source/original.txt"),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 202);
    assert!(resp.headers().contains_key("x-ms-copy-id"));
    assert_eq!(resp.headers()["x-ms-copy-status"], "success");

    // Verify copied blob has the same content
    let resp = client
        .get(format!("{base}/blobacct/dest/copied.txt"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "text/plain");
    let body = resp.text().await.unwrap();
    assert_eq!(body, "copy me please");

    // Verify copy properties on the destination blob
    let resp = client
        .head(format!("{base}/blobacct/dest/copied.txt"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-length"], "14");
}

#[tokio::test]
async fn copy_blob_source_not_found() {
    let (client, base) = setup_with_storage_account().await;

    // Create destination container only
    client
        .put(format!("{base}/blobacct/destonly"))
        .send()
        .await
        .unwrap();

    let resp = client
        .put(format!("{base}/blobacct/destonly/missing.txt"))
        .header(
            "x-ms-copy-source",
            format!("{base}/blobacct/nosuchcontainer/nofile.txt"),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

// ── Snapshot Blob ─────────────────────────────────────────────────────

#[tokio::test]
async fn snapshot_blob_success() {
    let (client, base) = setup_with_storage_account().await;

    // Create container + blob
    client
        .put(format!("{base}/blobacct/snapcontainer"))
        .send()
        .await
        .unwrap();
    client
        .put(format!("{base}/blobacct/snapcontainer/snap.txt"))
        .body("snapshot data")
        .send()
        .await
        .unwrap();

    // Snapshot
    let resp = client
        .put(format!(
            "{base}/blobacct/snapcontainer/snap.txt?comp=snapshot"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);
    let snapshot = resp.headers()["x-ms-snapshot"].to_str().unwrap();
    assert!(!snapshot.is_empty());
    // Should look like a datetime: starts with 20xx
    assert!(snapshot.starts_with("20"));
}

#[tokio::test]
async fn snapshot_blob_not_found() {
    let (client, base) = setup_with_storage_account().await;

    client
        .put(format!("{base}/blobacct/snapempty"))
        .send()
        .await
        .unwrap();

    let resp = client
        .put(format!(
            "{base}/blobacct/snapempty/missing.txt?comp=snapshot"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

// ── Service Properties (Get/Set) ──────────────────────────────────────

#[tokio::test]
async fn get_service_properties_default() {
    let (client, base) = setup_with_storage_account().await;

    let resp = client
        .get(format!("{base}/blobacct?restype=service&comp=properties"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["DefaultServiceVersion"], "2023-11-03");
}

#[tokio::test]
async fn set_and_get_service_properties() {
    let (client, base) = setup_with_storage_account().await;

    // Set service properties
    let resp = client
        .put(format!("{base}/blobacct?restype=service&comp=properties"))
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "DefaultServiceVersion": "2024-01-01",
            "Cors": null
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 202);

    // Get and verify
    let resp = client
        .get(format!("{base}/blobacct?restype=service&comp=properties"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["DefaultServiceVersion"], "2024-01-01");
}

// ── Account Information ───────────────────────────────────────────────

#[tokio::test]
async fn get_account_information() {
    let (client, base) = setup_with_storage_account().await;

    let resp = client
        .get(format!("{base}/blobacct?restype=account&comp=properties"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["x-ms-sku-name"], "Standard_LRS");
    assert_eq!(resp.headers()["x-ms-account-kind"], "StorageV2");
    assert_eq!(resp.headers()["x-ms-is-hns-enabled"], "false");
}

// ── Account-level PUT with bad params ─────────────────────────────────

#[tokio::test]
async fn put_account_unsupported_operation() {
    let (client, base) = setup_with_storage_account().await;

    let resp = client
        .put(format!("{base}/blobacct?comp=badop"))
        .body("nonsense")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

// ── Block Blob Operations ────────────────────────────────────────────

#[tokio::test]
async fn block_blob_multipart_upload() {
    let (client, base) = setup_with_storage_account().await;

    // Create container
    client
        .put(format!("{base}/blobacct/blockcontainer"))
        .send()
        .await
        .unwrap();

    // Put 3 blocks
    for (id, data) in [("block1", "Hello, "), ("block2", "world"), ("block3", "!")] {
        let resp = client
            .put(format!(
                "{base}/blobacct/blockcontainer/assembled.txt?comp=block&blockid={id}"
            ))
            .body(data)
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), 201, "put_block should return 201");
    }

    // Commit blocks via put block list
    let resp = client
        .put(format!(
            "{base}/blobacct/blockcontainer/assembled.txt?comp=blocklist"
        ))
        .json(&serde_json::json!({
            "blockIds": ["block1", "block2", "block3"]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201, "put_block_list should return 201");
    assert!(resp.headers().contains_key("etag"));

    // Get blob — should be the concatenation of all blocks
    let resp = client
        .get(format!("{base}/blobacct/blockcontainer/assembled.txt"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.text().await.unwrap();
    assert_eq!(body, "Hello, world!");
}

#[tokio::test]
async fn block_list_uncommitted_then_committed() {
    let (client, base) = setup_with_storage_account().await;

    // Create container
    client
        .put(format!("{base}/blobacct/blocklist"))
        .send()
        .await
        .unwrap();

    // Put 2 blocks (uncommitted)
    for (id, data) in [("blk_a", "aaa"), ("blk_b", "bbb")] {
        client
            .put(format!(
                "{base}/blobacct/blocklist/parts.bin?comp=block&blockid={id}"
            ))
            .body(data)
            .send()
            .await
            .unwrap();
    }

    // Get block list — should show uncommitted blocks
    let resp = client
        .get(format!(
            "{base}/blobacct/blocklist/parts.bin?comp=blocklist"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let uncommitted = body["uncommittedBlocks"].as_array().unwrap();
    assert_eq!(uncommitted.len(), 2);
    let committed = body["committedBlocks"].as_array().unwrap();
    assert_eq!(committed.len(), 0);

    // Commit the blocks
    client
        .put(format!(
            "{base}/blobacct/blocklist/parts.bin?comp=blocklist"
        ))
        .json(&serde_json::json!({
            "blockIds": ["blk_a", "blk_b"]
        }))
        .send()
        .await
        .unwrap();

    // Get block list again — should now show committed, no uncommitted
    let resp = client
        .get(format!(
            "{base}/blobacct/blocklist/parts.bin?comp=blocklist"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let uncommitted = body["uncommittedBlocks"].as_array().unwrap();
    assert_eq!(uncommitted.len(), 0);
    let committed = body["committedBlocks"].as_array().unwrap();
    assert_eq!(committed.len(), 2);

    // Verify committed block sizes
    let names: Vec<&str> = committed
        .iter()
        .map(|b| b["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"blk_a"));
    assert!(names.contains(&"blk_b"));
    for block in committed {
        assert_eq!(block["size"], 3);
    }
}

#[tokio::test]
async fn set_blob_tier() {
    let (client, base) = setup_with_storage_account().await;

    // Create container + blob
    client
        .put(format!("{base}/blobacct/tiercontainer"))
        .send()
        .await
        .unwrap();
    client
        .put(format!("{base}/blobacct/tiercontainer/data.bin"))
        .body("some data")
        .send()
        .await
        .unwrap();

    // Verify initial tier is Hot
    let resp = client
        .head(format!("{base}/blobacct/tiercontainer/data.bin"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.headers()["x-ms-access-tier"], "Hot");

    // Set tier to Cool
    let resp = client
        .put(format!("{base}/blobacct/tiercontainer/data.bin?comp=tier"))
        .header("x-ms-access-tier", "Cool")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Verify tier changed
    let resp = client
        .head(format!("{base}/blobacct/tiercontainer/data.bin"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.headers()["x-ms-access-tier"], "Cool");

    // Set tier to Archive
    let resp = client
        .put(format!("{base}/blobacct/tiercontainer/data.bin?comp=tier"))
        .header("x-ms-access-tier", "Archive")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let resp = client
        .head(format!("{base}/blobacct/tiercontainer/data.bin"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.headers()["x-ms-access-tier"], "Archive");
}

#[tokio::test]
async fn put_block_auto_creates_blob() {
    let (client, base) = setup_with_storage_account().await;

    // Create container but NOT the blob
    client
        .put(format!("{base}/blobacct/autocontainer"))
        .send()
        .await
        .unwrap();

    // Put a block on a non-existent blob — should auto-create
    let resp = client
        .put(format!(
            "{base}/blobacct/autocontainer/newblob.txt?comp=block&blockid=first"
        ))
        .body("first block data")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);

    // Block list should show the uncommitted block
    let resp = client
        .get(format!(
            "{base}/blobacct/autocontainer/newblob.txt?comp=blocklist"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["uncommittedBlocks"].as_array().unwrap().len(), 1);
    assert_eq!(body["uncommittedBlocks"][0]["name"], "first");
    assert_eq!(body["uncommittedBlocks"][0]["size"], 16);

    // Commit it
    let resp = client
        .put(format!(
            "{base}/blobacct/autocontainer/newblob.txt?comp=blocklist"
        ))
        .json(&serde_json::json!({ "blockIds": ["first"] }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);

    // Now get the blob data
    let resp = client
        .get(format!("{base}/blobacct/autocontainer/newblob.txt"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.text().await.unwrap(), "first block data");
}

#[tokio::test]
async fn set_blob_tier_missing_header_returns_400() {
    let (client, base) = setup_with_storage_account().await;

    // Create container + blob
    client
        .put(format!("{base}/blobacct/tiercontainer2"))
        .send()
        .await
        .unwrap();
    client
        .put(format!("{base}/blobacct/tiercontainer2/data.bin"))
        .body("data")
        .send()
        .await
        .unwrap();

    // Set tier without x-ms-access-tier header → 400
    let resp = client
        .put(format!("{base}/blobacct/tiercontainer2/data.bin?comp=tier"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn put_block_missing_blockid_returns_400() {
    let (client, base) = setup_with_storage_account().await;

    client
        .put(format!("{base}/blobacct/blockcontainer2"))
        .send()
        .await
        .unwrap();

    // Put block without blockid query param → 400
    let resp = client
        .put(format!(
            "{base}/blobacct/blockcontainer2/test.txt?comp=block"
        ))
        .body("data")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}
