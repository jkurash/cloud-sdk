use serde::{Deserialize, Serialize};

use super::encryption::Encryption;
use super::endpoints::StorageEndpoints;
use super::network::NetworkRuleSet;

/// Full StorageAccountProperties matching Azure REST API 2023-05-01.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountProperties {
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "creationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_time: Option<String>,
    #[serde(
        rename = "primaryLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_location: Option<String>,
    #[serde(
        rename = "secondaryLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_location: Option<String>,
    #[serde(
        rename = "statusOfPrimary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_of_primary: Option<String>,
    #[serde(
        rename = "statusOfSecondary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_of_secondary: Option<String>,
    #[serde(
        rename = "primaryEndpoints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_endpoints: Option<StorageEndpoints>,
    #[serde(
        rename = "secondaryEndpoints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_endpoints: Option<StorageEndpoints>,

    // Access & authentication
    #[serde(
        rename = "accessTier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_tier: Option<String>,
    #[serde(
        rename = "allowBlobPublicAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_blob_public_access: Option<bool>,
    #[serde(
        rename = "allowSharedKeyAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_shared_key_access: Option<bool>,
    #[serde(
        rename = "allowCrossTenantReplication",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_cross_tenant_replication: Option<bool>,
    #[serde(
        rename = "defaultToOAuthAuthentication",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_to_oauth_authentication: Option<bool>,
    #[serde(
        rename = "allowedCopyScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_copy_scope: Option<String>,

    // TLS & HTTPS
    #[serde(
        rename = "minimumTlsVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_tls_version: Option<String>,
    #[serde(
        rename = "supportsHttpsTrafficOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_https_traffic_only: Option<bool>,

    // Network
    #[serde(
        rename = "networkAcls",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_acls: Option<NetworkRuleSet>,
    #[serde(
        rename = "publicNetworkAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_network_access: Option<String>,
    #[serde(
        rename = "dnsEndpointType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_endpoint_type: Option<String>,

    // Routing
    #[serde(
        rename = "routingPreference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub routing_preference: Option<RoutingPreference>,

    // Encryption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,

    // Custom domain
    #[serde(
        rename = "customDomain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_domain: Option<CustomDomain>,

    // Azure Files auth
    #[serde(
        rename = "azureFilesIdentityBasedAuthentication",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_files_identity_based_authentication: Option<AzureFilesIdentityBasedAuthentication>,

    // Feature flags
    #[serde(
        rename = "isHnsEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_hns_enabled: Option<bool>,
    #[serde(
        rename = "isSftpEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_sftp_enabled: Option<bool>,
    #[serde(
        rename = "isNfsV3Enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_nfs_v3_enabled: Option<bool>,
    #[serde(
        rename = "isLocalUserEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_local_user_enabled: Option<bool>,
    #[serde(
        rename = "enableExtendedGroups",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_extended_groups: Option<bool>,
    #[serde(
        rename = "largeFileSharesState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub large_file_shares_state: Option<String>,

    // Immutability
    #[serde(
        rename = "immutableStorageWithVersioning",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub immutable_storage_with_versioning: Option<ImmutableStorageAccount>,

    // Key & SAS policy
    #[serde(rename = "keyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub key_policy: Option<KeyPolicy>,
    #[serde(rename = "sasPolicy", default, skip_serializing_if = "Option::is_none")]
    pub sas_policy: Option<SasPolicy>,
    #[serde(
        rename = "keyCreationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_creation_time: Option<KeyCreationTime>,

    // Failover
    #[serde(
        rename = "failoverInProgress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failover_in_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingPreference {
    #[serde(
        rename = "routingChoice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub routing_choice: Option<String>,
    #[serde(
        rename = "publishMicrosoftEndpoints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publish_microsoft_endpoints: Option<bool>,
    #[serde(
        rename = "publishInternetEndpoints",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publish_internet_endpoints: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDomain {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "useSubDomainName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_sub_domain_name: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureFilesIdentityBasedAuthentication {
    #[serde(rename = "directoryServiceOptions")]
    pub directory_service_options: String,
    #[serde(
        rename = "activeDirectoryProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_directory_properties: Option<ActiveDirectoryProperties>,
    #[serde(
        rename = "defaultSharePermission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_share_permission: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveDirectoryProperties {
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[serde(
        rename = "netBiosDomainName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub net_bios_domain_name: Option<String>,
    #[serde(
        rename = "forestName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forest_name: Option<String>,
    #[serde(rename = "domainGuid")]
    pub domain_guid: String,
    #[serde(rename = "domainSid", default, skip_serializing_if = "Option::is_none")]
    pub domain_sid: Option<String>,
    #[serde(
        rename = "azureStorageSid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_storage_sid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmutableStorageAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPolicy {
    #[serde(rename = "keyExpirationPeriodInDays")]
    pub key_expiration_period_in_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SasPolicy {
    #[serde(rename = "sasExpirationPeriod")]
    pub sas_expiration_period: String,
    #[serde(
        rename = "expirationAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCreationTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
