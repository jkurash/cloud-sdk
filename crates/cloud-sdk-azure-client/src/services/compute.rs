use cloud_sdk_core::error::CloudSdkError;
use cloud_sdk_core::models::Page;
use cloud_sdk_core::services::compute::{
    ComputeService, CreateVirtualMachineParams, VirtualMachine, VirtualMachineSizeListResult,
};
use std::sync::Arc;

use crate::client::AzureClient;

const API_VERSION: &str = "2024-07-01";

/// Azure implementation of `ComputeService`.
pub struct AzureComputeService {
    client: Arc<AzureClient>,
}

impl AzureComputeService {
    pub fn new(client: Arc<AzureClient>) -> Self {
        Self { client }
    }
}

impl ComputeService for AzureComputeService {
    async fn create_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateVirtualMachineParams,
    ) -> Result<VirtualMachine, CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_url(resource_group, name);
        let (vm, _status) = self.client.put(url, API_VERSION, &params).await?;
        Ok(vm)
    }

    async fn get_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<VirtualMachine, CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_url(resource_group, name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_virtual_machines(
        &self,
        resource_group: &str,
    ) -> Result<Page<VirtualMachine>, CloudSdkError> {
        let url = self.client.config().virtual_machines_url(resource_group);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_url(resource_group, name);
        self.client.delete(url, API_VERSION).await
    }

    async fn start_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_action_url(resource_group, name, "start");
        self.client.post_empty(url, API_VERSION).await
    }

    async fn stop_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_action_url(resource_group, name, "powerOff");
        self.client.post_empty(url, API_VERSION).await
    }

    async fn restart_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_action_url(resource_group, name, "restart");
        self.client.post_empty(url, API_VERSION).await
    }

    async fn deallocate_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url =
            self.client
                .config()
                .virtual_machine_action_url(resource_group, name, "deallocate");
        self.client.post_empty(url, API_VERSION).await
    }

    async fn update_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
        patch: serde_json::Value,
    ) -> Result<VirtualMachine, CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_url(resource_group, name);
        self.client.patch(url, API_VERSION, &patch).await
    }

    async fn list_all_virtual_machines(&self) -> Result<Page<VirtualMachine>, CloudSdkError> {
        let url = self.client.config().virtual_machines_all_url();
        self.client.get(url, API_VERSION).await
    }

    async fn list_virtual_machines_by_location(
        &self,
        location: &str,
    ) -> Result<Page<VirtualMachine>, CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machines_by_location_url(location);
        self.client.get(url, API_VERSION).await
    }

    async fn list_available_sizes(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<VirtualMachineSizeListResult, CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_sizes_url(resource_group, name);
        self.client.get(url, API_VERSION).await
    }

    async fn generalize_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url =
            self.client
                .config()
                .virtual_machine_action_url(resource_group, name, "generalize");
        self.client.post_empty(url, API_VERSION).await
    }

    async fn reapply_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_action_url(resource_group, name, "reapply");
        self.client.post_empty(url, API_VERSION).await
    }

    async fn simulate_eviction(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self.client.config().virtual_machine_action_url(
            resource_group,
            name,
            "simulateEviction",
        );
        self.client.post_empty(url, API_VERSION).await
    }

    async fn redeploy_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_action_url(resource_group, name, "redeploy");
        self.client.post_empty(url, API_VERSION).await
    }

    async fn reimage_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_machine_action_url(resource_group, name, "reimage");
        self.client.post_empty(url, API_VERSION).await
    }
}
