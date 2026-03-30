use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application Security Group resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationSecurityGroup {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    pub properties: ApplicationSecurityGroupProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationSecurityGroupProperties {
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "resourceGuid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_guid: Option<String>,
}

/// Parameters for creating an application security group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApplicationSecurityGroupParams {
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}
