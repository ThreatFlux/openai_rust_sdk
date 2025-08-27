//! # Images API
//!
//! This module provides access to OpenAI's Images API for generating,
//! editing, and creating variations of images using DALL-E models.

mod builders;
mod operations;
mod utilities;

pub use builders::*;
pub use operations::*;
pub use utilities::*;

use crate::api::base::HttpClient;

/// Images API client
pub struct ImagesApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl crate::api::common::ApiClientConstructors for ImagesApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl ImagesApi {
    /// Get the API key (for testing purposes)
    #[cfg(test)]
    pub fn api_key(&self) -> &str {
        self.http_client.api_key()
    }
}
