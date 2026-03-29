use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::SubResourceRef;

/// Network interface resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    pub properties: NetworkInterfaceProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceProperties {
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "ipConfigurations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_configurations: Option<Vec<NetworkInterfaceIPConfiguration>>,
    #[serde(
        rename = "dnsSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_settings: Option<NetworkInterfaceDnsSettings>,
    #[serde(
        rename = "macAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(
        rename = "enableAcceleratedNetworking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_accelerated_networking: Option<bool>,
    #[serde(
        rename = "enableIPForwarding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_ip_forwarding: Option<bool>,
    #[serde(
        rename = "networkSecurityGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_security_group: Option<SubResourceRef>,
    #[serde(
        rename = "resourceGuid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_guid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceIPConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfaceIPConfigurationProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceIPConfigurationProperties {
    #[serde(
        rename = "privateIPAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_ip_address: Option<String>,
    #[serde(
        rename = "privateIPAllocationMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_ip_allocation_method: Option<String>,
    #[serde(
        rename = "privateIPAddressVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_ip_address_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResourceRef>,
    #[serde(
        rename = "publicIPAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ip_address: Option<SubResourceRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceDnsSettings {
    #[serde(
        rename = "dnsServers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_servers: Option<Vec<String>>,
    #[serde(
        rename = "appliedDnsServers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub applied_dns_servers: Option<Vec<String>>,
    #[serde(
        rename = "internalDomainNameSuffix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_domain_name_suffix: Option<String>,
}

/// Parameters for creating a network interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNetworkInterfaceParams {
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    pub properties: NetworkInterfaceProperties,
}
