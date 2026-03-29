use cloud_sdk_core::error::CloudSdkError;
use cloud_sdk_core::models::Page;
use cloud_sdk_core::models::resource::{CreateResourceGroupParams, ResourceGroup, Subscription};
use cloud_sdk_core::services::ResourceManagerService;
use std::sync::Arc;

use crate::client::AzureClient;

const API_VERSION: &str = "2021-04-01";
const SUBSCRIPTIONS_API_VERSION: &str = "2022-12-01";

/// Azure implementation of `ResourceManagerService`.
pub struct AzureResourceManagerService {
    client: Arc<AzureClient>,
}

impl AzureResourceManagerService {
    pub fn new(client: Arc<AzureClient>) -> Self {
        Self { client }
    }
}

impl ResourceManagerService for AzureResourceManagerService {
    async fn list_subscriptions(&self) -> Result<Page<Subscription>, CloudSdkError> {
        let url = self.client.config().subscriptions_url();
        self.client.get(url, SUBSCRIPTIONS_API_VERSION).await
    }

    async fn get_subscription(&self, subscription_id: &str) -> Result<Subscription, CloudSdkError> {
        let url = self.client.config().subscription_url(subscription_id);
        self.client.get(url, SUBSCRIPTIONS_API_VERSION).await
    }

    async fn create_resource_group(
        &self,
        name: &str,
        params: CreateResourceGroupParams,
    ) -> Result<ResourceGroup, CloudSdkError> {
        let url = self.client.config().resource_group_url(name);
        let (rg, _status) = self.client.put(url, API_VERSION, &params).await?;
        Ok(rg)
    }

    async fn get_resource_group(&self, name: &str) -> Result<ResourceGroup, CloudSdkError> {
        let url = self.client.config().resource_group_url(name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_resource_groups(&self) -> Result<Page<ResourceGroup>, CloudSdkError> {
        let url = self.client.config().resource_groups_url();
        self.client.get(url, API_VERSION).await
    }

    async fn delete_resource_group(&self, name: &str) -> Result<(), CloudSdkError> {
        let url = self.client.config().resource_group_url(name);
        self.client.delete(url, API_VERSION).await
    }

    async fn resource_group_exists(&self, name: &str) -> Result<bool, CloudSdkError> {
        let url = self.client.config().resource_group_url(name);
        self.client.head(url, API_VERSION).await
    }
}
