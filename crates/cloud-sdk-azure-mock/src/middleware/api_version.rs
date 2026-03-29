use axum::{extract::Request, middleware::Next, response::Response};

use crate::routes::error_response;

/// Middleware that validates the `api-version` query parameter is present.
/// Returns 400 if missing, matching Azure behavior.
pub async fn validate_api_version(request: Request, next: Next) -> Response {
    let has_api_version = request
        .uri()
        .query()
        .map(|q| url::form_urlencoded::parse(q.as_bytes()).any(|(k, _)| k == "api-version"))
        .unwrap_or(false);

    if !has_api_version {
        return error_response(
            http::StatusCode::BAD_REQUEST,
            "MissingApiVersionParameter",
            "The api-version query parameter (?api-version=) is required for all requests.",
        );
    }

    next.run(request).await
}
