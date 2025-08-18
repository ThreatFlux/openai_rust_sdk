//! Comprehensive tests for the GPT-5 API
//!
//! This test suite covers all functionality of the GPT-5 API including:
//! - GPT-5 specific features and responses
//! - Different reasoning levels and efforts  
//! - Text configuration and verbosity settings
//! - Model variants (GPT-5, GPT-5-mini, GPT-5-nano)
//! - Builder pattern and fluent interface
//! - Tool calling with GPT-5
//! - Conversation continuation
//! - Error handling and validation

use openai_rust_sdk::api::gpt5::{GPT5Api, GPT5RequestBuilder};
use openai_rust_sdk::error::OpenAIError;
use openai_rust_sdk::models::functions::{FunctionTool, Tool, ToolChoice};
use openai_rust_sdk::models::gpt5::{
    ReasoningConfig, ReasoningEffort, TextConfig, Verbosity, models,
};
use openai_rust_sdk::models::responses::{
    Message, MessageContentInput, MessageRole, ResponseInput, ResponseRequest, ResponseResult,
};
use serde_json::json;

// Helper function to create a mock response
fn mock_gpt5_response() -> ResponseResult {
    ResponseResult {
        id: Some("resp_gpt5_test".to_string()),
        object: "response".to_string(),
        created: 1640995200,
        model: models::GPT_5.to_string(),
        choices: vec![],
        usage: None,
    }
}

// Helper function to create a test function tool
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
mod gpt5_api_creation_tests {
    use super::*;

    #[test]
    fn test_gpt5_api_creation() {
        let api = GPT5Api::new("test-api-key".to_string());
        assert!(api.is_ok());
    }

    #[test]
    fn test_gpt5_api_creation_with_empty_key() {
        let api = GPT5Api::new("".to_string());
        assert!(api.is_err());
        if let Err(OpenAIError::Authentication(msg)) = api {
            assert!(msg.contains("API key cannot be empty"));
        } else {
            panic!("Expected Authentication error");
        }
    }
}

#[cfg(test)]
mod reasoning_config_tests {
    use super::*;

    #[test]
    fn test_reasoning_config_creation() {
        // Test different reasoning efforts
        let minimal = ReasoningConfig::minimal();
        let low = ReasoningConfig::low();
        let medium = ReasoningConfig::medium();
        let high = ReasoningConfig::high();

        // Verify they create distinct configs
        assert_ne!(
            serde_json::to_string(&minimal).unwrap(),
            serde_json::to_string(&high).unwrap()
        );
    }

    #[test]
    fn test_reasoning_config_new() {
        let config = ReasoningConfig::new(ReasoningEffort::High);
        let json_str = serde_json::to_string(&config).unwrap();
        assert!(json_str.contains("high"));
    }

    #[test]
    fn test_reasoning_effort_enum() {
        let efforts = vec![
            ReasoningEffort::Minimal,
            ReasoningEffort::Low,
            ReasoningEffort::Medium,
            ReasoningEffort::High,
        ];

        for effort in efforts {
            let config = ReasoningConfig::new(effort);
            let serialized = serde_json::to_string(&config);
            assert!(serialized.is_ok());
        }
    }
}

#[cfg(test)]
mod text_config_tests {
    use super::*;

    #[test]
    fn test_text_config_creation() {
        let low = TextConfig::low();
        let medium = TextConfig::medium();
        let high = TextConfig::high();

        // Verify they create distinct configs
        assert_ne!(
            serde_json::to_string(&low).unwrap(),
            serde_json::to_string(&high).unwrap()
        );
    }

    #[test]
    fn test_text_config_new() {
        let config = TextConfig::new(Verbosity::High);
        let json_str = serde_json::to_string(&config).unwrap();
        assert!(json_str.contains("high"));
    }

    #[test]
    fn test_verbosity_enum() {
        let verbosity_levels = vec![Verbosity::Low, Verbosity::Medium, Verbosity::High];

        for verbosity in verbosity_levels {
            let config = TextConfig::new(verbosity);
            let serialized = serde_json::to_string(&config);
            assert!(serialized.is_ok());
        }
    }
}

#[cfg(test)]
mod gpt5_models_tests {
    use super::*;

    #[test]
    fn test_gpt5_model_constants() {
        assert_eq!(models::GPT_5, "gpt-5");
        assert_eq!(models::GPT_5_MINI, "gpt-5-mini");
        assert_eq!(models::GPT_5_NANO, "gpt-5-nano");
    }

    #[test]
    fn test_model_names_are_valid() {
        let model_names = vec![models::GPT_5, models::GPT_5_MINI, models::GPT_5_NANO];

        for model in model_names {
            assert!(!model.is_empty());
            assert!(model.starts_with("gpt-5"));
        }
    }
}

#[cfg(test)]
mod gpt5_request_builder_tests {
    use super::*;

    #[test]
    fn test_builder_default_creation() {
        let builder = GPT5RequestBuilder::new();
        let result = builder.input("Test input").build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.model, models::GPT_5);
    }

    #[test]
    fn test_builder_model_selection() {
        // Test GPT-5 (default)
        let request = GPT5RequestBuilder::new()
            .gpt5()
            .input("test")
            .build()
            .unwrap();
        assert_eq!(request.model, models::GPT_5);

        // Test GPT-5-mini
        let request = GPT5RequestBuilder::new()
            .gpt5_mini()
            .input("test")
            .build()
            .unwrap();
        assert_eq!(request.model, models::GPT_5_MINI);

        // Test GPT-5-nano
        let request = GPT5RequestBuilder::new()
            .gpt5_nano()
            .input("test")
            .build()
            .unwrap();
        assert_eq!(request.model, models::GPT_5_NANO);
    }

    #[test]
    fn test_builder_custom_model() {
        let request = GPT5RequestBuilder::new()
            .model("custom-gpt-5")
            .input("test")
            .build()
            .unwrap();
        assert_eq!(request.model, "custom-gpt-5");
    }

    #[test]
    fn test_builder_reasoning_configuration() {
        // Test minimal reasoning
        let request = GPT5RequestBuilder::new()
            .input("test")
            .minimal_reasoning()
            .build()
            .unwrap();
        assert!(request.reasoning.is_some());

        // Test high reasoning
        let request = GPT5RequestBuilder::new()
            .input("test")
            .high_reasoning()
            .build()
            .unwrap();
        assert!(request.reasoning.is_some());

        // Test custom reasoning
        let request = GPT5RequestBuilder::new()
            .input("test")
            .reasoning(ReasoningEffort::Medium)
            .build()
            .unwrap();
        assert!(request.reasoning.is_some());
    }

    #[test]
    fn test_builder_verbosity_configuration() {
        // Test low verbosity
        let request = GPT5RequestBuilder::new()
            .input("test")
            .low_verbosity()
            .build()
            .unwrap();
        assert!(request.text.is_some());

        // Test high verbosity
        let request = GPT5RequestBuilder::new()
            .input("test")
            .high_verbosity()
            .build()
            .unwrap();
        assert!(request.text.is_some());

        // Test custom verbosity
        let request = GPT5RequestBuilder::new()
            .input("test")
            .verbosity(Verbosity::Medium)
            .build()
            .unwrap();
        assert!(request.text.is_some());
    }

    #[test]
    fn test_builder_with_instructions() {
        let request = GPT5RequestBuilder::new()
            .input("test")
            .instructions("You are a helpful assistant")
            .build()
            .unwrap();
        assert_eq!(
            request.instructions,
            Some("You are a helpful assistant".to_string())
        );
    }

    #[test]
    fn test_builder_with_previous_response() {
        let request = GPT5RequestBuilder::new()
            .input("test")
            .previous_response("resp_123")
            .build()
            .unwrap();
        assert_eq!(request.previous_response_id, Some("resp_123".to_string()));
    }

    #[test]
    fn test_builder_with_tools() {
        let tools = vec![create_test_function_tool()];
        let request = GPT5RequestBuilder::new()
            .input("test")
            .tools(tools.clone())
            .tool_choice(ToolChoice::Auto)
            .build()
            .unwrap();

        assert!(request.tools.is_some());
        assert!(request.tool_choice.is_some());
    }

    #[test]
    fn test_builder_with_generation_params() {
        let request = GPT5RequestBuilder::new()
            .input("test")
            .temperature(0.7)
            .max_tokens(1000)
            .build()
            .unwrap();

        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.max_tokens, Some(1000));
    }

    #[test]
    fn test_builder_missing_input_error() {
        let result = GPT5RequestBuilder::new().build();
        assert!(result.is_err());
        match result.unwrap_err() {
            OpenAIError::InvalidRequest(msg) => {
                assert!(msg.contains("Input is required"));
            }
            _ => panic!("Expected InvalidRequest error"),
        }
    }

    #[test]
    fn test_builder_fluent_chaining() {
        let request = GPT5RequestBuilder::new()
            .gpt5_mini()
            .input("Complex reasoning task")
            .instructions("Think step by step")
            .high_reasoning()
            .medium_verbosity()
            .temperature(0.3)
            .max_tokens(2000)
            .build()
            .unwrap();

        assert_eq!(request.model, models::GPT_5_MINI);
        assert!(request.instructions.is_some());
        assert!(request.reasoning.is_some());
        assert!(request.text.is_some());
        assert_eq!(request.temperature, Some(0.3));
        assert_eq!(request.max_tokens, Some(2000));
    }
}

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
        // Test string into
        let input: ResponseInput = "test string".into();
        match input {
            ResponseInput::Text(text) => assert_eq!(text, "test string"),
            _ => panic!("Expected Text variant"),
        }

        // Test message vector into
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

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_reasoning_config_serialization() {
        let config = ReasoningConfig::high();
        let json = serde_json::to_string(&config).unwrap();
        assert!(!json.is_empty());

        let deserialized: ReasoningConfig = serde_json::from_str(&json).unwrap();
        let rejson = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, rejson);
    }

    #[test]
    fn test_text_config_serialization() {
        let config = TextConfig::medium();
        let json = serde_json::to_string(&config).unwrap();
        assert!(!json.is_empty());

        let deserialized: TextConfig = serde_json::from_str(&json).unwrap();
        let rejson = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, rejson);
    }

    #[test]
    fn test_response_request_serialization() {
        let request = ResponseRequest::new_text("gpt-5", "test").with_instructions("Be helpful");

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("gpt-5"));
        assert!(json.contains("test"));
        assert!(json.contains("Be helpful"));

        let deserialized: ResponseRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.model, request.model);
        assert_eq!(deserialized.instructions, request.instructions);
    }

    #[test]
    fn test_response_result_serialization() {
        let result = mock_gpt5_response();
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("resp_gpt5_test"));
        assert!(json.contains("gpt-5"));

        let deserialized: ResponseResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, result.id);
        assert_eq!(deserialized.model, result.model);
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_temperature_validation_range() {
        // Valid temperature values
        let valid_temps = vec![0.0, 0.5, 1.0, 1.5, 2.0];
        for temp in valid_temps {
            let request = GPT5RequestBuilder::new()
                .input("test")
                .temperature(temp)
                .build();
            assert!(request.is_ok());
        }
    }

    #[test]
    fn test_max_tokens_validation() {
        // Valid max_tokens values
        let valid_tokens = vec![1, 100, 1000, 4096];
        for tokens in valid_tokens {
            let request = GPT5RequestBuilder::new()
                .input("test")
                .max_tokens(tokens)
                .build();
            assert!(request.is_ok());
        }
    }

    #[test]
    fn test_model_name_validation() {
        // Test with empty model name
        let request = GPT5RequestBuilder::new().model("").input("test").build();
        // Should still succeed as we don't validate model names in builder
        assert!(request.is_ok());
    }
}

#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_empty_input_handling() {
        let request = GPT5RequestBuilder::new().input("").build();
        // Empty input should still be valid
        assert!(request.is_ok());
    }

    #[test]
    fn test_very_long_input() {
        let long_input = "a".repeat(10000);
        let request = GPT5RequestBuilder::new().input(long_input.clone()).build();
        assert!(request.is_ok());

        match request.unwrap().input {
            ResponseInput::Text(text) => assert_eq!(text.len(), 10000),
            _ => panic!("Expected Text input"),
        }
    }

    #[test]
    fn test_unicode_input_handling() {
        let unicode_input = "Hello 世界 🌍 👋";
        let request = GPT5RequestBuilder::new().input(unicode_input).build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_special_characters_in_instructions() {
        let special_instructions = "Use <tags>, \"quotes\", 'apostrophes', and [brackets]";
        let request = GPT5RequestBuilder::new()
            .input("test")
            .instructions(special_instructions)
            .build();
        assert!(request.is_ok());
        assert_eq!(
            request.unwrap().instructions,
            Some(special_instructions.to_string())
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_gpt5_request_workflow() {
        // Create a comprehensive request with all features
        let tools = vec![create_test_function_tool()];

        let request = GPT5RequestBuilder::new()
            .gpt5()
            .input("Solve this complex problem step by step")
            .instructions("You are an expert problem solver")
            .previous_response("resp_previous")
            .high_reasoning()
            .medium_verbosity()
            .tools(tools)
            .tool_choice(ToolChoice::Auto)
            .temperature(0.7)
            .max_tokens(2000)
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();

        // Verify all components are set correctly
        assert_eq!(req.model, models::GPT_5);
        assert!(req.instructions.is_some());
        assert!(req.previous_response_id.is_some());
        assert!(req.reasoning.is_some());
        assert!(req.text.is_some());
        assert!(req.tools.is_some());
        assert!(req.tool_choice.is_some());
        assert!(req.temperature.is_some());
        assert!(req.max_tokens.is_some());
    }

    #[test]
    fn test_minimal_gpt5_request() {
        let request = GPT5RequestBuilder::new().input("Hello").build();

        assert!(request.is_ok());
        let req = request.unwrap();

        // Verify minimal setup
        assert_eq!(req.model, models::GPT_5);
        assert!(req.instructions.is_none());
        assert!(req.reasoning.is_none());
        assert!(req.text.is_none());
        assert!(req.tools.is_none());
        assert!(req.tool_choice.is_none());
    }

    #[test]
    fn test_different_model_variants() {
        let models_to_test = vec![
            (GPT5RequestBuilder::new().gpt5(), models::GPT_5),
            (GPT5RequestBuilder::new().gpt5_mini(), models::GPT_5_MINI),
            (GPT5RequestBuilder::new().gpt5_nano(), models::GPT_5_NANO),
        ];

        for (builder, expected_model) in models_to_test {
            let request = builder.input("test").build().unwrap();
            assert_eq!(request.model, expected_model);
        }
    }
}

// Note: Tests involving actual API calls would go here but are commented out
// since they require a real API key and network access

/*
#[cfg(test)]
mod api_integration_tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY environment variable
    async fn test_gpt5_minimal_response() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let api = GPT5Api::new(api_key).unwrap();

        let result = api.create_minimal_response(
            models::GPT_5,
            "What is 2+2?"
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY environment variable
    async fn test_gpt5_with_reasoning() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let api = GPT5Api::new(api_key).unwrap();

        let result = api.create_reasoned_response(
            models::GPT_5,
            "Explain quantum computing",
            ReasoningEffort::High,
            Verbosity::Medium
        ).await;

        assert!(result.is_ok());
    }
}
*/
