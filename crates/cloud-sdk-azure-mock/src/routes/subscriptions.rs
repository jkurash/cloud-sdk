use axum::{
    Json,
    extract::{Path, State},
};
use cloud_sdk_core::models::{Page, resource::Subscription};
use http::StatusCode;
use std::sync::Arc;

use super::error_response;
use crate::state::MockState;

/// GET /subscriptions
pub async fn list_subscriptions(
    State(state): State<Arc<MockState>>,
) -> (StatusCode, Json<Page<Subscription>>) {
    let page = state.list_subscriptions().await;
    (StatusCode::OK, Json(page))
}

/// GET /subscriptions/{subscriptionId}
pub async fn get_subscription(
    State(state): State<Arc<MockState>>,
    Path(subscription_id): Path<String>,
) -> axum::response::Response {
    match state.get_subscription(&subscription_id).await {
        Some(sub) => {
            let json = serde_json::to_string(&sub).unwrap();
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
