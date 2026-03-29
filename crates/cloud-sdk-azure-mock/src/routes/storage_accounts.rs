use axum::{
    Json,
    extract::{Path, State},
};
use cloud_sdk_core::services::storage::{
    CheckNameAvailabilityResult, CreateStorageAccountParams,
    StorageAccountCheckNameAvailabilityParameters, StorageAccountKey, StorageAccountListKeysResult,
    StorageAccountRegenerateKeyParameters,
};
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

/// PATCH .../storageAccounts/{name}
pub async fn update(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, account_name)): Path<(String, String, String)>,
    Json(patch): Json<serde_json::Value>,
) -> axum::response::Response {
    match state
        .update_storage_account(&subscription_id, &rg_name, &account_name, patch)
        .await
    {
        Ok(sa) => {
            let json = serde_json::to_string(&sa).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// GET /subscriptions/{sub}/providers/Microsoft.Storage/storageAccounts
pub async fn list_all(
    State(state): State<Arc<MockState>>,
    Path(subscription_id): Path<String>,
) -> axum::response::Response {
    match state.list_all_storage_accounts(&subscription_id).await {
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
            &format!("Subscription '{subscription_id}' not found."),
        ),
    }
}

/// POST /subscriptions/{sub}/providers/Microsoft.Storage/checkNameAvailability
pub async fn check_name_availability(
    State(state): State<Arc<MockState>>,
    Json(params): Json<StorageAccountCheckNameAvailabilityParameters>,
) -> axum::response::Response {
    let (available, reason, message) = state.check_storage_name_availability(&params.name).await;
    let result = CheckNameAvailabilityResult {
        name_available: Some(available),
        reason,
        message,
    };
    let json = serde_json::to_string(&result).unwrap();
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(json))
        .unwrap()
}

/// POST .../storageAccounts/{name}/listKeys
pub async fn list_keys(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, account_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .list_storage_keys(&subscription_id, &rg_name, &account_name)
        .await
    {
        Ok(keys) => {
            let result = StorageAccountListKeysResult {
                keys: Some(
                    keys.into_iter()
                        .map(|(name, value, perms)| StorageAccountKey {
                            key_name: Some(name),
                            value: Some(value),
                            permissions: Some(perms),
                            creation_time: Some(chrono::Utc::now().to_rfc3339()),
                        })
                        .collect(),
                ),
            };
            let json = serde_json::to_string(&result).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// POST .../storageAccounts/{name}/regenerateKey
pub async fn regenerate_key(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, account_name)): Path<(String, String, String)>,
    Json(params): Json<StorageAccountRegenerateKeyParameters>,
) -> axum::response::Response {
    match state
        .list_storage_keys(&subscription_id, &rg_name, &account_name)
        .await
    {
        Ok(mut keys) => {
            for key in &mut keys {
                if key.0 == params.key_name {
                    key.1 = format!(
                        "regen{}{account_name}00000000000000000000000000000000==",
                        params.key_name
                    );
                }
            }
            let result = StorageAccountListKeysResult {
                keys: Some(
                    keys.into_iter()
                        .map(|(name, value, perms)| StorageAccountKey {
                            key_name: Some(name),
                            value: Some(value),
                            permissions: Some(perms),
                            creation_time: Some(chrono::Utc::now().to_rfc3339()),
                        })
                        .collect(),
                ),
            };
            let json = serde_json::to_string(&result).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// POST .../storageAccounts/{name}/ListAccountSas
pub async fn list_account_sas(
    State(_state): State<Arc<MockState>>,
    Path((_subscription_id, _rg_name, account_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    let result = serde_json::json!({
        "accountSasToken": format!("sv=2023-05-01&ss=bfqt&srt=sco&sp=rwdlacup&se=2099-01-01T00:00:00Z&sig=mock-sas-{account_name}")
    });
    let json = serde_json::to_string(&result).unwrap();
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(json))
        .unwrap()
}

/// POST .../storageAccounts/{name}/ListServiceSas
pub async fn list_service_sas(
    State(_state): State<Arc<MockState>>,
    Path((_subscription_id, _rg_name, account_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    let result = serde_json::json!({
        "serviceSasToken": format!("sv=2023-05-01&sr=c&sp=rwdl&se=2099-01-01T00:00:00Z&sig=mock-service-sas-{account_name}")
    });
    let json = serde_json::to_string(&result).unwrap();
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(json))
        .unwrap()
}

/// POST .../storageAccounts/{name}/revokeUserDelegationKeys
pub async fn revoke_user_delegation_keys(
    State(state): State<Arc<MockState>>,
    Path((subscription_id, rg_name, account_name)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state
        .get_storage_account(&subscription_id, &rg_name, &account_name)
        .await
    {
        Some(_) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        None => error_response(
            StatusCode::NOT_FOUND,
            "StorageAccountNotFound",
            &format!("Storage account '{account_name}' not found."),
        ),
    }
}
