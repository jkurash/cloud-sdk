use cloud_sdk_core::error::CloudSdkError;
use cloud_sdk_core::models::Page;
use cloud_sdk_core::services::identity::{IdentityService, Principal, RoleAssignment};
use std::sync::Arc;

use crate::client::AzureClient;

const API_VERSION: &str = "2022-04-01";

pub struct AzureIdentityService {
    client: Arc<AzureClient>,
}

impl AzureIdentityService {
    pub fn new(client: Arc<AzureClient>) -> Self {
        Self { client }
    }
}

impl IdentityService for AzureIdentityService {
    async fn get_current_principal(&self) -> Result<Principal, CloudSdkError> {
        let url = self.client.config().me_url();
        self.client.get(url, API_VERSION).await
    }

    async fn list_role_assignments(
        &self,
        _scope: &str,
    ) -> Result<Page<RoleAssignment>, CloudSdkError> {
        let url = self.client.config().role_assignments_url();
        self.client.get(url, API_VERSION).await
    }
}
