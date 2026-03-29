use cloud_sdk_core::auth::{AccessToken, Credential};
use cloud_sdk_core::error::CloudSdkError;

use super::BoxedCredential;

/// Tries multiple credentials in order, returning the first successful token.
///
/// Similar to Azure's `DefaultAzureCredential` — chains credentials so the
/// same code works in different environments (local dev with CLI, CI with
/// service principal, production with managed identity).
pub struct ChainedCredential {
    credentials: Vec<BoxedCredential>,
}

impl ChainedCredential {
    pub fn new(credentials: Vec<BoxedCredential>) -> Self {
        Self { credentials }
    }

    /// Convenience: build a chain from multiple `Credential` implementations.
    pub fn builder() -> ChainedCredentialBuilder {
        ChainedCredentialBuilder {
            credentials: Vec::new(),
        }
    }
}

impl Credential for ChainedCredential {
    async fn get_token(&self, scopes: &[&str]) -> Result<AccessToken, CloudSdkError> {
        let mut last_error = None;
        for cred in &self.credentials {
            match cred.get_token(scopes).await {
                Ok(token) => return Ok(token),
                Err(e) => last_error = Some(e),
            }
        }
        Err(
            last_error.unwrap_or_else(|| CloudSdkError::AuthenticationError {
                message: "no credentials configured in chain".to_string(),
            }),
        )
    }
}

pub struct ChainedCredentialBuilder {
    credentials: Vec<BoxedCredential>,
}

impl ChainedCredentialBuilder {
    pub fn with(mut self, cred: impl Credential + 'static) -> Self {
        self.credentials.push(BoxedCredential::new(cred));
        self
    }

    pub fn build(self) -> ChainedCredential {
        ChainedCredential::new(self.credentials)
    }
}
