use std::sync::Arc;

use crate::client::AzureClient;
use crate::services::{
    AzureComputeService, AzureIdentityService, AzureNetworkingService, AzureResourceManagerService,
    AzureStorageService,
};

/// Azure implementation of `CloudProvider`.
///
/// Implements all five service traits: ResourceManager, Storage, Compute,
/// Networking, and Identity.
pub struct AzureProvider {
    resource_manager: AzureResourceManagerService,
    storage: AzureStorageService,
    compute: AzureComputeService,
    networking: AzureNetworkingService,
    identity: AzureIdentityService,
}

impl AzureProvider {
    pub fn new(client: AzureClient) -> Self {
        let client = Arc::new(client);
        Self {
            resource_manager: AzureResourceManagerService::new(client.clone()),
            storage: AzureStorageService::new(client.clone()),
            compute: AzureComputeService::new(client.clone()),
            networking: AzureNetworkingService::new(client.clone()),
            identity: AzureIdentityService::new(client.clone()),
        }
    }

    pub fn resource_manager(&self) -> &AzureResourceManagerService {
        &self.resource_manager
    }

    pub fn storage(&self) -> &AzureStorageService {
        &self.storage
    }

    pub fn compute(&self) -> &AzureComputeService {
        &self.compute
    }

    pub fn networking(&self) -> &AzureNetworkingService {
        &self.networking
    }

    pub fn identity(&self) -> &AzureIdentityService {
        &self.identity
    }
}
