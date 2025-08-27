use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// Usage statistics for the response
#[derive(Debug, Clone, Ser, De)]
pub struct Usage {
    /// Number of tokens in the prompt
    #[serde(default)]
    pub prompt_tokens: u32,
    /// Number of tokens in the completion
    #[serde(default)]
    pub completion_tokens: u32,
    /// Total number of tokens used
    #[serde(default)]
    pub total_tokens: u32,
    /// Detailed prompt token information including caching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokenDetails>,
    /// Detailed completion token information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokenDetails>,
}

/// Detailed prompt token information including caching
#[derive(Debug, Clone, Ser, De)]
pub struct PromptTokenDetails {
    /// Number of cached tokens used from prompt cache
    #[serde(default)]
    pub cached_tokens: u32,
    /// Audio tokens if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
}

/// Detailed completion token information
#[derive(Debug, Clone, Ser, De)]
pub struct CompletionTokenDetails {
    /// Reasoning tokens used (for models with reasoning capabilities)
    #[serde(default)]
    pub reasoning_tokens: u32,
    /// Accepted prediction tokens
    #[serde(default)]
    pub accepted_prediction_tokens: u32,
    /// Rejected prediction tokens
    #[serde(default)]
    pub rejected_prediction_tokens: u32,
    /// Audio tokens if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
}

/// Prompt template with variables for reusable prompts
#[derive(Debug, Clone, Ser, De)]
pub struct PromptTemplate {
    /// Unique identifier for the template (e.g., "`pmpt_abc123`")
    pub id: String,
    /// Version of the template (defaults to "current" if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Variables to substitute in the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, PromptVariable>>,
}

/// Variable value for prompt templates
#[derive(Debug, Clone, Ser, De)]
#[serde(untagged)]
pub enum PromptVariable {
    /// String variable
    String(String),
    /// Image input variable
    Image(ImageInput),
    /// File input variable
    File(FileInput),
    /// JSON value for complex data
    Json(serde_json::Value),
}

/// Image input for prompt variables
#[derive(Debug, Clone, Ser, De)]
pub struct ImageInput {
    /// Type indicator
    #[serde(rename = "type")]
    pub input_type: String,
    /// Image URL or base64 data
    pub url: String,
    /// Detail level for image processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// File input for prompt variables
#[derive(Debug, Clone, Ser, De)]
pub struct FileInput {
    /// Type indicator
    #[serde(rename = "type")]
    pub input_type: String,
    /// File ID
    pub file_id: String,
}

/// Tool call information
#[derive(Debug, Clone, Ser, De)]
pub struct ToolCall {
    /// Unique identifier for the tool call
    pub id: String,
    /// Name of the tool being called
    pub name: String,
    /// Arguments passed to the tool
    pub arguments: serde_json::Value,
}

/// Default object type for responses
pub(crate) fn default_object_type() -> String {
    "chat.completion".to_string()
}

/// Default model name for responses
pub(crate) fn default_model() -> String {
    "unknown".to_string()
}
