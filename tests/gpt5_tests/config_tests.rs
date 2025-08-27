//! GPT-5 configuration tests

use openai_rust_sdk::models::gpt5::{ReasoningConfig, ReasoningEffort, TextConfig, Verbosity};

#[cfg(test)]
mod reasoning_tests {
    use super::*;

    #[test]
    fn test_reasoning_config_creation() {
        // Test different reasoning efforts
        let minimal = ReasoningConfig::minimal();
        let _low = ReasoningConfig::low();
        let _medium = ReasoningConfig::medium();
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
mod text_tests {
    use super::*;

    #[test]
    fn test_text_config_creation() {
        let low = TextConfig::low();
        let _medium = TextConfig::medium();
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
