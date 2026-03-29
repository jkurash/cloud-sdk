// cloud-sdk: facade crate re-exporting core + provider crates via feature flags

pub use cloud_sdk_core::*;

#[cfg(feature = "azure")]
pub use cloud_sdk_azure_client;

#[cfg(feature = "azure-mock")]
pub use cloud_sdk_azure_mock;
