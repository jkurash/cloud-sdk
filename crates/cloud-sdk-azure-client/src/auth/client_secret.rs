use cloud_sdk_core::auth::{AccessToken, Credential};
use cloud_sdk_core::error::CloudSdkError;

const DEFAULT_AUTHORITY: &str = "https://login.microsoftonline.com";

/// Authenticates using a service principal (client ID + client secret).
///
/// Exchanges credentials for a token via the Azure AD `/oauth2/v2.0/token` endpoint.
/// The authority URL is configurable for mock server testing.
pub struct ClientSecretCredential {
    tenant_id: String,
    client_id: String,
    client_secret: String,
    authority: String,
    http: reqwest::Client,
}

impl ClientSecretCredential {
    /// Create with the default Azure AD authority (`login.microsoftonline.com`).
    pub fn new(
        tenant_id: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            authority: DEFAULT_AUTHORITY.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Create with a custom authority URL (e.g., mock server URL).
    /// The token endpoint will be `{authority}/{tenant_id}/oauth2/v2.0/token`.
    pub fn with_authority(
        tenant_id: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        authority: impl Into<String>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            authority: authority.into(),
            http: reqwest::Client::new(),
        }
    }
}

impl Credential for ClientSecretCredential {
    async fn get_token(&self, scopes: &[&str]) -> Result<AccessToken, CloudSdkError> {
        let url = format!(
            "{}/{}/oauth2/v2.0/token",
            self.authority.trim_end_matches('/'),
            self.tenant_id
        );

        let scope = scopes.join(" ");
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("scope", &scope),
        ];

        let resp = self
            .http
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(CloudSdkError::AuthenticationError {
                message: format!("token request failed: {body}"),
            });
        }

        let body: TokenResponse = resp
            .json()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;

        Ok(AccessToken {
            token: body.access_token,
            expires_on: chrono::Utc::now() + chrono::Duration::seconds(body.expires_in as i64),
        })
    }
}

#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}
