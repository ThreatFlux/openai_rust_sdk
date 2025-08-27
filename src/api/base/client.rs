//! Core HTTP client implementation for OpenAI API

use crate::api::base::config::ClientConfig;
use crate::error::{OpenAIError, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;

/// Common HTTP client for all OpenAI API operations
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// The underlying reqwest HTTP client
    client: reqwest::Client,
    /// Client configuration
    config: ClientConfig,
}

impl HttpClient {
    /// Create a new HTTP client with the given API key
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        let config = ClientConfig::new(api_key)?;
        Ok(Self {
            client: reqwest::Client::new(),
            config,
        })
    }

    /// Create a new HTTP client with custom base URL
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        let config = ClientConfig::new_with_base_url(api_key, base_url)?;
        Ok(Self {
            client: reqwest::Client::new(),
            config,
        })
    }

    /// Get the API key
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.config.api_key()
    }

    /// Get the base URL
    #[must_use]
    pub fn base_url(&self) -> &str {
        self.config.base_url()
    }

    /// Get the underlying reqwest client
    #[must_use]
    pub const fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Build authorization header value
    fn build_auth_header(&self) -> Result<HeaderValue> {
        HeaderValue::from_str(&format!("Bearer {}", self.config.api_key()))
            .map_err(crate::invalid_request_err!("Invalid API key format: {}"))
    }

    /// Build standard headers for API requests
    pub fn build_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, self.build_auth_header()?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok(headers)
    }

    /// Build headers without Content-Type (for multipart requests)
    pub fn build_auth_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, self.build_auth_header()?);
        Ok(headers)
    }

    /// Build headers with additional OpenAI-Beta header for assistant APIs
    pub fn build_headers_with_beta(&self) -> Result<HeaderMap> {
        let mut headers = self.build_headers()?;
        headers.insert("OpenAI-Beta", HeaderValue::from_static("assistants=v2"));
        Ok(headers)
    }

    /// Build a URL from the base URL and path
    pub fn build_simple_url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url(), path)
    }

    /// Build URL with path and optional query parameters
    #[must_use]
    pub fn build_url(&self, path: &str, query_params: &[(String, String)]) -> String {
        let mut url = format!("{}{}", self.config.base_url(), path);

        if !query_params.is_empty() {
            url.push('?');
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            url.push_str(&query_string);
        }

        url
    }
}
