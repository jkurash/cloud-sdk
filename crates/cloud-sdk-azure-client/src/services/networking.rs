use cloud_sdk_core::error::CloudSdkError;
use cloud_sdk_core::models::Page;
use cloud_sdk_core::services::networking::*;
use std::collections::HashMap;
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

    async fn list_all_virtual_networks(&self) -> Result<Page<VirtualNetwork>, CloudSdkError> {
        let url = self.client.config().virtual_networks_all_url();
        self.client.get(url, API_VERSION).await
    }

    async fn update_virtual_network_tags(
        &self,
        resource_group: &str,
        name: &str,
        tags: HashMap<String, String>,
    ) -> Result<VirtualNetwork, CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_network_url(resource_group, name);
        let body = serde_json::json!({ "tags": tags });
        self.client.patch(url, API_VERSION, &body).await
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

    async fn list_all_network_security_groups(
        &self,
    ) -> Result<Page<NetworkSecurityGroup>, CloudSdkError> {
        let url = self.client.config().nsgs_all_url();
        self.client.get(url, API_VERSION).await
    }

    async fn update_nsg_tags(
        &self,
        resource_group: &str,
        name: &str,
        tags: HashMap<String, String>,
    ) -> Result<NetworkSecurityGroup, CloudSdkError> {
        let url = self.client.config().nsg_url(resource_group, name);
        let body = serde_json::json!({ "tags": tags });
        self.client.patch(url, API_VERSION, &body).await
    }

    // ── Security Rules ────────────────────────────────────────────────

    async fn create_or_update_security_rule(
        &self,
        resource_group: &str,
        nsg_name: &str,
        rule_name: &str,
        params: CreateSecurityRuleParams,
    ) -> Result<SecurityRule, CloudSdkError> {
        let url = self
            .client
            .config()
            .security_rule_url(resource_group, nsg_name, rule_name);
        let (rule, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(rule)
    }

    async fn get_security_rule(
        &self,
        resource_group: &str,
        nsg_name: &str,
        rule_name: &str,
    ) -> Result<SecurityRule, CloudSdkError> {
        let url = self
            .client
            .config()
            .security_rule_url(resource_group, nsg_name, rule_name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_security_rules(
        &self,
        resource_group: &str,
        nsg_name: &str,
    ) -> Result<Page<SecurityRule>, CloudSdkError> {
        let url = self
            .client
            .config()
            .security_rules_url(resource_group, nsg_name);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_security_rule(
        &self,
        resource_group: &str,
        nsg_name: &str,
        rule_name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .security_rule_url(resource_group, nsg_name, rule_name);
        self.client.delete(url, API_VERSION).await
    }

    // ── Network Interfaces ────────────────────────────────────────────

    async fn create_network_interface(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateNetworkInterfaceParams,
    ) -> Result<NetworkInterface, CloudSdkError> {
        let url = self
            .client
            .config()
            .network_interface_url(resource_group, name);
        let (nic, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(nic)
    }

    async fn get_network_interface(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<NetworkInterface, CloudSdkError> {
        let url = self
            .client
            .config()
            .network_interface_url(resource_group, name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_network_interfaces(
        &self,
        resource_group: &str,
    ) -> Result<Page<NetworkInterface>, CloudSdkError> {
        let url = self.client.config().network_interfaces_url(resource_group);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_network_interface(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .network_interface_url(resource_group, name);
        self.client.delete(url, API_VERSION).await
    }

    // ── Public IP Addresses ───────────────────────────────────────────

    async fn create_public_ip_address(
        &self,
        resource_group: &str,
        name: &str,
        params: CreatePublicIPAddressParams,
    ) -> Result<PublicIPAddress, CloudSdkError> {
        let url = self
            .client
            .config()
            .public_ip_address_url(resource_group, name);
        let (ip, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(ip)
    }

    async fn get_public_ip_address(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<PublicIPAddress, CloudSdkError> {
        let url = self
            .client
            .config()
            .public_ip_address_url(resource_group, name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_public_ip_addresses(
        &self,
        resource_group: &str,
    ) -> Result<Page<PublicIPAddress>, CloudSdkError> {
        let url = self.client.config().public_ip_addresses_url(resource_group);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_public_ip_address(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .public_ip_address_url(resource_group, name);
        self.client.delete(url, API_VERSION).await
    }

    // ── Route Tables ─────────────────────────────────────────────────

    async fn create_route_table(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateRouteTableParams,
    ) -> Result<RouteTable, CloudSdkError> {
        let url = self.client.config().route_table_url(resource_group, name);
        let (table, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(table)
    }

    async fn get_route_table(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<RouteTable, CloudSdkError> {
        let url = self.client.config().route_table_url(resource_group, name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_route_tables(
        &self,
        resource_group: &str,
    ) -> Result<Page<RouteTable>, CloudSdkError> {
        let url = self.client.config().route_tables_url(resource_group);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_route_table(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self.client.config().route_table_url(resource_group, name);
        self.client.delete(url, API_VERSION).await
    }

    // ── Routes (within Route Tables) ─────────────────────────────────

    async fn create_route(
        &self,
        resource_group: &str,
        table_name: &str,
        route_name: &str,
        params: CreateRouteParams,
    ) -> Result<Route, CloudSdkError> {
        let url = self
            .client
            .config()
            .route_url(resource_group, table_name, route_name);
        let (route, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(route)
    }

    async fn get_route(
        &self,
        resource_group: &str,
        table_name: &str,
        route_name: &str,
    ) -> Result<Route, CloudSdkError> {
        let url = self
            .client
            .config()
            .route_url(resource_group, table_name, route_name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_routes(
        &self,
        resource_group: &str,
        table_name: &str,
    ) -> Result<Page<Route>, CloudSdkError> {
        let url = self.client.config().routes_url(resource_group, table_name);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_route(
        &self,
        resource_group: &str,
        table_name: &str,
        route_name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .route_url(resource_group, table_name, route_name);
        self.client.delete(url, API_VERSION).await
    }

    // ── Virtual Network Peerings ─────────────────────────────────────

    async fn create_virtual_network_peering(
        &self,
        resource_group: &str,
        vnet_name: &str,
        peering_name: &str,
        params: CreateVirtualNetworkPeeringParams,
    ) -> Result<VirtualNetworkPeering, CloudSdkError> {
        let url = self.client.config().virtual_network_peering_url(
            resource_group,
            vnet_name,
            peering_name,
        );
        let (peering, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(peering)
    }

    async fn get_virtual_network_peering(
        &self,
        resource_group: &str,
        vnet_name: &str,
        peering_name: &str,
    ) -> Result<VirtualNetworkPeering, CloudSdkError> {
        let url = self.client.config().virtual_network_peering_url(
            resource_group,
            vnet_name,
            peering_name,
        );
        self.client.get(url, API_VERSION).await
    }

    async fn list_virtual_network_peerings(
        &self,
        resource_group: &str,
        vnet_name: &str,
    ) -> Result<Page<VirtualNetworkPeering>, CloudSdkError> {
        let url = self
            .client
            .config()
            .virtual_network_peerings_url(resource_group, vnet_name);
        self.client.get(url, API_VERSION).await
    }

    async fn delete_virtual_network_peering(
        &self,
        resource_group: &str,
        vnet_name: &str,
        peering_name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self.client.config().virtual_network_peering_url(
            resource_group,
            vnet_name,
            peering_name,
        );
        self.client.delete(url, API_VERSION).await
    }

    // ── Application Security Groups ─────────────────────────────────

    async fn create_application_security_group(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateApplicationSecurityGroupParams,
    ) -> Result<ApplicationSecurityGroup, CloudSdkError> {
        let url = self
            .client
            .config()
            .application_security_group_url(resource_group, name);
        let (asg, _) = self.client.put(url, API_VERSION, &params).await?;
        Ok(asg)
    }

    async fn get_application_security_group(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<ApplicationSecurityGroup, CloudSdkError> {
        let url = self
            .client
            .config()
            .application_security_group_url(resource_group, name);
        self.client.get(url, API_VERSION).await
    }

    async fn list_application_security_groups(
        &self,
        resource_group: &str,
    ) -> Result<Page<ApplicationSecurityGroup>, CloudSdkError> {
        let url = self
            .client
            .config()
            .application_security_groups_url(resource_group);
        self.client.get(url, API_VERSION).await
    }

    async fn list_all_application_security_groups(
        &self,
    ) -> Result<Page<ApplicationSecurityGroup>, CloudSdkError> {
        let url = self.client.config().application_security_groups_all_url();
        self.client.get(url, API_VERSION).await
    }

    async fn delete_application_security_group(
        &self,
        resource_group: &str,
        name: &str,
    ) -> Result<(), CloudSdkError> {
        let url = self
            .client
            .config()
            .application_security_group_url(resource_group, name);
        self.client.delete(url, API_VERSION).await
    }

    async fn update_application_security_group_tags(
        &self,
        resource_group: &str,
        name: &str,
        tags: HashMap<String, String>,
    ) -> Result<ApplicationSecurityGroup, CloudSdkError> {
        let url = self
            .client
            .config()
            .application_security_group_url(resource_group, name);
        let body = serde_json::json!({ "tags": tags });
        self.client.patch(url, API_VERSION, &body).await
    }

    // ── Service Tags ────────────────────────────────────────────────

    async fn list_service_tags(
        &self,
        location: &str,
    ) -> Result<ServiceTagsListResult, CloudSdkError> {
        let url = self.client.config().service_tags_url(location);
        self.client.get(url, API_VERSION).await
    }
}
