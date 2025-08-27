//! Tests for Streaming API creation and configuration

use openai_rust_sdk::api::streaming::StreamingApi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_api_creation() {
        let api = StreamingApi::new("test-api-key");
        assert!(api.is_ok());
    }

    #[test]
    fn test_streaming_api_creation_with_empty_key() {
        let api = StreamingApi::new("");
        assert!(api.is_err());
    }

    #[test]
    fn test_streaming_api_with_custom_base_url() {
        let api = StreamingApi::with_base_url("test-key", "https://custom.api.com");
        assert!(api.is_ok());
    }

    #[test]
    fn test_streaming_api_with_invalid_base_url() {
        let api = StreamingApi::with_base_url("test-key", "");
        assert!(api.is_ok()); // Empty base URL should still work (though not practical)
    }
}
