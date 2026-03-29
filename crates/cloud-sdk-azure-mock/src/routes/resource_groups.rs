use axum::{
    Json,
    extract::{Path, State},
};
use cloud_sdk_core::models::resource::CreateResourceGroupParams;
use http::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;

use super::error_response;
use crate::state::MockState;

/// PUT /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}
pub async fn create_or_update(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name)): Path<(String, String)>,
    Json(params): Json<CreateResourceGroupParams>,
) -> axum::response::Response {
    match state
        .create_resource_group(&subscription_id, &rg_name, &params)
        .await
    {
        Ok((rg, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&rg).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "SubscriptionNotFound", &msg),
    }
}

/// GET /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}
pub async fn get(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name)): Path<(String, String)>,
) -> axum::response::Response {
    match state.get_resource_group(&subscription_id, &rg_name).await {
        Some(rg) => {
            let json = serde_json::to_string(&rg).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg_name}' could not be found."),
        ),
    }
}

/// GET /subscriptions/{subscriptionId}/resourcegroups
pub async fn list(
    State(state): State<Arc<MockState>>,
    Path(subscription_id): Path<String>,
) -> axum::response::Response {
    match state.list_resource_groups(&subscription_id).await {
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
            &format!("The subscription '{subscription_id}' could not be found."),
        ),
    }
}

/// DELETE /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}
pub async fn delete(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name)): Path<(String, String)>,
) -> axum::response::Response {
    match state
        .delete_resource_group(&subscription_id, &rg_name)
        .await
    {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg_name}' could not be found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "SubscriptionNotFound", &msg),
    }
}

/// PATCH /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}
pub async fn update(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name)): Path<(String, String)>,
    Json(body): Json<PatchResourceGroupBody>,
) -> axum::response::Response {
    match state
        .update_resource_group(&subscription_id, &rg_name, body.tags)
        .await
    {
        Some(rg) => {
            let json = serde_json::to_string(&rg).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ResourceGroupNotFound",
            &format!("Resource group '{rg_name}' could not be found."),
        ),
    }
}

/// HEAD /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}
pub async fn check_existence(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name)): Path<(String, String)>,
) -> StatusCode {
    if state
        .resource_group_exists(&subscription_id, &rg_name)
        .await
    {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Request body for PATCH resource group (only tags can be updated).
#[derive(serde::Deserialize)]
pub struct PatchResourceGroupBody {
    #[serde(default)]
    pub tags: Option<HashMap<String, String>>,
}
