//! GPT-5 API creation tests

use openai_rust_sdk::api::gpt5::GPT5Api;
use openai_rust_sdk::error::OpenAIError;

#[cfg(test)]
mod gpt5_api_creation_tests {
    use super::*;

    #[test]
    fn test_gpt5_api_creation() {
        let api = GPT5Api::new("test-api-key".to_string());
        assert!(api.is_ok());
    }

    #[test]
    fn test_gpt5_api_creation_with_empty_key() {
        let api = GPT5Api::new("".to_string());
        assert!(api.is_err());
        if let Err(OpenAIError::Authentication(msg)) = api {
            assert!(msg.contains("API key cannot be empty"));
        } else {
            panic!("Expected Authentication error");
        }
    }
}
