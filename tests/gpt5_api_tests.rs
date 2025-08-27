#![allow(clippy::pedantic, clippy::nursery)]
//! Comprehensive tests for the GPT-5 API
//!
//! This test suite covers all functionality of the GPT-5 API including:
//! - GPT-5 specific features and responses
//! - Different reasoning levels and efforts  
//! - Text configuration and verbosity settings
//! - Model variants (GPT-5, GPT-5-mini, GPT-5-nano)
//! - Builder pattern and fluent interface
//! - Tool calling with GPT-5
//! - Conversation continuation
//! - Error handling and validation

mod gpt5_tests;

// Re-export all test modules for easy access
pub use gpt5_tests::*;

// Note: Tests involving actual API calls would go here but are commented out
// since they require a real API key and network access

/*
#[cfg(test)]
mod api_integration_tests {
    use openai_rust_sdk::api::gpt5::GPT5Api;
    use openai_rust_sdk::models::gpt5::{models, ReasoningEffort, Verbosity};

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY environment variable
    async fn test_gpt5_minimal_response() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let api = GPT5Api::new(api_key).unwrap();

        let result = api.create_minimal_response(
            models::GPT_5,
            "What is 2+2?"
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY environment variable
    async fn test_gpt5_with_reasoning() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let api = GPT5Api::new(api_key).unwrap();

        let result = api.create_reasoned_response(
            models::GPT_5,
            "Explain quantum computing",
            ReasoningEffort::High,
            Verbosity::Medium
        ).await;

        assert!(result.is_ok());
    }
}
*/
