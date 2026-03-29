use crate::error::CloudSdkError;
use crate::models::Page;
use serde::{Deserialize, Serialize};

/// Security principal (user, service principal, managed identity).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "type")]
    pub principal_type: PrincipalType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrincipalType {
    User,
    ServicePrincipal,
    ManagedIdentity,
    Group,
}

/// Role assignment resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignment {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub properties: RoleAssignmentProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignmentProperties {
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(
        rename = "principalType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_type: Option<String>,
    pub scope: String,
}

/// Operations for identity and access management.
pub trait IdentityService: Send + Sync {
    fn get_current_principal(
        &self,
    ) -> impl std::future::Future<Output = Result<Principal, CloudSdkError>> + Send;

    fn list_role_assignments(
        &self,
        scope: &str,
    ) -> impl std::future::Future<Output = Result<Page<RoleAssignment>, CloudSdkError>> + Send;
}
