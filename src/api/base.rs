//! Base HTTP client abstraction for OpenAI API
//!
//! This module provides common HTTP client functionality to reduce code duplication
//! across all API clients. It includes:
//! - Common HTTP client configuration
//! - Request building utilities
//! - Response handling with error conversion
//! - Header management

use crate::error::{OpenAIError, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::multipart::Form;
use serde::de::DeserializeOwned;

/// Default `OpenAI` API base URL
pub const DEFAULT_BASE_URL: &str = "https://api.openai.com";

/// Common HTTP client for all `OpenAI` API operations
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// The underlying reqwest HTTP client
    client: reqwest::Client,
    /// `OpenAI` API key for authentication
    api_key: String,
    /// Base URL for API requests
    base_url: String,
}

impl HttpClient {
    /// Handle error response by extracting text and parsing as API error
    async fn handle_error_response<T>(
        &self,
        response: reqwest::Response,
        status: reqwest::StatusCode,
    ) -> Result<T> {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        // Try to parse as API error response
        serde_json::from_str::<crate::error::ApiErrorResponse>(&error_text).map_or_else(
            |_| {
                Err(OpenAIError::ApiError {
                    status: status.as_u16(),
                    message: error_text,
                })
            },
            |api_error| Err(OpenAIError::from_api_response(status.as_u16(), api_error)),
        )
    }

    /// Build a URL from the base URL and path
    fn build_simple_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Execute a GET request with the given headers
    async fn execute_get_request<T>(&self, url: &str, headers: HeaderMap) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.client.get(url).headers(headers).send().await?;
        self.handle_response(response).await
    }

    /// Execute a POST request with JSON body and the given headers
    async fn execute_post_request<T, B>(&self, url: &str, headers: HeaderMap, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(body)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Execute a DELETE request with the given headers
    async fn execute_delete_request<T>(&self, url: &str, headers: HeaderMap) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.client.delete(url).headers(headers).send().await?;
        self.handle_response(response).await
    }

    /// Validate API key and return it if valid
    fn validate_api_key<S: Into<String>>(api_key: S) -> Result<String> {
        let api_key = api_key.into();
        if api_key.trim().is_empty() {
            return Err(OpenAIError::authentication("API key cannot be empty"));
        }
        Ok(api_key)
    }

    /// Create a new HTTP client with the given API key
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        let api_key = Self::validate_api_key(api_key)?;
        let client = reqwest::Client::new();
        Ok(Self {
            client,
            api_key,
            base_url: DEFAULT_BASE_URL.to_string(),
        })
    }

    /// Create a new HTTP client with custom base URL
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        let api_key = Self::validate_api_key(api_key)?;
        let client = reqwest::Client::new();
        Ok(Self {
            client,
            api_key,
            base_url: base_url.into(),
        })
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

    /// Get the underlying reqwest client
    #[must_use]
    pub const fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Build authorization header value
    fn build_auth_header(&self) -> Result<HeaderValue> {
        HeaderValue::from_str(&format!("Bearer {}", self.api_key))
            .map_err(|e| OpenAIError::InvalidRequest(format!("Invalid API key format: {e}")))
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

    /// Handle API response and convert to the desired type
    pub async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text).map_err(|e| {
                OpenAIError::ParseError(format!("Failed to parse response: {e}. Response: {text}"))
            })
        } else {
            self.handle_error_response(response, status).await
        }
    }

    /// Make a GET request to the specified path
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_simple_url(path);
        let headers = self.build_headers()?;
        self.execute_get_request(&url, headers).await
    }

    /// Make a GET request with beta headers to the specified path
    pub async fn get_with_beta<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_simple_url(path);
        let headers = self.build_headers_with_beta()?;
        self.execute_get_request(&url, headers).await
    }

    /// Make a POST request with JSON body to the specified path
    #[allow(clippy::future_not_send)]
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let url = self.build_simple_url(path);
        let headers = self.build_headers()?;
        self.execute_post_request(&url, headers, body).await
    }

    /// Make a POST request with JSON body and beta headers to the specified path
    #[allow(clippy::future_not_send)]
    pub async fn post_with_beta<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let url = self.build_simple_url(path);
        let headers = self.build_headers_with_beta()?;
        self.execute_post_request(&url, headers, body).await
    }

    /// Make a DELETE request to the specified path
    pub async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_simple_url(path);
        let headers = self.build_headers()?;
        self.execute_delete_request(&url, headers).await
    }

    /// Make a DELETE request with beta headers to the specified path
    pub async fn delete_with_beta<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_simple_url(path);
        let headers = self.build_headers_with_beta()?;
        self.execute_delete_request(&url, headers).await
    }

    /// Build URL with path and optional query parameters
    #[must_use]
    pub fn build_url(&self, path: &str, query_params: &[(String, String)]) -> String {
        let mut url = format!("{}{}", self.base_url, path);

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

    /// Make a GET request with query parameters
    pub async fn get_with_query<T>(
        &self,
        path: &str,
        query_params: &[(String, String)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(path, query_params);
        let headers = self.build_headers()?;
        self.execute_get_request(&url, headers).await
    }

    /// Make a GET request with query parameters and beta headers
    pub async fn get_with_query_and_beta<T>(
        &self,
        path: &str,
        query_params: &[(String, String)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(path, query_params);
        let headers = self.build_headers_with_beta()?;
        self.execute_get_request(&url, headers).await
    }

    /// Make a POST request with multipart form data
    pub async fn post_multipart<T>(&self, path: &str, form: Form) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_simple_url(path);
        let headers = self.build_auth_headers()?; // Don't set Content-Type for multipart

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a GET request and return raw text content
    pub async fn get_text(&self, path: &str) -> Result<String> {
        let url = self.build_simple_url(path);
        let headers = self.build_headers()?;

        let response = self.client.get(&url).headers(headers).send().await?;

        let status = response.status();
        if status.is_success() {
            response.text().await.map_err(|e| {
                OpenAIError::RequestError(format!("Failed to read response text: {e}"))
            })
        } else {
            self.handle_error_response(response, status).await
        }
    }

    /// Make a GET request and return raw bytes
    pub async fn get_bytes(&self, path: &str) -> Result<Vec<u8>> {
        let url = self.build_simple_url(path);
        let headers = self.build_headers()?;

        let response = self.client.get(&url).headers(headers).send().await?;

        let status = response.status();
        if status.is_success() {
            let bytes = response.bytes().await.map_err(|e| {
                OpenAIError::RequestError(format!("Failed to read response bytes: {e}"))
            })?;
            Ok(bytes.to_vec())
        } else {
            self.handle_error_response(response, status).await
        }
    }
}

/// Request error utilities
#[must_use]
pub fn map_request_error(e: &reqwest::Error, context: &str) -> OpenAIError {
    OpenAIError::RequestError(format!("{context}: {e}"))
}

/// Create standard error mappers for common operations
#[must_use]
pub fn map_parse_error(e: &serde_json::Error, context: &str) -> OpenAIError {
    OpenAIError::ParseError(format!("{context}: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new("test-key").unwrap();
        assert_eq!(client.api_key(), "test-key");
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_http_client_with_custom_base_url() {
        let client = HttpClient::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(client.api_key(), "test-key");
        assert_eq!(client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_build_headers() {
        let client = HttpClient::new("test-key").unwrap();
        let headers = client.build_headers().unwrap();

        assert!(headers.contains_key(AUTHORIZATION));
        assert!(headers.contains_key(CONTENT_TYPE));

        let auth_header = headers.get(AUTHORIZATION).unwrap();
        assert_eq!(auth_header, "Bearer test-key");
    }

    #[test]
    fn test_build_headers_with_beta() {
        let client = HttpClient::new("test-key").unwrap();
        let headers = client.build_headers_with_beta().unwrap();

        assert!(headers.contains_key(AUTHORIZATION));
        assert!(headers.contains_key(CONTENT_TYPE));
        assert!(headers.contains_key("OpenAI-Beta"));

        let beta_header = headers.get("OpenAI-Beta").unwrap();
        assert_eq!(beta_header, "assistants=v2");
    }

    #[test]
    fn test_build_url() {
        let client = HttpClient::new("test-key").unwrap();

        // No query parameters
        let url = client.build_url("/v1/models", &[]);
        assert_eq!(url, "https://api.openai.com/v1/models");

        // With query parameters
        let query_params = vec![
            ("limit".to_string(), "10".to_string()),
            ("order".to_string(), "desc".to_string()),
        ];
        let url = client.build_url("/v1/models", &query_params);
        assert_eq!(url, "https://api.openai.com/v1/models?limit=10&order=desc");
    }
}
