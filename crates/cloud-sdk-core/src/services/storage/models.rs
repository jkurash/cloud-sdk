use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::identity::StorageAccountIdentity;
use super::properties::StorageAccountProperties;

/// Storage account resource — full Azure API response shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccount {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    pub kind: String,
    pub sku: StorageSku,
    pub properties: StorageAccountProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<StorageAccountIdentity>,
    #[serde(
        rename = "extendedLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extended_location: Option<ExtendedLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
}
