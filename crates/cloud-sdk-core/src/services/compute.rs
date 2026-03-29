use crate::error::CloudSdkError;
use crate::models::Page;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ════════════════════════════════════════════════════════════════════════
// Top-level VirtualMachine resource
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachine {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VirtualMachineIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
    #[serde(
        rename = "extendedLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extended_location: Option<ExtendedLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    pub properties: VirtualMachineProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<VirtualMachineExtension>>,
}

// ════════════════════════════════════════════════════════════════════════
// VirtualMachineProperties
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineProperties {
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "hardwareProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hardware_profile: Option<HardwareProfile>,
    #[serde(
        rename = "storageProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[serde(
        rename = "networkProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_profile: Option<NetworkProfile>,
    #[serde(
        rename = "securityProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_profile: Option<SecurityProfile>,
    #[serde(
        rename = "diagnosticsProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub diagnostics_profile: Option<DiagnosticsProfile>,
    #[serde(
        rename = "availabilitySet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub availability_set: Option<SubResource>,
    #[serde(
        rename = "virtualMachineScaleSet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_machine_scale_set: Option<SubResource>,
    #[serde(
        rename = "proximityPlacementGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proximity_placement_group: Option<SubResource>,
    #[serde(rename = "hostGroup", default, skip_serializing_if = "Option::is_none")]
    pub host_group: Option<SubResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<SubResource>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub license_type: Option<String>,
    #[serde(
        rename = "timeCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_created: Option<String>,
    #[serde(
        rename = "additionalCapabilities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_capabilities: Option<AdditionalCapabilities>,
    #[serde(
        rename = "billingProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_profile: Option<BillingProfile>,
    #[serde(
        rename = "evictionPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub eviction_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(
        rename = "scheduledEventsProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_events_profile: Option<ScheduledEventsProfile>,
    #[serde(rename = "userData", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(
        rename = "capacityReservation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub capacity_reservation: Option<CapacityReservationProfile>,
    #[serde(
        rename = "applicationProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_profile: Option<ApplicationProfile>,
    #[serde(
        rename = "extensionsTimeBudget",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extensions_time_budget: Option<String>,
    #[serde(
        rename = "instanceView",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_view: Option<serde_json::Value>,
    #[serde(
        rename = "platformFaultDomain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub platform_fault_domain: Option<i32>,
}

// ════════════════════════════════════════════════════════════════════════
// Hardware
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(
        rename = "vmSizeProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_size_properties: Option<VmSizeProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmSizeProperties {
    #[serde(
        rename = "vCPUsAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vcpus_available: Option<i32>,
    #[serde(
        rename = "vCPUsPerCore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vcpus_per_core: Option<i32>,
}

// ════════════════════════════════════════════════════════════════════════
// Storage
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProfile {
    #[serde(
        rename = "imageReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_reference: Option<ImageReference>,
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<OsDisk>,
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Option::is_none")]
    pub data_disks: Option<Vec<DataDisk>>,
    #[serde(
        rename = "diskControllerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disk_controller_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        rename = "exactVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exact_version: Option<String>,
    #[serde(
        rename = "sharedGalleryImageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_gallery_image_id: Option<String>,
    #[serde(
        rename = "communityGalleryImageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub community_gallery_image_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsDisk {
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<String>,
    #[serde(
        rename = "writeAcceleratorEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub write_accelerator_enabled: Option<bool>,
    #[serde(rename = "createOption")]
    pub create_option: String,
    #[serde(
        rename = "diskSizeGB",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disk_size_gb: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<VirtualHardDisk>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<VirtualHardDisk>,
    #[serde(
        rename = "managedDisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub managed_disk: Option<ManagedDiskParameters>,
    #[serde(
        rename = "encryptionSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_settings: Option<serde_json::Value>,
    #[serde(
        rename = "deleteOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_option: Option<String>,
    #[serde(
        rename = "diffDiskSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub diff_disk_settings: Option<DiffDiskSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDisk {
    pub lun: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<String>,
    #[serde(
        rename = "writeAcceleratorEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub write_accelerator_enabled: Option<bool>,
    #[serde(rename = "createOption")]
    pub create_option: String,
    #[serde(
        rename = "diskSizeGB",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disk_size_gb: Option<i32>,
    #[serde(
        rename = "managedDisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub managed_disk: Option<ManagedDiskParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<VirtualHardDisk>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<VirtualHardDisk>,
    #[serde(
        rename = "deleteOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_option: Option<String>,
    #[serde(
        rename = "toBeDetached",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub to_be_detached: Option<bool>,
    #[serde(
        rename = "detachOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub detach_option: Option<String>,
    #[serde(
        rename = "diskIOPSReadWrite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disk_iops_read_write: Option<i64>,
    #[serde(
        rename = "diskMBpsReadWrite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disk_mbps_read_write: Option<i64>,
    #[serde(
        rename = "sourceResource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_resource: Option<SubResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedDiskParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "storageAccountType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_account_type: Option<String>,
    #[serde(
        rename = "diskEncryptionSet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disk_encryption_set: Option<SubResource>,
    #[serde(
        rename = "securityProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_profile: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualHardDisk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffDiskSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
}

// ════════════════════════════════════════════════════════════════════════
// OS Profile
// ════════════════════════════════════════════════════════════════════════

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
    pub additional_unattend_content: Option<Vec<serde_json::Value>>,
    #[serde(rename = "winRM", default, skip_serializing_if = "Option::is_none")]
    pub win_rm: Option<serde_json::Value>,
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

// ════════════════════════════════════════════════════════════════════════
// Network Profile
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfile {
    #[serde(
        rename = "networkInterfaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_interfaces: Option<Vec<NetworkInterfaceReference>>,
    #[serde(
        rename = "networkInterfaceConfigurations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_interface_configurations: Option<Vec<serde_json::Value>>,
    #[serde(
        rename = "networkApiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_api_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfaceReferenceProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceReferenceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(
        rename = "deleteOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_option: Option<String>,
}

// ════════════════════════════════════════════════════════════════════════
// Security Profile
// ════════════════════════════════════════════════════════════════════════

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
    pub proxy_agent_settings: Option<serde_json::Value>,
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

// ════════════════════════════════════════════════════════════════════════
// Diagnostics Profile
// ════════════════════════════════════════════════════════════════════════

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

// ════════════════════════════════════════════════════════════════════════
// Spot / Billing / Scheduling / Capacity
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalCapabilities {
    #[serde(
        rename = "ultraSSDEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ultra_ssd_enabled: Option<bool>,
    #[serde(
        rename = "hibernationEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hibernation_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingProfile {
    #[serde(rename = "maxPrice", default, skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledEventsProfile {
    #[serde(
        rename = "terminateNotificationProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub terminate_notification_profile: Option<TerminateNotificationProfile>,
    #[serde(
        rename = "osImageNotificationProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub os_image_notification_profile: Option<OsImageNotificationProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateNotificationProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(
        rename = "notBeforeTimeout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_before_timeout: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsImageNotificationProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(
        rename = "notBeforeTimeout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_before_timeout: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityReservationProfile {
    #[serde(
        rename = "capacityReservationGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub capacity_reservation_group: Option<SubResource>,
}

// ════════════════════════════════════════════════════════════════════════
// Application Profile
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationProfile {
    #[serde(
        rename = "galleryApplications",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gallery_applications: Option<Vec<VmGalleryApplication>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmGalleryApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "packageReferenceId")]
    pub package_reference_id: String,
    #[serde(
        rename = "configurationReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_reference: Option<String>,
    #[serde(
        rename = "treatFailureAsDeploymentFailure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub treat_failure_as_deployment_failure: Option<bool>,
    #[serde(
        rename = "enableAutomaticUpgrade",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_automatic_upgrade: Option<bool>,
}

// ════════════════════════════════════════════════════════════════════════
// Identity, Location, Plan, Extensions
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineIdentity {
    #[serde(
        rename = "principalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(
        rename = "userAssignedIdentities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_assigned_identities: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(
        rename = "promotionCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub promotion_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineExtension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineExtensionProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineExtensionProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub extension_type: Option<String>,
    #[serde(
        rename = "typeHandlerVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_handler_version: Option<String>,
    #[serde(
        rename = "autoUpgradeMinorVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_upgrade_minor_version: Option<bool>,
    #[serde(
        rename = "enableAutomaticUpgrade",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_automatic_upgrade: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(
        rename = "protectedSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protected_settings: Option<serde_json::Value>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "instanceView",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_view: Option<serde_json::Value>,
}

// ════════════════════════════════════════════════════════════════════════
// Common SubResource reference
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ════════════════════════════════════════════════════════════════════════
// Create params + PowerState + Service trait
// ════════════════════════════════════════════════════════════════════════

/// Parameters for creating a virtual machine (PUT request body).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualMachineParams {
    pub location: String,
    pub properties: VirtualMachineProperties,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VirtualMachineIdentity>,
    #[serde(
        rename = "extendedLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extended_location: Option<ExtendedLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
}

/// VM size information returned by List Available Sizes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineSize {
    pub name: String,
    #[serde(rename = "numberOfCores")]
    pub number_of_cores: i32,
    #[serde(rename = "osDiskSizeInMB")]
    pub os_disk_size_in_mb: i32,
    #[serde(rename = "resourceDiskSizeInMB")]
    pub resource_disk_size_in_mb: i32,
    #[serde(rename = "memoryInMB")]
    pub memory_in_mb: i32,
    #[serde(rename = "maxDataDiskCount")]
    pub max_data_disk_count: i32,
}

/// VM size list result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineSizeListResult {
    pub value: Vec<VirtualMachineSize>,
}

/// VM power state for instance view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerState {
    Running,
    Stopped,
    Deallocated,
    Starting,
    Stopping,
}

/// Operations for managing virtual machines.
pub trait ComputeService: Send + Sync {
    fn create_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
        params: CreateVirtualMachineParams,
    ) -> impl std::future::Future<Output = Result<VirtualMachine, CloudSdkError>> + Send;

    fn get_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<VirtualMachine, CloudSdkError>> + Send;

    fn list_virtual_machines(
        &self,
        resource_group: &str,
    ) -> impl std::future::Future<Output = Result<Page<VirtualMachine>, CloudSdkError>> + Send;

    fn delete_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn start_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn stop_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn restart_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn deallocate_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn update_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
        patch: serde_json::Value,
    ) -> impl std::future::Future<Output = Result<VirtualMachine, CloudSdkError>> + Send;

    fn list_all_virtual_machines(
        &self,
    ) -> impl std::future::Future<Output = Result<Page<VirtualMachine>, CloudSdkError>> + Send;

    fn list_virtual_machines_by_location(
        &self,
        location: &str,
    ) -> impl std::future::Future<Output = Result<Page<VirtualMachine>, CloudSdkError>> + Send;

    fn list_available_sizes(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<VirtualMachineSizeListResult, CloudSdkError>> + Send;

    fn generalize_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn reapply_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn simulate_eviction(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn redeploy_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;

    fn reimage_virtual_machine(
        &self,
        resource_group: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<(), CloudSdkError>> + Send;
}
