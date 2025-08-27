//! GPT-5 serialization tests

use crate::gpt5_tests::test_helpers::mock_gpt5_response;
use openai_rust_sdk::models::gpt5::{ReasoningConfig, TextConfig};
use openai_rust_sdk::models::responses::{ResponseRequest, ResponseResult};

#[cfg(test)]
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
