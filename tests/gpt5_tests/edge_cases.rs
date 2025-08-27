//! GPT-5 edge case tests

use openai_rust_sdk::api::gpt5::GPT5RequestBuilder;
use openai_rust_sdk::models::responses::ResponseInput;

#[cfg(test)]
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
