//! Core client implementation for the Moderations API

use crate::api::base::HttpClient;
use crate::error::Result;
use crate::http_post;
use crate::models::moderations::{ModerationRequest, ModerationResponse};

/// Moderations API client
pub struct ModerationsApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl crate::api::common::ApiClientConstructors for ModerationsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl ModerationsApi {
    // Generate HTTP client methods using macro
    http_post!(create_moderation, "/v1/moderations", request: &ModerationRequest, ModerationResponse);
}
