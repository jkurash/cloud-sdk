use std::collections::HashMap;

use crate::error::CloudSdkError;
use crate::models::Page;

use super::asg::{ApplicationSecurityGroup, CreateApplicationSecurityGroupParams};
use super::nic::{CreateNetworkInterfaceParams, NetworkInterface};
use super::nsg::{CreateNsgParams, CreateSecurityRuleParams, NetworkSecurityGroup, SecurityRule};
use super::peering::{CreateVirtualNetworkPeeringParams, VirtualNetworkPeering};
use super::public_ip::{CreatePublicIPAddressParams, PublicIPAddress};
use super::route_table::{CreateRouteParams, CreateRouteTableParams, Route, RouteTable};
use super::service_tags::ServiceTagsListResult;
use super::subnet::{CreateSubnetParams, Subnet};
use super::vnet::{CreateVirtualNetworkParams, VirtualNetwork};

/// Operations for managing virtual networks, subnets, NSGs, NICs, and public IPs.
pub trait NetworkingService: Send + Sync {
    // ── Virtual Networks ──────────────────────────────────────────────

    fn create_virtual_network(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateVirtualNetworkParams,
    ) -> impl std::future::Future<Output = Result<VirtualNetwork, CloudSdkError>> + Send;

    fn get_virtual_network(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<VirtualNetwork, CloudSdkError>> + Send;

    fn list_virtual_networks(
        &self,
        resource_group: &str,
    ) -> impl std::future::Future<Output = Result<Page<VirtualNetwork>, CloudSdkError>> + Send;

    fn delete_virtual_network(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn list_all_virtual_networks(
        &self,
    ) -> impl std::future::Future<Output = Result<Page<VirtualNetwork>, CloudSdkError>> + Send;

    fn update_virtual_network_tags(
        &self,
        resource_group: &str,
        name: &str,
        tags: HashMap<String, String>,
    ) -> impl std::future::Future<Output = Result<VirtualNetwork, CloudSdkError>> + Send;

    // ── Subnets ───────────────────────────────────────────────────────

    fn create_subnet(
        &self,
        resource_group: &str,
        vnet_name: &str,
        subnet_name: &str,
        params: CreateSubnetParams,
    ) -> impl std::future::Future<Output = Result<Subnet, CloudSdkError>> + Send;

    fn get_subnet(
        &self,
        resource_group: &str,
        vnet_name: &str,
        subnet_name: &str,
    ) -> impl std::future::Future<Output = Result<Subnet, CloudSdkError>> + Send;

    fn list_subnets(
        &self,
        resource_group: &str,
        vnet_name: &str,
    ) -> impl std::future::Future<Output = Result<Page<Subnet>, CloudSdkError>> + Send;

    fn delete_subnet(
        &self,
        resource_group: &str,
        vnet_name: &str,
        subnet_name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    // ── Network Security Groups ───────────────────────────────────────

    fn create_network_security_group(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateNsgParams,
    ) -> impl std::future::Future<Output = Result<NetworkSecurityGroup, CloudSdkError>> + Send;

    fn get_network_security_group(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<NetworkSecurityGroup, CloudSdkError>> + Send;

    fn list_network_security_groups(
        &self,
        resource_group: &str,
    ) -> impl std::future::Future<Output = Result<Page<NetworkSecurityGroup>, CloudSdkError>> + Send;

    fn delete_network_security_group(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn list_all_network_security_groups(
        &self,
    ) -> impl std::future::Future<Output = Result<Page<NetworkSecurityGroup>, CloudSdkError>> + Send;

    fn update_nsg_tags(
        &self,
        resource_group: &str,
        name: &str,
        tags: HashMap<String, String>,
    ) -> impl std::future::Future<Output = Result<NetworkSecurityGroup, CloudSdkError>> + Send;

    // ── Security Rules (individual CRUD within NSGs) ──────────────────

    fn create_or_update_security_rule(
        &self,
        resource_group: &str,
        nsg_name: &str,
        rule_name: &str,
        params: CreateSecurityRuleParams,
    ) -> impl std::future::Future<Output = Result<SecurityRule, CloudSdkError>> + Send;

    fn get_security_rule(
        &self,
        resource_group: &str,
        nsg_name: &str,
        rule_name: &str,
    ) -> impl std::future::Future<Output = Result<SecurityRule, CloudSdkError>> + Send;

    fn list_security_rules(
        &self,
        resource_group: &str,
        nsg_name: &str,
    ) -> impl std::future::Future<Output = Result<Page<SecurityRule>, CloudSdkError>> + Send;

    fn delete_security_rule(
        &self,
        resource_group: &str,
        nsg_name: &str,
        rule_name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    // ── Network Interfaces ────────────────────────────────────────────

    fn create_network_interface(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateNetworkInterfaceParams,
    ) -> impl std::future::Future<Output = Result<NetworkInterface, CloudSdkError>> + Send;

    fn get_network_interface(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<NetworkInterface, CloudSdkError>> + Send;

    fn list_network_interfaces(
        &self,
        resource_group: &str,
    ) -> impl std::future::Future<Output = Result<Page<NetworkInterface>, CloudSdkError>> + Send;

    fn delete_network_interface(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    // ── Public IP Addresses ───────────────────────────────────────────

    fn create_public_ip_address(
        &self,
        resource_group: &str,
        name: &str,
        params: CreatePublicIPAddressParams,
    ) -> impl std::future::Future<Output = Result<PublicIPAddress, CloudSdkError>> + Send;

    fn get_public_ip_address(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<PublicIPAddress, CloudSdkError>> + Send;

    fn list_public_ip_addresses(
        &self,
        resource_group: &str,
    ) -> impl std::future::Future<Output = Result<Page<PublicIPAddress>, CloudSdkError>> + Send;

    fn delete_public_ip_address(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    // ── Route Tables ─────────────────────────────────────────────────

    fn create_route_table(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateRouteTableParams,
    ) -> impl std::future::Future<Output = Result<RouteTable, CloudSdkError>> + Send;

    fn get_route_table(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<RouteTable, CloudSdkError>> + Send;

    fn list_route_tables(
        &self,
        resource_group: &str,
    ) -> impl std::future::Future<Output = Result<Page<RouteTable>, CloudSdkError>> + Send;

    fn delete_route_table(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    // ── Routes (within Route Tables) ─────────────────────────────────

    fn create_route(
        &self,
        resource_group: &str,
        table_name: &str,
        route_name: &str,
        params: CreateRouteParams,
    ) -> impl std::future::Future<Output = Result<Route, CloudSdkError>> + Send;

    fn get_route(
        &self,
        resource_group: &str,
        table_name: &str,
        route_name: &str,
    ) -> impl std::future::Future<Output = Result<Route, CloudSdkError>> + Send;

    fn list_routes(
        &self,
        resource_group: &str,
        table_name: &str,
    ) -> impl std::future::Future<Output = Result<Page<Route>, CloudSdkError>> + Send;

    fn delete_route(
        &self,
        resource_group: &str,
        table_name: &str,
        route_name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    // ── Virtual Network Peerings ─────────────────────────────────────

    fn create_virtual_network_peering(
        &self,
        resource_group: &str,
        vnet_name: &str,
        peering_name: &str,
        params: CreateVirtualNetworkPeeringParams,
    ) -> impl std::future::Future<Output = Result<VirtualNetworkPeering, CloudSdkError>> + Send;

    fn get_virtual_network_peering(
        &self,
        resource_group: &str,
        vnet_name: &str,
        peering_name: &str,
    ) -> impl std::future::Future<Output = Result<VirtualNetworkPeering, CloudSdkError>> + Send;

    fn list_virtual_network_peerings(
        &self,
        resource_group: &str,
        vnet_name: &str,
    ) -> impl std::future::Future<Output = Result<Page<VirtualNetworkPeering>, CloudSdkError>> + Send;

    fn delete_virtual_network_peering(
        &self,
        resource_group: &str,
        vnet_name: &str,
        peering_name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    // ── Application Security Groups ─────────────────────────────────

    fn create_application_security_group(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateApplicationSecurityGroupParams,
    ) -> impl std::future::Future<Output = Result<ApplicationSecurityGroup, CloudSdkError>> + Send;

    fn get_application_security_group(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<ApplicationSecurityGroup, CloudSdkError>> + Send;

    fn list_application_security_groups(
        &self,
        resource_group: &str,
    ) -> impl std::future::Future<Output = Result<Page<ApplicationSecurityGroup>, CloudSdkError>> + Send;

    fn list_all_application_security_groups(
        &self,
    ) -> impl std::future::Future<Output = Result<Page<ApplicationSecurityGroup>, CloudSdkError>> + Send;

    fn delete_application_security_group(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn update_application_security_group_tags(
        &self,
        resource_group: &str,
        name: &str,
        tags: HashMap<String, String>,
    ) -> impl std::future::Future<Output = Result<ApplicationSecurityGroup, CloudSdkError>> + Send;

    // ── Service Tags ────────────────────────────────────────────────

    fn list_service_tags(
        &self,
        location: &str,
    ) -> impl std::future::Future<Output = Result<ServiceTagsListResult, CloudSdkError>> + Send;
}
