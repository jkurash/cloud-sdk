use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Duration;

/// Middleware that adds a configurable delay to every response.
/// Simulates network latency for testing async client behavior.
pub fn make_delay_middleware(
    delay_ms: u64,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
+ Clone
+ Send {
    move |request: Request, next: Next| {
        Box::pin(async move {
            if delay_ms > 0 {
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            }
            next.run(request).await
        })
    }
}
