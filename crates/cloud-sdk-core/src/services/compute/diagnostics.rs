use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsProfile {
    #[serde(
        rename = "bootDiagnostics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub boot_diagnostics: Option<BootDiagnostics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootDiagnostics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "storageUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_uri: Option<String>,
}
