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
    // Leak the handle so the server stays alive for the test
    std::mem::forget(handle);
    (client, base)
}

fn auth_header() -> (reqwest::header::HeaderName, reqwest::header::HeaderValue) {
    (
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_static(BEARER),
    )
}

// ── Middleware tests ────────────────────────────────────────────────────

#[tokio::test]
async fn missing_api_version_returns_400() {
    let (client, base) = start_server().await;

    let resp = client
        .get(format!("{base}/subscriptions"))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 400);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"], "MissingApiVersionParameter");
}

#[tokio::test]
async fn missing_auth_returns_401() {
    let (client, base) = start_server().await;

    let resp = client
        .get(format!("{base}/subscriptions?api-version=2022-12-01"))
        // No Authorization header
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 401);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"], "AuthenticationFailed");
}

#[tokio::test]
async fn response_has_azure_headers() {
    let (client, base) = start_server().await;

    let resp = client
        .get(format!("{base}/subscriptions?api-version=2022-12-01"))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    assert!(resp.headers().get("x-ms-request-id").is_some());
    assert!(resp.headers().get("x-ms-correlation-id").is_some());
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "application/json"
    );
}

// ── Subscription tests ─────────────────────────────────────────────────

#[tokio::test]
async fn list_subscriptions() {
    let (client, base) = start_server().await;

    let resp = client
        .get(format!("{base}/subscriptions?api-version=2022-12-01"))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    let subs = body["value"].as_array().unwrap();
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0]["subscriptionId"], SUB_ID);
    assert_eq!(subs[0]["state"], "Enabled");
    assert_eq!(subs[0]["displayName"], "Mock Subscription");
}

#[tokio::test]
async fn get_subscription() {
    let (client, base) = start_server().await;

    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}?api-version=2022-12-01"
        ))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["subscriptionId"], SUB_ID);
}

#[tokio::test]
async fn get_nonexistent_subscription_returns_404() {
    let (client, base) = start_server().await;

    let resp = client
        .get(format!(
            "{base}/subscriptions/nonexistent?api-version=2022-12-01"
        ))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"], "SubscriptionNotFound");
}

// ── Resource Group tests ───────────────────────────────────────────────

#[tokio::test]
async fn create_resource_group_returns_201() {
    let (client, base) = start_server().await;

    let resp = client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .json(&serde_json::json!({ "location": "eastus" }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 201);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["name"], "test-rg");
    assert_eq!(body["location"], "eastus");
    assert_eq!(body["type"], "Microsoft.Resources/resourceGroups");
    assert_eq!(body["properties"]["provisioningState"], "Succeeded");
    assert_eq!(
        body["id"],
        format!("/subscriptions/{SUB_ID}/resourceGroups/test-rg")
    );
}

#[tokio::test]
async fn update_existing_resource_group_returns_200() {
    let (client, base) = start_server().await;

    // Create
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .json(&serde_json::json!({ "location": "eastus" }))
        .send()
        .await
        .unwrap();

    // Update (PUT again)
    let resp = client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/test-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .json(&serde_json::json!({ "location": "eastus", "tags": { "env": "prod" } }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["tags"]["env"], "prod");
}

#[tokio::test]
async fn get_resource_group() {
    let (client, base) = start_server().await;

    // Create first
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/my-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .json(&serde_json::json!({ "location": "westus" }))
        .send()
        .await
        .unwrap();

    // Get
    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/my-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["name"], "my-rg");
    assert_eq!(body["location"], "westus");
}

#[tokio::test]
async fn get_nonexistent_resource_group_returns_404() {
    let (client, base) = start_server().await;

    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/nope?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"], "ResourceGroupNotFound");
}

#[tokio::test]
async fn list_resource_groups() {
    let (client, base) = start_server().await;

    // Create two
    for name in &["rg-1", "rg-2"] {
        client
            .put(format!(
                "{base}/subscriptions/{SUB_ID}/resourcegroups/{name}?api-version=2021-04-01"
            ))
            .header(auth_header().0, auth_header().1)
            .json(&serde_json::json!({ "location": "eastus" }))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["value"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn delete_resource_group() {
    let (client, base) = start_server().await;

    // Create
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/del-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .json(&serde_json::json!({ "location": "eastus" }))
        .send()
        .await
        .unwrap();

    // Delete
    let resp = client
        .delete(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/del-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    // Verify gone
    let resp = client
        .get(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/del-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn head_resource_group_exists() {
    let (client, base) = start_server().await;

    // Does not exist yet
    let resp = client
        .head(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/head-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);

    // Create
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/head-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .json(&serde_json::json!({ "location": "eastus" }))
        .send()
        .await
        .unwrap();

    // Now exists
    let resp = client
        .head(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/head-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 204);
}

#[tokio::test]
async fn patch_resource_group_tags() {
    let (client, base) = start_server().await;

    // Create
    client
        .put(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/patch-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .json(&serde_json::json!({ "location": "eastus" }))
        .send()
        .await
        .unwrap();

    // Patch tags
    let resp = client
        .patch(format!(
            "{base}/subscriptions/{SUB_ID}/resourcegroups/patch-rg?api-version=2021-04-01"
        ))
        .header(auth_header().0, auth_header().1)
        .json(&serde_json::json!({ "tags": { "env": "staging" } }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["tags"]["env"], "staging");
    assert_eq!(body["location"], "eastus");
}
