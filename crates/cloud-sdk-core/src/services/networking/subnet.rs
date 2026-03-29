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
}

/// Parameters for creating a subnet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubnetParams {
    pub properties: SubnetProperties,
}
