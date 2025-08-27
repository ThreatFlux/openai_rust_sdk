//! Core client implementation for the Threads API
//!
//! This module contains the main ThreadsApi struct and its core functionality,
//! including initialization and common configuration methods.

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::Result;

/// `OpenAI` Threads API client for managing conversation threads and messages
#[derive(Debug, Clone)]
pub struct ThreadsApi {
    /// HTTP client for making API requests
    pub(crate) http_client: HttpClient,
}

impl ApiClientConstructors for ThreadsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl ThreadsApi {
    /// Creates a new Threads API client with custom base URL
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    /// * `base_url` - Custom base URL for the API
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
    ///
    /// let api = ThreadsApi::with_base_url("your-api-key", "https://custom-api.example.com")?;
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// ```
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::common::ApiClientConstructors;

    #[test]
    fn test_threads_api_creation() {
        let api = ThreadsApi::new("test-api-key").unwrap();
        assert_eq!(api.http_client.api_key(), "test-api-key");
        assert_eq!(api.http_client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_threads_api_with_custom_url() {
        let api = ThreadsApi::with_base_url("test-api-key", "https://custom.api.com").unwrap();
        assert_eq!(api.http_client.api_key(), "test-api-key");
        assert_eq!(api.http_client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_create_headers() {
        let api = ThreadsApi::new("test-api-key").unwrap();
        let headers = api.http_client.build_headers_with_beta().unwrap();

        assert!(headers.contains_key("Content-Type"));
        assert!(headers.contains_key("Authorization"));
        assert!(headers.contains_key("OpenAI-Beta"));
    }
}
