//! Tests for request builder patterns

use openai_rust_sdk::models::responses::{
    Message, MessageContentInput, MessageRole, ResponseInput, ResponseRequest,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_request_builder() {
        let request = ResponseRequest::new_text("gpt-4", "Test prompt");

        assert_eq!(request.model, "gpt-4");
        match request.input {
            ResponseInput::Text(text) => assert_eq!(text, "Test prompt"),
            _ => panic!("Expected Text input"),
        }
        assert!(request.stream.is_none());
    }

    #[test]
    fn test_messages_request_builder() {
        let messages = vec![Message {
            role: MessageRole::User,
            content: MessageContentInput::Text("Hello".to_string()),
        }];

        let request = ResponseRequest::new_messages("gpt-4", messages);

        assert_eq!(request.model, "gpt-4");
        match request.input {
            ResponseInput::Messages(msgs) => assert_eq!(msgs.len(), 1),
            _ => panic!("Expected Messages input"),
        }
    }

    #[test]
    fn test_request_builder_chaining() {
        let request = ResponseRequest::new_text("gpt-4", "Test")
            .with_streaming(true)
            .with_instructions("Be helpful");

        assert_eq!(request.stream, Some(true));
        assert_eq!(request.instructions, Some("Be helpful".to_string()));
    }

    #[test]
    fn test_request_with_temperature() {
        let request = ResponseRequest {
            model: "gpt-4".to_string(),
            input: ResponseInput::Text("Test".to_string()),
            temperature: Some(0.7),
            ..Default::default()
        };

        assert_eq!(request.temperature, Some(0.7));
    }

    #[test]
    fn test_request_with_max_tokens() {
        let request = ResponseRequest {
            model: "gpt-4".to_string(),
            input: ResponseInput::Text("Test".to_string()),
            max_tokens: Some(1000),
            ..Default::default()
        };

        assert_eq!(request.max_tokens, Some(1000));
    }
}
