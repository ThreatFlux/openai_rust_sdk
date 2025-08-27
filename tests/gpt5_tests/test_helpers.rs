//! Test helper functions for GPT-5 tests

use openai_rust_sdk::models::functions::{FunctionTool, Tool};
use openai_rust_sdk::models::gpt5::models;
use openai_rust_sdk::models::responses::ResponseResult;
use serde_json::json;

// Helper function to create a mock response
pub fn mock_gpt5_response() -> ResponseResult {
    ResponseResult {
        id: Some("resp_gpt5_test".to_string()),
        object: "response".to_string(),
        created: 1_640_995_200,
        model: models::GPT_5.to_string(),
        choices: vec![],
        usage: None,
    }
}

// Helper function to create a test function tool
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
