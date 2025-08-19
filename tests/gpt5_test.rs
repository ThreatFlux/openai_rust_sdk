use openai_rust_sdk::{
    api::gpt5::{GPT5Api, GPT5RequestBuilder},
    models::{
        gpt5::{models, ReasoningEffort, Verbosity},
        responses::{Message, ResponseInput},
    },
};

#[tokio::test]
async fn test_gpt5_request_builder() {
    let builder = GPT5RequestBuilder::new()
        .input("Test prompt")
        .reasoning(ReasoningEffort::Low)
        .verbosity(Verbosity::Medium)
        .gpt5_mini();

    let request = builder.build().unwrap();

    assert_eq!(request.model, models::GPT_5_MINI);
    assert!(matches!(request.input, ResponseInput::Text(_)));
    assert!(request.reasoning.is_some());
    assert!(request.text.is_some());
}

#[tokio::test]
async fn test_gpt5_request_builder_with_messages() {
    let messages = vec![
        Message::user("Hello"),
        Message::assistant("Hi there!"),
        Message::user("How are you?"),
    ];

    let builder = GPT5RequestBuilder::new()
        .input(messages.clone())
        .high_reasoning()
        .high_verbosity()
        .gpt5();

    let request = builder.build().unwrap();

    assert_eq!(request.model, models::GPT_5);
    assert!(matches!(request.input, ResponseInput::Messages(_)));
    assert!(request.reasoning.is_some());
    assert!(request.text.is_some());
}

#[tokio::test]
async fn test_gpt5_request_builder_with_conversation() {
    let builder = GPT5RequestBuilder::new()
        .input("Continue this conversation")
        .previous_response("previous-response-123")
        .medium_reasoning()
        .low_verbosity()
        .temperature(0.7)
        .max_tokens(1000);

    let request = builder.build().unwrap();

    assert_eq!(
        request.previous_response_id,
        Some("previous-response-123".to_string())
    );
    assert_eq!(request.temperature, Some(0.7));
    assert_eq!(request.max_tokens, Some(1000));
}

#[test]
fn test_gpt5_model_constants() {
    assert_eq!(models::GPT_5, "gpt-5");
    assert_eq!(models::GPT_5_MINI, "gpt-5-mini");
    assert_eq!(models::GPT_5_NANO, "gpt-5-nano");
    assert_eq!(models::GPT_5_CHAT_LATEST, "gpt-5-chat-latest");
}

#[test]
fn test_reasoning_effort_variants() {
    let minimal = ReasoningEffort::Minimal;
    let _low = ReasoningEffort::Low;
    let _medium = ReasoningEffort::Medium;
    let _high = ReasoningEffort::High;

    // Test they can be created and used
    let builder = GPT5RequestBuilder::new().input("test").reasoning(minimal);

    assert!(builder.build().unwrap().reasoning.is_some());
}

#[test]
fn test_verbosity_variants() {
    let low = Verbosity::Low;
    let _medium = Verbosity::Medium;
    let _high = Verbosity::High;

    // Test they can be created and used
    let builder = GPT5RequestBuilder::new().input("test").verbosity(low);

    assert!(builder.build().unwrap().text.is_some());
}

#[tokio::test]
async fn test_gpt5_api_creation() {
    // Test we can create the API client
    let result = GPT5Api::new("test-key".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_default_gpt5_request_builder() {
    let builder = GPT5RequestBuilder::default();
    let request = builder.input("test").build().unwrap();
    assert_eq!(request.model, models::GPT_5);
}

#[test]
fn test_gpt5_request_builder_fluent_api() {
    let builder = GPT5RequestBuilder::new()
        .gpt5_nano()
        .input("Test input")
        .instructions("Be helpful")
        .minimal_reasoning()
        .medium_verbosity()
        .temperature(0.8)
        .max_tokens(500);

    let request = builder.build().unwrap();

    assert_eq!(request.model, models::GPT_5_NANO);
    assert_eq!(request.instructions, Some("Be helpful".to_string()));
    assert_eq!(request.temperature, Some(0.8));
    assert_eq!(request.max_tokens, Some(500));
}
