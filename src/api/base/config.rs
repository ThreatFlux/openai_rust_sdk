//! Configuration utilities for the HTTP client

use crate::error::{OpenAIError, Result};

/// Default OpenAI API base URL
pub const DEFAULT_BASE_URL: &str = "https://api.openai.com";

/// Configuration builder for HTTP client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// OpenAI API key for authentication
    pub api_key: String,
    /// Base URL for API requests
    pub base_url: String,
}

impl ClientConfig {
    /// Create a new client configuration with the given API key
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        let api_key = Self::validate_api_key(api_key)?;
        Ok(Self {
            api_key,
            base_url: DEFAULT_BASE_URL.to_string(),
        })
    }

    /// Create a new client configuration with custom base URL
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        let api_key = Self::validate_api_key(api_key)?;
        Ok(Self {
            api_key,
            base_url: base_url.into(),
        })
    }

    /// Validate API key and return it if valid
    fn validate_api_key<S: Into<String>>(api_key: S) -> Result<String> {
        let api_key = api_key.into();
        if api_key.trim().is_empty() {
            return Err(OpenAIError::authentication("API key cannot be empty"));
        }
        Ok(api_key)
    }

    /// Set a custom base URL
    pub fn with_base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Get the API key
    #[must_use]
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get the base URL
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

/// Trait for request types that can be validated
pub trait Validate {
    /// Validates the request and returns an error message if invalid
    fn validate(&self) -> std::result::Result<(), String>;
}

/// Validates a request object that implements a validate method
///
/// This helper consolidates the common pattern of calling `validate()` on request objects
/// and mapping validation errors to `OpenAIError::InvalidRequest`.
///
/// # Arguments
///
/// * `request` - Any object that implements a `validate()` method returning `Result<(), String>`
///
/// # Returns
///
/// * `Result<()>` - Ok if validation passes, InvalidRequest error if validation fails
///
/// # Example
///
/// ```rust,ignore
/// validate_request(&request)?;
/// ```
pub fn validate_request<T>(request: &T) -> Result<()>
where
    T: Validate,
{
    request.validate().map_err(OpenAIError::InvalidRequest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_new() {
        let config = ClientConfig::new("test-key").unwrap();
        assert_eq!(config.api_key(), "test-key");
        assert_eq!(config.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_config_with_base_url() {
        let config = ClientConfig::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(config.api_key(), "test-key");
        assert_eq!(config.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_client_config_empty_api_key() {
        let result = ClientConfig::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_request_success() {
        struct TestRequest {
            valid: bool,
        }

        impl Validate for TestRequest {
            fn validate(&self) -> std::result::Result<(), String> {
                if self.valid {
                    Ok(())
                } else {
                    Err("Invalid request".to_string())
                }
            }
        }

        let valid_request = TestRequest { valid: true };
        let result = validate_request(&valid_request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_request_failure() {
        struct TestRequest {
            valid: bool,
        }

        impl Validate for TestRequest {
            fn validate(&self) -> std::result::Result<(), String> {
                if self.valid {
                    Ok(())
                } else {
                    Err("Invalid request".to_string())
                }
            }
        }

        let invalid_request = TestRequest { valid: false };
        let result = validate_request(&invalid_request);
        assert!(result.is_err());

        if let Err(e) = result {
            if let OpenAIError::InvalidRequest(msg) = e {
                assert_eq!(msg, "Invalid request");
            } else {
                panic!("Expected InvalidRequest error, got: {:?}", e);
            }
        }
    }
}
