//! # Moderations API
//!
//! This module provides access to OpenAI's moderations API for classifying
//! content according to OpenAI's usage policies.

pub mod client;
pub mod operations;
pub mod types;

pub use client::ModerationsApi;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::common::ApiClientConstructors;

    #[test]
    fn test_moderation_api_creation() {
        let api = ModerationsApi::new("test-key");
        assert!(api.is_ok());
    }

    #[test]
    fn test_moderation_api_empty_key() {
        let api = ModerationsApi::new("");
        assert!(api.is_err());
    }

    #[test]
    fn test_moderation_api_whitespace_key() {
        let api = ModerationsApi::new("   ");
        assert!(api.is_err());
    }

    #[test]
    fn test_moderation_request_creation() {
        use crate::models::moderations::{ModerationInput, ModerationRequest};
        let request = ModerationRequest::new("test text");
        match request.input {
            ModerationInput::String(s) => assert_eq!(s, "test text"),
            _ => panic!("Expected single string input"),
        }
    }

    #[test]
    fn test_moderation_batch_request() {
        use crate::models::moderations::{ModerationInput, ModerationRequest};
        let texts = vec!["text1".to_string(), "text2".to_string()];
        let request = ModerationRequest::new_batch(texts.clone());
        match request.input {
            ModerationInput::StringArray(arr) => assert_eq!(arr.len(), 2),
            _ => panic!("Expected array input"),
        }
    }

    #[test]
    fn test_moderation_with_model() {
        use crate::models::moderations::ModerationRequest;
        let request = ModerationRequest::new("test").with_model("text-moderation-stable");
        assert_eq!(request.model, Some("text-moderation-stable".to_string()));
    }
}
