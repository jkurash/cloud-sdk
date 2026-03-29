pub mod azure_cli;
pub mod chained;
pub mod client_secret;
pub mod mock_credential;

pub use azure_cli::AzureCliCredential;
pub use chained::ChainedCredential;
pub use client_secret::ClientSecretCredential;
pub use mock_credential::MockCredential;

use cloud_sdk_core::auth::{AccessToken, Credential};
use cloud_sdk_core::error::CloudSdkError;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Object-safe wrapper trait for credentials.
/// Allows `AzureClient` to store any `Credential` without generics.
trait ErasedCredential: Send + Sync {
    fn get_token_boxed<'a>(
        &'a self,
        scopes: &'a [&'a str],
    ) -> Pin<Box<dyn Future<Output = Result<AccessToken, CloudSdkError>> + Send + 'a>>;
}

impl<T: Credential> ErasedCredential for T {
    fn get_token_boxed<'a>(
        &'a self,
        scopes: &'a [&'a str],
    ) -> Pin<Box<dyn Future<Output = Result<AccessToken, CloudSdkError>> + Send + 'a>> {
        Box::pin(self.get_token(scopes))
    }
}

/// Type-erased credential that can be stored in `AzureClient`.
#[derive(Clone)]
pub struct BoxedCredential {
    inner: Arc<dyn ErasedCredential>,
}

impl BoxedCredential {
    /// Wrap any `Credential` implementation.
    pub fn new(cred: impl Credential + 'static) -> Self {
        Self {
            inner: Arc::new(cred),
        }
    }

    /// Get an access token for the given scopes.
    pub async fn get_token(&self, scopes: &[&str]) -> Result<AccessToken, CloudSdkError> {
        self.inner.get_token_boxed(scopes).await
    }
}
