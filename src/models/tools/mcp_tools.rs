//! MCP (Model Context Protocol) tool configuration and types

use crate::{De, Ser};
use std::collections::HashMap;

/// Remote MCP (Model Context Protocol) server configuration
#[derive(Debug, Clone, Ser, De)]
pub struct McpTool {
    /// Label for the MCP server
    pub server_label: String,

    /// URL of the MCP server
    pub server_url: String,

    /// Approval requirement for tool calls
    #[serde(default = "default_approval")]
    pub require_approval: McpApproval,

    /// Custom headers for MCP server authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Timeout for MCP server calls (in milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u32>,
}

/// MCP approval requirement levels
#[derive(Debug, Clone, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum McpApproval {
    /// Never require approval
    Never,
    /// Always require approval
    Always,
    /// Require approval for sensitive operations
    Sensitive,
}

/// Default approval behavior for MCP tools when omitted by callers.
fn default_approval() -> McpApproval {
    McpApproval::Sensitive
}
