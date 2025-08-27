//! # Models API Client
//!
//! Core client functionality for accessing OpenAI's Models API.

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::Result;
use crate::http_get;
use crate::models::models::{ListModelsResponse, Model};

/// Models API client for accessing `OpenAI` model information
pub struct ModelsApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for ModelsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl ModelsApi {
    /// Create a new Models API client
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
        })
    }

    /// Create a new client with custom base URL
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Get the API key (for testing purposes)
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.http_client.api_key()
    }

    // Generate HTTP client methods using macro
    http_get!(list_models, "/v1/models", ListModelsResponse);
    http_get!(retrieve_model, "/v1/models/{}", model_id: impl AsRef<str>, Model);
}
