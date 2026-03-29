use cloud_sdk_azure_client::AzureClient;
use cloud_sdk_azure_client::auth::ClientSecretCredential;
use cloud_sdk_azure_mock::{AzureMockConfig, AzureMockServer};
use cloud_sdk_core::models::resource::CreateResourceGroupParams;
use cloud_sdk_core::services::ResourceManagerService;
use std::collections::HashMap;

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

/// Test that ClientSecretCredential authenticates against the mock OAuth2 endpoint
/// and the resulting token is accepted by the mock ARM endpoints.
#[tokio::test]
async fn client_secret_credential_against_mock() {
    let handle = AzureMockServer::from_config(test_config())
        .start_on_random_port()
        .await
        .unwrap();
    let base = handle.url();

    // Use ClientSecretCredential with mock server as authority
    let credential = ClientSecretCredential::with_authority(
        "mock-tenant-id",
        "mock-client-id",
        "mock-client-secret",
        &base, // mock server acts as the OAuth2 authority
    );

    let client = AzureClient::builder()
        .arm_base_url(&base)
        .credential(credential)
        .subscription_id("00000000-0000-0000-0000-000000000000")
        .build()
        .unwrap();

    let provider = cloud_sdk_azure_client::AzureProvider::new(client);
    let rm = provider.resource_manager();

    // This should: 1) get token from mock OAuth2, 2) use it to call mock ARM
    let page = rm.list_subscriptions().await.unwrap();
    assert_eq!(page.value.len(), 1);
    assert_eq!(page.value[0].display_name, "Mock Subscription");
}

/// Test that ClientSecretCredential token is used for resource creation.
#[tokio::test]
async fn client_secret_full_lifecycle() {
    let handle = AzureMockServer::from_config(test_config())
        .start_on_random_port()
        .await
        .unwrap();
    let base = handle.url();

    let credential =
        ClientSecretCredential::with_authority("tenant-1", "app-id-123", "secret-xyz", &base);

    let client = AzureClient::builder()
        .arm_base_url(&base)
        .credential(credential)
        .subscription_id("00000000-0000-0000-0000-000000000000")
        .build()
        .unwrap();

    let provider = cloud_sdk_azure_client::AzureProvider::new(client);
    let rm = provider.resource_manager();

    // Create a resource group using the mock-authenticated client
    let rg = rm
        .create_resource_group(
            "auth-test-rg",
            CreateResourceGroupParams {
                location: "eastus".to_string(),
                tags: HashMap::new(),
            },
        )
        .await
        .unwrap();

    assert_eq!(rg.name, "auth-test-rg");

    // Verify it persists
    let fetched = rm.get_resource_group("auth-test-rg").await.unwrap();
    assert_eq!(fetched.name, "auth-test-rg");
}

/// Test that ChainedCredential works against the mock.
#[tokio::test]
async fn chained_credential_against_mock() {
    use cloud_sdk_azure_client::auth::ChainedCredential;

    let handle = AzureMockServer::from_config(test_config())
        .start_on_random_port()
        .await
        .unwrap();
    let base = handle.url();

    let credential = ChainedCredential::builder()
        .with(ClientSecretCredential::with_authority(
            "tenant", "client", "secret", &base,
        ))
        .build();

    let client = AzureClient::builder()
        .arm_base_url(&base)
        .credential(credential)
        .subscription_id("00000000-0000-0000-0000-000000000000")
        .build()
        .unwrap();

    let provider = cloud_sdk_azure_client::AzureProvider::new(client);
    let subs = provider
        .resource_manager()
        .list_subscriptions()
        .await
        .unwrap();
    assert_eq!(subs.value.len(), 1);
}
