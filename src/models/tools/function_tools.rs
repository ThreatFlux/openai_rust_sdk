//! Function tool configuration and types

use crate::{De, Ser};
use serde_json::Value;

/// Function tool (existing from functions module)
#[derive(Debug, Clone, Ser, De)]
pub struct FunctionTool {
    /// Function name
    pub name: String,

    /// Function description
    pub description: String,

    /// Function parameters schema
    pub parameters: Value,

    /// Whether to enforce strict parameter validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}
