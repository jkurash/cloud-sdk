use axum::{
    body::Bytes,
    extract::{Path, State},
};
use http::StatusCode;
use std::sync::Arc;

use super::error_response;
use crate::state::MockState;

// ── Container operations ───────────────────────────────────────────────
// PUT  /{account}/{container}?restype=container         → Create Container
// DELETE /{account}/{container}?restype=container        → Delete Container
// GET  /{account}/{container}?restype=container&comp=list → List Blobs
// GET  /{account}?comp=list                              → List Containers

/// GET /{account}?comp=list — List Containers
pub async fn list_containers(
    State(state): State<Arc<MockState>>,
    Path(account): Path<String>,
) -> axum::response::Response {
    match state.list_containers(&account).await {
        Some(containers) => {
            // Azure returns XML for blob data plane, but we use JSON for simplicity
            let body = serde_json::json!({ "containers": containers });
            let json = serde_json::to_string(&body).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "AccountNotFound",
            &format!("The storage account '{account}' was not found."),
        ),
    }
}

/// PUT /{account}/{container}?restype=container — Create Container
pub async fn create_container(
    State(state): State<Arc<MockState>>,
    Path((account, container)): Path<(String, String)>,
) -> axum::response::Response {
    match state.create_container(&account, &container).await {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::CREATED)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "AccountNotFound", &msg),
    }
}

/// DELETE /{account}/{container}?restype=container — Delete Container
pub async fn delete_container(
    State(state): State<Arc<MockState>>,
    Path((account, container)): Path<(String, String)>,
) -> axum::response::Response {
    match state.delete_container(&account, &container).await {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::ACCEPTED)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "ContainerNotFound",
            &format!("The container '{container}' was not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "AccountNotFound", &msg),
    }
}

/// GET /{account}/{container}?restype=container&comp=list — List Blobs
pub async fn list_blobs(
    State(state): State<Arc<MockState>>,
    Path((account, container)): Path<(String, String)>,
) -> axum::response::Response {
    match state.list_blobs(&account, &container).await {
        Some(blobs) => {
            let body = serde_json::json!({ "blobs": blobs });
            let json = serde_json::to_string(&body).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ContainerNotFound",
            &format!("The container '{container}' in account '{account}' was not found."),
        ),
    }
}

// ── Blob operations ────────────────────────────────────────────────────
// PUT    /{account}/{container}/{blob}  → Put Blob
// GET    /{account}/{container}/{blob}  → Get Blob
// DELETE /{account}/{container}/{blob}  → Delete Blob
// HEAD   /{account}/{container}/{blob}  → Get Blob Properties

/// PUT /{account}/{container}/{blob} — Put Blob
pub async fn put_blob(
    State(state): State<Arc<MockState>>,
    Path((account, container, blob)): Path<(String, String, String)>,
    headers: http::HeaderMap,
    body: Bytes,
) -> axum::response::Response {
    let content_type = headers
        .get(http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");

    match state
        .put_blob(&account, &container, &blob, body, Some(content_type))
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::CREATED)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// GET /{account}/{container}/{blob} — Get Blob
pub async fn get_blob(
    State(state): State<Arc<MockState>>,
    Path((account, container, blob)): Path<(String, String, String)>,
) -> axum::response::Response {
    // Get properties for content-type header
    let props = state.get_blob_properties(&account, &container, &blob).await;
    match state.get_blob(&account, &container, &blob).await {
        Some(data) => {
            let content_type = props
                .and_then(|p| p.content_type)
                .unwrap_or_else(|| "application/octet-stream".to_string());
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, content_type)
                .header(http::header::CONTENT_LENGTH, data.len())
                .body(axum::body::Body::from(data))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "BlobNotFound",
            &format!("The blob '{blob}' was not found in container '{container}'."),
        ),
    }
}

/// DELETE /{account}/{container}/{blob} — Delete Blob
pub async fn delete_blob(
    State(state): State<Arc<MockState>>,
    Path((account, container, blob)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.delete_blob(&account, &container, &blob).await {
        Ok(true) => axum::response::Response::builder()
            .status(StatusCode::ACCEPTED)
            .body(axum::body::Body::empty())
            .unwrap(),
        Ok(false) => error_response(
            StatusCode::NOT_FOUND,
            "BlobNotFound",
            &format!("The blob '{blob}' was not found."),
        ),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

/// HEAD /{account}/{container}/{blob} — Get Blob Properties
pub async fn head_blob(
    State(state): State<Arc<MockState>>,
    Path((account, container, blob)): Path<(String, String, String)>,
) -> axum::response::Response {
    match state.get_blob_properties(&account, &container, &blob).await {
        Some(props) => {
            let content_type = props
                .content_type
                .unwrap_or_else(|| "application/octet-stream".to_string());
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, content_type)
                .header(http::header::CONTENT_LENGTH, props.content_length)
                .header("x-ms-blob-content-length", props.content_length.to_string())
                .body(axum::body::Body::empty())
                .unwrap()
        }
        None => axum::response::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .unwrap(),
    }
}
