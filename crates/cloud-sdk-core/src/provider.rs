use crate::services::{
    ComputeService, IdentityService, NetworkingService, ResourceManagerService, StorageService,
};

/// A cloud provider that vends service-specific clients.
///
/// Uses associated types for static dispatch and zero-cost abstraction.
/// Implementations: `AzureProvider` (cloud-sdk-azure-client), future: `AwsProvider`, `GcpProvider`.
pub trait CloudProvider: Send + Sync {
    type Compute: ComputeService;
    type Storage: StorageService;
    type Networking: NetworkingService;
    type Identity: IdentityService;
    type ResourceManager: ResourceManagerService;

    fn compute(&self) -> &Self::Compute;
    fn storage(&self) -> &Self::Storage;
    fn networking(&self) -> &Self::Networking;
    fn identity(&self) -> &Self::Identity;
    fn resource_manager(&self) -> &Self::ResourceManager;

    /// Provider name for diagnostics (e.g., "azure", "aws", "mock").
    fn name(&self) -> &str;
}
