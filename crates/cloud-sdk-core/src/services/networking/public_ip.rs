use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::SubResourceRef;

/// Public IP address resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicIPAddress {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PublicIPAddressSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
    pub properties: PublicIPAddressProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicIPAddressProperties {
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "publicIPAllocationMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ip_allocation_method: Option<String>,
    #[serde(
        rename = "publicIPAddressVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ip_address_version: Option<String>,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(
        rename = "idleTimeoutInMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub idle_timeout_in_minutes: Option<i32>,
    #[serde(
        rename = "dnsSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_settings: Option<PublicIPAddressDnsSettings>,
    #[serde(
        rename = "ipConfiguration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_configuration: Option<SubResourceRef>,
    #[serde(
        rename = "resourceGuid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_guid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicIPAddressDnsSettings {
    #[serde(
        rename = "domainNameLabel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub domain_name_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(
        rename = "reverseFqdn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reverse_fqdn: Option<String>,
    #[serde(
        rename = "domainNameLabelScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub domain_name_label_scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicIPAddressSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// Parameters for creating a public IP address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePublicIPAddressParams {
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PublicIPAddressSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
    pub properties: PublicIPAddressProperties,
}
