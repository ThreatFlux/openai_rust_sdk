#![allow(clippy::pedantic, clippy::nursery)]
//! Comprehensive tests for the Streaming API
//!
//! This test suite covers core functionality of the Streaming API including:
//! - Stream API creation and configuration
//! - Request building for streaming
//! - Function calling configuration
//! - Error handling
//! - Streaming utilities and helpers

use openai_rust_sdk::api::streaming::StreamingApi;
use openai_rust_sdk::error::OpenAIError;
use openai_rust_sdk::models::functions::{FunctionCall, FunctionTool, Tool, ToolChoice};
use openai_rust_sdk::models::responses::{
    Message, MessageContentInput, MessageRole, ResponseInput, ResponseRequest,
};
use serde_json::json;

// Helper function for creating test tools
fn create_test_function_tool() -> Tool {
    let function = FunctionTool {
        name: "calculate_sum".to_string(),
        description: "Calculate the sum of two numbers".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "a": {
                    "type": "number",
                    "description": "First number"
                },
                "b": {
                    "type": "number",
                    "description": "Second number"
                }
            },
            "required": ["a", "b"]
        }),
        strict: Some(true),
    };
    Tool::Function { function }
}

#[cfg(test)]
mod streaming_api_creation_tests {
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

#[cfg(test)]
mod streaming_request_tests {
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

#[cfg(test)]
mod function_call_tests {
    use super::*;

    #[test]
    fn test_function_call_creation() {
        let call = FunctionCall::new(
            "call_123".to_string(),
            "calculate_sum".to_string(),
            "{\"a\": 5, \"b\": 3}".to_string(),
        );

        assert_eq!(call.call_id, "call_123");
        assert_eq!(call.name, "calculate_sum");
        assert_eq!(call.arguments, "{\"a\": 5, \"b\": 3}");
    }

    #[test]
    fn test_function_call_serialization() {
        let call = FunctionCall::new(
            "call_456".to_string(),
            "get_weather".to_string(),
            "{\"location\": \"New York\"}".to_string(),
        );

        let json = serde_json::to_string(&call).unwrap();
        assert!(json.contains("call_456"));
        assert!(json.contains("get_weather"));
        assert!(json.contains("New York"));

        let deserialized: FunctionCall = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.call_id, call.call_id);
        assert_eq!(deserialized.name, call.name);
        assert_eq!(deserialized.arguments, call.arguments);
    }

    #[test]
    fn test_function_call_with_empty_arguments() {
        let call = FunctionCall::new(
            "call_789".to_string(),
            "simple_function".to_string(),
            "{}".to_string(),
        );

        assert_eq!(call.arguments, "{}");
    }

    #[test]
    fn test_function_call_with_complex_arguments() {
        let complex_args =
            r#"{"numbers": [1, 2, 3], "options": {"precision": 2, "format": "decimal"}}"#;
        let call = FunctionCall::new(
            "call_complex".to_string(),
            "complex_calculation".to_string(),
            complex_args.to_string(),
        );

        assert!(call.arguments.contains("numbers"));
        assert!(call.arguments.contains("options"));
        assert!(call.arguments.contains("precision"));
    }
}

#[cfg(test)]
mod tool_configuration_tests {
    use super::*;

    #[test]
    fn test_function_tool_creation() {
        let tool = create_test_function_tool();

        match tool {
            Tool::Function { function } => {
                assert_eq!(function.name, "calculate_sum");
                assert_eq!(function.description, "Calculate the sum of two numbers");
                assert!(function.parameters.is_object());
                assert_eq!(function.strict, Some(true));
            }
            _ => panic!("Expected Function tool"),
        }
    }

    #[test]
    fn test_tool_choice_variants() {
        let auto = ToolChoice::Auto;
        let required = ToolChoice::Required;
        let none = ToolChoice::None;

        // Test serialization of each variant
        let auto_json = serde_json::to_string(&auto);
        let required_json = serde_json::to_string(&required);
        let none_json = serde_json::to_string(&none);

        assert!(auto_json.is_ok());
        assert!(required_json.is_ok());
        assert!(none_json.is_ok());
    }

    #[test]
    fn test_tool_serialization() {
        let tool = create_test_function_tool();
        let json = serde_json::to_string(&tool).unwrap();

        assert!(json.contains("calculate_sum"));
        assert!(json.contains("Calculate the sum"));
        assert!(json.contains("function"));
        assert!(json.contains("type"));

        let deserialized: Tool = serde_json::from_str(&json).unwrap();
        match deserialized {
            Tool::Function { function } => {
                assert_eq!(function.name, "calculate_sum");
            }
            _ => panic!("Expected Function tool"),
        }
    }

    #[test]
    fn test_tool_parameters_schema() {
        let tool = create_test_function_tool();

        match tool {
            Tool::Function { function } => {
                let params = function.parameters;
                assert!(params.is_object());

                let properties = params.get("properties").expect("Should have properties");
                assert!(properties.is_object());

                let required = params.get("required").expect("Should have required fields");
                assert!(required.is_array());

                let required_fields = required.as_array().unwrap();
                assert_eq!(required_fields.len(), 2);
                assert!(required_fields.contains(&json!("a")));
                assert!(required_fields.contains(&json!("b")));
            }
            _ => panic!("Expected Function tool"),
        }
    }
}

#[cfg(test)]
mod error_handling_tests {
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

#[cfg(test)]
mod message_tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message {
            role: MessageRole::User,
            content: MessageContentInput::Text("Hello world".to_string()),
        };

        assert_eq!(message.role, MessageRole::User);
        // Verify content by matching the enum variant
        match message.content {
            MessageContentInput::Text(ref text) => assert_eq!(text, "Hello world"),
            _ => panic!("Expected text content"),
        }
    }

    #[test]
    fn test_message_roles() {
        let roles = vec![
            MessageRole::Developer,
            MessageRole::User,
            MessageRole::Assistant,
            MessageRole::System,
        ];

        for role in roles {
            let message = Message {
                role: role.clone(),
                content: MessageContentInput::Text("Test message".to_string()),
            };

            // Each role should be valid
            let json = serde_json::to_string(&message);
            assert!(json.is_ok());
        }
    }

    #[test]
    fn test_message_serialization() {
        let message = Message {
            role: MessageRole::Assistant,
            content: MessageContentInput::Text("Hello! How can I help you?".to_string()),
        };

        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("assistant"));
        assert!(json.contains("Hello! How can I help you?"));

        let deserialized: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.role, message.role);
        // Compare content by extracting text from both
        match (&deserialized.content, &message.content) {
            (MessageContentInput::Text(t1), MessageContentInput::Text(t2)) => assert_eq!(t1, t2),
            _ => panic!("Content type mismatch"),
        }
    }

    #[test]
    fn test_messages_in_streaming_request() {
        let messages = vec![
            Message {
                role: MessageRole::User,
                content: MessageContentInput::Text("What's the weather like?".to_string()),
            },
            Message {
                role: MessageRole::Assistant,
                content: MessageContentInput::Text(
                    "I'd be happy to help you check the weather.".to_string(),
                ),
            },
        ];

        let request = ResponseRequest::new_messages("gpt-4", messages).with_streaming(true);

        match request.input {
            ResponseInput::Messages(msgs) => {
                assert_eq!(msgs.len(), 2);
                assert_eq!(msgs[0].role, MessageRole::User);
                assert_eq!(msgs[1].role, MessageRole::Assistant);
            }
            _ => panic!("Expected Messages input"),
        }
    }
}

#[cfg(test)]
mod request_builder_tests {
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

#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_empty_input_text() {
        let request = ResponseRequest::new_text("gpt-4", "");

        match request.input {
            ResponseInput::Text(text) => assert!(text.is_empty()),
            _ => panic!("Expected Text input"),
        }
    }

    #[test]
    fn test_very_long_input() {
        let long_input = "a".repeat(10000);
        let request = ResponseRequest::new_text("gpt-4", long_input.clone());

        match request.input {
            ResponseInput::Text(text) => assert_eq!(text.len(), 10000),
            _ => panic!("Expected Text input"),
        }
    }

    #[test]
    fn test_unicode_input() {
        let unicode_input = "Hello 世界 🌍 👋";
        let request = ResponseRequest::new_text("gpt-4", unicode_input);

        match request.input {
            ResponseInput::Text(text) => {
                assert!(text.contains("世界"));
                assert!(text.contains("🌍"));
            }
            _ => panic!("Expected Text input"),
        }
    }

    #[test]
    fn test_special_characters_in_instructions() {
        let special_instructions = "Use <tags>, \"quotes\", 'apostrophes', and [brackets]";
        let request =
            ResponseRequest::new_text("gpt-4", "Test").with_instructions(special_instructions);

        assert_eq!(request.instructions, Some(special_instructions.to_string()));
    }

    #[test]
    fn test_empty_messages_list() {
        let empty_messages: Vec<Message> = vec![];
        let request = ResponseRequest::new_messages("gpt-4", empty_messages);

        match request.input {
            ResponseInput::Messages(msgs) => assert!(msgs.is_empty()),
            _ => panic!("Expected Messages input"),
        }
    }

    #[test]
    fn test_multiple_function_tools() {
        let tools = vec![
            create_test_function_tool(),
            create_test_function_tool(), // Duplicate for testing
        ];

        let request = ResponseRequest {
            model: "gpt-4".to_string(),
            input: ResponseInput::Text("Call functions".to_string()),
            tools: Some(tools),
            ..Default::default()
        };

        assert!(request.tools.is_some());
        assert_eq!(request.tools.unwrap().len(), 2);
    }
}

#[cfg(test)]
mod integration_preparation_tests {
    use super::*;

    #[test]
    fn test_complete_streaming_request_setup() {
        let tools = vec![create_test_function_tool()];
        let messages = vec![Message {
            role: MessageRole::User,
            content: MessageContentInput::Text(
                "Calculate 15 + 27 using the available function".to_string(),
            ),
        }];

        let request = ResponseRequest {
            model: "gpt-4".to_string(),
            input: ResponseInput::Messages(messages),
            stream: Some(true),
            instructions: Some("Use the provided function to calculate".to_string()),
            tools: Some(tools),
            tool_choice: Some(ToolChoice::Auto),
            ..Default::default()
        };

        // Verify all components are configured
        assert_eq!(request.stream, Some(true));
        assert!(request.instructions.is_some());
        assert!(request.tools.is_some());
        assert!(request.tool_choice.is_some());

        // Should serialize correctly for API calls
        let json = serde_json::to_string(&request);
        assert!(json.is_ok());
    }

    #[test]
    fn test_minimal_streaming_request() {
        let request = ResponseRequest::new_text("gpt-4", "Hello").with_streaming(true);

        // Should have minimal required fields
        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.stream, Some(true));
        assert!(request.instructions.is_none());
        assert!(request.tools.is_none());

        // Should still serialize correctly
        let json = serde_json::to_string(&request);
        assert!(json.is_ok());
    }
}

// Note: Actual streaming integration tests would require network access
// and are commented out for unit testing purposes

/*
#[cfg(test)]
mod integration_tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY environment variable
    async fn test_streaming_text_response() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let api = StreamingApi::new(api_key).unwrap();

        let mut stream = api.create_text_stream("gpt-3.5-turbo", "Count to 5").await.unwrap();

        let mut content = String::new();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.unwrap();
            for choice in chunk.choices {
                if let Some(delta_content) = choice.delta.content {
                    content.push_str(&delta_content);
                }
                if choice.finish_reason.is_some() {
                    break;
                }
            }
        }

        assert!(!content.is_empty());
    }
}
*/
