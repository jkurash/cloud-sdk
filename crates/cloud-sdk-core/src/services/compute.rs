use crate::error::CloudSdkError;
use crate::models::Page;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Virtual machine resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachine {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    pub properties: VirtualMachineProperties,
}

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
    #[serde(rename = "hardwareProfile")]
    pub hardware_profile: HardwareProfile,
    #[serde(rename = "storageProfile")]
    pub storage_profile: StorageProfile,
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[serde(rename = "networkProfile")]
    pub network_profile: NetworkProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    #[serde(rename = "vmSize")]
    pub vm_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProfile {
    #[serde(
        rename = "imageReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_reference: Option<ImageReference>,
    #[serde(rename = "osDisk")]
    pub os_disk: OsDisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsDisk {
    pub name: String,
    #[serde(rename = "createOption")]
    pub create_option: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<String>,
    #[serde(
        rename = "managedDisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub managed_disk: Option<ManagedDisk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedDisk {
    #[serde(
        rename = "storageAccountType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_account_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsProfile {
    #[serde(rename = "computerName")]
    pub computer_name: String,
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    #[serde(
        rename = "linuxConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_configuration: Option<LinuxConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxConfiguration {
    #[serde(rename = "disablePasswordAuthentication", default)]
    pub disable_password_authentication: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfile {
    #[serde(rename = "networkInterfaces")]
    pub network_interfaces: Vec<NetworkInterfaceReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceReference {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfaceReferenceProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceReferenceProperties {
    #[serde(default)]
    pub primary: bool,
}

/// Parameters for creating a virtual machine (PUT request body).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualMachineParams {
    pub location: String,
    pub properties: VirtualMachineProperties,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
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
}
