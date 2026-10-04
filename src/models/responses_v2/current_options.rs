//! Current Responses options added without changing the existing request layout.

use super::CreateResponseRequest;
use crate::{De, Ser};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Retention requested for a model's prompt-cache entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ser, De)]
pub enum PromptCacheRetention {
    /// Retain entries in memory.
    #[serde(rename = "in_memory")]
    InMemory,
    /// Request a 24-hour cache lifetime where the model supports it.
    #[serde(rename = "24h")]
    TwentyFourHours,
}

/// A supported model-managed context operation.
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContextManagement {
    /// Automatically compact context at the optional input-token threshold.
    Compaction {
        /// Threshold at which the server should compact context.
        #[serde(skip_serializing_if = "Option::is_none")]
        compact_threshold: Option<u64>,
    },
}

/// Current optional create fields supplied alongside an existing request.
#[derive(Debug, Clone, Default, Ser, De)]
pub struct ResponseCreateOptions {
    /// Automatic context-management operations supported by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_management: Option<Vec<ContextManagement>>,
    /// Prompt-cache retention supported by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_retention: Option<PromptCacheRetention>,
}

impl ResponseCreateOptions {
    /// Serialize the existing request and these documented optional fields.
    pub fn to_payload(&self, request: &CreateResponseRequest) -> serde_json::Result<Value> {
        let mut payload = request.to_payload()?;
        if let (Some(target), Value::Object(options)) =
            (payload.as_object_mut(), serde_json::to_value(self)?)
        {
            target.extend(options);
        }
        Ok(payload)
    }
}
