use serde::{Deserialize, Serialize};

use super::identity::SubResource;
use super::security::{DiskEncryptionSettings, VMDiskSecurityProfile};

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
    pub encryption_settings: Option<DiskEncryptionSettings>,
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
    pub security_profile: Option<VMDiskSecurityProfile>,
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
