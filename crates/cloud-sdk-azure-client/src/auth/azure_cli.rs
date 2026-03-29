use cloud_sdk_core::auth::{AccessToken, Credential};
use cloud_sdk_core::error::CloudSdkError;

/// Authenticates by shelling out to `az account get-access-token`.
///
/// Requires the Azure CLI to be installed and logged in.
pub struct AzureCliCredential;

impl Credential for AzureCliCredential {
    async fn get_token(&self, scopes: &[&str]) -> Result<AccessToken, CloudSdkError> {
        let resource = scopes
            .first()
            .map(|s| s.trim_end_matches("/.default"))
            .unwrap_or("https://management.azure.com");

        let output = tokio::process::Command::new("az")
            .args([
                "account",
                "get-access-token",
                "--resource",
                resource,
                "--output",
                "json",
            ])
            .output()
            .await
            .map_err(|e| CloudSdkError::AuthenticationError {
                message: format!("failed to run 'az' CLI: {e}"),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CloudSdkError::AuthenticationError {
                message: format!("az CLI failed: {stderr}"),
            });
        }

        let resp: AzCliTokenResponse = serde_json::from_slice(&output.stdout).map_err(|e| {
            CloudSdkError::AuthenticationError {
                message: format!("failed to parse az CLI output: {e}"),
            }
        })?;

        let expires_on = chrono::DateTime::parse_from_rfc3339(&resp.expires_on)
            .or_else(|_| {
                // Azure CLI sometimes returns "2024-01-01 12:00:00.000000" format
                chrono::NaiveDateTime::parse_from_str(&resp.expires_on, "%Y-%m-%d %H:%M:%S%.f")
                    .map(|dt| dt.and_utc().fixed_offset())
            })
            .map_err(|e| CloudSdkError::AuthenticationError {
                message: format!("failed to parse expiry: {e}"),
            })?;

        Ok(AccessToken {
            token: resp.access_token,
            expires_on: expires_on.to_utc(),
        })
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AzCliTokenResponse {
    access_token: String,
    expires_on: String,
}
