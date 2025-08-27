//! Builder for MCP (Model Context Protocol) tool configurations

use super::*;
use std::collections::HashMap;

/// Builder for MCP server tools
pub struct McpBuilder {
    /// The MCP tool being built
    tool: McpTool,
}

impl McpBuilder {
    /// Create a new McpBuilder with the specified server label and URL
    pub fn new(server_label: impl Into<String>, server_url: impl Into<String>) -> Self {
        Self {
            tool: McpTool {
                server_label: server_label.into(),
                server_url: server_url.into(),
                require_approval: McpApproval::Sensitive,
                headers: None,
                timeout_ms: None,
            },
        }
    }

    /// Set the approval requirement for MCP operations
    #[must_use]
    pub fn require_approval(mut self, approval: McpApproval) -> Self {
        self.tool.require_approval = approval;
        self
    }

    /// Add a custom header to MCP requests
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let headers = self.tool.headers.get_or_insert_with(HashMap::new);
        headers.insert(key.into(), value.into());
        self
    }

    /// Set the timeout in milliseconds for MCP operations
    #[must_use]
    pub fn timeout_ms(mut self, timeout: u32) -> Self {
        self.tool.timeout_ms = Some(timeout);
        self
    }

    /// Build the configured MCP tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::Mcp(self.tool)
    }
}
