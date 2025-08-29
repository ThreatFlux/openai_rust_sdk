//! Builder for function tool configurations

use super::{EnhancedTool, FunctionTool};
use serde_json::Value;

/// Builder for function tools
pub struct FunctionBuilder {
    /// The function tool being built
    tool: FunctionTool,
}

impl FunctionBuilder {
    /// Create a new FunctionBuilder with the specified name and description
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            tool: FunctionTool {
                name: name.into(),
                description: description.into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
                strict: None,
            },
        }
    }

    /// Set the function parameters schema
    #[must_use]
    pub fn parameters(mut self, params: Value) -> Self {
        self.tool.parameters = params;
        self
    }

    /// Enable or disable strict mode for function calls
    #[must_use]
    pub fn strict(mut self, strict: bool) -> Self {
        self.tool.strict = Some(strict);
        self
    }

    /// Build the configured function tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::Function(self.tool)
    }
}
