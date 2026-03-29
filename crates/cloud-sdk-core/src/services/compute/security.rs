use serde::{Deserialize, Serialize};

use super::identity::SubResource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProfile {
    #[serde(
        rename = "securityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_type: Option<String>,
    #[serde(
        rename = "uefiSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub uefi_settings: Option<UefiSettings>,
    #[serde(
        rename = "encryptionAtHost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_at_host: Option<bool>,
    #[serde(
        rename = "encryptionIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_identity: Option<EncryptionIdentity>,
    #[serde(
        rename = "proxyAgentSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proxy_agent_settings: Option<ProxyAgentSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UefiSettings {
    #[serde(
        rename = "secureBootEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_boot_enabled: Option<bool>,
    #[serde(
        rename = "vTpmEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub v_tpm_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionIdentity {
    #[serde(
        rename = "userAssignedIdentityResourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_assigned_identity_resource_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskEncryptionSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "diskEncryptionKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disk_encryption_key: Option<KeyVaultSecretReference>,
    #[serde(
        rename = "keyEncryptionKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_encryption_key: Option<KeyVaultKeyReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVaultSecretReference {
    #[serde(rename = "secretUrl")]
    pub secret_url: String,
    #[serde(
        rename = "sourceVault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_vault: Option<SubResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVaultKeyReference {
    #[serde(rename = "keyUrl")]
    pub key_url: String,
    #[serde(
        rename = "sourceVault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_vault: Option<SubResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMDiskSecurityProfile {
    #[serde(
        rename = "securityEncryptionType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_encryption_type: Option<String>,
    #[serde(
        rename = "diskEncryptionSet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disk_encryption_set: Option<SubResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyAgentSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(
        rename = "keyIncarnationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_incarnation_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinRMConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<WinRMListener>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinRMListener {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(
        rename = "certificateUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalUnattendContent {
    #[serde(rename = "passName", default, skip_serializing_if = "Option::is_none")]
    pub pass_name: Option<String>,
    #[serde(
        rename = "componentName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub component_name: Option<String>,
    #[serde(
        rename = "settingName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub setting_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
