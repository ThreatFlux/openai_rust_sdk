use crate::models::functions::FunctionCall;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

use super::{
    schema_types::SchemaValidationResult,
    usage_types::{default_model, default_object_type, ToolCall, Usage},
};

/// Output content for a response
#[derive(Debug, Clone, Ser, De)]
pub struct ResponseOutput {
    /// The generated text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Tool calls made by the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Function calls made by the model (enhanced format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calls: Option<Vec<FunctionCall>>,
    /// Structured output data (when using response format enforcement)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_data: Option<serde_json::Value>,
    /// Schema validation result for structured outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_validation: Option<SchemaValidationResult>,
}

/// Individual choice in the response
#[derive(Debug, Clone, Ser, De)]
pub struct ResponseChoice {
    /// Index of this choice in the choices array
    pub index: u32,
    /// The generated message content
    pub message: ResponseOutput,
    /// Reason why the generation finished
    pub finish_reason: Option<String>,
}

/// Full response from the API
#[derive(Debug, Clone, Ser, De)]
pub struct ResponseResult {
    /// Unique identifier for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Object type (usually "response")
    #[serde(default = "default_object_type")]
    pub object: String,
    /// Unix timestamp when the response was created
    #[serde(default)]
    pub created: u64,
    /// Model used to generate the response
    #[serde(default = "default_model")]
    pub model: String,
    /// List of response choices
    #[serde(default)]
    pub choices: Vec<ResponseChoice>,
    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

impl ResponseResult {
    /// Get the text output from the first choice
    #[must_use]
    pub fn output_text(&self) -> String {
        self.choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .unwrap_or_default()
    }

    /// Get structured data from the first choice
    #[must_use]
    pub fn structured_data(&self) -> Option<&serde_json::Value> {
        self.choices
            .first()
            .and_then(|choice| choice.message.structured_data.as_ref())
    }

    /// Get parsed structured data as a specific type
    pub fn parse_structured_data<T>(&self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        if let Some(data) = self.structured_data() {
            serde_json::from_value(data.clone())
        } else {
            // Try parsing from text content if no structured data
            let text = self.output_text();
            serde_json::from_str(&text)
        }
    }

    /// Check if the response has valid structured data
    #[must_use]
    pub fn has_valid_structured_data(&self) -> bool {
        self.choices
            .first()
            .and_then(|choice| choice.message.schema_validation.as_ref())
            .is_some_and(|validation| validation.is_valid)
    }

    /// Get schema validation errors
    #[must_use]
    pub fn schema_validation_errors(&self) -> Vec<String> {
        self.choices
            .first()
            .and_then(|choice| choice.message.schema_validation.as_ref())
            .map(|validation| validation.errors.clone())
            .unwrap_or_default()
    }

    /// Get all text outputs from all choices
    #[must_use]
    pub fn all_outputs(&self) -> Vec<String> {
        self.choices
            .iter()
            .filter_map(|choice| choice.message.content.clone())
            .collect()
    }

    /// Get the number of cached tokens used
    #[must_use]
    pub fn cached_tokens(&self) -> u32 {
        self.usage
            .as_ref()
            .and_then(|u| u.prompt_tokens_details.as_ref())
            .map_or(0, |d| d.cached_tokens)
    }

    /// Calculate the cache hit rate as a percentage
    #[must_use]
    pub fn cache_hit_rate(&self) -> f32 {
        if let Some(usage) = &self.usage {
            if usage.prompt_tokens > 0 {
                let cached = self.cached_tokens() as f32;
                let total = usage.prompt_tokens as f32;
                return (cached / total) * 100.0;
            }
        }
        0.0
    }

    /// Check if prompt caching was used
    #[must_use]
    pub fn used_cache(&self) -> bool {
        self.cached_tokens() > 0
    }
}
