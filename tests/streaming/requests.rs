//! Tests for streaming request building and configuration

use openai_rust_sdk::models::functions::ToolChoice;
use openai_rust_sdk::models::responses::{
    Message, MessageContentInput, MessageRole, ResponseInput, ResponseRequest,
};

use super::test_helpers::create_test_function_tool;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_request_with_streaming() {
        let request = ResponseRequest::new_text("gpt-4", "Hello world").with_streaming(true);

        assert_eq!(request.stream, Some(true));
        assert_eq!(request.model, "gpt-4");

        match request.input {
            ResponseInput::Text(text) => assert_eq!(text, "Hello world"),
            _ => panic!("Expected Text input"),
        }
    }

    #[test]
    fn test_response_request_without_streaming() {
        let request = ResponseRequest::new_text("gpt-4", "Hello world");

        assert_eq!(request.stream, None);
    }

    #[test]
    fn test_response_request_with_messages() {
        let messages = vec![Message {
            role: MessageRole::User,
            content: MessageContentInput::Text("Hello".to_string()),
        }];

        let request = ResponseRequest::new_messages("gpt-4", messages).with_streaming(true);

        assert_eq!(request.stream, Some(true));
        match request.input {
            ResponseInput::Messages(msgs) => assert_eq!(msgs.len(), 1),
            _ => panic!("Expected Messages input"),
        }
    }

    #[test]
    fn test_response_request_with_instructions_and_streaming() {
        let request = ResponseRequest::new_text("gpt-4", "Hello")
            .with_instructions("Be helpful")
            .with_streaming(true);

        assert_eq!(request.instructions, Some("Be helpful".to_string()));
        assert_eq!(request.stream, Some(true));
    }

    #[test]
    fn test_streaming_with_tools() {
        let tools = vec![create_test_function_tool()];
        let request = ResponseRequest {
            model: "gpt-4".to_string(),
            input: ResponseInput::Text("Call a function".to_string()),
            stream: Some(true),
            tools: Some(tools),
            tool_choice: Some(ToolChoice::Auto),
            ..Default::default()
        };

        assert_eq!(request.stream, Some(true));
        assert!(request.tools.is_some());
        assert!(request.tool_choice.is_some());
    }

    #[test]
    fn test_streaming_serialization() {
        let request = ResponseRequest::new_text("gpt-4", "Test").with_streaming(true);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("true")); // stream: true
        assert!(json.contains("gpt-4"));
        assert!(json.contains("Test"));
    }
}
