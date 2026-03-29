use crate::error::CloudSdkError;
use crate::models::Page;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- ARM-level resources (management plane) ---

/// Storage account resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccount {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    pub kind: String,
    pub sku: StorageSku,
    pub properties: StorageAccountProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountProperties {
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "primaryEndpoints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_endpoints: Option<StorageEndpoints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEndpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

/// Parameters for creating a storage account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStorageAccountParams {
    pub location: String,
    pub kind: String,
    pub sku: StorageSku,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

// --- Data plane resources (blob storage) ---

/// Blob container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobContainer {
    pub name: String,
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified: Option<String>,
}

/// Blob properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobProperties {
    pub name: String,
    #[serde(rename = "contentLength", default)]
    pub content_length: u64,
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified: Option<String>,
}

/// Operations for managing storage accounts and blob storage.
pub trait StorageService: Send + Sync {
    // --- Management plane (storage accounts) ---

    fn create_storage_account(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateStorageAccountParams,
    ) -> impl std::future::Future<Output = Result<StorageAccount, CloudSdkError>> + Send;

    fn get_storage_account(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<StorageAccount, CloudSdkError>> + Send;

    fn list_storage_accounts(
        &self,
        resource_group: &str,
    ) -> impl std::future::Future<Output = Result<Page<StorageAccount>, CloudSdkError>> + Send;

    fn delete_storage_account(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    // --- Data plane (blob containers) ---

    fn create_container(
        &self,
        account: &str,
        container: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn delete_container(
        &self,
        account: &str,
        container: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn list_containers(
        &self,
        account: &str,
    ) -> impl std::future::Future<Output = Result<Vec<BlobContainer>, CloudSdkError>> + Send;

    // --- Data plane (blobs) ---

    fn put_blob(
        &self,
        account: &str,
        container: &str,
        blob: &str,
        data: bytes::Bytes,
        content_type: Option<&str>,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn get_blob(
        &self,
        account: &str,
        container: &str,
        blob: &str,
    ) -> impl std::future::Future<Output = Result<bytes::Bytes, CloudSdkError>> + Send;

    fn delete_blob(
        &self,
        account: &str,
        container: &str,
        blob: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn list_blobs(
        &self,
        account: &str,
        container: &str,
    ) -> impl std::future::Future<Output = Result<Vec<BlobProperties>, CloudSdkError>> + Send;

    fn get_blob_properties(
        &self,
        account: &str,
        container: &str,
        blob: &str,
    ) -> impl std::future::Future<Output = Result<BlobProperties, CloudSdkError>> + Send;
}
