//! Core tool types and enumerations

use super::{
    CodeInterpreterConfig, ComputerUseConfig, FileSearchConfig, FunctionTool,
    ImageGenerationConfig, McpTool, WebSearchConfig,
};
use crate::{De, Ser};

/// Main tool enum representing all available tool types
#[derive(Debug, Clone, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EnhancedTool {
    /// Web search tool for including internet data
    WebSearchPreview,

    /// Advanced web search with configuration
    WebSearch(WebSearchConfig),

    /// File search tool for searching uploaded files
    FileSearch(FileSearchConfig),

    /// Function calling tool
    Function(FunctionTool),

    /// Remote MCP server tool
    Mcp(McpTool),

    /// Image generation tool
    ImageGeneration(ImageGenerationConfig),

    /// Code interpreter tool
    CodeInterpreter(CodeInterpreterConfig),

    /// Computer use tool for agentic workflows
    ComputerUse(ComputerUseConfig),
}
