//! # Models API
//!
//! Data structures for the OpenAI Models API that provides information about
//! available models, their capabilities, and permissions.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// Information about an `OpenAI` model
#[derive(Debug, Clone, Ser, De)]
pub struct Model {
    /// Unique identifier for the model
    pub id: String,

    /// Object type (always "model")
    pub object: String,

    /// Unix timestamp when the model was created
    pub created: u64,

    /// Organization that owns the model
    pub owned_by: String,

    /// Root model identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,

    /// Parent model identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// List of permissions for this model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<Vec<ModelPermission>>,
}

/// Permissions for a model
#[derive(Debug, Clone, Ser, De)]
pub struct ModelPermission {
    /// Unique identifier for the permission
    pub id: String,

    /// Object type (always "`model_permission`")
    pub object: String,

    /// Unix timestamp when the permission was created
    pub created: u64,

    /// Whether creation of completions is allowed
    pub allow_create_engine: bool,

    /// Whether sampling is allowed
    pub allow_sampling: bool,

    /// Whether log probabilities are allowed
    pub allow_logprobs: bool,

    /// Whether search indices can be created
    pub allow_search_indices: bool,

    /// Whether the model can be viewed
    pub allow_view: bool,

    /// Whether fine-tuning is allowed
    pub allow_fine_tuning: bool,

    /// Organization identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// Group identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Whether this is a blocking permission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blocking: Option<bool>,
}

/// Response from the list models API
#[derive(Debug, Clone, Ser, De)]
pub struct ListModelsResponse {
    /// Object type (always "list")
    pub object: String,

    /// List of available models
    pub data: Vec<Model>,
}

/// Detailed information about model capabilities
#[derive(Debug, Clone)]
pub struct ModelCapabilities {
    /// Maximum context window size in tokens
    pub max_tokens: Option<u32>,

    /// Training data cutoff date
    pub training_cutoff: Option<String>,

    /// Supported completion types
    pub completion_types: Vec<CompletionType>,

    /// Whether the model supports function calling
    pub supports_function_calling: bool,

    /// Whether the model supports vision
    pub supports_vision: bool,

    /// Whether the model supports code interpreter
    pub supports_code_interpreter: bool,

    /// Model family (e.g., "gpt-4", "gpt-3.5", "dall-e")
    pub family: ModelFamily,

    /// Model tier/quality level
    pub tier: ModelTier,

    /// Input cost per 1M tokens (if available)
    pub input_cost_per_1m_tokens: Option<f64>,

    /// Output cost per 1M tokens (if available)
    pub output_cost_per_1m_tokens: Option<f64>,
}

/// Types of completions a model supports
#[derive(Debug, Clone, PartialEq)]
pub enum CompletionType {
    /// Text completion (legacy)
    Text,
    /// Chat completion
    Chat,
    /// Code completion
    Code,
    /// Image generation
    Image,
    /// Audio processing (speech-to-text, text-to-speech)
    Audio,
    /// Embeddings
    Embeddings,
    /// Moderation
    Moderation,
}

/// Model family categorization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelFamily {
    /// GPT-4 series models
    GPT4,
    /// GPT-4 Turbo models
    GPT4Turbo,
    /// GPT-4o series models
    GPT4o,
    /// GPT-3.5 series models
    GPT35,
    /// DALL-E image generation models
    #[allow(clippy::upper_case_acronyms)]
    DALLE,
    /// Whisper audio models
    Whisper,
    /// TTS (Text-to-Speech) models
    #[allow(clippy::upper_case_acronyms)]
    TTS,
    /// Embedding models
    Embeddings,
    /// Moderation models
    Moderation,
    /// Unknown or unclassified
    Unknown,
}

/// Model tier/quality classification
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ModelTier {
    /// Legacy models (deprecated or older)
    Legacy,
    /// Standard quality models
    Standard,
    /// Premium/high-quality models
    Premium,
    /// Cutting-edge/experimental models
    Experimental,
}

/// Internal enum to categorize model types for capability determination
#[derive(Debug, Clone, Copy, PartialEq)]
enum ModelType {
    /// GPT-4o series models
    Gpt4o,
    /// GPT-4 Turbo models
    Gpt4Turbo,
    /// GPT-4 models
    Gpt4,
    /// GPT-3.5 models
    Gpt35,
    /// DALL-E image generation models
    Dalle,
    /// Whisper audio models
    Whisper,
    /// TTS (Text-to-Speech) models
    Tts,
    /// Embedding models
    Embedding,
    /// Moderation models
    Moderation,
    /// Legacy/unknown models
    Legacy,
}

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

impl ModelCapabilities {
    /// Create model capabilities from a model ID
    #[must_use]
    pub fn from_model_id(model_id: &str) -> Self {
        let family = Self::classify_family(model_id);
        let tier = Self::classify_tier(model_id);

        // Determine model type based on ID patterns
        let model_type = Self::determine_model_type(model_id);

        match model_type {
            ModelType::Gpt4o => Self::gpt4o_capabilities(family, tier),
            ModelType::Gpt4Turbo => Self::gpt4_turbo_capabilities(model_id, family, tier),
            ModelType::Gpt4 => Self::gpt4_capabilities(model_id, family, tier),
            ModelType::Gpt35 => Self::gpt35_capabilities(model_id, family, tier),
            ModelType::Dalle => Self::dalle_capabilities(family, tier),
            ModelType::Whisper => Self::whisper_capabilities(family, tier),
            ModelType::Tts => Self::tts_capabilities(family, tier),
            ModelType::Embedding => Self::embedding_capabilities(family, tier),
            ModelType::Moderation => Self::moderation_capabilities(family, tier),
            ModelType::Legacy => Self::legacy_capabilities(family, tier),
        }
    }

    /// Determine the model type from the model ID
    fn determine_model_type(model_id: &str) -> ModelType {
        Self::match_model_prefix(model_id)
            .or_else(|| Self::match_model_contains(model_id))
            .unwrap_or(ModelType::Legacy)
    }

    /// Match model type by prefix patterns
    fn match_model_prefix(model_id: &str) -> Option<ModelType> {
        if model_id.starts_with("gpt-4o") {
            return Some(ModelType::Gpt4o);
        }

        if Self::is_gpt4_turbo(model_id) {
            return Some(ModelType::Gpt4Turbo);
        }

        let prefix_mappings = [
            ("gpt-4", ModelType::Gpt4),
            ("gpt-3.5-turbo", ModelType::Gpt35),
            ("dall-e", ModelType::Dalle),
            ("whisper", ModelType::Whisper),
            ("tts", ModelType::Tts),
        ];

        for (prefix, model_type) in &prefix_mappings {
            if model_id.starts_with(prefix) {
                return Some(*model_type);
            }
        }

        None
    }

    /// Match model type by substring patterns
    fn match_model_contains(model_id: &str) -> Option<ModelType> {
        let contains_mappings = [
            ("embedding", ModelType::Embedding),
            ("moderation", ModelType::Moderation),
        ];

        for (substring, model_type) in &contains_mappings {
            if model_id.contains(substring) {
                return Some(*model_type);
            }
        }

        None
    }

    /// Check if the model ID represents a GPT-4 Turbo model
    fn is_gpt4_turbo(model_id: &str) -> bool {
        model_id.starts_with("gpt-4-turbo")
            || model_id.contains("gpt-4-1106")
            || model_id.contains("gpt-4-0125")
    }

    /// Create capabilities for GPT-4o models
    fn gpt4o_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: Some(128_000),
            training_cutoff: Some("2023-10".to_string()),
            completion_types: vec![CompletionType::Chat, CompletionType::Code],
            supports_function_calling: true,
            supports_vision: true,
            supports_code_interpreter: true,
            family,
            tier,
            input_cost_per_1m_tokens: Some(5.0),
            output_cost_per_1m_tokens: Some(15.0),
        }
    }

    /// Create capabilities for GPT-4 Turbo models
    fn gpt4_turbo_capabilities(model_id: &str, family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: Some(128_000),
            training_cutoff: Some("2023-04".to_string()),
            completion_types: vec![CompletionType::Chat, CompletionType::Code],
            supports_function_calling: true,
            supports_vision: model_id.contains("vision"),
            supports_code_interpreter: true,
            family,
            tier,
            input_cost_per_1m_tokens: Some(10.0),
            output_cost_per_1m_tokens: Some(30.0),
        }
    }

    /// Create capabilities for GPT-4 models
    fn gpt4_capabilities(model_id: &str, family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: Some(if model_id.contains("32k") {
                32_768
            } else {
                8_192
            }),
            training_cutoff: Some("2021-09".to_string()),
            completion_types: vec![CompletionType::Chat, CompletionType::Code],
            supports_function_calling: true,
            supports_vision: false,
            supports_code_interpreter: false,
            family,
            tier,
            input_cost_per_1m_tokens: Some(30.0),
            output_cost_per_1m_tokens: Some(60.0),
        }
    }

    /// Create capabilities for GPT-3.5 models
    fn gpt35_capabilities(model_id: &str, family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: Some(if model_id.contains("16k") {
                16_385
            } else {
                4_097
            }),
            training_cutoff: Some("2021-09".to_string()),
            completion_types: vec![CompletionType::Chat, CompletionType::Code],
            supports_function_calling: !model_id.contains("0301"),
            supports_vision: false,
            supports_code_interpreter: false,
            family,
            tier,
            input_cost_per_1m_tokens: Some(0.5),
            output_cost_per_1m_tokens: Some(1.5),
        }
    }

    /// Create capabilities for DALL-E models
    fn dalle_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: None,
            training_cutoff: None,
            completion_types: vec![CompletionType::Image],
            supports_function_calling: false,
            supports_vision: false,
            supports_code_interpreter: false,
            family,
            tier,
            input_cost_per_1m_tokens: None,
            output_cost_per_1m_tokens: None,
        }
    }

    /// Create capabilities for Whisper models
    fn whisper_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: None,
            training_cutoff: None,
            completion_types: vec![CompletionType::Audio],
            supports_function_calling: false,
            supports_vision: false,
            supports_code_interpreter: false,
            family,
            tier,
            input_cost_per_1m_tokens: None,
            output_cost_per_1m_tokens: None,
        }
    }

    /// Create capabilities for TTS models
    fn tts_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: None,
            training_cutoff: None,
            completion_types: vec![CompletionType::Audio],
            supports_function_calling: false,
            supports_vision: false,
            supports_code_interpreter: false,
            family,
            tier,
            input_cost_per_1m_tokens: None,
            output_cost_per_1m_tokens: None,
        }
    }

    /// Create capabilities for embedding models
    fn embedding_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: Some(8_191),
            training_cutoff: None,
            completion_types: vec![CompletionType::Embeddings],
            supports_function_calling: false,
            supports_vision: false,
            supports_code_interpreter: false,
            family,
            tier,
            input_cost_per_1m_tokens: Some(0.02),
            output_cost_per_1m_tokens: None,
        }
    }

    /// Create capabilities for moderation models
    fn moderation_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: None,
            training_cutoff: None,
            completion_types: vec![CompletionType::Moderation],
            supports_function_calling: false,
            supports_vision: false,
            supports_code_interpreter: false,
            family,
            tier,
            input_cost_per_1m_tokens: None,
            output_cost_per_1m_tokens: None,
        }
    }

    /// Create capabilities for legacy models
    fn legacy_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
        Self {
            max_tokens: Some(4_097),
            training_cutoff: Some("2021-09".to_string()),
            completion_types: vec![CompletionType::Text],
            supports_function_calling: false,
            supports_vision: false,
            supports_code_interpreter: false,
            family,
            tier,
            input_cost_per_1m_tokens: None,
            output_cost_per_1m_tokens: None,
        }
    }

    /// Classify the model family based on the model ID
    #[must_use]
    pub fn classify_family(model_id: &str) -> ModelFamily {
        // Use a simpler approach with early returns to reduce complexity
        if Self::is_gpt4o_model(model_id) {
            return ModelFamily::GPT4o;
        }
        if Self::is_gpt4_turbo_model(model_id) {
            return ModelFamily::GPT4Turbo;
        }
        if Self::is_gpt4_model(model_id) {
            return ModelFamily::GPT4;
        }
        if Self::is_gpt35_model(model_id) {
            return ModelFamily::GPT35;
        }
        if Self::is_dalle_model(model_id) {
            return ModelFamily::DALLE;
        }
        if Self::is_whisper_model(model_id) {
            return ModelFamily::Whisper;
        }
        if Self::is_tts_model(model_id) {
            return ModelFamily::TTS;
        }
        if Self::is_embedding_model(model_id) {
            return ModelFamily::Embeddings;
        }
        if Self::is_moderation_model(model_id) {
            return ModelFamily::Moderation;
        }

        ModelFamily::Unknown
    }

    /// Check if model ID matches GPT-4o family
    fn is_gpt4o_model(model_id: &str) -> bool {
        model_id.starts_with("gpt-4o")
    }

    /// Check if model ID matches GPT-4 Turbo family
    fn is_gpt4_turbo_model(model_id: &str) -> bool {
        model_id.starts_with("gpt-4-turbo")
            || model_id.contains("gpt-4-1106")
            || model_id.contains("gpt-4-0125")
    }

    /// Check if model ID matches GPT-4 family
    fn is_gpt4_model(model_id: &str) -> bool {
        model_id.starts_with("gpt-4")
    }

    /// Check if model ID matches GPT-3.5 family
    fn is_gpt35_model(model_id: &str) -> bool {
        model_id.starts_with("gpt-3.5")
    }

    /// Check if model ID matches DALL-E family
    fn is_dalle_model(model_id: &str) -> bool {
        model_id.starts_with("dall-e")
    }

    /// Check if model ID matches Whisper family
    fn is_whisper_model(model_id: &str) -> bool {
        model_id.starts_with("whisper")
    }

    /// Check if model ID matches TTS family
    fn is_tts_model(model_id: &str) -> bool {
        model_id.starts_with("tts")
    }

    /// Check if model ID matches Embeddings family
    fn is_embedding_model(model_id: &str) -> bool {
        model_id.contains("embedding")
    }

    /// Check if model ID matches Moderation family
    fn is_moderation_model(model_id: &str) -> bool {
        model_id.contains("moderation")
    }

    /// Classify the model tier based on the model ID
    #[must_use]
    pub fn classify_tier(model_id: &str) -> ModelTier {
        match model_id {
            // Legacy/deprecated models
            "text-davinci-003" | "text-davinci-002" | "text-curie-001" | "text-babbage-001"
            | "text-ada-001" | "davinci" | "curie" | "babbage" | "ada" | "gpt-3.5-turbo-0301"
            | "gpt-4-0314" => ModelTier::Legacy,

            // Experimental/preview models
            id if id.contains("preview") || id.contains("alpha") || id.contains("beta") => {
                ModelTier::Experimental
            }

            // Premium models
            id if id.starts_with("gpt-4") => ModelTier::Premium,

            // Standard models
            _ => ModelTier::Standard,
        }
    }

    /// Get estimated monthly cost for processing tokens
    #[must_use]
    pub fn estimate_monthly_cost(
        &self,
        input_tokens_per_month: u64,
        output_tokens_per_month: u64,
    ) -> Option<f64> {
        match (
            self.input_cost_per_1m_tokens,
            self.output_cost_per_1m_tokens,
        ) {
            (Some(input_cost), Some(output_cost)) => {
                let input_cost_total = (input_tokens_per_month as f64 / 1_000_000.0) * input_cost;
                let output_cost_total =
                    (output_tokens_per_month as f64 / 1_000_000.0) * output_cost;
                Some(input_cost_total + output_cost_total)
            }
            (Some(input_cost), None) => {
                Some((input_tokens_per_month as f64 / 1_000_000.0) * input_cost)
            }
            _ => None,
        }
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

/// Requirements for finding suitable models
#[derive(Debug, Clone)]
pub struct ModelRequirements {
    /// Required completion types
    pub completion_types: Vec<CompletionType>,

    /// Minimum context window size needed
    pub min_max_tokens: Option<u32>,

    /// Whether function calling is required
    pub requires_function_calling: bool,

    /// Whether vision capabilities are required
    pub requires_vision: bool,

    /// Whether code interpreter is required
    pub requires_code_interpreter: bool,

    /// Whether to exclude deprecated models
    pub exclude_deprecated: bool,
}

impl Default for ModelRequirements {
    fn default() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            min_max_tokens: None,
            requires_function_calling: false,
            requires_vision: false,
            requires_code_interpreter: false,
            exclude_deprecated: true,
        }
    }
}

impl ModelRequirements {
    /// Create requirements for basic chat
    #[must_use]
    pub fn chat() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            ..Default::default()
        }
    }

    /// Create requirements for function calling
    #[must_use]
    pub fn function_calling() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            requires_function_calling: true,
            ..Default::default()
        }
    }

    /// Create requirements for vision tasks
    #[must_use]
    pub fn vision() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            requires_vision: true,
            ..Default::default()
        }
    }

    /// Create requirements for code tasks
    #[must_use]
    pub fn code() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat, CompletionType::Code],
            requires_code_interpreter: true,
            ..Default::default()
        }
    }

    /// Create requirements for high-context tasks
    #[must_use]
    pub fn high_context(min_tokens: u32) -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            min_max_tokens: Some(min_tokens),
            ..Default::default()
        }
    }
}

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
}
