//! Tests for the 9 additional VM operations: Update, List All, List By Location,
//! List Available Sizes, Generalize, Reapply, Simulate Eviction, Redeploy, Reimage.

use cloud_sdk_azure_mock::{AzureMockConfig, AzureMockServer};
use serde_json::Value;

const BEARER: &str = "Bearer mock-token";
const SUB_ID: &str = "00000000-0000-0000-0000-000000000000";
const API: &str = "api-version=2024-07-01";

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

async fn create_vm(client: &reqwest::Client, base: &str, rg: &str, name: &str, location: &str) {
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/{rg}/providers/Microsoft.Compute/virtualMachines/{name}?{API}"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "location": location,
            "properties": {
                "hardwareProfile": { "vmSize": "Standard_D2s_v3" },
                "storageProfile": { "osDisk": { "createOption": "FromImage" } },
                "networkProfile": { "networkInterfaces": [] }
            }
        }))
        .send()
        .await
        .unwrap();
}

// ── Update (PATCH) ─────────────────────────────────────────────────────

#[tokio::test]
async fn patch_update_vm_tags() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "patch-vm", "eastus").await;

    let resp = client
        .patch(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/patch-vm?{API}"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({ "tags": { "env": "staging" } }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["tags"]["env"], "staging");
    assert_eq!(body["name"], "patch-vm");
    // etag should be bumped
    assert!(body["etag"].is_string());
}

#[tokio::test]
async fn patch_update_vm_size() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "resize-vm", "eastus").await;

    let resp = client
        .patch(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/resize-vm?{API}"
        ))
        .header(auth().0, auth().1)
        .json(&serde_json::json!({
            "properties": { "hardwareProfile": { "vmSize": "Standard_D4s_v3" } }
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(
        body["properties"]["hardwareProfile"]["vmSize"],
        "Standard_D4s_v3"
    );
}

// ── List All ───────────────────────────────────────────────────────────

#[tokio::test]
async fn list_all_vms_across_resource_groups() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "vm-in-rg1", "eastus").await;
    create_vm(&client, &base, "rg2", "vm-in-rg2", "westus").await;

    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/providers/Microsoft.Compute/virtualMachines?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["value"].as_array().unwrap().len(), 2);
}

// ── List By Location ───────────────────────────────────────────────────

#[tokio::test]
async fn list_vms_by_location() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "east-vm", "eastus").await;
    create_vm(&client, &base, "rg2", "west-vm", "westus").await;

    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/providers/Microsoft.Compute/locations/eastus/virtualMachines?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let vms = body["value"].as_array().unwrap();
    assert_eq!(vms.len(), 1);
    assert_eq!(vms[0]["name"], "east-vm");
}

// ── List Available Sizes ───────────────────────────────────────────────

#[tokio::test]
async fn list_available_vm_sizes() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "size-vm", "eastus").await;

    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/size-vm/vmSizes?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let sizes = body["value"].as_array().unwrap();
    assert!(!sizes.is_empty());
    // Verify shape matches Azure's VirtualMachineSize
    let first = &sizes[0];
    assert!(first["name"].is_string());
    assert!(first["numberOfCores"].is_number());
    assert!(first["memoryInMB"].is_number());
    assert!(first["osDiskSizeInMB"].is_number());
    assert!(first["resourceDiskSizeInMB"].is_number());
    assert!(first["maxDataDiskCount"].is_number());
}

// ── Generalize ─────────────────────────────────────────────────────────

#[tokio::test]
async fn generalize_vm() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "gen-vm", "eastus").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/gen-vm/generalize?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    // Instance view should show stopped after generalize
    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/gen-vm/instanceView?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();
    let view: Value = resp.json().await.unwrap();
    let statuses = view["statuses"].as_array().unwrap();
    assert_eq!(statuses[1]["code"], "PowerState/stopped");
}

// ── Reapply ────────────────────────────────────────────────────────────

#[tokio::test]
async fn reapply_vm() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "reapply-vm", "eastus").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/reapply-vm/reapply?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn reapply_nonexistent_vm_returns_404() {
    let (client, base) = start().await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/nope/reapply?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}

// ── Simulate Eviction ──────────────────────────────────────────────────

#[tokio::test]
async fn simulate_eviction_deallocates_vm() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "spot-vm", "eastus").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/spot-vm/simulateEviction?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 204);

    // Should be deallocated
    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/spot-vm/instanceView?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();
    let view: Value = resp.json().await.unwrap();
    assert_eq!(view["statuses"][1]["code"], "PowerState/deallocated");
}

// ── Redeploy ───────────────────────────────────────────────────────────

#[tokio::test]
async fn redeploy_vm() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "redeploy-vm", "eastus").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/redeploy-vm/redeploy?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

// ── Reimage ────────────────────────────────────────────────────────────

#[tokio::test]
async fn reimage_vm() {
    let (client, base) = start().await;
    create_vm(&client, &base, "rg1", "reimage-vm", "eastus").await;

    let resp = client
        .post(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/rg1/providers/Microsoft.Compute/virtualMachines/reimage-vm/reimage?{API}"
        ))
        .header(auth().0, auth().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}
