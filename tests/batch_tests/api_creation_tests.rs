//! Tests for batch API client creation

use openai_rust_sdk::api::{batch::BatchApi, common::ApiClientConstructors};

#[cfg(test)]
mod batch_api_creation_tests {
    use super::*;

    #[test]
    fn test_batch_api_creation() {
        let api = BatchApi::new("test-api-key");
        assert!(api.is_ok());
    }

    #[test]
    fn test_batch_api_creation_with_empty_key() {
        let api = BatchApi::new("");
        assert!(api.is_err());
    }

    #[test]
    fn test_batch_api_with_custom_base_url() {
        let api = BatchApi::new_with_base_url("test-key", "https://custom.api.com");
        assert!(api.is_ok());
    }

    #[test]
    fn test_batch_api_with_invalid_base_url() {
        let api = BatchApi::new_with_base_url("test-key", "");
        assert!(api.is_ok()); // Empty base URL should still work (though not practical)
    }
}
