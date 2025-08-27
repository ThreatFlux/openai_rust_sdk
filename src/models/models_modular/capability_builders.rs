//! # Model Capability Builders
//!
//! Functions to build capabilities for different model types.

use super::enums::{CompletionType, ModelFamily, ModelTier};
use super::types::ModelCapabilities;

impl ModelCapabilities {
    /// Create capabilities for GPT-4o models
    pub(crate) fn gpt4o_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
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
    pub(crate) fn gpt4_turbo_capabilities(
        model_id: &str,
        family: ModelFamily,
        tier: ModelTier,
    ) -> Self {
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
    pub(crate) fn gpt4_capabilities(model_id: &str, family: ModelFamily, tier: ModelTier) -> Self {
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
    pub(crate) fn gpt35_capabilities(model_id: &str, family: ModelFamily, tier: ModelTier) -> Self {
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
    pub(crate) fn dalle_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
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
    pub(crate) fn whisper_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
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
    pub(crate) fn tts_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
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
    pub(crate) fn embedding_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
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
    pub(crate) fn moderation_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
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
    pub(crate) fn legacy_capabilities(family: ModelFamily, tier: ModelTier) -> Self {
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
}
