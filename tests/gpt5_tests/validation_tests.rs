//! GPT-5 validation tests

use openai_rust_sdk::api::gpt5::GPT5RequestBuilder;

#[cfg(test)]
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
