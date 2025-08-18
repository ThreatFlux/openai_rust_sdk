use thiserror::Error;

/// Errors that can occur when using the `OpenAI` client
#[derive(Error, Debug)]
pub enum OpenAIError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON serialization/deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API returned an error: {status_code} - {message}")]
    Api {
        /// HTTP status code from the API
        status_code: u16,
        /// Error message from the API
        message: String,
    },

    /// Authentication failed (invalid API key, etc.)
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Error occurred during streaming
    #[error("Streaming error: {0}")]
    Streaming(String),

    /// Invalid request parameters or format
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Request timed out
    #[error("Timeout occurred: {0}")]
    Timeout(String),

    /// File operation failed
    #[error("File error: {0}")]
    FileError(String),

    /// HTTP request failed (detailed)
    #[error("Request error: {0}")]
    RequestError(String),

    /// Response parsing failed
    #[error("Parse error: {0}")]
    ParseError(String),

    /// API returned an error with status and message
    #[error("API error: {status} - {message}")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message
        message: String,
    },

    /// Unknown or unexpected error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// API error response format
#[derive(serde::Deserialize, Debug)]
pub struct ApiErrorResponse {
    /// The error details
    pub error: ApiError,
}

/// Error details from the API
#[derive(serde::Deserialize, Debug)]
pub struct ApiError {
    /// Human-readable error message
    pub message: String,
    /// Error type classification
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    /// Error code for programmatic handling
    pub code: Option<String>,
}

/// Result type for `OpenAI` operations
pub type Result<T> = std::result::Result<T, OpenAIError>;

impl From<String> for OpenAIError {
    fn from(message: String) -> Self {
        Self::InvalidRequest(message)
    }
}

impl OpenAIError {
    /// Create an API error from a response
    #[must_use]
    pub fn from_api_response(status_code: u16, error_response: ApiErrorResponse) -> Self {
        Self::Api {
            status_code,
            message: error_response.error.message,
        }
    }

    /// Create an authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication(message.into())
    }

    /// Create a streaming error
    pub fn streaming(message: impl Into<String>) -> Self {
        Self::Streaming(message.into())
    }

    /// Create an invalid request error
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::InvalidRequest(message.into())
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::InvalidRequest(message.into())
    }

    /// Create a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Unknown(message.into())
    }

    /// Create a parsing error  
    pub fn parsing(message: impl Into<String>) -> Self {
        Self::InvalidRequest(message.into())
    }

    /// Create an API error with status code and message
    pub fn api_error(status_code: u16, message: impl Into<String>) -> Self {
        Self::Api {
            status_code,
            message: message.into(),
        }
    }
}
