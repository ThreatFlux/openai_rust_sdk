//! Dedicated compaction models and a documented token-count projection.

use super::{
    CreateResponseRequest, Instructions, PromptCacheRetention, ResponseInput, ResponseItem,
    ServiceTier,
};
use crate::error::{OpenAIError, Result};
use crate::models::gpt5::PromptCacheOptions;
use crate::{De, Ser};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Request accepted by `POST /v1/responses/compact`.
#[derive(Debug, Clone, Default, Ser, De)]
pub struct CompactResponseRequest {
    /// Model used for compaction.
    pub model: String,
    /// Conversation input; may be omitted when continuing a stored response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<ResponseInput>,
    /// New system or developer instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Previous response to compact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// Cache-routing key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Explicit or implicit cache-breakpoint configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_options: Option<PromptCacheOptions>,
    /// Cache retention where supported by the selected model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_retention: Option<PromptCacheRetention>,
    /// Serving tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
}

impl CompactResponseRequest {
    /// Create a compaction request with explicit context input.
    #[must_use]
    pub fn new(model: impl Into<String>, input: impl Into<ResponseInput>) -> Self {
        Self {
            model: model.into(),
            input: Some(input.into()),
            ..Self::default()
        }
    }

    /// Convert message input into the same wire format used by response creation.
    pub fn to_payload(&self) -> serde_json::Result<Value> {
        let mut payload = serde_json::to_value(self)?;
        if let Some(input) = &self.input {
            let create = CreateResponseRequest {
                input: input.clone(),
                ..Default::default()
            };
            payload["input"] = create.to_payload()?["input"].clone();
        }
        Ok(payload)
    }
}

impl TryFrom<&CreateResponseRequest> for CompactResponseRequest {
    type Error = OpenAIError;

    fn try_from(request: &CreateResponseRequest) -> Result<Self> {
        if request.conversation.is_some() || request.prompt.is_some() {
            return Err(OpenAIError::invalid_request(
                "Compaction accepts explicit input or previous_response_id; conversation and prompt are unsupported",
            ));
        }
        Ok(Self {
            model: request.model.clone(),
            input: Some(request.input.clone()),
            instructions: plain_instructions(request.instructions.as_ref())?,
            previous_response_id: request.previous_response_id.clone(),
            prompt_cache_key: request.prompt_cache_key.clone(),
            prompt_cache_options: request.prompt_cache_options.clone(),
            service_tier: request.service_tier,
            prompt_cache_retention: None,
        })
    }
}

/// Context returned by compaction; replay the complete `output` array.
#[derive(Debug, Clone, Ser, De)]
pub struct CompactedResponse {
    /// Compaction identifier.
    pub id: String,
    /// Resource discriminator (`response.compaction`).
    pub object: String,
    /// Unix timestamp when compaction finished.
    pub created_at: u64,
    /// User items followed by the opaque compaction item.
    pub output: Vec<ResponseItem>,
    /// Accounting for this compaction pass.
    pub usage: CompactionUsage,
    /// Additional server fields.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Token accounting including cache-write metrics absent from legacy usage.
#[derive(Debug, Clone, Ser, De)]
pub struct CompactionUsage {
    /// Input tokens consumed.
    pub input_tokens: u64,
    /// Output tokens produced.
    pub output_tokens: u64,
    /// Combined input and output tokens.
    pub total_tokens: u64,
    /// Input cache metrics.
    pub input_tokens_details: CompactionTokenDetails,
    /// Output reasoning metrics.
    pub output_tokens_details: CompactionTokenDetails,
    /// Additional server fields.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Input cache or output reasoning token details.
#[derive(Debug, Clone, Default, Ser, De)]
pub struct CompactionTokenDetails {
    /// Tokens read from cache, when reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u64>,
    /// Tokens written to cache, when reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<u64>,
    /// Reasoning tokens, when reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u64>,
    /// Additional token metrics.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl CreateResponseRequest {
    /// Project the request onto the documented input-token-count schema.
    ///
    /// Generation, storage, streaming, metadata, and cache-routing controls are
    /// excluded. Prompt-template expansion is unsupported by the count endpoint
    /// and is rejected rather than silently counting different input.
    pub fn to_input_token_payload(&self) -> Result<Value> {
        if self.prompt.is_some() {
            return Err(OpenAIError::invalid_request(
                "Input token counting does not accept prompt templates",
            ));
        }
        plain_instructions(self.instructions.as_ref())?;
        let Value::Object(mut payload) = self.to_payload()? else {
            unreachable!()
        };
        payload.retain(|key, _| is_count_field(key));
        Ok(Value::Object(payload))
    }
}

/// Identify fields explicitly accepted by the input-token endpoint.
fn is_count_field(key: &str) -> bool {
    matches!(
        key,
        "model"
            | "input"
            | "instructions"
            | "conversation"
            | "previous_response_id"
            | "parallel_tool_calls"
            | "personality"
            | "reasoning"
            | "text"
            | "tool_choice"
            | "tools"
            | "truncation"
    )
}

/// Require the plain-text instruction shape used by counting and compaction.
fn plain_instructions(instructions: Option<&Instructions>) -> Result<Option<String>> {
    match instructions {
        None => Ok(None),
        Some(Instructions::Text(text) | Instructions::Raw(Value::String(text))) => {
            Ok(Some(text.clone()))
        }
        Some(_) => Err(OpenAIError::invalid_request(
            "This endpoint accepts instructions as plain text",
        )),
    }
}
