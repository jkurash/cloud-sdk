use serde::{Deserialize, Serialize};

use super::common::SubResourceRef;

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
    #[serde(
        rename = "serviceEndpoints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_endpoints: Option<Vec<ServiceEndpointProperties>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegations: Option<Vec<Delegation>>,
    #[serde(
        rename = "privateEndpointNetworkPolicies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_endpoint_network_policies: Option<String>,
    #[serde(
        rename = "privateLinkServiceNetworkPolicies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_link_service_network_policies: Option<String>,
    #[serde(
        rename = "natGateway",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub nat_gateway: Option<SubResourceRef>,
}

/// Service endpoint properties for a subnet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpointProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
}

/// Delegation on a subnet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DelegationProperties>,
}

/// Properties for a subnet delegation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationProperties {
    #[serde(
        rename = "serviceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_name: Option<String>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
}

/// Parameters for creating a subnet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubnetParams {
    pub properties: SubnetProperties,
}
