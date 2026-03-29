use serde::{Deserialize, Serialize};

use super::common::SubResourceRef;
use super::vnet::AddressSpace;

/// Virtual network peering resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetworkPeering {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkPeeringProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetworkPeeringProperties {
    #[serde(
        rename = "allowVirtualNetworkAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_virtual_network_access: Option<bool>,
    #[serde(
        rename = "allowForwardedTraffic",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_forwarded_traffic: Option<bool>,
    #[serde(
        rename = "allowGatewayTransit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_gateway_transit: Option<bool>,
    #[serde(
        rename = "useRemoteGateways",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_remote_gateways: Option<bool>,
    #[serde(
        rename = "remoteVirtualNetwork",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_virtual_network: Option<SubResourceRef>,
    #[serde(
        rename = "peeringState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peering_state: Option<String>,
    #[serde(
        rename = "peeringSyncLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peering_sync_level: Option<String>,
    #[serde(
        rename = "provisioningState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioning_state: Option<String>,
    #[serde(
        rename = "remoteAddressSpace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_address_space: Option<AddressSpace>,
    #[serde(
        rename = "remoteBgpCommunities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_bgp_communities: Option<serde_json::Value>,
}

/// Parameters for creating a virtual network peering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualNetworkPeeringParams {
    pub properties: VirtualNetworkPeeringProperties,
}
