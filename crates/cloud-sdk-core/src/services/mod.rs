pub mod compute;
pub mod identity;
pub mod networking;
pub mod resource_manager;
pub mod storage;

// Re-export service traits and commonly-used types at the services level.
// For type-specific imports, use the full path: e.g., services::compute::VirtualMachine
pub use compute::ComputeService;
pub use identity::IdentityService;
pub use networking::NetworkingService;
pub use resource_manager::ResourceManagerService;
pub use storage::StorageService;
