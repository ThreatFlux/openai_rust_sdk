//! GPT-5 request builder tests

#[cfg(test)]
mod gpt5_request_builder_tests {
    use crate::gpt5_tests::test_helpers::create_test_function_tool;
    use openai_rust_sdk::api::gpt5::GPT5RequestBuilder;
    use openai_rust_sdk::error::OpenAIError;
    use openai_rust_sdk::models::functions::ToolChoice;
    use openai_rust_sdk::models::gpt5::{models, ReasoningEffort, Verbosity};

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
        test_gpt5_model();
        test_gpt5_mini_model();
        test_gpt5_nano_model();
    }

    fn test_gpt5_model() {
        let request = GPT5RequestBuilder::new()
            .gpt5()
            .input("test")
            .build()
            .unwrap();
        assert_eq!(request.model, models::GPT_5);
    }

    fn test_gpt5_mini_model() {
        let request = GPT5RequestBuilder::new()
            .gpt5_mini()
            .input("test")
            .build()
            .unwrap();
        assert_eq!(request.model, models::GPT_5_MINI);
    }

    fn test_gpt5_nano_model() {
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
        test_minimal_reasoning();
        test_high_reasoning();
        test_custom_reasoning();
    }

    fn test_minimal_reasoning() {
        let request = GPT5RequestBuilder::new()
            .input("test")
            .minimal_reasoning()
            .build()
            .unwrap();
        assert!(request.reasoning.is_some());
    }

    fn test_high_reasoning() {
        let request = GPT5RequestBuilder::new()
            .input("test")
            .high_reasoning()
            .build()
            .unwrap();
        assert!(request.reasoning.is_some());
    }

    fn test_custom_reasoning() {
        let request = GPT5RequestBuilder::new()
            .input("test")
            .reasoning(ReasoningEffort::Medium)
            .build()
            .unwrap();
        assert!(request.reasoning.is_some());
    }

    #[test]
    fn test_builder_verbosity_configuration() {
        test_low_verbosity();
        test_high_verbosity();
        test_custom_verbosity();
    }

    fn test_low_verbosity() {
        let request = GPT5RequestBuilder::new()
            .input("test")
            .low_verbosity()
            .build()
            .unwrap();
        assert!(request.text.is_some());
    }

    fn test_high_verbosity() {
        let request = GPT5RequestBuilder::new()
            .input("test")
            .high_verbosity()
            .build()
            .unwrap();
        assert!(request.text.is_some());
    }

    fn test_custom_verbosity() {
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
