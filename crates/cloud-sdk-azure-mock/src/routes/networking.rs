use axum::{
    Json,
    extract::{Path, Query, State},
};
use cloud_sdk_core::services::networking::{
    CreateApplicationSecurityGroupParams, CreateNetworkInterfaceParams, CreateNsgParams,
    CreatePublicIPAddressParams, CreateRouteParams, CreateRouteTableParams,
    CreateSecurityRuleParams, CreateSubnetParams, CreateVirtualNetworkParams,
    CreateVirtualNetworkPeeringParams,
};
use http::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;

use super::error_response;
use crate::state::MockState;

// ── Virtual Networks ───────────────────────────────────────────────────

pub async fn create_or_update_vnet(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name)): Path<(String, String, String)>,
    Json(params): Json<CreateVirtualNetworkParams>,
) -> axum::response::Response {
    match state
        .create_virtual_network(&sub_id, &rg, &vnet_name, &params)
        .await
    {
        Ok((vnet, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&vnet).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_vnet(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.get_virtual_network(&sub_id, &rg, &vnet_name).await {
        Some(vnet) => {
            let json = serde_json::to_string(&vnet).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Virtual network '{vnet_name}' not found."),
        ),
    }
}

pub async fn list_vnets(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg)): Path<(String, String)>,
) -> axum::response::Response {
    match state.list_virtual_networks(&sub_id, &rg).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg}' not found."),
        ),
    }
}

pub async fn delete_vnet(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.delete_virtual_network(&sub_id, &rg, &vnet_name).await {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Virtual network '{vnet_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Subnets ────────────────────────────────────────────────────────────

pub async fn create_or_update_subnet(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name, subnet_name)): Path<(String, String, String, String)>,
    Json(params): Json<CreateSubnetParams>,
) -> axum::response::Response {
    match state
        .create_subnet(&sub_id, &rg, &vnet_name, &subnet_name, &params)
        .await
    {
        Ok(subnet) => {
            let json = serde_json::to_string(&subnet).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::CREATED)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_subnet(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name, subnet_name)): Path<(String, String, String, String)>,
) -> axum::response::Response {
    match state
        .get_subnet(&sub_id, &rg, &vnet_name, &subnet_name)
        .await
    {
        Some(subnet) => {
            let json = serde_json::to_string(&subnet).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Subnet '{subnet_name}' not found."),
        ),
    }
}

pub async fn list_subnets(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.list_subnets(&sub_id, &rg, &vnet_name).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Virtual network '{vnet_name}' not found."),
        ),
    }
}

pub async fn delete_subnet(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name, subnet_name)): Path<(String, String, String, String)>,
) -> axum::response::Response {
    match state
        .delete_subnet(&sub_id, &rg, &vnet_name, &subnet_name)
        .await
    {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Subnet '{subnet_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Network Security Groups ────────────────────────────────────────────

pub async fn create_or_update_nsg(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nsg_name)): Path<(String, String, String)>,
    Json(params): Json<CreateNsgParams>,
) -> axum::response::Response {
    match state.create_nsg(&sub_id, &rg, &nsg_name, &params).await {
        Ok((nsg, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&nsg).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_nsg(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nsg_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.get_nsg(&sub_id, &rg, &nsg_name).await {
        Some(nsg) => {
            let json = serde_json::to_string(&nsg).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("NSG '{nsg_name}' not found."),
        ),
    }
}

pub async fn list_nsgs(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg)): Path<(String, String)>,
) -> axum::response::Response {
    match state.list_nsgs(&sub_id, &rg).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg}' not found."),
        ),
    }
}

pub async fn delete_nsg(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nsg_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.delete_nsg(&sub_id, &rg, &nsg_name).await {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("NSG '{nsg_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Security Rules (individual CRUD within NSGs) ──────────────────────

pub async fn create_or_update_security_rule(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nsg_name, rule_name)): Path<(String, String, String, String)>,
    Json(params): Json<CreateSecurityRuleParams>,
) -> axum::response::Response {
    match state
        .create_or_update_security_rule(&sub_id, &rg, &nsg_name, &rule_name, &params)
        .await
    {
        Ok((rule, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&rule).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_security_rule(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nsg_name, rule_name)): Path<(String, String, String, String)>,
) -> axum::response::Response {
    match state
        .get_security_rule(&sub_id, &rg, &nsg_name, &rule_name)
        .await
    {
        Some(rule) => {
            let json = serde_json::to_string(&rule).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Security rule '{rule_name}' not found."),
        ),
    }
}

pub async fn list_security_rules(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nsg_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.list_security_rules(&sub_id, &rg, &nsg_name).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("NSG '{nsg_name}' not found."),
        ),
    }
}

pub async fn delete_security_rule(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nsg_name, rule_name)): Path<(String, String, String, String)>,
) -> axum::response::Response {
    match state
        .delete_security_rule(&sub_id, &rg, &nsg_name, &rule_name)
        .await
    {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Security rule '{rule_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Network Interfaces ────────────────────────────────────────────────

pub async fn create_or_update_nic(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nic_name)): Path<(String, String, String)>,
    Json(params): Json<CreateNetworkInterfaceParams>,
) -> axum::response::Response {
    match state
        .create_network_interface(&sub_id, &rg, &nic_name, &params)
        .await
    {
        Ok((nic, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&nic).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_nic(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nic_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.get_network_interface(&sub_id, &rg, &nic_name).await {
        Some(nic) => {
            let json = serde_json::to_string(&nic).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Network interface '{nic_name}' not found."),
        ),
    }
}

pub async fn list_nics(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg)): Path<(String, String)>,
) -> axum::response::Response {
    match state.list_network_interfaces(&sub_id, &rg).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg}' not found."),
        ),
    }
}

pub async fn delete_nic(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nic_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .delete_network_interface(&sub_id, &rg, &nic_name)
        .await
    {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Network interface '{nic_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Public IP Addresses ───────────────────────────────────────────────

pub async fn create_or_update_public_ip(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, ip_name)): Path<(String, String, String)>,
    Json(params): Json<CreatePublicIPAddressParams>,
) -> axum::response::Response {
    match state
        .create_public_ip_address(&sub_id, &rg, &ip_name, &params)
        .await
    {
        Ok((ip, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&ip).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_public_ip(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, ip_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.get_public_ip_address(&sub_id, &rg, &ip_name).await {
        Some(ip) => {
            let json = serde_json::to_string(&ip).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Public IP address '{ip_name}' not found."),
        ),
    }
}

pub async fn list_public_ips(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg)): Path<(String, String)>,
) -> axum::response::Response {
    match state.list_public_ip_addresses(&sub_id, &rg).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg}' not found."),
        ),
    }
}

pub async fn delete_public_ip(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, ip_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.delete_public_ip_address(&sub_id, &rg, &ip_name).await {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Public IP address '{ip_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── VNet extended operations ──────────────────────────────────────────

pub async fn list_all_vnets(
    State(state): State<Arc<MockState>>,
    Path(sub_id): Path<String>,
) -> axum::response::Response {
    match state.list_all_virtual_networks(&sub_id).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "SubscriptionNotFound",
            &format!("Subscription '{sub_id}' not found."),
        ),
    }
}

pub async fn update_vnet_tags(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name)): Path<(String, String, String)>,
    Json(body): Json<serde_json::Value>,
) -> axum::response::Response {
    let tags: HashMap<String, String> = body
        .get("tags")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    match state
        .update_virtual_network_tags(&sub_id, &rg, &vnet_name, tags)
        .await
    {
        Ok(vnet) => {
            let json = serde_json::to_string(&vnet).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

#[derive(serde::Deserialize)]
pub struct CheckIpQuery {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
}

pub async fn check_ip_availability(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name)): Path<(String, String, String)>,
    Query(query): Query<CheckIpQuery>,
) -> axum::response::Response {
    match state
        .check_ip_availability(&sub_id, &rg, &vnet_name, &query.ip_address)
        .await
    {
        Ok((available, available_ips)) => {
            let body = serde_json::json!({
                "available": available,
                "availableIPAddresses": available_ips,
            });
            let json = serde_json::to_string(&body).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── NSG extended operations ───────────────────────────────────────────

pub async fn list_all_nsgs(
    State(state): State<Arc<MockState>>,
    Path(sub_id): Path<String>,
) -> axum::response::Response {
    match state.list_all_nsgs(&sub_id).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "SubscriptionNotFound",
            &format!("Subscription '{sub_id}' not found."),
        ),
    }
}

pub async fn update_nsg_tags(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, nsg_name)): Path<(String, String, String)>,
    Json(body): Json<serde_json::Value>,
) -> axum::response::Response {
    let tags: HashMap<String, String> = body
        .get("tags")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    match state.update_nsg_tags(&sub_id, &rg, &nsg_name, tags).await {
        Ok(nsg) => {
            let json = serde_json::to_string(&nsg).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Route Tables ──────────────���──────────────────────���────────────────

pub async fn create_or_update_route_table(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, table_name)): Path<(String, String, String)>,
    Json(params): Json<CreateRouteTableParams>,
) -> axum::response::Response {
    match state
        .create_route_table(&sub_id, &rg, &table_name, &params)
        .await
    {
        Ok((table, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&table).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_route_table(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, table_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.get_route_table(&sub_id, &rg, &table_name).await {
        Some(table) => {
            let json = serde_json::to_string(&table).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Route table '{table_name}' not found."),
        ),
    }
}

pub async fn list_route_tables(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg)): Path<(String, String)>,
) -> axum::response::Response {
    match state.list_route_tables(&sub_id, &rg).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg}' not found."),
        ),
    }
}

pub async fn delete_route_table(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, table_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.delete_route_table(&sub_id, &rg, &table_name).await {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Route table '{table_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Routes (within Route Tables) ──────────────────────────────────────

pub async fn create_or_update_route(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, table_name, route_name)): Path<(String, String, String, String)>,
    Json(params): Json<CreateRouteParams>,
) -> axum::response::Response {
    match state
        .create_route(&sub_id, &rg, &table_name, &route_name, &params)
        .await
    {
        Ok((route, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&route).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_route(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, table_name, route_name)): Path<(String, String, String, String)>,
) -> axum::response::Response {
    match state
        .get_route(&sub_id, &rg, &table_name, &route_name)
        .await
    {
        Some(route) => {
            let json = serde_json::to_string(&route).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Route '{route_name}' not found."),
        ),
    }
}

pub async fn list_routes(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, table_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.list_routes(&sub_id, &rg, &table_name).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Route table '{table_name}' not found."),
        ),
    }
}

pub async fn delete_route(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, table_name, route_name)): Path<(String, String, String, String)>,
) -> axum::response::Response {
    match state
        .delete_route(&sub_id, &rg, &table_name, &route_name)
        .await
    {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Route '{route_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Virtual Network Peerings ───────��───────────────────────────��──────

pub async fn create_or_update_peering(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name, peering_name)): Path<(String, String, String, String)>,
    Json(params): Json<CreateVirtualNetworkPeeringParams>,
) -> axum::response::Response {
    match state
        .create_virtual_network_peering(&sub_id, &rg, &vnet_name, &peering_name, &params)
        .await
    {
        Ok((peering, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&peering).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_peering(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name, peering_name)): Path<(String, String, String, String)>,
) -> axum::response::Response {
    match state
        .get_virtual_network_peering(&sub_id, &rg, &vnet_name, &peering_name)
        .await
    {
        Some(peering) => {
            let json = serde_json::to_string(&peering).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Virtual network peering '{peering_name}' not found."),
        ),
    }
}

pub async fn list_peerings(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .list_virtual_network_peerings(&sub_id, &rg, &vnet_name)
        .await
    {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Virtual network '{vnet_name}' not found."),
        ),
    }
}

pub async fn delete_peering(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, vnet_name, peering_name)): Path<(String, String, String, String)>,
) -> axum::response::Response {
    match state
        .delete_virtual_network_peering(&sub_id, &rg, &vnet_name, &peering_name)
        .await
    {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Virtual network peering '{peering_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Application Security Groups ──────────────────────────────────────

pub async fn create_or_update_asg(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, asg_name)): Path<(String, String, String)>,
    Json(params): Json<CreateApplicationSecurityGroupParams>,
) -> axum::response::Response {
    match state
        .create_application_security_group(&sub_id, &rg, &asg_name, &params)
        .await
    {
        Ok((asg, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&asg).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn get_asg(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, asg_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .get_application_security_group(&sub_id, &rg, &asg_name)
        .await
    {
        Some(asg) => {
            let json = serde_json::to_string(&asg).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Application security group '{asg_name}' not found."),
        ),
    }
}

pub async fn list_asgs(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg)): Path<(String, String)>,
) -> axum::response::Response {
    match state.list_application_security_groups(&sub_id, &rg).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg}' not found."),
        ),
    }
}

pub async fn list_all_asgs(
    State(state): State<Arc<MockState>>,
    Path(sub_id): Path<String>,
) -> axum::response::Response {
    match state.list_all_application_security_groups(&sub_id).await {
        Some(page) => {
            let json = serde_json::to_string(&page).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "SubscriptionNotFound",
            &format!("Subscription '{sub_id}' not found."),
        ),
    }
}

pub async fn delete_asg(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, asg_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .delete_application_security_group(&sub_id, &rg, &asg_name)
        .await
    {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceNotFound",
            &format!("Application security group '{asg_name}' not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

pub async fn update_asg_tags(
    State(state): State<Arc<MockState>>,
    Path((sub_id, rg, asg_name)): Path<(String, String, String)>,
    Json(body): Json<serde_json::Value>,
) -> axum::response::Response {
    let tags: HashMap<String, String> = body
        .get("tags")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    match state
        .update_application_security_group_tags(&sub_id, &rg, &asg_name, tags)
        .await
    {
        Ok(asg) => {
            let json = serde_json::to_string(&asg).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

// ── Service Tags ─────────────────────────────────────────────────────

pub async fn list_service_tags(
    State(state): State<Arc<MockState>>,
    Path((_sub_id, location)): Path<(String, String)>,
) -> axum::response::Response {
    let result = state.list_service_tags(&location).await;
    let json = serde_json::to_string(&result).unwrap();
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(json))
        .unwrap()
}
