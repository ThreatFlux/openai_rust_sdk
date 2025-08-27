//! Request structures for the moderations API

use super::ModerationInput;
use crate::{De, Ser};

/// Request for content moderation
#[derive(Debug, Clone, Ser, De)]
pub struct ModerationRequest {
    /// Input text to moderate (string or array of strings)
    pub input: ModerationInput,

    /// ID of the model to use (e.g., "text-moderation-stable", "text-moderation-latest")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl ModerationRequest {
    /// Create a new moderation request with a single string
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: ModerationInput::String(input.into()),
            model: None,
        }
    }

    /// Create a new moderation request with multiple strings
    #[must_use]
    pub fn new_batch(inputs: Vec<String>) -> Self {
        Self {
            input: ModerationInput::StringArray(inputs),
            model: None,
        }
    }

    /// Set the model to use for moderation
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
}
