use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::SubResourceRef;

/// Route table resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTable {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    pub properties: RouteTableProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTableProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<Route>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<SubResourceRef>>,
    #[serde(
        rename = "disableBgpRoutePropagation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_bgp_route_propagation: Option<bool>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "resourceGuid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_guid: Option<String>,
}

/// Individual route within a route table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    pub properties: RouteProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteProperties {
    #[serde(
        rename = "addressPrefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_prefix: Option<String>,
    #[serde(rename = "nextHopType")]
    pub next_hop_type: String,
    #[serde(
        rename = "nextHopIpAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_hop_ip_address: Option<String>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "hasBgpOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_bgp_override: Option<bool>,
}

/// Parameters for creating a route table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRouteTableParams {
    pub location: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
    pub properties: RouteTableProperties,
}

/// Parameters for creating a route within a route table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRouteParams {
    pub properties: RouteProperties,
}
