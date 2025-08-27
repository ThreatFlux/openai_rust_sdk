//! GPT-5 integration tests

use crate::gpt5_tests::test_helpers::create_test_function_tool;
use openai_rust_sdk::api::gpt5::GPT5RequestBuilder;
use openai_rust_sdk::models::functions::ToolChoice;
use openai_rust_sdk::models::gpt5::models;

#[cfg(test)]
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
