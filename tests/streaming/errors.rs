//! Tests for error handling in streaming API

use openai_rust_sdk::error::OpenAIError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_error_variants() {
        let request_error = OpenAIError::RequestError("Connection failed".to_string());
        let parse_error = OpenAIError::ParseError("Invalid JSON".to_string());
        let streaming_error = OpenAIError::streaming("Stream interrupted".to_string());

        // Test error message content
        match request_error {
            OpenAIError::RequestError(msg) => {
                assert_eq!(msg, "Connection failed");
            }
            _ => panic!("Expected RequestError"),
        }

        match parse_error {
            OpenAIError::ParseError(msg) => {
                assert_eq!(msg, "Invalid JSON");
            }
            _ => panic!("Expected ParseError"),
        }

        // Test streaming error helper
        let streaming_msg = match streaming_error {
            OpenAIError::Streaming(msg) => msg,
            _ => panic!("Expected Streaming error from streaming helper"),
        };
        assert!(streaming_msg.contains("Stream interrupted"));
    }

    #[test]
    fn test_error_display() {
        let error = OpenAIError::RequestError("Test error".to_string());
        let error_string = format!("{error}");
        assert!(error_string.contains("Test error"));
    }

    #[test]
    fn test_streaming_error_creation() {
        let error = OpenAIError::streaming("Connection timeout");
        match error {
            OpenAIError::Streaming(msg) => {
                assert!(msg.contains("Connection timeout"));
            }
            _ => panic!("Expected Streaming error from streaming helper"),
        }
    }
}
