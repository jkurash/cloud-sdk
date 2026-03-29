use axum::extract::{Form, Path};
use http::StatusCode;
use serde::{Deserialize, Serialize};

/// POST /{tenantId}/oauth2/v2.0/token — Mock OAuth2 token endpoint
///
/// Accepts client_credentials grant type and returns a mock access token.
/// Mimics the response from `https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token`.
pub async fn token(
    Path(_tenant_id): Path<String>,
    Form(params): Form<TokenRequest>,
) -> axum::response::Response {
    // Validate grant type
    if params.grant_type != "client_credentials" {
        let body = serde_json::json!({
            "error": "unsupported_grant_type",
            "error_description": "Only client_credentials grant type is supported by the mock."
        });
        return axum::response::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&body).unwrap(),
            ))
            .unwrap();
    }

    // Validate client_id and client_secret are present
    if params.client_id.is_empty() || params.client_secret.is_empty() {
        let body = serde_json::json!({
            "error": "invalid_client",
            "error_description": "client_id and client_secret are required."
        });
        return axum::response::Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&body).unwrap(),
            ))
            .unwrap();
    }

    let response = TokenResponse {
        access_token: format!("mock-token-for-{}", params.client_id),
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        ext_expires_in: 3600,
    };

    let json = serde_json::to_string(&response).unwrap();
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(json))
        .unwrap()
}

#[derive(Deserialize)]
pub struct TokenRequest {
    #[serde(default)]
    pub grant_type: String,
    #[serde(default)]
    pub client_id: String,
    #[serde(default)]
    pub client_secret: String,
    #[serde(default)]
    pub scope: String,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    ext_expires_in: u64,
}
