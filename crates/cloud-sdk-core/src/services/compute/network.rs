use serde::{Deserialize, Serialize};

use super::identity::SubResource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfile {
    #[serde(
        rename = "networkInterfaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_interfaces: Option<Vec<NetworkInterfaceReference>>,
    #[serde(
        rename = "networkInterfaceConfigurations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_interface_configurations: Option<Vec<VirtualMachineNetworkInterfaceConfiguration>>,
    #[serde(
        rename = "networkApiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_api_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfaceReferenceProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceReferenceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(
        rename = "deleteOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_option: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineNetworkInterfaceConfiguration {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VmNicConfigProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmNicConfigProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(
        rename = "deleteOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_option: Option<String>,
    #[serde(
        rename = "networkSecurityGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_security_group: Option<SubResource>,
    #[serde(
        rename = "dnsSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_settings: Option<VirtualMachineNetworkInterfaceDnsSettingsConfiguration>,
    #[serde(
        rename = "ipConfigurations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_configurations: Option<Vec<VirtualMachineNetworkInterfaceIPConfiguration>>,
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
        rename = "disableTcpStateTracking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_tcp_state_tracking: Option<bool>,
    #[serde(rename = "auxMode", default, skip_serializing_if = "Option::is_none")]
    pub aux_mode: Option<String>,
    #[serde(rename = "auxSku", default, skip_serializing_if = "Option::is_none")]
    pub aux_sku: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineNetworkInterfaceDnsSettingsConfiguration {
    #[serde(
        rename = "dnsServers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_servers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineNetworkInterfaceIPConfiguration {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VmNicIpConfigProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmNicIpConfigProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(
        rename = "publicIPAddressConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ip_address_configuration: Option<VirtualMachinePublicIPAddressConfiguration>,
    #[serde(
        rename = "privateIPAddressVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_ip_address_version: Option<String>,
    #[serde(
        rename = "applicationSecurityGroups",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_security_groups: Option<Vec<SubResource>>,
    #[serde(
        rename = "loadBalancerBackendAddressPools",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_balancer_backend_address_pools: Option<Vec<SubResource>>,
    #[serde(
        rename = "applicationGatewayBackendAddressPools",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_gateway_backend_address_pools: Option<Vec<SubResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachinePublicIPAddressConfiguration {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PublicIPAddressSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VmPublicIpConfigProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmPublicIpConfigProperties {
    #[serde(
        rename = "idleTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub idle_timeout_in_minutes: Option<i32>,
    #[serde(
        rename = "dnsSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_settings: Option<VirtualMachinePublicIPAddressDnsSettingsConfiguration>,
    #[serde(rename = "ipTags", default, skip_serializing_if = "Option::is_none")]
    pub ip_tags: Option<Vec<VirtualMachineIpTag>>,
    #[serde(
        rename = "publicIPPrefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ip_prefix: Option<SubResource>,
    #[serde(
        rename = "publicIPAddressVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ip_address_version: Option<String>,
    #[serde(
        rename = "publicIPAllocationMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ip_allocation_method: Option<String>,
    #[serde(
        rename = "deleteOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_option: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachinePublicIPAddressDnsSettingsConfiguration {
    #[serde(
        rename = "domainNameLabel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub domain_name_label: Option<String>,
    #[serde(
        rename = "domainNameLabelScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub domain_name_label_scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineIpTag {
    #[serde(rename = "ipTagType", default, skip_serializing_if = "Option::is_none")]
    pub ip_tag_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicIPAddressSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
