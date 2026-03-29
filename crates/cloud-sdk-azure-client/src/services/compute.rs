use cloud_sdk_core::error::CloudSdkError;
use cloud_sdk_core::models::Page;
use cloud_sdk_core::services::compute::{
    ComputeService, CreateVirtualMachineParams, VirtualMachine,
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
}
