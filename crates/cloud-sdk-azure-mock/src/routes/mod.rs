pub mod blobs;
pub mod compute;
pub mod identity;
pub mod networking;
pub mod oauth;
pub mod resource_groups;
pub mod storage_accounts;
pub mod subscriptions;

use axum::response::Response;
use cloud_sdk_core::error::CloudErrorResponse;
use http::StatusCode;

/// Build an Azure-compatible JSON error response.
pub fn error_response(status: StatusCode, code: &str, message: &str) -> Response {
    let body = CloudErrorResponse::new(code, message);
    let json = serde_json::to_string(&body).unwrap();

    Response::builder()
        .status(status)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(json))
        .unwrap()
}
