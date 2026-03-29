use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Placement {
    #[serde(
        rename = "zonePlacementPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zone_placement_policy: Option<String>,
    #[serde(
        rename = "includeZones",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_zones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(
        rename = "createdByType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_type: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by: Option<String>,
    #[serde(
        rename = "lastModifiedByType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_by_type: Option<String>,
    #[serde(
        rename = "lastModifiedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_at: Option<String>,
}
