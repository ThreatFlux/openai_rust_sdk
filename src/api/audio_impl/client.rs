//! # Audio API Client
//!
//! Main client implementation for the Audio API.

use super::types::*;

/// Audio API client
pub struct AudioApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for AudioApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl AudioApi {
    /// Get the API key (for testing purposes)
    #[cfg(test)]
    pub fn api_key(&self) -> &str {
        self.http_client.api_key()
    }

    /// Get reference to the HTTP client for use by other modules
    pub(crate) fn http_client(&self) -> &HttpClient {
        &self.http_client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_api_creation() {
        let api = AudioApi::new("test-key".to_string()).unwrap();
        assert_eq!(api.api_key(), "test-key");
    }

    #[test]
    fn test_empty_api_key() {
        let result = AudioApi::new("".to_string());
        assert!(result.is_err());
    }
}
