use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass: Option<String>,
    #[serde(
        rename = "defaultAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_action: Option<String>,
    #[serde(rename = "ipRules", default, skip_serializing_if = "Option::is_none")]
    pub ip_rules: Option<Vec<IPRule>>,
    #[serde(
        rename = "virtualNetworkRules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_network_rules: Option<Vec<VirtualNetworkRule>>,
    #[serde(
        rename = "resourceAccessRules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_access_rules: Option<Vec<ResourceAccessRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPRule {
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAccessRule {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(
        rename = "resourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<String>,
}
