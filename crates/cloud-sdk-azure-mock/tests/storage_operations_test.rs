//! Tests for the new storage account operations: Update, List All, Check Name,
//! List Keys, Regenerate Key, SAS, Revoke Delegation Keys.

use cloud_sdk_azure_mock::{AzureMockConfig, AzureMockServer};
use serde_json::Value;

const BEARER: &str = "Bearer mock-token";
const SUB_ID: &str = "00000000-0000-0000-0000-000000000000";
const API: &str = "api-version=2023-05-01";

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
name = "rg1"
location = "eastus"

[[subscriptions.default.resource_groups]]
name = "rg2"
location = "westus"
"#,
    )
    .unwrap()
}

async fn start() -> (reqwest::Client, String) {
    let handle = AzureMockServer::from_config(test_config())
        .start_on_random_port()
        .await
        .unwrap();
    let base = handle.url();
    let client = reqwest::Client::new();
    std::mem::forget(handle);
    (client, base)
}

fn auth() -> (&'static str, &'static str) {
    ("Authorization", BEARER)
}

async fn create_sa(client: &reqwest::Client, base: &str, rg: &str, name: &str) {
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/{rg}/providers/Microsoft.Storage/storageAccounts/{name}?{API}"
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

// ── Update (PATCH) ─────────────────────────────────────────────────────

#[tokio::test]
async fn patch_storage_account_tags() {
    let (client, base) = start().await;
    create_sa(&client, &base, "rg1", "patchsa").await;

    let resp = client
        .patch(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Storage/storageAccounts/patchsa?{API}"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({ "tags": { "env": "prod" } }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["tags"]["env"], "prod");
    assert_eq!(body["name"], "patchsa");
}

// ── List All ───────────────────────────────────────────────────────────

#[tokio::test]
async fn list_all_storage_accounts() {
    let (client, base) = start().await;
    create_sa(&client, &base, "rg1", "sa1").await;
    create_sa(&client, &base, "rg2", "sa2").await;

    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/providers/Microsoft.Storage/storageAccounts?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["value"].as_array().unwrap().len(), 2);
}

// ── Check Name Availability ────────────────────────────────────────────

#[tokio::test]
async fn check_name_available() {
    let (client, base) = start().await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/providers/Microsoft.Storage/checkNameAvailability?{API}"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "name": "newaccount",
            "type": "Microsoft.Storage/storageAccounts"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["nameAvailable"], true);
}

#[tokio::test]
async fn check_name_taken() {
    let (client, base) = start().await;
    create_sa(&client, &base, "rg1", "takenname").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/providers/Microsoft.Storage/checkNameAvailability?{API}"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "name": "takenname",
            "type": "Microsoft.Storage/storageAccounts"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["nameAvailable"], false);
    assert_eq!(body["reason"], "AlreadyExists");
}

// ── List Keys ──────────────────────────────────────────────────────────

#[tokio::test]
async fn list_storage_keys() {
    let (client, base) = start().await;
    create_sa(&client, &base, "rg1", "keysa").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Storage/storageAccounts/keysa/listKeys?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let keys = body["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 2);
    assert_eq!(keys[0]["keyName"], "key1");
    assert_eq!(keys[1]["keyName"], "key2");
    assert!(keys[0]["value"].as_str().unwrap().contains("keysa"));
    assert_eq!(keys[0]["permissions"], "FULL");
}

// ── Regenerate Key ─────────────────────────────────────────────────────

#[tokio::test]
async fn regenerate_storage_key() {
    let (client, base) = start().await;
    create_sa(&client, &base, "rg1", "regensa").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Storage/storageAccounts/regensa/regenerateKey?{API}"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({ "keyName": "key1" }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let keys = body["keys"].as_array().unwrap();
    assert!(keys[0]["value"].as_str().unwrap().starts_with("regenkey1"));
}

// ── SAS ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn list_account_sas() {
    let (client, base) = start().await;
    create_sa(&client, &base, "rg1", "sassa").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Storage/storageAccounts/sassa/ListAccountSas?{API}"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "signedServices": "bfqt",
            "signedResourceTypes": "sco",
            "signedPermission": "rwdlacup",
            "signedExpiry": "2099-01-01T00:00:00Z"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body["accountSasToken"].as_str().unwrap().contains("sassa"));
}

#[tokio::test]
async fn list_service_sas() {
    let (client, base) = start().await;
    create_sa(&client, &base, "rg1", "svcsas").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Storage/storageAccounts/svcsas/ListServiceSas?{API}"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "canonicalizedResource": "/blob/svcsas/container1",
            "signedResource": "c",
            "signedPermission": "rwdl",
            "signedExpiry": "2099-01-01T00:00:00Z"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body["serviceSasToken"].as_str().unwrap().contains("svcsas"));
}

// ── Revoke User Delegation Keys ────────────────────────────────────────

#[tokio::test]
async fn revoke_delegation_keys() {
    let (client, base) = start().await;
    create_sa(&client, &base, "rg1", "revokesa").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Storage/storageAccounts/revokesa/revokeUserDelegationKeys?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}
