use crate::error::CloudSdkError;
use crate::models::Page;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::blobs::{BlobContainer, BlobProperties};
use super::identity::StorageAccountIdentity;
use super::models::{ExtendedLocation, StorageAccount, StorageSku};
use super::properties::StorageAccountProperties;

/// Parameters for creating a storage account (PUT request body).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStorageAccountParams {
    pub location: String,
    pub kind: String,
    pub sku: StorageSku,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<StorageAccountIdentity>,
    #[serde(
        rename = "extendedLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extended_location: Option<ExtendedLocation>,
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
