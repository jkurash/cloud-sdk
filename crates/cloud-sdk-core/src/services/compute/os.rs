use serde::{Deserialize, Serialize};

use super::identity::SubResource;
use super::security::{AdditionalUnattendContent, WinRMConfiguration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsProfile {
    #[serde(
        rename = "computerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub computer_name: Option<String>,
    #[serde(
        rename = "adminUsername",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_username: Option<String>,
    #[serde(
        rename = "adminPassword",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_password: Option<String>,
    #[serde(
        rename = "customData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_data: Option<String>,
    #[serde(
        rename = "windowsConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub windows_configuration: Option<WindowsConfiguration>,
    #[serde(
        rename = "linuxConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_configuration: Option<LinuxConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<VaultSecretGroup>>,
    #[serde(
        rename = "allowExtensionOperations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_extension_operations: Option<bool>,
    #[serde(
        rename = "requireGuestProvisionSignal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_guest_provision_signal: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsConfiguration {
    #[serde(
        rename = "provisionVMAgent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provision_vm_agent: Option<bool>,
    #[serde(
        rename = "enableAutomaticUpdates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_automatic_updates: Option<bool>,
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(
        rename = "additionalUnattendContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_unattend_content: Option<Vec<AdditionalUnattendContent>>,
    #[serde(rename = "winRM", default, skip_serializing_if = "Option::is_none")]
    pub win_rm: Option<WinRMConfiguration>,
    #[serde(
        rename = "patchSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub patch_settings: Option<WindowsPatchSettings>,
    #[serde(
        rename = "enableVMAgentPlatformUpdates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_vm_agent_platform_updates: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsPatchSettings {
    #[serde(rename = "patchMode", default, skip_serializing_if = "Option::is_none")]
    pub patch_mode: Option<String>,
    #[serde(
        rename = "enableHotpatching",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_hotpatching: Option<bool>,
    #[serde(
        rename = "assessmentMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assessment_mode: Option<String>,
    #[serde(
        rename = "automaticByPlatformSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_by_platform_settings: Option<AutomaticByPlatformSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxConfiguration {
    #[serde(
        rename = "disablePasswordAuthentication",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_password_authentication: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh: Option<SshConfiguration>,
    #[serde(
        rename = "provisionVMAgent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provision_vm_agent: Option<bool>,
    #[serde(
        rename = "patchSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub patch_settings: Option<LinuxPatchSettings>,
    #[serde(
        rename = "enableVMAgentPlatformUpdates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_vm_agent_platform_updates: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxPatchSettings {
    #[serde(rename = "patchMode", default, skip_serializing_if = "Option::is_none")]
    pub patch_mode: Option<String>,
    #[serde(
        rename = "assessmentMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assessment_mode: Option<String>,
    #[serde(
        rename = "automaticByPlatformSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_by_platform_settings: Option<AutomaticByPlatformSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomaticByPlatformSettings {
    #[serde(
        rename = "rebootSetting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reboot_setting: Option<String>,
    #[serde(
        rename = "bypassPlatformSafetyChecksOnUserSchedule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bypass_platform_safety_checks_on_user_schedule: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfiguration {
    #[serde(
        rename = "publicKeys",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_keys: Option<Vec<SshPublicKey>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshPublicKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "keyData", default, skip_serializing_if = "Option::is_none")]
    pub key_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultSecretGroup {
    #[serde(
        rename = "sourceVault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_vault: Option<SubResource>,
    #[serde(
        rename = "vaultCertificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vault_certificates: Option<Vec<VaultCertificate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultCertificate {
    #[serde(
        rename = "certificateUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_url: Option<String>,
    #[serde(
        rename = "certificateStore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_store: Option<String>,
}
