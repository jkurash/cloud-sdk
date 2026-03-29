use crate::error::CloudSdkError;
use crate::models::{CreateResourceGroupParams, Page, ResourceGroup, Subscription};

/// Operations for managing subscriptions and resource groups.
pub trait ResourceManagerService: Send + Sync {
    // Subscriptions
    fn list_subscriptions(
        &self,
    ) -> impl std::future::Future<Output = Result<Page<Subscription>, CloudSdkError>> + Send;

    fn get_subscription(
        &self,
        subscription_id: &str,
    ) -> impl std::future::Future<Output = Result<Subscription, CloudSdkError>> + Send;

    // Resource Groups
    fn create_resource_group(
        &self,
        name: &str,
        params: CreateResourceGroupParams,
    ) -> impl std::future::Future<Output = Result<ResourceGroup, CloudSdkError>> + Send;

    fn get_resource_group(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<ResourceGroup, CloudSdkError>> + Send;

    fn list_resource_groups(
        &self,
    ) -> impl std::future::Future<Output = Result<Page<ResourceGroup>, CloudSdkError>> + Send;

    fn delete_resource_group(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn resource_group_exists(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<bool, CloudSdkError>> + Send;
}
