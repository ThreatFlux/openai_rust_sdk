//! # Core Model Types
//!
//! Core data structures for the OpenAI Models API.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

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
    pub completion_types: Vec<super::enums::CompletionType>,

    /// Whether the model supports function calling
    pub supports_function_calling: bool,

    /// Whether the model supports vision
    pub supports_vision: bool,

    /// Whether the model supports code interpreter
    pub supports_code_interpreter: bool,

    /// Model family (e.g., "gpt-4", "gpt-3.5", "dall-e")
    pub family: super::enums::ModelFamily,

    /// Model tier/quality level
    pub tier: super::enums::ModelTier,

    /// Input cost per 1M tokens (if available)
    pub input_cost_per_1m_tokens: Option<f64>,

    /// Output cost per 1M tokens (if available)
    pub output_cost_per_1m_tokens: Option<f64>,
}

/// Requirements for filtering and finding suitable models
#[derive(Debug, Clone, Default)]
pub struct ModelRequirements {
    /// Supported completion types
    pub completion_types: Vec<super::enums::CompletionType>,

    /// Minimum max tokens requirement
    pub min_max_tokens: Option<u32>,

    /// Requires function calling support
    pub requires_function_calling: bool,

    /// Requires vision support
    pub requires_vision: bool,

    /// Requires code interpreter support
    pub requires_code_interpreter: bool,

    /// Exclude deprecated models
    pub exclude_deprecated: bool,
}
