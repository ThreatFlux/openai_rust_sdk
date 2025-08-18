//! # OpenAI Tools Module
//!
//! This module provides comprehensive support for all OpenAI tools including:
//! - Web Search: Include internet data in responses
//! - File Search: Search uploaded files for context
//! - Function Calling: Call custom functions (already implemented)
//! - Remote MCP: Access Model Context Protocol servers
//! - Image Generation: Generate or edit images
//! - Code Interpreter: Execute code in secure containers
//! - Computer Use: Agentic computer interface control

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Main tool enum representing all available tool types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EnhancedTool {
    /// Web search tool for including internet data
    WebSearchPreview,

    /// Advanced web search with configuration
    WebSearch(WebSearchConfig),

    /// File search tool for searching uploaded files
    FileSearch(FileSearchConfig),

    /// Function calling tool (existing implementation)
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

/// Web search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchConfig {
    /// Maximum number of search results to include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,

    /// Search query filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<SearchFilters>,

    /// Time range for search results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
}

/// Search filters for web search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    /// Domains to include in search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_domains: Option<Vec<String>>,

    /// Domains to exclude from search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,

    /// Language filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Region filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// File search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchConfig {
    /// Vector store IDs to search
    pub vector_store_ids: Vec<String>,

    /// Maximum number of file chunks to retrieve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chunks: Option<u32>,

    /// File types to search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_types: Option<Vec<String>>,
}

/// Function tool (existing from functions module)
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Remote MCP (Model Context Protocol) server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpApproval {
    /// Never require approval
    Never,
    /// Always require approval
    Always,
    /// Require approval for sensitive operations
    Sensitive,
}

fn default_approval() -> McpApproval {
    McpApproval::Sensitive
}

/// Image generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationConfig {
    /// Image size (e.g., "1024x1024", "512x512")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// Image quality ("standard" or "hd")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,

    /// Image style ("vivid" or "natural")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

    /// Number of images to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
}

/// Code interpreter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeInterpreterConfig {
    /// Container mode: "auto" or "explicit"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_mode: Option<crate::models::containers::ContainerMode>,

    /// Container ID for explicit mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,

    /// Container configuration for auto mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_config: Option<crate::models::containers::ContainerConfig>,

    /// Programming language to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Maximum execution time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_execution_time_ms: Option<u32>,

    /// Available libraries/packages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<String>>,

    /// File IDs accessible to the interpreter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    /// Whether to persist container after execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist_container: Option<bool>,

    /// Whether to include file citations in responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_citations: Option<bool>,
}

/// Computer use configuration for agentic workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputerUseConfig {
    /// Screen resolution for the virtual display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    /// Operating system type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,

    /// Available applications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<String>>,

    /// Maximum action count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_actions: Option<u32>,
}

/// Tool choice configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecificToolChoice {
    /// Type of tool to use
    #[serde(rename = "type")]
    pub tool_type: String,

    /// Name of the specific tool (for functions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

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

/// Builder for web search configuration
pub struct WebSearchBuilder {
    config: WebSearchConfig,
}

impl Default for WebSearchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSearchBuilder {
    /// Create a new WebSearchBuilder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: WebSearchConfig {
                max_results: None,
                filters: None,
                time_range: None,
            },
        }
    }

    /// Set the maximum number of search results to return
    #[must_use]
    pub fn max_results(mut self, max: u32) -> Self {
        self.config.max_results = Some(max);
        self
    }

    /// Include only results from specified domains
    #[must_use]
    pub fn include_domains(mut self, domains: Vec<String>) -> Self {
        let filters = self.config.filters.get_or_insert(SearchFilters {
            include_domains: None,
            exclude_domains: None,
            language: None,
            region: None,
        });
        filters.include_domains = Some(domains);
        self
    }

    /// Exclude results from specified domains
    #[must_use]
    pub fn exclude_domains(mut self, domains: Vec<String>) -> Self {
        let filters = self.config.filters.get_or_insert(SearchFilters {
            include_domains: None,
            exclude_domains: None,
            language: None,
            region: None,
        });
        filters.exclude_domains = Some(domains);
        self
    }

    /// Set the time range for search results (e.g., "past_week", "past_month")
    pub fn time_range(mut self, range: impl Into<String>) -> Self {
        self.config.time_range = Some(range.into());
        self
    }

    /// Build the configured web search tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::WebSearch(self.config)
    }
}

/// Builder for file search configuration
pub struct FileSearchBuilder {
    config: FileSearchConfig,
}

impl FileSearchBuilder {
    /// Create a new FileSearchBuilder with the specified vector store IDs
    #[must_use]
    pub fn new(vector_store_ids: Vec<String>) -> Self {
        Self {
            config: FileSearchConfig {
                vector_store_ids,
                max_chunks: None,
                file_types: None,
            },
        }
    }

    /// Set the maximum number of chunks to search
    #[must_use]
    pub fn max_chunks(mut self, max: u32) -> Self {
        self.config.max_chunks = Some(max);
        self
    }

    /// Specify the file types to search
    #[must_use]
    pub fn file_types(mut self, types: Vec<String>) -> Self {
        self.config.file_types = Some(types);
        self
    }

    /// Build the configured file search tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::FileSearch(self.config)
    }
}

/// Builder for function tools
pub struct FunctionBuilder {
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

/// Builder for MCP server tools
pub struct McpBuilder {
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

/// Builder for image generation tools
pub struct ImageGenerationToolBuilder {
    config: ImageGenerationConfig,
}

impl Default for ImageGenerationToolBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageGenerationToolBuilder {
    /// Create a new ImageGenerationToolBuilder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ImageGenerationConfig {
                size: None,
                quality: None,
                style: None,
                n: None,
            },
        }
    }

    /// Set the image size (e.g., "1024x1024", "1792x1024")
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.config.size = Some(size.into());
        self
    }

    /// Set the image quality (e.g., "standard", "hd")
    pub fn quality(mut self, quality: impl Into<String>) -> Self {
        self.config.quality = Some(quality.into());
        self
    }

    /// Set the image style (e.g., "vivid", "natural")
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.config.style = Some(style.into());
        self
    }

    /// Set the number of images to generate
    #[must_use]
    pub fn count(mut self, n: u32) -> Self {
        self.config.n = Some(n);
        self
    }

    /// Build the configured image generation tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::ImageGeneration(self.config)
    }
}

/// Builder for code interpreter tools
pub struct CodeInterpreterBuilder {
    config: CodeInterpreterConfig,
}

impl Default for CodeInterpreterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeInterpreterBuilder {
    /// Create a new CodeInterpreterBuilder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: CodeInterpreterConfig {
                container_mode: None,
                container_id: None,
                container_config: None,
                language: None,
                max_execution_time_ms: None,
                libraries: None,
                file_ids: None,
                persist_container: None,
                include_citations: None,
            },
        }
    }

    /// Set the container mode for code execution
    #[must_use]
    pub fn container_mode(mut self, mode: crate::models::containers::ContainerMode) -> Self {
        self.config.container_mode = Some(mode);
        self
    }

    /// Set the container ID for explicit container mode
    pub fn container_id(mut self, id: impl Into<String>) -> Self {
        self.config.container_id = Some(id.into());
        self.config.container_mode = Some(crate::models::containers::ContainerMode::Explicit);
        self
    }

    /// Set the container configuration for auto mode
    #[must_use]
    pub fn container_config(mut self, config: crate::models::containers::ContainerConfig) -> Self {
        self.config.container_config = Some(config);
        self.config.container_mode = Some(crate::models::containers::ContainerMode::Auto);
        self
    }

    /// Set the programming language to use
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.config.language = Some(lang.into());
        self
    }

    /// Set the maximum execution time in milliseconds
    #[must_use]
    pub fn max_execution_time_ms(mut self, ms: u32) -> Self {
        self.config.max_execution_time_ms = Some(ms);
        self
    }

    /// Set the libraries to make available in the code interpreter
    #[must_use]
    pub fn libraries(mut self, libs: Vec<String>) -> Self {
        self.config.libraries = Some(libs);
        self
    }

    /// Set the file IDs to make available to the code interpreter
    #[must_use]
    pub fn file_ids(mut self, ids: Vec<String>) -> Self {
        self.config.file_ids = Some(ids);
        self
    }

    /// Enable or disable container persistence between executions
    #[must_use]
    pub fn persist_container(mut self, persist: bool) -> Self {
        self.config.persist_container = Some(persist);
        self
    }

    /// Enable or disable citation inclusion in outputs
    #[must_use]
    pub fn include_citations(mut self, include: bool) -> Self {
        self.config.include_citations = Some(include);
        self
    }

    /// Build the configured code interpreter tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::CodeInterpreter(self.config)
    }
}

/// Builder for computer use tools
pub struct ComputerUseBuilder {
    config: ComputerUseConfig,
}

impl Default for ComputerUseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputerUseBuilder {
    /// Create a new ComputerUseBuilder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ComputerUseConfig {
                resolution: None,
                os_type: None,
                applications: None,
                max_actions: None,
            },
        }
    }

    /// Set the screen resolution (e.g., "1920x1080")
    pub fn resolution(mut self, res: impl Into<String>) -> Self {
        self.config.resolution = Some(res.into());
        self
    }

    /// Set the operating system type (e.g., "windows", "macos", "linux")
    pub fn os_type(mut self, os: impl Into<String>) -> Self {
        self.config.os_type = Some(os.into());
        self
    }

    /// Set the applications available for computer use
    #[must_use]
    pub fn applications(mut self, apps: Vec<String>) -> Self {
        self.config.applications = Some(apps);
        self
    }

    /// Set the maximum number of actions allowed
    #[must_use]
    pub fn max_actions(mut self, max: u32) -> Self {
        self.config.max_actions = Some(max);
        self
    }

    /// Build the configured computer use tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::ComputerUse(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_search_tool() {
        let tool = ToolBuilder::web_search();
        match tool {
            EnhancedTool::WebSearchPreview => assert!(true),
            _ => panic!("Expected WebSearchPreview"),
        }
    }

    #[test]
    fn test_file_search_builder() {
        let tool = ToolBuilder::file_search(vec!["store_123".to_string()])
            .max_chunks(10)
            .file_types(vec!["pdf".to_string(), "txt".to_string()])
            .build();

        match tool {
            EnhancedTool::FileSearch(config) => {
                assert_eq!(config.vector_store_ids, vec!["store_123"]);
                assert_eq!(config.max_chunks, Some(10));
                assert_eq!(
                    config.file_types,
                    Some(vec!["pdf".to_string(), "txt".to_string()])
                );
            }
            _ => panic!("Expected FileSearch"),
        }
    }

    #[test]
    fn test_mcp_builder() {
        let tool = ToolBuilder::mcp("deepwiki", "https://mcp.deepwiki.com/mcp")
            .require_approval(McpApproval::Never)
            .header("Authorization", "Bearer token")
            .timeout_ms(5000)
            .build();

        match tool {
            EnhancedTool::Mcp(config) => {
                assert_eq!(config.server_label, "deepwiki");
                assert_eq!(config.server_url, "https://mcp.deepwiki.com/mcp");
                assert!(matches!(config.require_approval, McpApproval::Never));
                assert!(config.headers.is_some());
                assert_eq!(config.timeout_ms, Some(5000));
            }
            _ => panic!("Expected Mcp"),
        }
    }
}
