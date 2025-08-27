//! Tool choice configuration and types

use crate::{De, Ser};

/// Tool choice configuration
#[derive(Debug, Clone, Ser, De)]
#[serde(untagged)]
#[derive(Default)]
pub enum EnhancedToolChoice {
    /// Let the model decide whether to use tools
    #[default]
    Auto,

    /// Force the model to use tools
    Required,

    /// Prevent the model from using tools
    None,

    /// Force the model to use a specific tool
    Specific(SpecificToolChoice),
}

/// Specific tool choice configuration
#[derive(Debug, Clone, Ser, De)]
pub struct SpecificToolChoice {
    /// Type of tool to use
    #[serde(rename = "type")]
    pub tool_type: String,

    /// Name of the specific tool (for functions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
