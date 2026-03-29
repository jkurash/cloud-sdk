use cloud_sdk_core::error::CloudSdkError;
use cloud_sdk_core::models::Page;
use cloud_sdk_core::services::networking::*;
use std::sync::Arc;

use crate::client::AzureClient;

const API_VERSION: &str = "2024-01-01";

pub struct AzureNetworkingService {
    client: Arc<AzureClient>,
}

impl AzureNetworkingService {
    pub fn new(client: Arc<AzureClient>) -> Self {
        Self { client }
    }
}

impl NetworkingService for AzureNetworkingService {
    // ── Virtual Networks ───────────────────────────────────────────────

    async fn create_virtual_network(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateVirtualNetworkParams,
    ) -> Result<VirtualNetwork, CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_network_url(resource_group, name);
        let (vnet, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(vnet)
    }

    async fn get_virtual_network(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<VirtualNetwork, CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_network_url(resource_group, name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_virtual_networks(
        &self,
        resource_group: &str,
    ) -> Result<Page<VirtualNetwork>, CloudSdkError> {
        let url = self.client.config().virtual_networks_url(resource_group);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_virtual_network(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_network_url(resource_group, name);
        self.client.delete(url, API_VERSION).await
    }

    // ── Subnets ────────────────────────────────────────────────────────

    async fn create_subnet(
        &self,
        resource_group: &str,
        vnet_name: &str,
        subnet_name: &str,
        params: CreateSubnetParams,
    ) -> Result<Subnet, CloudSdkError> {
        let url = self
            .client
            .config()
            .subnet_url(resource_group, vnet_name, subnet_name);
        let (subnet, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(subnet)
    }

    async fn get_subnet(
        &self,
        resource_group: &str,
        vnet_name: &str,
        subnet_name: &str,
    ) -> Result<Subnet, CloudSdkError> {
        let url = self
            .client
            .config()
            .subnet_url(resource_group, vnet_name, subnet_name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_subnets(
        &self,
        resource_group: &str,
        vnet_name: &str,
    ) -> Result<Page<Subnet>, CloudSdkError> {
        let url = self.client.config().subnets_url(resource_group, vnet_name);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_subnet(
        &self,
        resource_group: &str,
        vnet_name: &str,
        subnet_name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .subnet_url(resource_group, vnet_name, subnet_name);
        self.client.delete(url, API_VERSION).await
    }

    // ── Network Security Groups ────────────────────────────────────────

    async fn create_network_security_group(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateNsgParams,
    ) -> Result<NetworkSecurityGroup, CloudSdkError> {
        let url = self.client.config().nsg_url(resource_group, name);
        let (nsg, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(nsg)
    }

    async fn get_network_security_group(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<NetworkSecurityGroup, CloudSdkError> {
        let url = self.client.config().nsg_url(resource_group, name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_network_security_groups(
        &self,
        resource_group: &str,
    ) -> Result<Page<NetworkSecurityGroup>, CloudSdkError> {
        let url = self.client.config().nsgs_url(resource_group);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_network_security_group(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self.client.config().nsg_url(resource_group, name);
        self.client.delete(url, API_VERSION).await
    }
}
