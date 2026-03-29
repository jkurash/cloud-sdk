use axum::{extract::Request, middleware::Next, response::Response};

use crate::routes::error_response;

/// Middleware that validates the `Authorization: Bearer <token>` header is present.
/// Accepts any token value — does not perform real RBAC.
pub async fn validate_auth(request: Request, next: Next) -> Response {
    let has_bearer = request
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.starts_with("Bearer "));

    if !has_bearer {
        return error_response(
            http::StatusCode::UNAUTHORIZED,
            "AuthenticationFailed",
            "The request did not have a valid Authorization header with a Bearer token.",
        );
    }

    next.run(request).await
}
