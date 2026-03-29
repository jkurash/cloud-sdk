use axum::extract::State;
use http::StatusCode;
use std::sync::Arc;

use crate::state::MockState;

/// GET /me — Get current principal (mock endpoint, not a real Azure path)
pub async fn get_current_principal(
    State(state): State<Arc<MockState>>,
) -> axum::response::Response {
    let principal = state.get_current_principal().await;
    let json = serde_json::to_string(&principal).unwrap();
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(json))
        .unwrap()
}

/// GET /providers/Microsoft.Authorization/roleAssignments — List role assignments
pub async fn list_role_assignments(
    State(state): State<Arc<MockState>>,
) -> axum::response::Response {
    let page = state.list_role_assignments("").await;
    let json = serde_json::to_string(&page).unwrap();
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(json))
        .unwrap()
}
