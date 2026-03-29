use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::subnet::Subnet;

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

/// Parameters for creating a virtual network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualNetworkParams {
    pub location: String,
    pub properties: VirtualNetworkProperties,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}
