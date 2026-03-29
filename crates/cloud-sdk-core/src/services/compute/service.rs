use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::CloudSdkError;
use crate::models::Page;

use super::common::Placement;
use super::identity::{ExtendedLocation, Plan, VirtualMachineIdentity};
use super::models::{VirtualMachine, VirtualMachineProperties};

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
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
