//! # Models API
//!
//! Data structures for the OpenAI Models API that provides information about
//! available models, their capabilities, and permissions.

/// Model capability builders for different model types
pub mod capability_builders;
/// Model capabilities including features and costs
pub mod capabilities;
/// Model struct implementations
pub mod model_impls;
/// Model requirements for filtering
pub mod requirements;
/// Response struct implementations
pub mod response_impls;
/// Core model types and structures
pub mod types;

// Re-export all public items to maintain API compatibility
pub use capabilities::*;
pub use model_impls::*;
pub use requirements::*;
pub use response_impls::*;
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

    fn create_test_model(id: &str, created: u64) -> Model {
        Model {
            id: id.to_string(),
            object: "model".to_string(),
            created,
            owned_by: "openai".to_string(),
            root: None,
            parent: None,
            permission: None,
        }
    }

    #[test]
    fn test_list_models_response_methods() {
        let response = ListModelsResponse {
            object: "list".to_string(),
            data: vec![
                create_test_model("gpt-4", 1000),
                create_test_model("dall-e-3", 2000),
                create_test_model("text-davinci-003", 500), // deprecated
            ],
        };

        // Test family filtering
        let gpt4_models = response.filter_by_family(&ModelFamily::GPT4);
        assert_eq!(gpt4_models.len(), 1);
        assert_eq!(gpt4_models[0].id, "gpt-4");

        // Test completion type filtering
        let chat_models = response.filter_by_completion_type(&CompletionType::Chat);
        assert_eq!(chat_models.len(), 1);
        assert_eq!(chat_models[0].id, "gpt-4");

        // Test available models (2 non-deprecated: gpt-4 and dall-e-3)
        assert_eq!(response.available_models().len(), 2);

        // Test grouping by family
        let grouped = response.group_by_family();
        assert!(grouped.contains_key(&ModelFamily::GPT4) && grouped.contains_key(&ModelFamily::DALLE));

        // Test latest models
        let latest = response.latest_models();
        assert!(latest.contains_key(&ModelFamily::GPT4));
        assert_eq!(latest.get(&ModelFamily::GPT4).unwrap().id, "gpt-4");

        // Test finding suitable models for chat
        let suitable = response.find_suitable_models(&ModelRequirements::chat());
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