use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountIdentity {
    #[serde(rename = "type")]
    pub identity_type: String,
    #[serde(
        rename = "principalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(
        rename = "userAssignedIdentities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_assigned_identities: Option<serde_json::Value>,
}
