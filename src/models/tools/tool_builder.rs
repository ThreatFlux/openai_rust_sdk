//! Main factory for creating tool configurations

use super::{
    CodeInterpreterBuilder, ComputerUseBuilder, EnhancedTool, FileSearchBuilder, FunctionBuilder,
    ImageGenerationToolBuilder, McpBuilder, WebSearchBuilder,
};

/// Builder for creating tool configurations
pub struct ToolBuilder;

impl ToolBuilder {
    /// Create a simple web search tool
    #[must_use]
    pub fn web_search() -> EnhancedTool {
        EnhancedTool::WebSearchPreview
    }

    /// Create an advanced web search tool with configuration
    #[must_use]
    pub fn web_search_advanced() -> WebSearchBuilder {
        WebSearchBuilder::new()
    }

    /// Create a file search tool
    #[must_use]
    pub fn file_search(vector_store_ids: Vec<String>) -> FileSearchBuilder {
        FileSearchBuilder::new(vector_store_ids)
    }

    /// Create a function tool
    pub fn function(name: impl Into<String>, description: impl Into<String>) -> FunctionBuilder {
        FunctionBuilder::new(name, description)
    }

    /// Create an MCP server tool
    pub fn mcp(server_label: impl Into<String>, server_url: impl Into<String>) -> McpBuilder {
        McpBuilder::new(server_label, server_url)
    }

    /// Create an image generation tool
    #[must_use]
    pub fn image_generation() -> ImageGenerationToolBuilder {
        ImageGenerationToolBuilder::new()
    }

    /// Create a code interpreter tool
    #[must_use]
    pub fn code_interpreter() -> CodeInterpreterBuilder {
        CodeInterpreterBuilder::new()
    }

    /// Create a computer use tool
    #[must_use]
    pub fn computer_use() -> ComputerUseBuilder {
        ComputerUseBuilder::new()
    }
}
