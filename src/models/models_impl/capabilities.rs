//! # Model Capabilities
//!
//! Detailed information about model capabilities including context windows,
//! supported features, and cost information.

use super::types::{CompletionType, ModelFamily, ModelTier, ModelType};

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

    /// Classify the model family based on the model ID
    #[must_use]
    pub fn classify_family(model_id: &str) -> ModelFamily {
        // Use a functional approach with iterators to reduce cyclomatic complexity
        
        // All patterns and their corresponding families in a single lookup table
        // (pattern, is_prefix_match, family)
        const MODEL_PATTERNS: &[(&str, bool, ModelFamily)] = &[
            // Special cases first (most specific patterns)
            ("gpt-4o", true, ModelFamily::GPT4o),
            ("gpt-4-turbo", true, ModelFamily::GPT4Turbo),
            ("gpt-4-1106", false, ModelFamily::GPT4Turbo),
            ("gpt-4-0125", false, ModelFamily::GPT4Turbo),
            
            // General patterns
            ("gpt-4", true, ModelFamily::GPT4),
            ("gpt-3.5", true, ModelFamily::GPT35),
            ("dall-e", true, ModelFamily::DALLE),
            ("whisper", true, ModelFamily::Whisper),
            ("tts", true, ModelFamily::TTS),
            ("embedding", false, ModelFamily::Embeddings),
            ("moderation", false, ModelFamily::Moderation),
        ];

        // Find first matching pattern using functional style
        MODEL_PATTERNS
            .iter()
            .find(|(pattern, is_prefix, _)| {
                if *is_prefix {
                    model_id.starts_with(pattern)
                } else {
                    model_id.contains(pattern)
                }
            })
            .map(|(_, _, family)| *family)
            .unwrap_or(ModelFamily::Unknown)
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