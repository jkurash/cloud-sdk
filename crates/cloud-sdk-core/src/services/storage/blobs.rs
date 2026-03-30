use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Blob container with full properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobContainer {
    pub name: String,
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(
        rename = "leaseStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lease_status: Option<String>,
    #[serde(
        rename = "leaseState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lease_state: Option<String>,
    #[serde(
        rename = "publicAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_access: Option<String>,
    #[serde(
        rename = "hasImmutabilityPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_immutability_policy: Option<bool>,
    #[serde(
        rename = "hasLegalHold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_legal_hold: Option<bool>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

/// Blob properties with full Azure fidelity.
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
        rename = "contentEncoding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_encoding: Option<String>,
    #[serde(
        rename = "contentLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_language: Option<String>,
    #[serde(
        rename = "contentDisposition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_disposition: Option<String>,
    #[serde(
        rename = "contentMD5",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_md5: Option<String>,
    #[serde(
        rename = "cacheControl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_control: Option<String>,
    #[serde(
        rename = "lastModified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "blobType", default, skip_serializing_if = "Option::is_none")]
    pub blob_type: Option<String>,
    #[serde(
        rename = "accessTier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_tier: Option<String>,
    #[serde(
        rename = "leaseStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lease_status: Option<String>,
    #[serde(
        rename = "leaseState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lease_state: Option<String>,
    #[serde(
        rename = "serverEncrypted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_encrypted: Option<bool>,
    #[serde(
        rename = "creationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_time: Option<String>,
    #[serde(rename = "copyId", default, skip_serializing_if = "Option::is_none")]
    pub copy_id: Option<String>,
    #[serde(
        rename = "copyStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_status: Option<String>,
    #[serde(
        rename = "copySource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_source: Option<String>,
    #[serde(
        rename = "copyProgress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_progress: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

/// Blob tag key-value pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobTag {
    pub key: String,
    pub value: String,
}

/// Blob tags wrapper for Get/Set Tags operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobTags {
    #[serde(
        rename = "blobTagSet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_tag_set: Option<Vec<BlobTag>>,
}

/// Storage service properties for Get/Set Service Properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageServiceProperties {
    #[serde(rename = "Logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingProperties>,
    #[serde(
        rename = "HourMetrics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hour_metrics: Option<MetricsProperties>,
    #[serde(
        rename = "MinuteMetrics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minute_metrics: Option<MetricsProperties>,
    #[serde(rename = "Cors", default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<CorsProperties>,
    #[serde(
        rename = "DefaultServiceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_service_version: Option<String>,
    #[serde(
        rename = "DeleteRetentionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_retention_policy: Option<RetentionPolicy>,
    #[serde(
        rename = "StaticWebsite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub static_website: Option<StaticWebsite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingProperties {
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "Read", default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "Write", default, skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[serde(rename = "Delete", default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(
        rename = "RetentionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_policy: Option<RetentionPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsProperties {
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "IncludeAPIs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_apis: Option<bool>,
    #[serde(
        rename = "RetentionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_policy: Option<RetentionPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsProperties {
    #[serde(rename = "CorsRule", default, skip_serializing_if = "Option::is_none")]
    pub cors_rules: Option<Vec<CorsRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsRule {
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: String,
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: String,
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: String,
    #[serde(rename = "ExposedHeaders")]
    pub exposed_headers: String,
    #[serde(rename = "MaxAgeInSeconds")]
    pub max_age_in_seconds: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticWebsite {
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "IndexDocument",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_document: Option<String>,
    #[serde(
        rename = "ErrorDocument404Path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_document_404_path: Option<String>,
}
