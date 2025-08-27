//! Builder patterns for creating moderation requests

use super::{ModerationModels, ModerationRequest};

/// Builder for creating moderation requests
pub struct ModerationBuilder {
    /// The underlying moderation request being built
    request: ModerationRequest,
}

impl ModerationBuilder {
    /// Create a new builder with a single input
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            request: ModerationRequest::new(input),
        }
    }

    /// Create a new builder with multiple inputs
    #[must_use]
    pub fn new_batch(inputs: Vec<String>) -> Self {
        Self {
            request: ModerationRequest::new_batch(inputs),
        }
    }

    /// Set the model to use
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.request.model = Some(model.into());
        self
    }

    /// Use the stable model
    #[must_use]
    pub fn stable_model(mut self) -> Self {
        self.request.model = Some(ModerationModels::STABLE.to_string());
        self
    }

    /// Use the latest model
    #[must_use]
    pub fn latest_model(mut self) -> Self {
        self.request.model = Some(ModerationModels::LATEST.to_string());
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> ModerationRequest {
        self.request
    }
}
