use serde::{Deserialize, Serialize};

use super::identity::SubResource;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledEventsPolicy {
    #[serde(
        rename = "userInitiatedReboot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_initiated_reboot: Option<UserInitiatedReboot>,
    #[serde(
        rename = "userInitiatedRedeploy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_initiated_redeploy: Option<UserInitiatedRedeploy>,
    #[serde(
        rename = "scheduledEventsAdditionalPublishingTargets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_events_additional_publishing_targets:
        Option<ScheduledEventsAdditionalPublishingTargets>,
    #[serde(
        rename = "eventGridAndResourceGraph",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_grid_and_resource_graph: Option<EventGridAndResourceGraph>,
    #[serde(
        rename = "allInstancesDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_instances_down: Option<AllInstancesDown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInitiatedReboot {
    #[serde(
        rename = "automaticallyApproveDelay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automatically_approve_delay: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInitiatedRedeploy {
    #[serde(
        rename = "automaticallyApproveDelay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automatically_approve_delay: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGridAndResourceGraph {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledEventsAdditionalPublishingTargets {
    #[serde(
        rename = "eventGridAndResourceGraph",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_grid_and_resource_graph: Option<EventGridAndResourceGraph>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllInstancesDown {
    #[serde(
        rename = "delayDurationInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delay_duration_in_minutes: Option<i32>,
}
