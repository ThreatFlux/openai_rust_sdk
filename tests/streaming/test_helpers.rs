//! Helper functions and utilities for streaming tests

use openai_rust_sdk::models::functions::{FunctionTool, Tool};
use serde_json::json;

/// Helper function for creating test tools
pub fn create_test_function_tool() -> Tool {
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
