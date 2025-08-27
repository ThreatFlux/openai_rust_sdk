//! GPT-5 model tests

use openai_rust_sdk::models::gpt5::models;

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
