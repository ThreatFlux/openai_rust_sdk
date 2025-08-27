//! Tests for function calling functionality in streaming

use openai_rust_sdk::models::functions::FunctionCall;

#[cfg(test)]
mod tests {
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
