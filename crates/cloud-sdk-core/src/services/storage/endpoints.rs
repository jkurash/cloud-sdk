use serde::{Deserialize, Serialize};

/// Storage account endpoints — full Azure shape with microsoft/internet routing variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEndpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dfs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
    #[serde(
        rename = "microsoftEndpoints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub microsoft_endpoints: Option<StorageAccountMicrosoftEndpoints>,
    #[serde(
        rename = "internetEndpoints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internet_endpoints: Option<StorageAccountInternetEndpoints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountMicrosoftEndpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dfs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountInternetEndpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dfs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}
