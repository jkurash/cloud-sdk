use cloud_sdk_azure_client::{AzureClient, AzureProvider, MockCredential};
use cloud_sdk_azure_mock::{AzureMockConfig, AzureMockServer, AzureMockServerHandle};
use cloud_sdk_core::error::CloudSdkError;

/// Test harness that starts a mock server and connects an AzureClient to it.
///
/// Provides the full round-trip: SDK → HTTP request → mock server → HTTP response → SDK.
pub struct TestHarness {
    _mock_handle: AzureMockServerHandle,
    provider: AzureProvider,
    base_url: String,
}

impl TestHarness {
    /// Start a mock server with default config and create an AzureProvider connected to it.
    pub async fn start() -> Result<Self, CloudSdkError> {
        let config = Self::default_config();
        Self::start_with_config(config).await
    }

    /// Start a mock server with a custom config.
    pub async fn start_with_config(config: AzureMockConfig) -> Result<Self, CloudSdkError> {
        // Use the first subscription ID from the config
        let subscription_id = config
            .subscriptions
            .values()
            .next()
            .map(|s| s.id.clone())
            .unwrap_or_else(|| "00000000-0000-0000-0000-000000000000".to_string());

        let mock_handle = AzureMockServer::from_config(config)
            .start_on_random_port()
            .await
            .map_err(|e| CloudSdkError::Internal(format!("failed to start mock server: {e}")))?;

        let base_url = mock_handle.url();

        let client = AzureClient::builder()
            .arm_base_url(&base_url)
            .storage_base_url(&base_url)
            .credential(MockCredential)
            .subscription_id(subscription_id)
            .build()?;

        let provider = AzureProvider::new(client);

        Ok(Self {
            _mock_handle: mock_handle,
            provider,
            base_url,
        })
    }

    /// Default config used by `start()`.
    pub fn default_config() -> AzureMockConfig {
        AzureMockConfig::from_toml(
            r#"
[server]

[subscriptions.default]
id = "00000000-0000-0000-0000-000000000000"
display_name = "Mock Subscription"
tenant_id = "00000000-0000-0000-0000-000000000001"
state = "Enabled"
"#,
        )
        .expect("default test config should be valid")
    }

    pub fn provider(&self) -> &AzureProvider {
        &self.provider
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
