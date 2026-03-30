use serde::{Deserialize, Serialize};

/// Result of listing service tags for a location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTagsListResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(
        rename = "changeNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<ServiceTagInformation>>,
}

/// Individual service tag information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTagInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "serviceTagChangeNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_tag_change_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceTagInformationProperties>,
}

/// Properties of a service tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTagInformationProperties {
    #[serde(
        rename = "changeNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub change_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(
        rename = "systemService",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_service: Option<String>,
    #[serde(
        rename = "addressPrefixes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_prefixes: Option<Vec<String>>,
}
