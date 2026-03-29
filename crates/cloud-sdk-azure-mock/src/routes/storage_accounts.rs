use axum::{
    Json,
    extract::{Path, State},
};
use cloud_sdk_core::services::storage::CreateStorageAccountParams;
use http::StatusCode;
use std::sync::Arc;

use super::error_response;
use crate::state::MockState;

/// PUT /subscriptions/{sub}/resourcegroups/{rg}/providers/Microsoft.Storage/storageAccounts/{name}
pub async fn create_or_update(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, account_name)): Path<(String, String, String)>,
    Json(params): Json<CreateStorageAccountParams>,
) -> axum::response::Response {
    match state
        .create_storage_account(&subscription_id, &rg_name, &account_name, &params)
        .await
    {
        Ok((sa, is_new)) => {
            let status = if is_new {
                StatusCode::CREATED
            } else {
                StatusCode::OK
            };
            let json = serde_json::to_string(&sa).unwrap();
            axum::response::Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// GET /subscriptions/{sub}/resourcegroups/{rg}/providers/Microsoft.Storage/storageAccounts/{name}
pub async fn get(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, account_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .get_storage_account(&subscription_id, &rg_name, &account_name)
        .await
    {
        Some(sa) => {
            let json = serde_json::to_string(&sa).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "StorageAccountNotFound",
            &format!("The storage account '{account_name}' was not found."),
        ),
    }
}

/// GET /subscriptions/{sub}/resourcegroups/{rg}/providers/Microsoft.Storage/storageAccounts
pub async fn list(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name)): Path<(String, String)>,
) -> axum::response::Response {
    match state
        .list_storage_accounts(&subscription_id, &rg_name)
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
            "ResourceGroupNotFound",
            &format!("Resource group '{rg_name}' could not be found."),
        ),
    }
}

/// DELETE /subscriptions/{sub}/resourcegroups/{rg}/providers/Microsoft.Storage/storageAccounts/{name}
pub async fn delete(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, account_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .delete_storage_account(&subscription_id, &rg_name, &account_name)
        .await
    {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "StorageAccountNotFound",
            &format!("The storage account '{account_name}' was not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}
