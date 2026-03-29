use serde::{Deserialize, Serialize};

/// Blob container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobContainer {
    pub name: String,
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified: Option<String>,
}

/// Blob properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobProperties {
    pub name: String,
    #[serde(rename = "contentLength", default)]
    pub content_length: u64,
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified: Option<String>,
}
