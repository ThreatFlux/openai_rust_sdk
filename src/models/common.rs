//! Common traits and utilities for model builders

use crate::models::assistants::AssistantTool;
use crate::models::images::{ImageResponseFormat, ImageSize};
use std::collections::HashMap;

/// Common builder methods for requests with assistant configuration
pub trait AssistantConfigBuilder: Sized {
    fn get_assistant_id_mut(&mut self) -> &mut Option<String>;
    fn get_model_mut(&mut self) -> &mut Option<String>;
    fn get_instructions_mut(&mut self) -> &mut Option<String>;
    fn get_tools_mut(&mut self) -> &mut Option<Vec<AssistantTool>>;
    fn get_metadata_mut(&mut self) -> &mut Option<HashMap<String, String>>;

    /// Set the assistant ID
    fn assistant_id<S: Into<String>>(mut self, assistant_id: S) -> Self {
        *self.get_assistant_id_mut() = Some(assistant_id.into());
        self
    }

    /// Set the model
    fn model<S: Into<String>>(mut self, model: S) -> Self {
        *self.get_model_mut() = Some(model.into());
        self
    }

    /// Set the instructions
    fn instructions<S: Into<String>>(mut self, instructions: S) -> Self {
        *self.get_instructions_mut() = Some(instructions.into());
        self
    }

    /// Add a tool
    fn tool(mut self, tool: AssistantTool) -> Self {
        self.get_tools_mut().get_or_insert_with(Vec::new).push(tool);
        self
    }

    /// Add multiple tools
    fn tools(mut self, tools: Vec<AssistantTool>) -> Self {
        *self.get_tools_mut() = Some(tools);
        self
    }

    /// Set metadata
    fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        *self.get_metadata_mut() = Some(metadata);
        self
    }

    /// Add a metadata key-value pair
    fn metadata_pair<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.get_metadata_mut()
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }
}

/// Common builder methods for image generation requests
pub trait ImageRequestBuilder: Sized {
    fn get_n_mut(&mut self) -> &mut Option<u32>;
    fn get_response_format_mut(&mut self) -> &mut Option<ImageResponseFormat>;
    fn get_size_mut(&mut self) -> &mut Option<ImageSize>;
    fn get_user_mut(&mut self) -> &mut Option<String>;

    /// Set the number of images to generate
    fn with_n(mut self, n: u32) -> Self {
        *self.get_n_mut() = Some(n.clamp(1, 10));
        self
    }

    /// Set the response format
    fn with_response_format(mut self, format: ImageResponseFormat) -> Self {
        *self.get_response_format_mut() = Some(format);
        self
    }

    /// Set the image size
    fn with_size(mut self, size: ImageSize) -> Self {
        *self.get_size_mut() = Some(size);
        self
    }

    /// Set the user identifier
    fn with_user(mut self, user: impl Into<String>) -> Self {
        *self.get_user_mut() = Some(user.into());
        self
    }
}
