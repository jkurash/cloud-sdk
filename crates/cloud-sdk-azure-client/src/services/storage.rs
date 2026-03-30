use cloud_sdk_core::error::CloudSdkError;
use cloud_sdk_core::models::Page;
use cloud_sdk_core::services::storage::{
    AccountSasParameters, BlobContainer, BlobProperties, BlobTag, BlobTags,
    CheckNameAvailabilityResult, CreateStorageAccountParams, ListAccountSasResponse,
    ListServiceSasResponse, ServiceSasParameters, StorageAccount,
    StorageAccountCheckNameAvailabilityParameters, StorageAccountListKeysResult,
    StorageAccountRegenerateKeyParameters, StorageService,
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::client::AzureClient;

const API_VERSION: &str = "2023-05-01";

/// Azure implementation of `StorageService`.
pub struct AzureStorageService {
    client: Arc<AzureClient>,
}

impl AzureStorageService {
    pub fn new(client: Arc<AzureClient>) -> Self {
        Self { client }
    }

    fn require_storage_url(&self) -> Result<(), CloudSdkError> {
        if self.client.config().storage_base_url.is_none() {
            return Err(CloudSdkError::ValidationError {
                message: "storage_base_url is required for blob operations".to_string(),
            });
        }
        Ok(())
    }
}

impl StorageService for AzureStorageService {
    // ── Management plane (storage accounts) ────────────────────────────

    async fn create_storage_account(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateStorageAccountParams,
    ) -> Result<StorageAccount, CloudSdkError> {
        let url = self
            .client
            .config()
            .storage_account_url(resource_group, name);
        let (sa, _status) = self.client.put(url, API_VERSION, &params).await?;
        Ok(sa)
    }

    async fn get_storage_account(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<StorageAccount, CloudSdkError> {
        let url = self
            .client
            .config()
            .storage_account_url(resource_group, name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_storage_accounts(
        &self,
        resource_group: &str,
    ) -> Result<Page<StorageAccount>, CloudSdkError> {
        let url = self.client.config().storage_accounts_url(resource_group);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_storage_account(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .storage_account_url(resource_group, name);
        self.client.delete(url, API_VERSION).await
    }

    // ── Data plane (blob containers) ───────────────────────────────────

    async fn create_container(&self, account: &str, container: &str) -> Result<(), CloudSdkError> {
        self.require_storage_url()?;
        let url = self
            .client
            .config()
            .blob_container_url(account, container)
            .unwrap();
        self.client.put_raw(url, bytes::Bytes::new(), None).await?;
        Ok(())
    }

    async fn delete_container(&self, account: &str, container: &str) -> Result<(), CloudSdkError> {
        self.require_storage_url()?;
        let url = self
            .client
            .config()
            .blob_container_url(account, container)
            .unwrap();
        self.client.delete_raw(url).await
    }

    async fn list_containers(&self, account: &str) -> Result<Vec<BlobContainer>, CloudSdkError> {
        self.require_storage_url()?;
        let url = self.client.config().blob_account_url(account).unwrap();
        let resp: ContainerListResponse = self.client.get_json_raw(url).await?;
        Ok(resp.containers)
    }

    // ── Data plane (blobs) ─────────────────────────────────────────────

    async fn put_blob(
        &self,
        account: &str,
        container: &str,
        blob: &str,
        data: bytes::Bytes,
        content_type: Option<&str>,
    ) -> Result<(), CloudSdkError> {
        self.require_storage_url()?;
        let url = self
            .client
            .config()
            .blob_url(account, container, blob)
            .unwrap();
        self.client.put_raw(url, data, content_type).await?;
        Ok(())
    }

    async fn get_blob(
        &self,
        account: &str,
        container: &str,
        blob: &str,
    ) -> Result<bytes::Bytes, CloudSdkError> {
        self.require_storage_url()?;
        let url = self
            .client
            .config()
            .blob_url(account, container, blob)
            .unwrap();
        self.client.get_raw(url).await
    }

    async fn delete_blob(
        &self,
        account: &str,
        container: &str,
        blob: &str,
    ) -> Result<(), CloudSdkError> {
        self.require_storage_url()?;
        let url = self
            .client
            .config()
            .blob_url(account, container, blob)
            .unwrap();
        self.client.delete_raw(url).await
    }

    async fn list_blobs(
        &self,
        account: &str,
        container: &str,
    ) -> Result<Vec<BlobProperties>, CloudSdkError> {
        self.require_storage_url()?;
        let url = self
            .client
            .config()
            .blob_container_url(account, container)
            .unwrap();
        let resp: BlobListResponse = self.client.get_json_raw(url).await?;
        Ok(resp.blobs)
    }

    async fn get_blob_properties(
        &self,
        account: &str,
        container: &str,
        blob: &str,
    ) -> Result<BlobProperties, CloudSdkError> {
        self.require_storage_url()?;
        let url = self
            .client
            .config()
            .blob_url(account, container, blob)
            .unwrap();
        // HEAD doesn't return a body, so we get properties from list_blobs or
        // we could parse response headers. For now, use the head check + list fallback.
        let exists = self.client.head_raw(url).await?;
        if !exists {
            return Err(CloudSdkError::NotFound {
                resource_type: "blob".to_string(),
                name: blob.to_string(),
            });
        }
        // Get from list (simple approach for mock compatibility)
        let blobs = self.list_blobs(account, container).await?;
        blobs
            .into_iter()
            .find(|b| b.name == blob)
            .ok_or_else(|| CloudSdkError::NotFound {
                resource_type: "blob".to_string(),
                name: blob.to_string(),
            })
    }

    // ── Management plane (extended) ───────────────────────────────────

    async fn update_storage_account(
        &self,
        resource_group: &str,
        name: &str,
        patch: serde_json::Value,
    ) -> Result<StorageAccount, CloudSdkError> {
        let url = self
            .client
            .config()
            .storage_account_url(resource_group, name);
        self.client.patch(url, API_VERSION, &patch).await
    }

    async fn list_all_storage_accounts(&self) -> Result<Page<StorageAccount>, CloudSdkError> {
        let url = self.client.config().storage_accounts_all_url();
        self.client.get(url, API_VERSION).await
    }

    async fn check_name_availability(
        &self,
        name: &str,
    ) -> Result<CheckNameAvailabilityResult, CloudSdkError> {
        let url = self.client.config().check_storage_name_url();
        let body = StorageAccountCheckNameAvailabilityParameters {
            name: name.to_string(),
            resource_type: "Microsoft.Storage/storageAccounts".to_string(),
        };
        self.client.post_json(url, API_VERSION, &body).await
    }

    async fn list_keys(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<StorageAccountListKeysResult, CloudSdkError> {
        let url = self
            .client
            .config()
            .storage_account_action_url(resource_group, name, "listKeys");
        self.client
            .post_json(url, API_VERSION, &serde_json::json!({}))
            .await
    }

    async fn regenerate_key(
        &self,
        resource_group: &str,
        name: &str,
        key_name: &str,
    ) -> Result<StorageAccountListKeysResult, CloudSdkError> {
        let url =
            self.client
                .config()
                .storage_account_action_url(resource_group, name, "regenerateKey");
        let body = StorageAccountRegenerateKeyParameters {
            key_name: key_name.to_string(),
        };
        self.client.post_json(url, API_VERSION, &body).await
    }

    async fn list_account_sas(
        &self,
        resource_group: &str,
        name: &str,
        params: AccountSasParameters,
    ) -> Result<ListAccountSasResponse, CloudSdkError> {
        let url =
            self.client
                .config()
                .storage_account_action_url(resource_group, name, "ListAccountSas");
        self.client.post_json(url, API_VERSION, &params).await
    }

    async fn list_service_sas(
        &self,
        resource_group: &str,
        name: &str,
        params: ServiceSasParameters,
    ) -> Result<ListServiceSasResponse, CloudSdkError> {
        let url =
            self.client
                .config()
                .storage_account_action_url(resource_group, name, "ListServiceSas");
        self.client.post_json(url, API_VERSION, &params).await
    }

    async fn revoke_user_delegation_keys(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self.client.config().storage_account_action_url(
            resource_group,
            name,
            "revokeUserDelegationKeys",
        );
        self.client.post_empty(url, API_VERSION).await
    }

    // ── Data plane (extended) ─────────────────────────────────────────

    async fn set_container_metadata(
        &self,
        account: &str,
        container: &str,
        metadata: HashMap<String, String>,
    ) -> Result<(), CloudSdkError> {
        self.require_storage_url()?;
        let mut url = self
            .client
            .config()
            .blob_container_url(account, container)
            .unwrap();
        url.query_pairs_mut().append_pair("comp", "metadata");

        let mut headers = reqwest::header::HeaderMap::new();
        for (key, value) in &metadata {
            let header_name =
                reqwest::header::HeaderName::from_bytes(format!("x-ms-meta-{key}").as_bytes())
                    .map_err(|e| CloudSdkError::ValidationError {
                        message: format!("invalid metadata key: {e}"),
                    })?;
            let header_value = reqwest::header::HeaderValue::from_str(value).map_err(|e| {
                CloudSdkError::ValidationError {
                    message: format!("invalid metadata value: {e}"),
                }
            })?;
            headers.insert(header_name, header_value);
        }

        self.client
            .put_with_headers(url, bytes::Bytes::new(), headers)
            .await?;
        Ok(())
    }

    async fn get_blob_metadata(
        &self,
        account: &str,
        container: &str,
        blob: &str,
    ) -> Result<HashMap<String, String>, CloudSdkError> {
        self.require_storage_url()?;
        let mut url = self
            .client
            .config()
            .blob_url(account, container, blob)
            .unwrap();
        url.query_pairs_mut().append_pair("comp", "metadata");

        let resp = self.client.get_response(url).await?;
        let mut metadata = HashMap::new();
        for (name, value) in resp.headers() {
            let name_str = name.as_str();
            if let Some(key) = name_str.strip_prefix("x-ms-meta-") {
                if let Ok(val) = value.to_str() {
                    metadata.insert(key.to_string(), val.to_string());
                }
            }
        }
        Ok(metadata)
    }

    async fn set_blob_metadata(
        &self,
        account: &str,
        container: &str,
        blob: &str,
        metadata: HashMap<String, String>,
    ) -> Result<(), CloudSdkError> {
        self.require_storage_url()?;
        let mut url = self
            .client
            .config()
            .blob_url(account, container, blob)
            .unwrap();
        url.query_pairs_mut().append_pair("comp", "metadata");

        let mut headers = reqwest::header::HeaderMap::new();
        for (key, value) in &metadata {
            let header_name =
                reqwest::header::HeaderName::from_bytes(format!("x-ms-meta-{key}").as_bytes())
                    .map_err(|e| CloudSdkError::ValidationError {
                        message: format!("invalid metadata key: {e}"),
                    })?;
            let header_value = reqwest::header::HeaderValue::from_str(value).map_err(|e| {
                CloudSdkError::ValidationError {
                    message: format!("invalid metadata value: {e}"),
                }
            })?;
            headers.insert(header_name, header_value);
        }

        self.client
            .put_with_headers(url, bytes::Bytes::new(), headers)
            .await?;
        Ok(())
    }

    async fn get_blob_tags(
        &self,
        account: &str,
        container: &str,
        blob: &str,
    ) -> Result<HashMap<String, String>, CloudSdkError> {
        self.require_storage_url()?;
        let mut url = self
            .client
            .config()
            .blob_url(account, container, blob)
            .unwrap();
        url.query_pairs_mut().append_pair("comp", "tags");

        let blob_tags: BlobTags = self.client.get_json_raw(url).await?;
        let tags = blob_tags
            .blob_tag_set
            .unwrap_or_default()
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();
        Ok(tags)
    }

    async fn set_blob_tags(
        &self,
        account: &str,
        container: &str,
        blob: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), CloudSdkError> {
        self.require_storage_url()?;
        let mut url = self
            .client
            .config()
            .blob_url(account, container, blob)
            .unwrap();
        url.query_pairs_mut().append_pair("comp", "tags");

        let blob_tags = BlobTags {
            blob_tag_set: Some(
                tags.into_iter()
                    .map(|(key, value)| BlobTag { key, value })
                    .collect(),
            ),
        };
        let body = serde_json::to_vec(&blob_tags)
            .map_err(|e| CloudSdkError::Internal(format!("failed to serialize tags: {e}")))?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        self.client
            .put_with_headers(url, bytes::Bytes::from(body), headers)
            .await?;
        Ok(())
    }

    async fn copy_blob(
        &self,
        account: &str,
        dest_container: &str,
        dest_blob: &str,
        source_url: &str,
    ) -> Result<String, CloudSdkError> {
        self.require_storage_url()?;
        let url = self
            .client
            .config()
            .blob_url(account, dest_container, dest_blob)
            .unwrap();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::HeaderName::from_static("x-ms-copy-source"),
            reqwest::header::HeaderValue::from_str(source_url).map_err(|e| {
                CloudSdkError::ValidationError {
                    message: format!("invalid copy source URL: {e}"),
                }
            })?,
        );

        let resp = self
            .client
            .put_with_headers(url, bytes::Bytes::new(), headers)
            .await?;
        let copy_id = resp
            .headers()
            .get("x-ms-copy-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();
        Ok(copy_id)
    }

    async fn set_blob_tier(
        &self,
        account: &str,
        container: &str,
        blob: &str,
        tier: &str,
    ) -> Result<(), CloudSdkError> {
        self.require_storage_url()?;
        let mut url = self
            .client
            .config()
            .blob_url(account, container, blob)
            .unwrap();
        url.query_pairs_mut().append_pair("comp", "tier");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::HeaderName::from_static("x-ms-access-tier"),
            reqwest::header::HeaderValue::from_str(tier).map_err(|e| {
                CloudSdkError::ValidationError {
                    message: format!("invalid tier value: {e}"),
                }
            })?,
        );

        self.client
            .put_with_headers(url, bytes::Bytes::new(), headers)
            .await?;
        Ok(())
    }
}

/// Response shape for list containers (matches mock server JSON).
#[derive(serde::Deserialize)]
struct ContainerListResponse {
    containers: Vec<BlobContainer>,
}

/// Response shape for list blobs (matches mock server JSON).
#[derive(serde::Deserialize)]
struct BlobListResponse {
    blobs: Vec<BlobProperties>,
}
