use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        rename = "defaultSecurityRules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_security_rules: Option<Vec<SecurityRule>>,
    #[serde(
        rename = "resourceGuid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_guid: Option<String>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    pub properties: SecurityRuleProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRuleProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub protocol: String,
    #[serde(rename = "sourceAddressPrefix")]
    pub source_address_prefix: String,
    #[serde(rename = "destinationAddressPrefix")]
    pub destination_address_prefix: String,
    #[serde(rename = "sourcePortRange")]
    pub source_port_range: String,
    #[serde(rename = "destinationPortRange")]
    pub destination_port_range: String,
    #[serde(
        rename = "sourcePortRanges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_port_ranges: Option<Vec<String>>,
    #[serde(
        rename = "destinationPortRanges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_port_ranges: Option<Vec<String>>,
    #[serde(
        rename = "sourceAddressPrefixes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_address_prefixes: Option<Vec<String>>,
    #[serde(
        rename = "destinationAddressPrefixes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_address_prefixes: Option<Vec<String>>,
    pub access: String,
    pub direction: String,
    pub priority: u32,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
}

/// Parameters for creating a network security group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNsgParams {
    pub location: String,
    pub properties: NsgProperties,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

/// Parameters for creating or updating an individual security rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSecurityRuleParams {
    pub properties: SecurityRuleProperties,
}
