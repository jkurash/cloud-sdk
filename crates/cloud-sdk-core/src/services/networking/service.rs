use crate::error::CloudSdkError;
use crate::models::Page;

use super::nic::{CreateNetworkInterfaceParams, NetworkInterface};
use super::nsg::{CreateNsgParams, CreateSecurityRuleParams, NetworkSecurityGroup, SecurityRule};
use super::public_ip::{CreatePublicIPAddressParams, PublicIPAddress};
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
}
