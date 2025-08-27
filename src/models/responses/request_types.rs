use crate::models::functions::{Tool, ToolChoice};
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    message_types::Message,
    schema_types::{JsonSchemaSpec, ResponseFormat},
    usage_types::{PromptTemplate, PromptVariable},
};

/// Input for response request - can be string or messages array
#[derive(Debug, Clone, Ser, De)]
#[serde(untagged)]
pub enum ResponseInput {
    /// Simple text input
    Text(String),
    /// Conversation messages
    Messages(Vec<Message>),
}

impl Default for ResponseInput {
    fn default() -> Self {
        ResponseInput::Text(String::new())
    }
}

impl From<String> for ResponseInput {
    fn from(s: String) -> Self {
        ResponseInput::Text(s)
    }
}

impl From<&str> for ResponseInput {
    fn from(s: &str) -> Self {
        ResponseInput::Text(s.to_string())
    }
}

impl From<Vec<Message>> for ResponseInput {
    fn from(messages: Vec<Message>) -> Self {
        ResponseInput::Messages(messages)
    }
}

/// Request for creating a response
#[derive(Debug, Clone, Ser, De, Default)]
pub struct ResponseRequest {
    /// Model to use for generating the response
    pub model: String,
    /// Input text or messages for the request
    pub input: ResponseInput,
    /// Optional instructions for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Previous response ID for multi-turn conversations (GPT-5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// Reasoning configuration (GPT-5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<crate::models::gpt5::ReasoningConfig>,
    /// Text output configuration (GPT-5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<crate::models::gpt5::TextConfig>,
    /// Temperature for randomness (0.0-2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Prompt template for reusable prompts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptTemplate>,
    /// Top-p sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Frequency penalty parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Presence penalty parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Tools available for function calling (legacy format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Tool choice configuration (legacy format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Enhanced tools support (web search, file search, MCP, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_tools: Option<Vec<crate::models::tools::EnhancedTool>>,
    /// Enhanced tool choice configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_tool_choice: Option<crate::models::tools::EnhancedToolChoice>,
    /// Whether to allow parallel function calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Prompt cache key for optimizing cache routing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Response format specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

impl ResponseRequest {
    /// Create a new response request with text input
    pub fn new_text(model: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            input: ResponseInput::Text(input.into()),
            instructions: None,
            previous_response_id: None,
            reasoning: None,
            text: None,
            temperature: None,
            max_tokens: None,
            stream: None,
            prompt: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            tools: None,
            tool_choice: None,
            enhanced_tools: None,
            enhanced_tool_choice: None,
            parallel_tool_calls: None,
            prompt_cache_key: None,
            response_format: None,
        }
    }

    /// Create a new response request with messages
    pub fn new_messages(model: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            model: model.into(),
            input: ResponseInput::Messages(messages),
            instructions: None,
            previous_response_id: None,
            reasoning: None,
            text: None,
            temperature: None,
            max_tokens: None,
            stream: None,
            prompt: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            tools: None,
            tool_choice: None,
            enhanced_tools: None,
            enhanced_tool_choice: None,
            parallel_tool_calls: None,
            prompt_cache_key: None,
            response_format: None,
        }
    }

    /// Set instructions for the request
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Set temperature for the request
    #[must_use]
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set max tokens for the request
    #[must_use]
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Enable streaming for the request
    #[must_use]
    pub fn with_streaming(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Set prompt template for reusable prompts
    #[must_use]
    pub fn with_prompt(mut self, template: PromptTemplate) -> Self {
        self.prompt = Some(template);
        self
    }

    /// Set prompt template with just ID
    pub fn with_prompt_id(mut self, id: impl Into<String>) -> Self {
        self.prompt = Some(PromptTemplate {
            id: id.into(),
            version: None,
            variables: None,
        });
        self
    }

    /// Set prompt template with ID and version
    pub fn with_prompt_template(
        mut self,
        id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        self.prompt = Some(PromptTemplate {
            id: id.into(),
            version: Some(version.into()),
            variables: None,
        });
        self
    }

    /// Set prompt template with ID and variables
    pub fn with_prompt_variables(
        mut self,
        id: impl Into<String>,
        variables: HashMap<String, PromptVariable>,
    ) -> Self {
        self.prompt = Some(PromptTemplate {
            id: id.into(),
            version: None,
            variables: Some(variables),
        });
        self
    }

    /// Set tools for function calling
    #[must_use]
    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set tool choice strategy
    #[must_use]
    pub fn with_tool_choice(mut self, choice: ToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    /// Enable/disable parallel tool calls
    #[must_use]
    pub fn with_parallel_tool_calls(mut self, enabled: bool) -> Self {
        self.parallel_tool_calls = Some(enabled);
        self
    }

    /// Add enhanced tools (web search, file search, MCP, etc.)
    #[must_use]
    pub fn with_enhanced_tools(mut self, tools: Vec<crate::models::tools::EnhancedTool>) -> Self {
        self.enhanced_tools = Some(tools);
        self
    }

    /// Add a single enhanced tool
    pub fn with_enhanced_tool(mut self, tool: crate::models::tools::EnhancedTool) -> Self {
        let tools = self.enhanced_tools.get_or_insert_with(Vec::new);
        tools.push(tool);
        self
    }

    /// Set enhanced tool choice
    #[must_use]
    pub fn with_enhanced_tool_choice(
        mut self,
        choice: crate::models::tools::EnhancedToolChoice,
    ) -> Self {
        self.enhanced_tool_choice = Some(choice);
        self
    }

    /// Enable web search for this request
    pub fn with_web_search(mut self) -> Self {
        let tools = self.enhanced_tools.get_or_insert_with(Vec::new);
        tools.push(crate::models::tools::EnhancedTool::WebSearchPreview);
        self
    }

    /// Enable file search with vector stores
    pub fn with_file_search(mut self, vector_store_ids: Vec<String>) -> Self {
        let tools = self.enhanced_tools.get_or_insert_with(Vec::new);
        tools.push(crate::models::tools::EnhancedTool::FileSearch(
            crate::models::tools::FileSearchConfig {
                vector_store_ids,
                max_chunks: None,
                file_types: None,
            },
        ));
        self
    }

    /// Add an MCP server
    pub fn with_mcp_server(
        mut self,
        server_label: impl Into<String>,
        server_url: impl Into<String>,
    ) -> Self {
        let tools = self.enhanced_tools.get_or_insert_with(Vec::new);
        tools.push(crate::models::tools::EnhancedTool::Mcp(
            crate::models::tools::McpTool {
                server_label: server_label.into(),
                server_url: server_url.into(),
                require_approval: crate::models::tools::McpApproval::Sensitive,
                headers: None,
                timeout_ms: None,
            },
        ));
        self
    }

    /// Set prompt cache key for optimizing cache routing
    pub fn with_prompt_cache_key(mut self, key: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(key.into());
        self
    }

    /// Set response format to JSON object mode
    #[must_use]
    pub fn with_json_mode(mut self) -> Self {
        self.response_format = Some(ResponseFormat::JsonObject);
        self
    }

    /// Set response format with JSON schema enforcement
    pub fn with_json_schema(mut self, name: impl Into<String>, schema: serde_json::Value) -> Self {
        self.response_format = Some(ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema,
                strict: false,
            },
            strict: false,
        });
        self
    }

    /// Set response format with strict JSON schema enforcement
    pub fn with_strict_json_schema(
        mut self,
        name: impl Into<String>,
        schema: serde_json::Value,
    ) -> Self {
        self.response_format = Some(ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema,
                strict: true,
            },
            strict: true,
        });
        self
    }

    /// Set response format with detailed JSON schema specification
    pub fn with_detailed_json_schema(
        mut self,
        name: impl Into<String>,
        description: Option<String>,
        schema: serde_json::Value,
        strict: bool,
    ) -> Self {
        self.response_format = Some(ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description,
                schema,
                strict,
            },
            strict,
        });
        self
    }

    /// Set response format with a schema builder
    pub fn with_schema_builder(
        mut self,
        name: impl Into<String>,
        builder: crate::schema::SchemaBuilder,
    ) -> Self {
        let schema = builder.build();
        self.response_format = Some(ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema: schema.to_value(),
                strict: false,
            },
            strict: false,
        });
        self
    }

    /// Set response format with a strict schema builder
    pub fn with_strict_schema_builder(
        mut self,
        name: impl Into<String>,
        builder: crate::schema::SchemaBuilder,
    ) -> Self {
        let schema = builder.build();
        self.response_format = Some(ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema: schema.to_value(),
                strict: true,
            },
            strict: true,
        });
        self
    }
}
