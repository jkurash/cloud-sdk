use crate::error::CloudSdkError;
use crate::models::Page;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Virtual network resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetwork {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    pub properties: VirtualNetworkProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetworkProperties {
    #[serde(rename = "addressSpace")]
    pub address_space: AddressSpace,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<Subnet>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressSpace {
    #[serde(rename = "addressPrefixes")]
    pub address_prefixes: Vec<String>,
}

/// Subnet resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    pub id: String,
    pub name: String,
    pub properties: SubnetProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetProperties {
    #[serde(rename = "addressPrefix")]
    pub address_prefix: String,
    #[serde(
        rename = "networkSecurityGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_security_group: Option<SubResourceRef>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
}

/// Network security group resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityGroup {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    pub properties: NsgProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NsgProperties {
    #[serde(
        rename = "securityRules",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub security_rules: Vec<SecurityRule>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub name: String,
    pub properties: SecurityRuleProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRuleProperties {
    pub protocol: String,
    #[serde(rename = "sourceAddressPrefix")]
    pub source_address_prefix: String,
    #[serde(rename = "destinationAddressPrefix")]
    pub destination_address_prefix: String,
    #[serde(rename = "sourcePortRange")]
    pub source_port_range: String,
    #[serde(rename = "destinationPortRange")]
    pub destination_port_range: String,
    pub access: String,
    pub direction: String,
    pub priority: u32,
}

/// Reference to a sub-resource by ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubResourceRef {
    pub id: String,
}

/// Parameters for creating a virtual network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualNetworkParams {
    pub location: String,
    pub properties: VirtualNetworkProperties,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

/// Parameters for creating a subnet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubnetParams {
    pub properties: SubnetProperties,
}

/// Parameters for creating a network security group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNsgParams {
    pub location: String,
    pub properties: NsgProperties,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

/// Operations for managing virtual networks, subnets, and NSGs.
pub trait NetworkingService: Send + Sync {
    // Virtual Networks
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

    // Subnets
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

    // Network Security Groups
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
}
