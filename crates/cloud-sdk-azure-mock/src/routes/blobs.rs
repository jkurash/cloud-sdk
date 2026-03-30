use axum::{
    body::Bytes,
    extract::{Path, Query, State},
};
use http::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

use super::error_response;
use crate::state::MockState;

/// Query parameters for blob data plane dispatch.
#[derive(Deserialize, Default)]
pub struct BlobQueryParams {
    #[serde(default)]
    pub comp: Option<String>,
    #[serde(default)]
    pub restype: Option<String>,
    #[serde(default)]
    pub blockid: Option<String>,
}

// ── Container operations ───────────────────────────────────────────────
// PUT  /{account}/{container}?restype=container         → Create Container
// PUT  /{account}/{container}?comp=metadata             → Set Container Metadata
// DELETE /{account}/{container}?restype=container        → Delete Container
// GET  /{account}/{container}?restype=container&comp=list → List Blobs
// GET  /{account}/{container}?comp=metadata             → Get Container Metadata
// HEAD /{account}/{container}                           → Get Container Properties
// GET  /{account}?comp=list                              → List Containers

/// GET /{account} — dispatches on restype+comp query params.
/// restype=service&comp=properties → Get Service Properties
/// restype=account&comp=properties → Get Account Information
/// default / comp=list → List Containers
pub async fn get_account(
    State(state): State<Arc<MockState>>,
    Path(account): Path<String>,
    Query(params): Query<BlobQueryParams>,
) -> axum::response::Response {
    match (params.restype.as_deref(), params.comp.as_deref()) {
        (Some("service"), Some("properties")) => {
            get_service_properties_handler(&state, &account).await
        }
        (Some("account"), Some("properties")) => get_account_info_handler(&account).await,
        _ => list_containers_handler(&state, &account).await,
    }
}

/// PUT /{account} — dispatches on restype+comp query params.
/// restype=service&comp=properties → Set Service Properties
pub async fn put_account(
    State(state): State<Arc<MockState>>,
    Path(account): Path<String>,
    Query(params): Query<BlobQueryParams>,
    body: Bytes,
) -> axum::response::Response {
    match (params.restype.as_deref(), params.comp.as_deref()) {
        (Some("service"), Some("properties")) => {
            set_service_properties_handler(&state, &account, body).await
        }
        _ => error_response(
            StatusCode::BAD_REQUEST,
            "InvalidQueryParameterValue",
            "Unsupported operation",
        ),
    }
}

async fn list_containers_handler(state: &MockState, account: &str) -> axum::response::Response {
    match state.list_containers(account).await {
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

async fn get_service_properties_handler(
    state: &MockState,
    account: &str,
) -> axum::response::Response {
    match state.get_service_properties(account).await {
        Some(props) => {
            let json = serde_json::to_string(&props).unwrap();
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

async fn set_service_properties_handler(
    state: &MockState,
    account: &str,
    body: Bytes,
) -> axum::response::Response {
    let props: cloud_sdk_core::services::storage::StorageServiceProperties =
        match serde_json::from_slice(&body) {
            Ok(p) => p,
            Err(e) => {
                return error_response(
                    StatusCode::BAD_REQUEST,
                    "InvalidInput",
                    &format!("Invalid service properties: {e}"),
                );
            }
        };
    match state.set_service_properties(account, props).await {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::ACCEPTED)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "AccountNotFound", &msg),
    }
}

async fn get_account_info_handler(_account: &str) -> axum::response::Response {
    // Return mock account info headers matching Azure's response
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("x-ms-sku-name", "Standard_LRS")
        .header("x-ms-account-kind", "StorageV2")
        .header("x-ms-is-hns-enabled", "false")
        .header(http::header::CONTENT_LENGTH, "0")
        .body(axum::body::Body::empty())
        .unwrap()
}

/// PUT /{account}/{container} — dispatches on comp query param.
/// No comp → Create Container. comp=metadata → Set Container Metadata.
pub async fn put_container(
    State(state): State<Arc<MockState>>,
    Path((account, container)): Path<(String, String)>,
    Query(params): Query<BlobQueryParams>,
    headers: http::HeaderMap,
) -> axum::response::Response {
    match params.comp.as_deref() {
        Some("metadata") => {
            set_container_metadata_handler(&state, &account, &container, &headers).await
        }
        _ => create_container_handler(&state, &account, &container).await,
    }
}

async fn create_container_handler(
    state: &MockState,
    account: &str,
    container: &str,
) -> axum::response::Response {
    match state.create_container(account, container).await {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::CREATED)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "AccountNotFound", &msg),
    }
}

async fn set_container_metadata_handler(
    state: &MockState,
    account: &str,
    container: &str,
    headers: &http::HeaderMap,
) -> axum::response::Response {
    let metadata = extract_metadata_from_headers(headers);
    match state
        .set_container_metadata(account, container, metadata)
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ContainerNotFound", &msg),
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

/// GET /{account}/{container} — dispatches on comp query param.
/// comp=list or comp=none → List Blobs. comp=metadata → Get Container Metadata.
pub async fn get_container(
    State(state): State<Arc<MockState>>,
    Path((account, container)): Path<(String, String)>,
    Query(params): Query<BlobQueryParams>,
) -> axum::response::Response {
    match params.comp.as_deref() {
        Some("metadata") => get_container_metadata_handler(&state, &account, &container).await,
        _ => list_blobs_handler(&state, &account, &container).await,
    }
}

async fn list_blobs_handler(
    state: &MockState,
    account: &str,
    container: &str,
) -> axum::response::Response {
    match state.list_blobs(account, container).await {
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

async fn get_container_metadata_handler(
    state: &MockState,
    account: &str,
    container: &str,
) -> axum::response::Response {
    match state.get_container_metadata(account, container).await {
        Some(metadata) => {
            let mut builder = axum::response::Response::builder().status(StatusCode::OK);
            for (key, value) in &metadata {
                builder = builder.header(format!("x-ms-meta-{key}"), value.as_str());
            }
            builder.body(axum::body::Body::empty()).unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "ContainerNotFound",
            &format!("The container '{container}' in account '{account}' was not found."),
        ),
    }
}

/// HEAD /{account}/{container} — Get Container Properties
pub async fn head_container(
    State(state): State<Arc<MockState>>,
    Path((account, container)): Path<(String, String)>,
) -> axum::response::Response {
    match state.get_container_properties(&account, &container).await {
        Some(props) => {
            let mut builder = axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(
                    "x-ms-lease-status",
                    props.lease_status.as_deref().unwrap_or("unlocked"),
                )
                .header(
                    "x-ms-lease-state",
                    props.lease_state.as_deref().unwrap_or("available"),
                )
                .header(
                    "x-ms-has-immutability-policy",
                    props.has_immutability_policy.unwrap_or(false).to_string(),
                )
                .header(
                    "x-ms-has-legal-hold",
                    props.has_legal_hold.unwrap_or(false).to_string(),
                );
            if let Some(ref etag) = props.etag {
                builder = builder.header("etag", etag.as_str());
            }
            if let Some(ref last_modified) = props.last_modified {
                builder = builder.header("last-modified", last_modified.as_str());
            }
            for (key, value) in &props.metadata {
                builder = builder.header(format!("x-ms-meta-{key}"), value.as_str());
            }
            builder.body(axum::body::Body::empty()).unwrap()
        }
        None => axum::response::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .unwrap(),
    }
}

// ── Blob operations ────────────────────────────────────────────────────
// PUT    /{account}/{container}/{blob}               → Put Blob
// PUT    /{account}/{container}/{blob}?comp=metadata  → Set Blob Metadata
// PUT    /{account}/{container}/{blob}?comp=tags      → Set Blob Tags
// PUT    /{account}/{container}/{blob}?comp=properties → Set Blob Properties
// GET    /{account}/{container}/{blob}               → Get Blob
// GET    /{account}/{container}/{blob}?comp=metadata  → Get Blob Metadata
// GET    /{account}/{container}/{blob}?comp=tags      → Get Blob Tags
// DELETE /{account}/{container}/{blob}               → Delete Blob
// HEAD   /{account}/{container}/{blob}               → Get Blob Properties

/// PUT /{account}/{container}/{blob} — dispatches on comp query param.
/// No comp + x-ms-copy-source header → Copy Blob
/// No comp + no copy header → Put Blob
/// comp=snapshot → Snapshot Blob
/// comp=metadata → Set Blob Metadata
/// comp=tags → Set Blob Tags
/// comp=properties → Set Blob Properties
pub async fn put_blob(
    State(state): State<Arc<MockState>>,
    Path((account, container, blob)): Path<(String, String, String)>,
    Query(params): Query<BlobQueryParams>,
    headers: http::HeaderMap,
    body: Bytes,
) -> axum::response::Response {
    match params.comp.as_deref() {
        Some("metadata") => {
            set_blob_metadata_handler(&state, &account, &container, &blob, &headers).await
        }
        Some("tags") => set_blob_tags_handler(&state, &account, &container, &blob, body).await,
        Some("properties") => {
            set_blob_properties_handler(&state, &account, &container, &blob, &headers).await
        }
        Some("snapshot") => snapshot_blob_handler(&state, &account, &container, &blob).await,
        Some("block") => {
            put_block_handler(&state, &account, &container, &blob, &params, body).await
        }
        Some("blocklist") => {
            put_block_list_handler(&state, &account, &container, &blob, &headers, body).await
        }
        Some("tier") => set_blob_tier_handler(&state, &account, &container, &blob, &headers).await,
        _ => {
            // Check for copy source header
            if headers.get("x-ms-copy-source").is_some() {
                copy_blob_handler(&state, &account, &container, &blob, &headers).await
            } else {
                put_blob_handler(&state, &account, &container, &blob, &headers, body).await
            }
        }
    }
}

async fn put_blob_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
    headers: &http::HeaderMap,
    body: Bytes,
) -> axum::response::Response {
    let content_type = headers
        .get(http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");

    match state
        .put_blob(account, container, blob, body, Some(content_type))
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::CREATED)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

async fn copy_blob_handler(
    state: &MockState,
    account: &str,
    dest_container: &str,
    dest_blob: &str,
    headers: &http::HeaderMap,
) -> axum::response::Response {
    let copy_source = match headers
        .get("x-ms-copy-source")
        .and_then(|v| v.to_str().ok())
    {
        Some(s) => s.to_string(),
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MissingRequiredHeader",
                "x-ms-copy-source header is required",
            );
        }
    };

    // Parse the copy source URL to extract account/container/blob
    // Format: http://host:port/{account}/{container}/{blob}
    let (source_account, source_container, source_blob) = match parse_copy_source_url(&copy_source)
    {
        Some(parts) => parts,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "InvalidHeaderValue",
                "Cannot parse x-ms-copy-source URL",
            );
        }
    };

    match state
        .copy_blob(
            account,
            dest_container,
            dest_blob,
            &source_account,
            &source_container,
            &source_blob,
        )
        .await
    {
        Ok(copy_id) => axum::response::Response::builder()
            .status(StatusCode::ACCEPTED)
            .header("x-ms-copy-id", &copy_id)
            .header("x-ms-copy-status", "success")
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

async fn snapshot_blob_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
) -> axum::response::Response {
    if !state.blob_exists(account, container, blob).await {
        return error_response(
            StatusCode::NOT_FOUND,
            "BlobNotFound",
            &format!("The blob '{blob}' was not found in container '{container}'."),
        );
    }

    // Return a snapshot datetime identifier (Azure format: 2024-01-15T12:00:00.0000000Z)
    let now = chrono::Utc::now();
    let snapshot = now.format("%Y-%m-%dT%H:%M:%S").to_string()
        + &format!(".{:07}Z", now.timestamp_subsec_nanos() / 100);
    axum::response::Response::builder()
        .status(StatusCode::CREATED)
        .header("x-ms-snapshot", &snapshot)
        .body(axum::body::Body::empty())
        .unwrap()
}

/// Parse a copy source URL like `http://127.0.0.1:PORT/account/container/blob`
/// Returns (account, container, blob).
fn parse_copy_source_url(url: &str) -> Option<(String, String, String)> {
    // Parse as URL, then extract path segments
    let parsed = url::Url::parse(url).ok()?;
    let path = parsed.path().trim_start_matches('/');
    let parts: Vec<&str> = path.splitn(3, '/').collect();
    if parts.len() == 3 {
        Some((
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
        ))
    } else {
        None
    }
}

async fn set_blob_metadata_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
    headers: &http::HeaderMap,
) -> axum::response::Response {
    let metadata = extract_metadata_from_headers(headers);
    match state
        .set_blob_metadata(account, container, blob, metadata)
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "BlobNotFound", &msg),
    }
}

async fn set_blob_tags_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
    body: Bytes,
) -> axum::response::Response {
    // Tags come as JSON body: { "blobTagSet": [{"key":"k","value":"v"}, ...] }
    // or as a flat map: { "tag1": "val1", ... }
    let tags: HashMap<String, String> = if body.is_empty() {
        HashMap::new()
    } else {
        // Try parsing as BlobTags format first
        if let Ok(blob_tags) =
            serde_json::from_slice::<cloud_sdk_core::services::storage::BlobTags>(&body)
        {
            blob_tags
                .blob_tag_set
                .unwrap_or_default()
                .into_iter()
                .map(|t| (t.key, t.value))
                .collect()
        } else if let Ok(flat) = serde_json::from_slice::<HashMap<String, String>>(&body) {
            flat
        } else {
            return error_response(
                StatusCode::BAD_REQUEST,
                "InvalidInput",
                "Invalid tags format",
            );
        }
    };

    match state.set_blob_tags(account, container, blob, tags).await {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "BlobNotFound", &msg),
    }
}

async fn set_blob_properties_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
    headers: &http::HeaderMap,
) -> axum::response::Response {
    let content_type = headers
        .get("x-ms-blob-content-type")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let content_encoding = headers
        .get("x-ms-blob-content-encoding")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let content_language = headers
        .get("x-ms-blob-content-language")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let content_disposition = headers
        .get("x-ms-blob-content-disposition")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let cache_control = headers
        .get("x-ms-blob-cache-control")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    match state
        .set_blob_properties(
            account,
            container,
            blob,
            content_type,
            content_encoding,
            content_language,
            content_disposition,
            cache_control,
        )
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "BlobNotFound", &msg),
    }
}

/// GET /{account}/{container}/{blob} — dispatches on comp query param.
pub async fn get_blob(
    State(state): State<Arc<MockState>>,
    Path((account, container, blob)): Path<(String, String, String)>,
    Query(params): Query<BlobQueryParams>,
) -> axum::response::Response {
    match params.comp.as_deref() {
        Some("metadata") => get_blob_metadata_handler(&state, &account, &container, &blob).await,
        Some("tags") => get_blob_tags_handler(&state, &account, &container, &blob).await,
        Some("blocklist") => get_block_list_handler(&state, &account, &container, &blob).await,
        _ => get_blob_data_handler(&state, &account, &container, &blob).await,
    }
}

async fn get_blob_data_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
) -> axum::response::Response {
    // Get properties for content-type header
    let props = state.get_blob_properties(account, container, blob).await;
    match state.get_blob(account, container, blob).await {
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

async fn get_blob_metadata_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
) -> axum::response::Response {
    match state.get_blob_metadata(account, container, blob).await {
        Some(metadata) => {
            let mut builder = axum::response::Response::builder().status(StatusCode::OK);
            for (key, value) in &metadata {
                builder = builder.header(format!("x-ms-meta-{key}"), value.as_str());
            }
            builder.body(axum::body::Body::empty()).unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "BlobNotFound",
            &format!("The blob '{blob}' was not found in container '{container}'."),
        ),
    }
}

async fn get_blob_tags_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
) -> axum::response::Response {
    match state.get_blob_tags(account, container, blob).await {
        Some(tags) => {
            let blob_tag_set: Vec<cloud_sdk_core::services::storage::BlobTag> = tags
                .into_iter()
                .map(|(key, value)| cloud_sdk_core::services::storage::BlobTag { key, value })
                .collect();
            let body = cloud_sdk_core::services::storage::BlobTags {
                blob_tag_set: Some(blob_tag_set),
            };
            let json = serde_json::to_string(&body).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
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
            let mut builder = axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, &content_type)
                .header(http::header::CONTENT_LENGTH, props.content_length)
                .header("x-ms-blob-content-length", props.content_length.to_string());

            if let Some(ref blob_type) = props.blob_type {
                builder = builder.header("x-ms-blob-type", blob_type.as_str());
            }
            if let Some(ref access_tier) = props.access_tier {
                builder = builder.header("x-ms-access-tier", access_tier.as_str());
            }
            if let Some(ref lease_status) = props.lease_status {
                builder = builder.header("x-ms-lease-status", lease_status.as_str());
            }
            if let Some(ref lease_state) = props.lease_state {
                builder = builder.header("x-ms-lease-state", lease_state.as_str());
            }
            if let Some(encrypted) = props.server_encrypted {
                builder = builder.header("x-ms-server-encrypted", encrypted.to_string());
            }
            if let Some(ref etag) = props.etag {
                builder = builder.header("etag", etag.as_str());
            }
            if let Some(ref last_modified) = props.last_modified {
                builder = builder.header("last-modified", last_modified.as_str());
            }
            if let Some(ref creation_time) = props.creation_time {
                builder = builder.header("x-ms-creation-time", creation_time.as_str());
            }
            if let Some(ref content_encoding) = props.content_encoding {
                builder = builder.header("x-ms-blob-content-encoding", content_encoding.as_str());
            }
            if let Some(ref content_language) = props.content_language {
                builder = builder.header("x-ms-blob-content-language", content_language.as_str());
            }
            if let Some(ref content_disposition) = props.content_disposition {
                builder = builder.header(
                    "x-ms-blob-content-disposition",
                    content_disposition.as_str(),
                );
            }
            if let Some(ref cache_control) = props.cache_control {
                builder = builder.header("x-ms-blob-cache-control", cache_control.as_str());
            }
            if let Some(ref content_md5) = props.content_md5 {
                builder = builder.header("content-md5", content_md5.as_str());
            }
            for (key, value) in &props.metadata {
                builder = builder.header(format!("x-ms-meta-{key}"), value.as_str());
            }

            builder.body(axum::body::Body::empty()).unwrap()
        }
        None => axum::response::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .unwrap(),
    }
}

// ── Block blob handlers ───────────────────────────────────────────────

async fn put_block_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
    params: &BlobQueryParams,
    body: Bytes,
) -> axum::response::Response {
    let block_id = match params.blockid.as_deref() {
        Some(id) => id,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MissingRequiredQueryParameter",
                "blockid query parameter is required for comp=block",
            );
        }
    };

    match state
        .put_block(account, container, blob, block_id, body)
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::CREATED)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "ResourceNotFound", &msg),
    }
}

async fn put_block_list_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
    headers: &http::HeaderMap,
    body: Bytes,
) -> axum::response::Response {
    // Accept JSON body: { "blockIds": ["id1", "id2", ...] }
    #[derive(Deserialize)]
    struct BlockListBody {
        #[serde(alias = "blockIds", alias = "block_ids")]
        block_ids: Vec<String>,
    }

    let block_list: BlockListBody = match serde_json::from_slice(&body) {
        Ok(bl) => bl,
        Err(e) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "InvalidInput",
                &format!("Invalid block list body: {e}"),
            );
        }
    };

    let content_type = headers
        .get(http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok());

    match state
        .put_block_list(account, container, blob, block_list.block_ids, content_type)
        .await
    {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::CREATED)
            .header("etag", format!("\"0x{}\"", uuid::Uuid::new_v4().simple()))
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::BAD_REQUEST, "InvalidBlockList", &msg),
    }
}

async fn get_block_list_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
) -> axum::response::Response {
    match state.get_block_list(account, container, blob).await {
        Some((committed, uncommitted)) => {
            let committed_json: Vec<serde_json::Value> = committed
                .into_iter()
                .map(|(name, size)| serde_json::json!({ "name": name, "size": size }))
                .collect();
            let uncommitted_json: Vec<serde_json::Value> = uncommitted
                .into_iter()
                .map(|(name, size)| serde_json::json!({ "name": name, "size": size }))
                .collect();
            let body = serde_json::json!({
                "committedBlocks": committed_json,
                "uncommittedBlocks": uncommitted_json,
            });
            let json = serde_json::to_string(&body).unwrap();
            axum::response::Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json))
                .unwrap()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            "BlobNotFound",
            &format!("The blob '{blob}' was not found in container '{container}'."),
        ),
    }
}

async fn set_blob_tier_handler(
    state: &MockState,
    account: &str,
    container: &str,
    blob: &str,
    headers: &http::HeaderMap,
) -> axum::response::Response {
    let tier = match headers
        .get("x-ms-access-tier")
        .and_then(|v| v.to_str().ok())
    {
        Some(t) => t.to_string(),
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MissingRequiredHeader",
                "x-ms-access-tier header is required",
            );
        }
    };

    match state.set_blob_tier(account, container, blob, &tier).await {
        Ok(()) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::empty())
            .unwrap(),
        Err(msg) => error_response(StatusCode::NOT_FOUND, "BlobNotFound", &msg),
    }
}

/// Extract x-ms-meta-* headers into a HashMap.
fn extract_metadata_from_headers(headers: &http::HeaderMap) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    for (name, value) in headers {
        let name_str = name.as_str();
        if let Some(key) = name_str.strip_prefix("x-ms-meta-") {
            if let Ok(val) = value.to_str() {
                metadata.insert(key.to_string(), val.to_string());
            }
        }
    }
    metadata
}
