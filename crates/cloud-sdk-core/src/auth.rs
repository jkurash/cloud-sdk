use crate::error::CloudSdkError;

/// An OAuth2 access token with expiration.
#[derive(Debug, Clone)]
pub struct AccessToken {
    pub token: String,
    pub expires_on: chrono::DateTime<chrono::Utc>,
}

/// Credential provider for acquiring access tokens.
///
/// Implementations include:
/// - `MockCredential` (always returns a fixed token)
/// - `ClientSecretCredential` (Azure service principal)
/// - `AzureCliCredential` (shells out to `az account get-access-token`)
/// - `ChainedCredential` (tries multiple in order)
pub trait Credential: Send + Sync {
    /// Acquire an access token for the given scope(s).
    fn get_token(
        &self,
        scopes: &[&str],
    ) -> impl std::future::Future<Output = Result<AccessToken, CloudSdkError>> + Send;
}
