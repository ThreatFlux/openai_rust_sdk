//! # Model Types
//!
//! Core types and enums for model classification and capabilities.

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
pub(crate) enum ModelType {
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