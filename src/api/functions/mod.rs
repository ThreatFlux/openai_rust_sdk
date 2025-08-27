/// Function calling operations and conversation management
pub mod calling;
/// Core function calling API client and configuration
pub mod client;
/// Tool call extraction utilities
pub mod extraction;
/// Helper functions and utilities
pub mod helpers;
/// Request/response parsing and API communication
pub mod parsing;
/// Tool registration, validation, and execution
pub mod tools;

// Re-export all public types and functions
pub use client::{ConversationState, FunctionCallEvent, FunctionConfig, FunctionsApi};
pub use helpers::FunctionResponseResult;

// Re-export extraction utilities that might be useful publicly
pub(crate) use extraction::ToolCallExtractor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::functions::FunctionTool;
    use crate::models::functions::Tool;
    use serde_json::json;

    #[test]
    fn test_function_config() {
        let config = FunctionConfig::new()
            .with_parallel_calls(true)
            .with_strict_mode(true)
            .with_max_calls(5);

        assert_eq!(config.parallel_function_calls, Some(true));
        assert_eq!(config.strict_mode, Some(true));
        assert_eq!(config.max_function_calls, Some(5));
    }

    #[test]
    fn test_conversation_state() {
        let mut state = ConversationState::default();
        assert!(state.pending_calls.is_empty());
        assert!(state.completed_calls.is_empty());
        assert!(state.call_history.is_empty());

        let call = crate::models::functions::FunctionCall::new("call-1", "test_fn", "{}");
        state.pending_calls.insert("call-1".to_string(), call);
        assert_eq!(state.pending_calls.len(), 1);
    }

    #[test]
    fn test_function_call_validation() {
        let api = FunctionsApi::new("test-key").unwrap();

        let tool = Tool::function(FunctionTool::simple("test_fn", "Test function"));
        let call = crate::models::functions::FunctionCall::new("call-1", "test_fn", "{}");

        let result = api.validate_function_call(&call, &[tool]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_serialize_tools() {
        let api = FunctionsApi::new("test-key").unwrap();

        let tools = vec![Tool::function(FunctionTool::new(
            "get_weather",
            "Get weather information",
            json!({
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                },
                "required": ["location"]
            }),
        ))];

        let serialized = api.serialize_tools(&tools).unwrap();
        assert_eq!(serialized.len(), 1);
        assert_eq!(serialized[0]["type"], "function");
        assert_eq!(serialized[0]["function"]["name"], "get_weather");
    }

    #[test]
    fn test_parse_tool_calls() {
        use extraction::parse_tool_calls;

        // Test data simulating OpenAI API response
        let tool_calls = vec![
            json!({
                "id": "call_1",
                "type": "function",
                "function": {
                    "name": "get_weather",
                    "arguments": r#"{"location": "New York"}"#
                }
            }),
            json!({
                "id": "call_2",
                "type": "function",
                "function": {
                    "name": "get_time",
                    "arguments": r#"{"timezone": "UTC"}"#
                }
            }),
        ];

        let result = parse_tool_calls(&tool_calls).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].call_id, "call_1");
        assert_eq!(result[0].name, "get_weather");
        assert_eq!(result[0].arguments, r#"{"location": "New York"}"#);

        assert_eq!(result[1].call_id, "call_2");
        assert_eq!(result[1].name, "get_time");
        assert_eq!(result[1].arguments, r#"{"timezone": "UTC"}"#);
    }

    #[test]
    fn test_tool_call_extractor() {
        use extraction::ToolCallExtractor;

        let call_data = json!({
            "id": "call_test",
            "type": "function",
            "function": {
                "name": "test_function",
                "arguments": r#"{"param": "value"}"#
            }
        });

        let extractor = ToolCallExtractor::new(&call_data);
        let (id, name, arguments) = extractor.extract_all().unwrap();

        assert_eq!(id, "call_test");
        assert_eq!(name, "test_function");
        assert_eq!(arguments, r#"{"param": "value"}"#);
    }
}
