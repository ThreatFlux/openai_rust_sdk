//! Common API client trait and implementations
//!
//! This module provides a shared trait and implementations for all OpenAI API clients
//! to reduce code duplication and standardize the client interface.

use crate::api::base::HttpClient;
use crate::error::Result;

/// Common trait for all `OpenAI` API clients
pub trait ApiClient {
    /// Create a new API client with the given API key
    fn new<S: Into<String>>(api_key: S) -> Result<Self>
    where
        Self: Sized;

    /// Create a new API client with custom base URL
    fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self>
    where
        Self: Sized;

    /// Get the underlying HTTP client
    fn http_client(&self) -> &HttpClient;

    /// Get the API key
    fn api_key(&self) -> &str {
        self.http_client().api_key()
    }

    /// Get the base URL
    fn base_url(&self) -> &str {
        self.http_client().base_url()
    }
}

/// Standard implementation for API clients that use the `HttpClient`
#[derive(Debug, Clone)]
pub struct StandardApiClient {
    /// HTTP client for making API requests
    pub(crate) http_client: HttpClient,
}

impl StandardApiClient {
    /// Create a new standard API client
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
        })
    }

    /// Create a new standard API client with custom base URL
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Get the HTTP client
    #[must_use]
    pub const fn http_client(&self) -> &HttpClient {
        &self.http_client
    }
}

impl ApiClient for StandardApiClient {
    #[allow(clippy::use_self)]
    fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        StandardApiClient::new(api_key)
    }

    #[allow(clippy::use_self)]
    fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        StandardApiClient::new_with_base_url(api_key, base_url)
    }

    fn http_client(&self) -> &HttpClient {
        &self.http_client
    }
}

/// Macro to implement the `ApiClient` trait for a struct that contains an `HttpClient`
#[macro_export]
macro_rules! impl_api_client {
    ($client_type:ty, $http_client_field:ident) => {
        impl $crate::api::client::ApiClient for $client_type {
            fn new<S: Into<String>>(api_key: S) -> $crate::error::Result<Self> {
                Ok(Self {
                    $http_client_field: $crate::api::base::HttpClient::new(api_key)?,
                })
            }

            fn new_with_base_url<S: Into<String>>(
                api_key: S,
                base_url: S,
            ) -> $crate::error::Result<Self> {
                Ok(Self {
                    $http_client_field: $crate::api::base::HttpClient::new_with_base_url(
                        api_key, base_url,
                    )?,
                })
            }

            fn http_client(&self) -> &$crate::api::base::HttpClient {
                &self.$http_client_field
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_api_client() {
        let client = StandardApiClient::new("test-key").unwrap();
        assert_eq!(client.api_key(), "test-key");
        assert_eq!(client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_standard_api_client_with_custom_base_url() {
        let client =
            StandardApiClient::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(client.api_key(), "test-key");
        assert_eq!(client.base_url(), "https://custom.api.com");
    }
}
