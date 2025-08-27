//! GPT-5 response tests

use openai_rust_sdk::models::responses::{
    Message, MessageContentInput, MessageRole, ResponseInput, ResponseRequest,
};

#[cfg(test)]
mod response_input_tests {
    use super::*;

    #[test]
    fn test_response_input_from_text() {
        let input = ResponseInput::Text("Test input".to_string());
        match input {
            ResponseInput::Text(text) => assert_eq!(text, "Test input"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_response_input_from_messages() {
        let messages = vec![Message {
            role: MessageRole::User,
            content: MessageContentInput::Text("Hello".to_string()),
        }];
        let input = ResponseInput::Messages(messages.clone());
        match input {
            ResponseInput::Messages(msgs) => assert_eq!(msgs.len(), 1),
            _ => panic!("Expected Messages variant"),
        }
    }

    #[test]
    fn test_response_input_into_conversions() {
        test_string_conversion();
        test_message_vector_conversion();
    }

    fn test_string_conversion() {
        let input: ResponseInput = "test string".into();
        match input {
            ResponseInput::Text(text) => assert_eq!(text, "test string"),
            _ => panic!("Expected Text variant"),
        }
    }

    fn test_message_vector_conversion() {
        let messages = vec![Message {
            role: MessageRole::User,
            content: MessageContentInput::Text("Hello".to_string()),
        }];
        let input: ResponseInput = messages.into();
        match input {
            ResponseInput::Messages(msgs) => assert_eq!(msgs.len(), 1),
            _ => panic!("Expected Messages variant"),
        }
    }
}

#[cfg(test)]
mod response_request_tests {
    use super::*;

    #[test]
    fn test_response_request_new_text() {
        let request = ResponseRequest::new_text("gpt-5", "Hello world");
        assert_eq!(request.model, "gpt-5");
        match request.input {
            ResponseInput::Text(text) => assert_eq!(text, "Hello world"),
            _ => panic!("Expected Text input"),
        }
    }

    #[test]
    fn test_response_request_new_messages() {
        let messages = vec![Message {
            role: MessageRole::User,
            content: MessageContentInput::Text("Hello".to_string()),
        }];
        let request = ResponseRequest::new_messages("gpt-5", messages);
        assert_eq!(request.model, "gpt-5");
        match request.input {
            ResponseInput::Messages(msgs) => assert_eq!(msgs.len(), 1),
            _ => panic!("Expected Messages input"),
        }
    }

    #[test]
    fn test_response_request_with_streaming() {
        let request = ResponseRequest::new_text("gpt-5", "test").with_streaming(true);
        assert_eq!(request.stream, Some(true));
    }

    #[test]
    fn test_response_request_with_instructions() {
        let request = ResponseRequest::new_text("gpt-5", "test").with_instructions("Be helpful");
        assert_eq!(request.instructions, Some("Be helpful".to_string()));
    }

    #[test]
    fn test_response_request_default() {
        let request = ResponseRequest {
            model: "gpt-5".to_string(),
            input: ResponseInput::Text("test".to_string()),
            ..Default::default()
        };

        assert!(request.instructions.is_none());
        assert!(request.previous_response_id.is_none());
        assert!(request.reasoning.is_none());
        assert!(request.text.is_none());
        assert!(request.tools.is_none());
        assert!(request.tool_choice.is_none());
        assert!(request.temperature.is_none());
        assert!(request.max_tokens.is_none());
        assert!(request.stream.is_none());
    }
}
