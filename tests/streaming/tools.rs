//! Tests for tool configuration and serialization

use openai_rust_sdk::models::functions::{Tool, ToolChoice};
use serde_json::json;

use super::test_helpers::create_test_function_tool;

#[cfg(test)]
mod tests {
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
