use cloud_sdk_core::auth::{AccessToken, Credential};
use cloud_sdk_core::error::CloudSdkError;

/// A credential that always returns a fixed mock token.
/// Used for local development against the mock server.
pub struct MockCredential;

impl Credential for MockCredential {
    async fn get_token(&self, _scopes: &[&str]) -> Result<AccessToken, CloudSdkError> {
        Ok(AccessToken {
            token: "mock-access-token".to_string(),
            expires_on: chrono::Utc::now() + chrono::Duration::hours(1),
        })
    }
}
