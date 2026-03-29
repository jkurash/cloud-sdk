use serde::{Deserialize, Serialize};

use super::security::DiskEncryptionSettings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineInstanceView {
    #[serde(
        rename = "platformUpdateDomain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub platform_update_domain: Option<i32>,
    #[serde(
        rename = "platformFaultDomain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub platform_fault_domain: Option<i32>,
    #[serde(
        rename = "computerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub computer_name: Option<String>,
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(
        rename = "hyperVGeneration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hyper_v_generation: Option<String>,
    #[serde(
        rename = "rdpThumbPrint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rdp_thumb_print: Option<String>,
    #[serde(rename = "vmAgent", default, skip_serializing_if = "Option::is_none")]
    pub vm_agent: Option<VirtualMachineAgentInstanceView>,
    #[serde(
        rename = "maintenanceRedeployStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_redeploy_status: Option<MaintenanceRedeployStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<DiskInstanceView>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<VirtualMachineExtensionInstanceView>>,
    #[serde(
        rename = "bootDiagnostics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub boot_diagnostics: Option<BootDiagnosticsInstanceView>,
    #[serde(
        rename = "assignedHost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<InstanceViewStatus>>,
    #[serde(rename = "vmHealth", default, skip_serializing_if = "Option::is_none")]
    pub vm_health: Option<VirtualMachineHealthStatus>,
    #[serde(
        rename = "patchStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub patch_status: Option<VirtualMachinePatchStatus>,
    #[serde(
        rename = "isVMInStandbyPool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_vm_in_standby_pool: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceViewStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(
        rename = "displayStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineAgentInstanceView {
    #[serde(
        rename = "vmAgentVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_agent_version: Option<String>,
    #[serde(
        rename = "extensionHandlers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_handlers: Option<Vec<VirtualMachineExtensionHandlerInstanceView>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<InstanceViewStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineExtensionHandlerInstanceView {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub handler_type: Option<String>,
    #[serde(
        rename = "typeHandlerVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_handler_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceViewStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineExtensionInstanceView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub extension_type: Option<String>,
    #[serde(
        rename = "typeHandlerVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_handler_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<InstanceViewStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substatuses: Option<Vec<InstanceViewStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInstanceView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "encryptionSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_settings: Option<Vec<DiskEncryptionSettings>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<InstanceViewStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootDiagnosticsInstanceView {
    #[serde(
        rename = "consoleScreenshotBlobUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub console_screenshot_blob_uri: Option<String>,
    #[serde(
        rename = "serialConsoleLogBlobUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub serial_console_log_blob_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceViewStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineHealthStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceViewStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachinePatchStatus {
    #[serde(
        rename = "availablePatchSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub available_patch_summary: Option<AvailablePatchSummary>,
    #[serde(
        rename = "lastPatchInstallationSummary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_patch_installation_summary: Option<LastPatchInstallationSummary>,
    #[serde(
        rename = "configurationStatuses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_statuses: Option<Vec<InstanceViewStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailablePatchSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(
        rename = "assessmentActivityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assessment_activity_id: Option<String>,
    #[serde(
        rename = "rebootPending",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reboot_pending: Option<bool>,
    #[serde(
        rename = "criticalAndSecurityPatchCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub critical_and_security_patch_count: Option<i32>,
    #[serde(
        rename = "otherPatchCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub other_patch_count: Option<i32>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(
        rename = "lastModifiedTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastPatchInstallationSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(
        rename = "installationActivityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub installation_activity_id: Option<String>,
    #[serde(
        rename = "maintenanceWindowExceeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_window_exceeded: Option<bool>,
    #[serde(
        rename = "notSelectedPatchCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_selected_patch_count: Option<i32>,
    #[serde(
        rename = "excludedPatchCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_patch_count: Option<i32>,
    #[serde(
        rename = "pendingPatchCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_patch_count: Option<i32>,
    #[serde(
        rename = "installedPatchCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub installed_patch_count: Option<i32>,
    #[serde(
        rename = "failedPatchCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_patch_count: Option<i32>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(
        rename = "lastModifiedTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceRedeployStatus {
    #[serde(
        rename = "isCustomerInitiatedMaintenanceAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_customer_initiated_maintenance_allowed: Option<bool>,
    #[serde(
        rename = "lastOperationMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_operation_message: Option<String>,
    #[serde(
        rename = "lastOperationResultCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_operation_result_code: Option<String>,
    #[serde(
        rename = "maintenanceWindowEndTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_window_end_time: Option<String>,
    #[serde(
        rename = "maintenanceWindowStartTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_window_start_time: Option<String>,
    #[serde(
        rename = "preMaintenanceWindowEndTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_maintenance_window_end_time: Option<String>,
    #[serde(
        rename = "preMaintenanceWindowStartTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_maintenance_window_start_time: Option<String>,
}
