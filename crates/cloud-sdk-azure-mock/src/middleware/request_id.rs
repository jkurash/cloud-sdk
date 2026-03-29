use axum::{extract::Request, middleware::Next, response::Response};
use uuid::Uuid;

/// Middleware that adds Azure-style response headers:
/// - `x-ms-request-id`: unique UUID per request
/// - `x-ms-correlation-id`: unique UUID per request
/// - `Content-Type: application/json` (only if not already set by the handler)
pub async fn add_response_headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    headers.insert(
        "x-ms-request-id",
        Uuid::new_v4().to_string().parse().unwrap(),
    );
    headers.insert(
        "x-ms-correlation-id",
        Uuid::new_v4().to_string().parse().unwrap(),
    );
    // Only set Content-Type if the handler didn't already set one
    if !headers.contains_key(http::header::CONTENT_TYPE) {
        headers.insert(
            http::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
    }

    response
}
