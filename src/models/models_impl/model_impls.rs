//! # Model Implementations
//!
//! Method implementations for the Model struct.

use super::types::{CompletionType, Model, ModelFamily};
use super::capabilities::ModelCapabilities;

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