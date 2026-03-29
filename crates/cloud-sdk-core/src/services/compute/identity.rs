use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineIdentity {
    #[serde(
        rename = "principalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(
        rename = "userAssignedIdentities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_assigned_identities: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(
        rename = "promotionCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub promotion_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineExtension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineExtensionProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineExtensionProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub extension_type: Option<String>,
    #[serde(
        rename = "typeHandlerVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_handler_version: Option<String>,
    #[serde(
        rename = "autoUpgradeMinorVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_upgrade_minor_version: Option<bool>,
    #[serde(
        rename = "enableAutomaticUpgrade",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_automatic_upgrade: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(
        rename = "protectedSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protected_settings: Option<serde_json::Value>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "instanceView",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_view: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
