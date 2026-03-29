use axum::{
    Json,
    extract::{Path, State},
};
use cloud_sdk_core::services::networking::{
    CreateNetworkInterfaceParams, CreateNsgParams, CreatePublicIPAddressParams,
    CreateSecurityRuleParams, CreateSubnetParams, CreateVirtualNetworkParams,
};
use http::StatusCode;
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
