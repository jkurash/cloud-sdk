use std::fmt;

/// Unified error type for all cloud-sdk operations.
///
/// Provider-specific errors are captured in the `ProviderError` variant,
/// which carries the exact error code and message from the cloud API
/// (e.g., Azure's `CloudError` format).
#[derive(Debug, thiserror::Error)]
pub enum CloudSdkError {
    #[error("resource not found: {resource_type} '{name}'")]
    NotFound { resource_type: String, name: String },

    #[error("resource already exists: {resource_type} '{name}'")]
    AlreadyExists { resource_type: String, name: String },

    #[error("authentication failed: {message}")]
    AuthenticationError { message: String },

    #[error("authorization denied: {message}")]
    AuthorizationError { message: String },

    #[error("invalid input: {message}")]
    ValidationError { message: String },

    #[error("rate limited, retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },

    #[error("provider error ({provider}): {status} - {message}")]
    ProviderError {
        provider: String,
        status: u16,
        code: String,
        message: String,
    },

    #[error("HTTP transport error: {0}")]
    HttpError(Box<dyn std::error::Error + Send + Sync>),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

/// Result type alias using `CloudSdkError`.
pub type Result<T> = std::result::Result<T, CloudSdkError>;

/// Azure-compatible cloud error response body.
/// Matches: `{ "error": { "code": "...", "message": "...", "details": [...] } }`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CloudErrorResponse {
    pub error: CloudErrorBody,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CloudErrorBody {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[serde(
        default,
        rename = "additionalInfo",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type")]
    pub info_type: String,
    pub info: serde_json::Value,
}

impl CloudErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: CloudErrorBody {
                code: code.into(),
                message: message.into(),
                details: Vec::new(),
                additional_info: Vec::new(),
            },
        }
    }
}

impl fmt::Display for CloudErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error.code, self.error.message)
    }
}
