//! Common utilities for API clients to reduce code duplication

use crate::api::base::HttpClient;
use crate::error::Result;

/// Common trait for API clients with standard constructors
pub trait ApiClientConstructors: Sized {
    /// Create a new instance with the HTTP client
    fn from_http_client(http_client: HttpClient) -> Self;

    /// Creates a new API client
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your OpenAI API key
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is empty or invalid
    fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self::from_http_client(HttpClient::new(api_key)?))
    }

    /// Creates a new API client with custom base URL
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your OpenAI API key
    /// * `base_url` - Custom base URL for the API
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is empty or invalid
    fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self::from_http_client(HttpClient::new_with_base_url(
            api_key, base_url,
        )?))
    }
}

/// Helper function to build query parameters for list operations
pub fn build_list_query_params<T>(params: &T) -> Vec<(String, String)>
where
    T: ListQueryParams,
{
    let mut query_params = Vec::new();

    if let Some(limit) = params.limit() {
        query_params.push(("limit".to_string(), limit.to_string()));
    }
    if let Some(order) = params.order_str() {
        query_params.push(("order".to_string(), order.to_string()));
    }
    if let Some(after) = params.after() {
        query_params.push(("after".to_string(), after.clone()));
    }
    if let Some(before) = params.before() {
        query_params.push(("before".to_string(), before.clone()));
    }

    query_params
}

/// Build query params from a runs list params struct
pub fn build_runs_query_params(
    params: &crate::models::runs::ListRunsParams,
) -> Vec<(String, String)> {
    let mut query_params = Vec::new();
    if let Some(limit) = params.limit {
        query_params.push(("limit".to_string(), limit.to_string()));
    }
    if let Some(ref order) = params.order {
        query_params.push(("order".to_string(), order.clone()));
    }
    if let Some(ref after) = params.after {
        query_params.push(("after".to_string(), after.clone()));
    }
    if let Some(ref before) = params.before {
        query_params.push(("before".to_string(), before.clone()));
    }
    query_params
}

/// Build query params from a run steps list params struct
pub fn build_run_steps_query_params(
    params: &crate::models::runs::ListRunStepsParams,
) -> Vec<(String, String)> {
    let mut query_params = Vec::new();
    if let Some(limit) = params.limit {
        query_params.push(("limit".to_string(), limit.to_string()));
    }
    if let Some(ref order) = params.order {
        query_params.push(("order".to_string(), order.clone()));
    }
    if let Some(ref after) = params.after {
        query_params.push(("after".to_string(), after.clone()));
    }
    if let Some(ref before) = params.before {
        query_params.push(("before".to_string(), before.clone()));
    }
    query_params
}

/// Common trait for list query parameters
pub trait ListQueryParams {
    /// Get the limit parameter
    fn limit(&self) -> Option<u32>;
    /// Get the order parameter as string
    fn order_str(&self) -> Option<&str>;
    /// Get the after cursor parameter
    fn after(&self) -> Option<&String>;
    /// Get the before cursor parameter
    fn before(&self) -> Option<&String>;
}

/// Standard implementation for common list parameters
#[derive(Debug, Clone, Default)]
pub struct StandardListParams {
    /// Maximum number of items to return
    pub limit: Option<u32>,
    /// Sort order (asc or desc)
    pub order: Option<String>,
    /// Cursor for pagination (return items after this ID)
    pub after: Option<String>,
    /// Cursor for pagination (return items before this ID)
    pub before: Option<String>,
}

impl ListQueryParams for StandardListParams {
    fn limit(&self) -> Option<u32> {
        self.limit
    }

    fn order_str(&self) -> Option<&str> {
        self.order.as_deref()
    }

    fn after(&self) -> Option<&String> {
        self.after.as_ref()
    }

    fn before(&self) -> Option<&String> {
        self.before.as_ref()
    }
}
