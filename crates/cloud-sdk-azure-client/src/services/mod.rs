pub mod compute;
pub mod identity;
pub mod networking;
pub mod resource_manager;
pub mod storage;

pub use compute::AzureComputeService;
pub use identity::AzureIdentityService;
pub use networking::AzureNetworkingService;
pub use resource_manager::AzureResourceManagerService;
pub use storage::AzureStorageService;
