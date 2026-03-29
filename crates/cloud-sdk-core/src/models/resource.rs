use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provisioning state of a cloud resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Succeeded,
    Creating,
    Updating,
    Deleting,
    Failed,
}

/// Common metadata shared by all Azure ARM resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGroup {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    pub properties: ResourceGroupProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGroupProperties {
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
}

/// Parameters for creating or updating a resource group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResourceGroupParams {
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

/// Subscription state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionState {
    Enabled,
    Warned,
    PastDue,
    Disabled,
    Deleted,
}

/// Subscription information matching Azure's response schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub state: SubscriptionState,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    #[serde(
        rename = "subscriptionPolicies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_policies: Option<SubscriptionPolicies>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPolicies {
    #[serde(rename = "locationPlacementId")]
    pub location_placement_id: String,
    #[serde(rename = "quotaId")]
    pub quota_id: String,
    #[serde(rename = "spendingLimit")]
    pub spending_limit: SpendingLimit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpendingLimit {
    On,
    Off,
    CurrentPeriodOff,
}
