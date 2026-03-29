use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<EncryptionServices>,
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<String>,
    #[serde(
        rename = "requireInfrastructureEncryption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_infrastructure_encryption: Option<bool>,
    #[serde(
        rename = "keyvaultproperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub keyvault_properties: Option<KeyVaultProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionIdentity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionServices {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<EncryptionService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<EncryptionService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<EncryptionService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<EncryptionService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionService {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(
        rename = "lastEnabledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_enabled_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyversion: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyvaulturi: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionIdentity {
    #[serde(
        rename = "userAssignedIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_assigned_identity: Option<String>,
}
