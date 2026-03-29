use cloud_sdk_core::error::{CloudErrorResponse, CloudSdkError};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::auth::BoxedCredential;
use crate::config::AzureConfig;

/// Azure REST API HTTP client.
///
/// Handles authentication, URL construction, and request/response serialization.
/// Works against real Azure or a mock server — the only difference is `arm_base_url`.
pub struct AzureClient {
    config: AzureConfig,
    credential: BoxedCredential,
    http: reqwest::Client,
}

impl AzureClient {
    pub(crate) fn new(config: AzureConfig, credential: BoxedCredential) -> Self {
        Self {
            config,
            credential,
            http: reqwest::Client::new(),
        }
    }

    pub fn builder() -> crate::config::AzureClientBuilder {
        crate::config::AzureClientBuilder::new()
    }

    pub fn config(&self) -> &AzureConfig {
        &self.config
    }

    /// Get a bearer token for Azure management scope.
    async fn bearer_token(&self) -> Result<String, CloudSdkError> {
        let token = self
            .credential
            .get_token(&["https://management.azure.com/.default"])
            .await?;
        Ok(format!("Bearer {}", token.token))
    }

    /// Send a GET request and deserialize the JSON response.
    pub async fn get<T: DeserializeOwned>(
        &self,
        url: Url,
        api_version: &str,
    ) -> Result<T, CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .get(url.as_str())
            .query(&[("api-version", api_version)])
            .header(AUTHORIZATION, &token)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;

        self.handle_response(resp).await
    }

    /// Send a PUT request with a JSON body and deserialize the response.
    pub async fn put<B: Serialize, T: DeserializeOwned>(
        &self,
        url: Url,
        api_version: &str,
        body: &B,
    ) -> Result<(T, u16), CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .put(url.as_str())
            .query(&[("api-version", api_version)])
            .header(AUTHORIZATION, &token)
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;

        let status = resp.status().as_u16();
        let value: T = self.handle_response(resp).await?;
        Ok((value, status))
    }

    /// Send a DELETE request.
    pub async fn delete(&self, url: Url, api_version: &str) -> Result<(), CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .delete(url.as_str())
            .query(&[("api-version", api_version)])
            .header(AUTHORIZATION, &token)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;

        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(self.parse_error(resp).await)
        }
    }

    /// Send a PATCH request with a JSON body and deserialize the response.
    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        url: Url,
        api_version: &str,
        body: &B,
    ) -> Result<T, CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .patch(url.as_str())
            .query(&[("api-version", api_version)])
            .header(AUTHORIZATION, &token)
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;

        self.handle_response(resp).await
    }

    /// Send a POST request with an empty body (for action endpoints like VM start/stop).
    pub async fn post_empty(&self, url: Url, api_version: &str) -> Result<(), CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .post(url.as_str())
            .query(&[("api-version", api_version)])
            .header(AUTHORIZATION, &token)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(self.parse_error(resp).await)
        }
    }

    /// Send a HEAD request and return whether the resource exists (204 vs 404).
    pub async fn head(&self, url: Url, api_version: &str) -> Result<bool, CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .head(url.as_str())
            .query(&[("api-version", api_version)])
            .header(AUTHORIZATION, &token)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;

        Ok(resp.status().as_u16() == 204)
    }

    // ── Raw HTTP methods (for blob data plane — no api-version, no JSON) ──

    /// PUT raw bytes to a URL with optional content-type.
    pub async fn put_raw(
        &self,
        url: Url,
        data: bytes::Bytes,
        content_type: Option<&str>,
    ) -> Result<u16, CloudSdkError> {
        let token = self.bearer_token().await?;
        let mut req = self
            .http
            .put(url.as_str())
            .header(AUTHORIZATION, &token)
            .body(data);
        if let Some(ct) = content_type {
            req = req.header(CONTENT_TYPE, ct);
        }
        let resp = req
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;
        let status = resp.status().as_u16();
        if resp.status().is_success() {
            Ok(status)
        } else {
            Err(self.parse_error(resp).await)
        }
    }

    /// GET raw bytes from a URL.
    pub async fn get_raw(&self, url: Url) -> Result<bytes::Bytes, CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .get(url.as_str())
            .header(AUTHORIZATION, &token)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;
        if resp.status().is_success() {
            resp.bytes()
                .await
                .map_err(|e| CloudSdkError::HttpError(Box::new(e)))
        } else {
            Err(self.parse_error(resp).await)
        }
    }

    /// DELETE a URL (no api-version).
    pub async fn delete_raw(&self, url: Url) -> Result<(), CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .delete(url.as_str())
            .header(AUTHORIZATION, &token)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(self.parse_error(resp).await)
        }
    }

    /// GET JSON from a URL (no api-version).
    pub async fn get_json_raw<T: DeserializeOwned>(&self, url: Url) -> Result<T, CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .get(url.as_str())
            .header(AUTHORIZATION, &token)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;
        self.handle_response(resp).await
    }

    /// HEAD a URL and return the response headers (no api-version).
    pub async fn head_raw(&self, url: Url) -> Result<bool, CloudSdkError> {
        let token = self.bearer_token().await?;
        let resp = self
            .http
            .head(url.as_str())
            .header(AUTHORIZATION, &token)
            .send()
            .await
            .map_err(|e| CloudSdkError::HttpError(Box::new(e)))?;
        Ok(resp.status().is_success())
    }

    /// Parse a successful response body as JSON.
    async fn handle_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, CloudSdkError> {
        let status = resp.status();
        if status.is_success() {
            resp.json::<T>()
                .await
                .map_err(|e| CloudSdkError::HttpError(Box::new(e)))
        } else {
            Err(self.parse_error(resp).await)
        }
    }

    /// Parse an error response into a `CloudSdkError`.
    async fn parse_error(&self, resp: reqwest::Response) -> CloudSdkError {
        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();

        // Try to parse as Azure CloudError format
        if let Ok(cloud_error) = serde_json::from_str::<CloudErrorResponse>(&body) {
            match status {
                401 => CloudSdkError::AuthenticationError {
                    message: cloud_error.error.message,
                },
                403 => CloudSdkError::AuthorizationError {
                    message: cloud_error.error.message,
                },
                404 => CloudSdkError::NotFound {
                    resource_type: "unknown".to_string(),
                    name: cloud_error.error.message.clone(),
                },
                429 => CloudSdkError::RateLimited {
                    retry_after_secs: 60,
                },
                _ => CloudSdkError::ProviderError {
                    provider: "azure".to_string(),
                    status,
                    code: cloud_error.error.code,
                    message: cloud_error.error.message,
                },
            }
        } else {
            CloudSdkError::ProviderError {
                provider: "azure".to_string(),
                status,
                code: "UnknownError".to_string(),
                message: body,
            }
        }
    }
}
