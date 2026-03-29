use serde::{Deserialize, Serialize};

/// Result of List Keys operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountListKeysResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<StorageAccountKey>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountKey {
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(
        rename = "creationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_time: Option<String>,
}

/// Result of Check Name Availability operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(
        rename = "nameAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Request body for Check Name Availability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountCheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
}

/// Request body for Regenerate Key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountRegenerateKeyParameters {
    #[serde(rename = "keyName")]
    pub key_name: String,
}

/// Request body for List Account SAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSasParameters {
    #[serde(rename = "signedServices")]
    pub signed_services: String,
    #[serde(rename = "signedResourceTypes")]
    pub signed_resource_types: String,
    #[serde(rename = "signedPermission")]
    pub signed_permission: String,
    #[serde(rename = "signedExpiry")]
    pub signed_expiry: String,
    #[serde(
        rename = "signedStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_start: Option<String>,
    #[serde(rename = "signedIp", default, skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[serde(
        rename = "signedProtocol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_protocol: Option<String>,
    #[serde(rename = "keyToSign", default, skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
}

/// Response for List Account SAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccountSasResponse {
    #[serde(
        rename = "accountSasToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_sas_token: Option<String>,
}

/// Request body for List Service SAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSasParameters {
    #[serde(rename = "canonicalizedResource")]
    pub canonicalized_resource: String,
    #[serde(rename = "signedResource")]
    pub signed_resource: String,
    #[serde(rename = "signedPermission")]
    pub signed_permission: String,
    #[serde(rename = "signedExpiry")]
    pub signed_expiry: String,
    #[serde(
        rename = "signedStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_start: Option<String>,
    #[serde(rename = "signedIp", default, skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[serde(
        rename = "signedProtocol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_protocol: Option<String>,
    #[serde(rename = "keyToSign", default, skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
}

/// Response for List Service SAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListServiceSasResponse {
    #[serde(
        rename = "serviceSasToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_sas_token: Option<String>,
}
