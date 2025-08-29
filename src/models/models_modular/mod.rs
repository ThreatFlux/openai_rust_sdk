//! # Models API Module
//!
//! Modular organization of the OpenAI Models API functionality.
//!
//! This module contains:
//! - `types` - Core model data structures
//! - `enums` - Model categorization enumerations  
//! - `implementations` - Method implementations for model types
//! - `classification` - Model classification and analysis utilities
//! - `capability_builders` - Functions to build capabilities for different model types

/// Model categorization enumerations
pub mod enums;

/// Core model data structures
pub mod types;

/// Method implementations for model types
pub mod implementations;

/// Model classification and analysis utilities
pub mod classification;

/// Functions to build capabilities for different model types
pub mod capability_builders;

// Re-export all public items for easy access
pub use capability_builders::*;
pub use classification::*;
pub use enums::*;
pub use implementations::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_family_classification() {
        assert_eq!(
            ModelCapabilities::classify_family("gpt-4o"),
            ModelFamily::GPT4o
        );
        assert_eq!(
            ModelCapabilities::classify_family("gpt-4-turbo"),
            ModelFamily::GPT4Turbo
        );
        assert_eq!(
            ModelCapabilities::classify_family("gpt-4"),
            ModelFamily::GPT4
        );
        assert_eq!(
            ModelCapabilities::classify_family("gpt-3.5-turbo"),
            ModelFamily::GPT35
        );
        assert_eq!(
            ModelCapabilities::classify_family("dall-e-3"),
            ModelFamily::DALLE
        );
        assert_eq!(
            ModelCapabilities::classify_family("whisper-1"),
            ModelFamily::Whisper
        );
        assert_eq!(
            ModelCapabilities::classify_family("text-embedding-ada-002"),
            ModelFamily::Embeddings
        );
    }

    #[test]
    fn test_model_tier_classification() {
        assert_eq!(
            ModelCapabilities::classify_tier("text-davinci-003"),
            ModelTier::Legacy
        );
        assert_eq!(
            ModelCapabilities::classify_tier("gpt-4"),
            ModelTier::Premium
        );
        assert_eq!(
            ModelCapabilities::classify_tier("gpt-3.5-turbo"),
            ModelTier::Standard
        );
        assert_eq!(
            ModelCapabilities::classify_tier("gpt-4-preview"),
            ModelTier::Experimental
        );
    }

    #[test]
    fn test_model_capabilities() {
        let caps = ModelCapabilities::from_model_id("gpt-4o");
        assert_eq!(caps.family, ModelFamily::GPT4o);
        assert!(caps.supports_function_calling);
        assert!(caps.supports_vision);
        assert_eq!(caps.max_tokens, Some(128_000));
    }

    #[test]
    fn test_model_is_deprecated() {
        let model = Model {
            id: "text-davinci-003".to_string(),
            object: "model".to_string(),
            created: 1_234_567_890,
            owned_by: "openai".to_string(),
            root: None,
            parent: None,
            permission: None,
        };

        assert!(model.is_deprecated());
        assert!(!model.is_available());
    }

    #[test]
    fn test_model_supports_completion_type() {
        let model = Model {
            id: "gpt-4".to_string(),
            object: "model".to_string(),
            created: 1_234_567_890,
            owned_by: "openai".to_string(),
            root: None,
            parent: None,
            permission: None,
        };

        assert!(model.supports_completion_type(&CompletionType::Chat));
        assert!(!model.supports_completion_type(&CompletionType::Image));
    }

    #[test]
    fn test_model_requirements() {
        let req = ModelRequirements::function_calling();
        assert!(req.requires_function_calling);
        assert!(req.completion_types.contains(&CompletionType::Chat));

        let req = ModelRequirements::high_context(100_000);
        assert_eq!(req.min_max_tokens, Some(100_000));
    }

    #[test]
    fn test_cost_estimation() {
        let caps = ModelCapabilities::from_model_id("gpt-4");
        let cost = caps.estimate_monthly_cost(1_000_000, 500_000);
        assert!(cost.is_some());

        let caps = ModelCapabilities::from_model_id("dall-e-3");
        let cost = caps.estimate_monthly_cost(1_000_000, 500_000);
        assert!(cost.is_none());
    }

    #[test]
    fn test_list_models_response_methods() {
        let response = create_test_models_response();

        test_family_filtering(&response);
        test_completion_type_filtering(&response);
        test_available_models(&response);
        test_grouping_by_family(&response);
        test_latest_models(&response);
        test_finding_suitable_models(&response);
    }

    fn create_test_models_response() -> ListModelsResponse {
        let models = vec![
            Model {
                id: "gpt-4".to_string(),
                object: "model".to_string(),
                created: 1000,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
            Model {
                id: "dall-e-3".to_string(),
                object: "model".to_string(),
                created: 2000,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
            Model {
                id: "text-davinci-003".to_string(), // deprecated
                object: "model".to_string(),
                created: 500,
                owned_by: "openai".to_string(),
                root: None,
                parent: None,
                permission: None,
            },
        ];

        ListModelsResponse {
            object: "list".to_string(),
            data: models,
        }
    }

    fn test_family_filtering(response: &ListModelsResponse) {
        let gpt4_models = response.filter_by_family(ModelFamily::GPT4);
        assert_eq!(gpt4_models.len(), 1);
        assert_eq!(gpt4_models[0].id, "gpt-4");
    }

    fn test_completion_type_filtering(response: &ListModelsResponse) {
        let chat_models = response.filter_by_completion_type(&CompletionType::Chat);
        assert_eq!(chat_models.len(), 1);
        assert_eq!(chat_models[0].id, "gpt-4");
    }

    fn test_available_models(response: &ListModelsResponse) {
        let available = response.available_models();
        assert_eq!(available.len(), 2); // gpt-4 and dall-e-3
    }

    fn test_grouping_by_family(response: &ListModelsResponse) {
        let grouped = response.group_by_family();
        assert!(grouped.contains_key(&ModelFamily::GPT4));
        assert!(grouped.contains_key(&ModelFamily::DALLE));
    }

    fn test_latest_models(response: &ListModelsResponse) {
        let latest = response.latest_models();
        assert!(latest.contains_key(&ModelFamily::GPT4));
        assert_eq!(latest.get(&ModelFamily::GPT4).unwrap().id, "gpt-4");
    }

    fn test_finding_suitable_models(response: &ListModelsResponse) {
        let requirements = ModelRequirements::chat();
        let suitable = response.find_suitable_models(&requirements);
        assert_eq!(suitable.len(), 1);
        assert_eq!(suitable[0].id, "gpt-4");
    }

    #[test]
    fn test_model_capabilities_cost_estimation() {
        let caps = ModelCapabilities::from_model_id("gpt-4");

        // Test with both input and output costs
        let cost = caps.estimate_monthly_cost(1_000_000, 500_000);
        assert!(cost.is_some());

        // Test with model that has no cost data
        let caps = ModelCapabilities::from_model_id("dall-e-3");
        let cost = caps.estimate_monthly_cost(1_000_000, 500_000);
        assert!(cost.is_none());
    }
}
