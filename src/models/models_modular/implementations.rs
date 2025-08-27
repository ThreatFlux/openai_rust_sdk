//! # Model Implementations
//!
//! Implementation blocks for model types and responses.

use super::enums::{CompletionType, ModelFamily};
use super::types::{ListModelsResponse, Model, ModelCapabilities, ModelRequirements};
use std::collections::HashMap;

impl Model {
    /// Get the model capabilities based on the model ID
    #[must_use]
    pub fn capabilities(&self) -> ModelCapabilities {
        ModelCapabilities::from_model_id(&self.id)
    }

    /// Check if this model supports a specific completion type
    #[must_use]
    pub fn supports_completion_type(&self, completion_type: &CompletionType) -> bool {
        self.capabilities()
            .completion_types
            .contains(completion_type)
    }

    /// Get the model family
    #[must_use]
    pub fn family(&self) -> ModelFamily {
        ModelCapabilities::classify_family(&self.id)
    }

    /// Check if this model is deprecated
    #[must_use]
    pub fn is_deprecated(&self) -> bool {
        // Models that are commonly known to be deprecated
        matches!(
            self.id.as_str(),
            "text-davinci-003"
                | "text-davinci-002"
                | "text-curie-001"
                | "text-babbage-001"
                | "text-ada-001"
                | "davinci"
                | "curie"
                | "babbage"
                | "ada"
                | "gpt-3.5-turbo-0301"
                | "gpt-4-0314"
        )
    }

    /// Check if this model is currently available
    #[must_use]
    pub fn is_available(&self) -> bool {
        !self.is_deprecated()
    }
}

impl ListModelsResponse {
    /// Filter models by family
    #[must_use]
    pub fn filter_by_family(&self, family: &ModelFamily) -> Vec<&Model> {
        self.data
            .iter()
            .filter(|model| model.family() == *family)
            .collect()
    }

    /// Filter models by completion type support
    #[must_use]
    pub fn filter_by_completion_type(&self, completion_type: &CompletionType) -> Vec<&Model> {
        self.data
            .iter()
            .filter(|model| model.supports_completion_type(completion_type))
            .collect()
    }

    /// Get only available (non-deprecated) models
    #[must_use]
    pub fn available_models(&self) -> Vec<&Model> {
        self.data
            .iter()
            .filter(|model| model.is_available())
            .collect()
    }

    /// Group models by family
    pub fn group_by_family(&self) -> HashMap<ModelFamily, Vec<&Model>> {
        let mut grouped = HashMap::new();

        for model in &self.data {
            let family = model.family();
            grouped.entry(family).or_insert_with(Vec::new).push(model);
        }

        grouped
    }

    /// Get the latest model from each family
    #[must_use]
    pub fn latest_models(&self) -> HashMap<ModelFamily, &Model> {
        let mut latest: HashMap<ModelFamily, &Model> = HashMap::new();

        for model in &self.data {
            let family = model.family();

            match latest.get(&family) {
                Some(current_latest) => {
                    // Prefer non-deprecated models and newer creation dates
                    if !model.is_deprecated()
                        && (current_latest.is_deprecated()
                            || model.created > current_latest.created)
                    {
                        latest.insert(family, model);
                    }
                }
                None => {
                    latest.insert(family, model);
                }
            }
        }

        latest
    }

    /// Find models suitable for a specific use case
    #[must_use]
    pub fn find_suitable_models(&self, requirements: &ModelRequirements) -> Vec<&Model> {
        self.data
            .iter()
            .filter(|model| Self::model_meets_requirements(model, requirements))
            .collect()
    }

    /// Check if a model meets all specified requirements
    fn model_meets_requirements(model: &Model, requirements: &ModelRequirements) -> bool {
        let caps = model.capabilities();

        Self::check_completion_type_requirement(&caps, requirements)
            && Self::check_max_tokens_requirement(&caps, requirements)
            && Self::check_function_calling_requirement(&caps, requirements)
            && Self::check_vision_requirement(&caps, requirements)
            && Self::check_code_interpreter_requirement(&caps, requirements)
            && Self::check_deprecated_requirement(model, requirements)
    }

    /// Check if model supports required completion types
    fn check_completion_type_requirement(
        caps: &ModelCapabilities,
        requirements: &ModelRequirements,
    ) -> bool {
        requirements
            .completion_types
            .iter()
            .any(|ct| caps.completion_types.contains(ct))
    }

    /// Check if model meets minimum max tokens requirement
    fn check_max_tokens_requirement(
        caps: &ModelCapabilities,
        requirements: &ModelRequirements,
    ) -> bool {
        match (requirements.min_max_tokens, caps.max_tokens) {
            (Some(required), Some(available)) => available >= required,
            _ => true,
        }
    }

    /// Check if model supports function calling when required
    fn check_function_calling_requirement(
        caps: &ModelCapabilities,
        requirements: &ModelRequirements,
    ) -> bool {
        !requirements.requires_function_calling || caps.supports_function_calling
    }

    /// Check if model supports vision capabilities when required
    fn check_vision_requirement(
        caps: &ModelCapabilities,
        requirements: &ModelRequirements,
    ) -> bool {
        !requirements.requires_vision || caps.supports_vision
    }

    /// Check if model supports code interpreter when required
    fn check_code_interpreter_requirement(
        caps: &ModelCapabilities,
        requirements: &ModelRequirements,
    ) -> bool {
        !requirements.requires_code_interpreter || caps.supports_code_interpreter
    }

    /// Check if model meets deprecation status requirement
    fn check_deprecated_requirement(model: &Model, requirements: &ModelRequirements) -> bool {
        !requirements.exclude_deprecated || !model.is_deprecated()
    }
}

impl ModelRequirements {
    /// Create requirements for chat completion models
    #[must_use]
    pub fn chat() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            exclude_deprecated: true,
            ..Default::default()
        }
    }

    /// Create requirements for models with function calling support
    #[must_use]
    pub fn function_calling() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            requires_function_calling: true,
            exclude_deprecated: true,
            ..Default::default()
        }
    }

    /// Create requirements for models with high context windows
    #[must_use]
    pub fn high_context(min_tokens: u32) -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            min_max_tokens: Some(min_tokens),
            exclude_deprecated: true,
            ..Default::default()
        }
    }

    /// Create requirements for vision-capable models
    #[must_use]
    pub fn vision() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            requires_vision: true,
            exclude_deprecated: true,
            ..Default::default()
        }
    }

    /// Create requirements for code interpreter models
    #[must_use]
    pub fn code_interpreter() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            requires_code_interpreter: true,
            exclude_deprecated: true,
            ..Default::default()
        }
    }
}
