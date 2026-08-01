//! Core tool types and enumerations

use super::{
    CodeInterpreterConfig, ComputerUseConfig, FileSearchConfig, FunctionTool,
    ImageGenerationConfig, McpTool, WebSearchConfig,
};
use crate::{De, Ser};
use serde_json::Value;

/// Whether tool search is executed by OpenAI or by the client.
#[derive(Debug, Clone, Copy, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolSearchExecution {
    /// OpenAI performs the tool search.
    Server,
    /// The client performs the tool search.
    Client,
}

/// Configuration for the hosted/client tool-search capability.
#[derive(Debug, Clone, Ser, De)]
pub struct ToolSearchConfig {
    /// Execution location for tool search.
    pub execution: ToolSearchExecution,
    /// Description shown to the model for client execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameter schema for client-executed searches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

/// Caller context allowed to invoke a callable tool.
#[derive(Debug, Clone, Copy, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AllowedToolCaller {
    /// Direct model invocation.
    Direct,
    /// Invocation from programmatic tool calling.
    Programmatic,
}

/// Configuration for the shell tool.
#[derive(Debug, Clone, Ser, De, Default)]
pub struct ShellToolConfig {
    /// Container or local execution environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Value>,
    /// Invocation contexts allowed by the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_callers: Option<Vec<AllowedToolCaller>>,
}

/// Configuration for the apply-patch tool.
#[derive(Debug, Clone, Ser, De, Default)]
pub struct ApplyPatchToolConfig {
    /// Invocation contexts allowed by the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_callers: Option<Vec<AllowedToolCaller>>,
}

/// Configuration for a custom text/grammar tool.
#[derive(Debug, Clone, Ser, De)]
pub struct CustomToolConfig {
    /// Name used to identify the custom tool.
    pub name: String,
    /// Optional model-facing description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Text or grammar input format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Value>,
    /// Whether this tool is discovered through tool search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_loading: Option<bool>,
    /// Invocation contexts allowed by the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_callers: Option<Vec<AllowedToolCaller>>,
}

/// A namespace grouping function or custom tools.
#[derive(Debug, Clone, Ser, De)]
pub struct NamespaceToolConfig {
    /// Namespace used in tool calls.
    pub name: String,
    /// Description shown to the model.
    pub description: String,
    /// Function/custom tools in this namespace.
    pub tools: Vec<Value>,
}

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

    /// Search deferred tools on the server or client.
    ToolSearch(ToolSearchConfig),

    /// Allow the model to invoke tools programmatically.
    ProgrammaticToolCalling,

    /// Execute shell commands in the local environment.
    LocalShell,

    /// Execute shell commands in a configured environment.
    Shell(ShellToolConfig),

    /// Create, update, or delete files with unified patches.
    ApplyPatch(ApplyPatchToolConfig),

    /// A custom text or grammar tool.
    Custom(CustomToolConfig),

    /// Group function or custom tools under a namespace.
    Namespace(NamespaceToolConfig),
}
