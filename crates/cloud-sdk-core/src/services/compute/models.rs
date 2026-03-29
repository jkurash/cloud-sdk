use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::{Placement, SystemData};
use super::diagnostics::DiagnosticsProfile;
use super::identity::{
    ExtendedLocation, Plan, SubResource, VirtualMachineExtension, VirtualMachineIdentity,
};
use super::instance_view::VirtualMachineInstanceView;
use super::network::NetworkProfile;
use super::os::OsProfile;
use super::scheduling::{
    AdditionalCapabilities, ApplicationProfile, BillingProfile, CapacityReservationProfile,
    ScheduledEventsPolicy, ScheduledEventsProfile,
};
use super::security::SecurityProfile;
use super::storage::{HardwareProfile, StorageProfile};

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
    #[serde(
        rename = "systemData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_data: Option<SystemData>,
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
    pub instance_view: Option<VirtualMachineInstanceView>,
    #[serde(
        rename = "platformFaultDomain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub platform_fault_domain: Option<i32>,
    #[serde(
        rename = "scheduledEventsPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_events_policy: Option<ScheduledEventsPolicy>,
}
