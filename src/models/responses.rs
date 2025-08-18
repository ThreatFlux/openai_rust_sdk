use crate::models::functions::{FunctionCall, Tool, ToolChoice};
use crate::schema::JsonSchema;
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Default object type for responses
fn default_object_type() -> String {
    "chat.completion".to_string()
}

/// Default model name for responses
fn default_model() -> String {
    "unknown".to_string()
}

/// Response format enforcement options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[derive(Default)]
pub enum ResponseFormat {
    /// Standard text response
    #[serde(rename = "text")]
    #[default]
    Text,
    /// JSON object response (legacy JSON mode)
    #[serde(rename = "json_object")]
    JsonObject,
    /// JSON schema-enforced response
    #[serde(rename = "json_schema")]
    JsonSchema {
        /// The JSON schema definition
        json_schema: JsonSchemaSpec,
        /// Whether to enable strict mode
        #[serde(default)]
        strict: bool,
    },
}

/// JSON Schema specification for structured outputs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonSchemaSpec {
    /// Name of the schema
    pub name: String,
    /// Description of the schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The JSON schema definition
    pub schema: serde_json::Value,
    /// Whether the schema is strict (additional properties not allowed)
    #[serde(default)]
    pub strict: bool,
}

/// Schema validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaValidationResult {
    /// Whether the data is valid according to the schema
    pub is_valid: bool,
    /// Validation errors if any
    pub errors: Vec<String>,
    /// The validated data
    pub data: Option<serde_json::Value>,
}

/// Role for message in conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// Developer message providing high-priority instructions
    Developer,
    /// User input message
    User,
    /// AI assistant response message
    Assistant,
    /// System message (legacy, use Developer for new code)
    System,
}

/// Detail level for image processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ImageDetail {
    /// Auto-detect appropriate detail level
    #[default]
    Auto,
    /// Low detail for faster processing
    Low,
    /// High detail for better accuracy
    High,
}

/// Image content for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    /// Type identifier for image content
    #[serde(rename = "type")]
    pub content_type: String,
    /// Image URL or base64 data
    pub image_url: ImageUrl,
}

/// Image URL specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    /// URL or base64-encoded image data
    pub url: String,
    /// Detail level for image processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// Text content for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    /// Type identifier for text content
    #[serde(rename = "type")]
    pub content_type: String,
    /// Text content
    pub text: String,
}

/// Content types for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageContent {
    /// Text content
    #[serde(rename = "text")]
    Text { text: String },
    /// Image content
    #[serde(rename = "image_url")]
    Image { image_url: ImageUrl },
}

/// Message content input - can be simple text or array of content items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContentInput {
    /// Simple text content
    Text(String),
    /// Array of multimodal content items
    Array(Vec<MessageContent>),
}

/// Message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender
    pub role: MessageRole,
    /// The content of the message (text or multimodal)
    pub content: MessageContentInput,
}

/// Tool call information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique identifier for the tool call
    pub id: String,
    /// Name of the tool being called
    pub name: String,
    /// Arguments passed to the tool
    pub arguments: serde_json::Value,
}

/// Input for response request - can be string or messages array
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Prompt template with variables for reusable prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    /// Unique identifier for the template (e.g., "`pmpt_abc123`")
    pub id: String,
    /// Version of the template (defaults to "current" if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Variables to substitute in the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, PromptVariable>>,
}

/// Variable value for prompt templates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PromptVariable {
    /// String variable
    String(String),
    /// Image input variable
    Image(ImageInput),
    /// File input variable
    File(FileInput),
    /// JSON value for complex data
    Json(serde_json::Value),
}

/// Image input for prompt variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInput {
    /// Type indicator
    #[serde(rename = "type")]
    pub input_type: String,
    /// Image URL or base64 data
    pub url: String,
    /// Detail level for image processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// File input for prompt variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInput {
    /// Type indicator
    #[serde(rename = "type")]
    pub input_type: String,
    /// File ID
    pub file_id: String,
}

/// Request for creating a response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// Output content for a response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseOutput {
    /// The generated text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Tool calls made by the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Function calls made by the model (enhanced format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calls: Option<Vec<FunctionCall>>,
    /// Structured output data (when using response format enforcement)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_data: Option<serde_json::Value>,
    /// Schema validation result for structured outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_validation: Option<SchemaValidationResult>,
}

/// Usage statistics for the response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Number of tokens in the prompt
    #[serde(default)]
    pub prompt_tokens: u32,
    /// Number of tokens in the completion
    #[serde(default)]
    pub completion_tokens: u32,
    /// Total number of tokens used
    #[serde(default)]
    pub total_tokens: u32,
    /// Detailed prompt token information including caching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokenDetails>,
    /// Detailed completion token information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokenDetails>,
}

/// Detailed prompt token information including caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTokenDetails {
    /// Number of cached tokens used from prompt cache
    #[serde(default)]
    pub cached_tokens: u32,
    /// Audio tokens if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
}

/// Detailed completion token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionTokenDetails {
    /// Reasoning tokens used (for models with reasoning capabilities)
    #[serde(default)]
    pub reasoning_tokens: u32,
    /// Accepted prediction tokens
    #[serde(default)]
    pub accepted_prediction_tokens: u32,
    /// Rejected prediction tokens
    #[serde(default)]
    pub rejected_prediction_tokens: u32,
    /// Audio tokens if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
}

/// Full response from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseResult {
    /// Unique identifier for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Object type (usually "response")
    #[serde(default = "default_object_type")]
    pub object: String,
    /// Unix timestamp when the response was created
    #[serde(default)]
    pub created: u64,
    /// Model used to generate the response
    #[serde(default = "default_model")]
    pub model: String,
    /// List of response choices
    #[serde(default)]
    pub choices: Vec<ResponseChoice>,
    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

impl ResponseResult {
    /// Get the text output from the first choice
    #[must_use]
    pub fn output_text(&self) -> String {
        self.choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .unwrap_or_default()
    }

    /// Get structured data from the first choice
    #[must_use]
    pub fn structured_data(&self) -> Option<&serde_json::Value> {
        self.choices
            .first()
            .and_then(|choice| choice.message.structured_data.as_ref())
    }

    /// Get parsed structured data as a specific type
    pub fn parse_structured_data<T>(&self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        if let Some(data) = self.structured_data() {
            serde_json::from_value(data.clone())
        } else {
            // Try parsing from text content if no structured data
            let text = self.output_text();
            serde_json::from_str(&text)
        }
    }

    /// Check if the response has valid structured data
    #[must_use]
    pub fn has_valid_structured_data(&self) -> bool {
        self.choices
            .first()
            .and_then(|choice| choice.message.schema_validation.as_ref())
            .is_some_and(|validation| validation.is_valid)
    }

    /// Get schema validation errors
    #[must_use]
    pub fn schema_validation_errors(&self) -> Vec<String> {
        self.choices
            .first()
            .and_then(|choice| choice.message.schema_validation.as_ref())
            .map(|validation| validation.errors.clone())
            .unwrap_or_default()
    }

    /// Get all text outputs from all choices
    #[must_use]
    pub fn all_outputs(&self) -> Vec<String> {
        self.choices
            .iter()
            .filter_map(|choice| choice.message.content.clone())
            .collect()
    }

    /// Get the number of cached tokens used
    #[must_use]
    pub fn cached_tokens(&self) -> u32 {
        self.usage
            .as_ref()
            .and_then(|u| u.prompt_tokens_details.as_ref())
            .map_or(0, |d| d.cached_tokens)
    }

    /// Calculate the cache hit rate as a percentage
    #[must_use]
    pub fn cache_hit_rate(&self) -> f32 {
        if let Some(usage) = &self.usage {
            if usage.prompt_tokens > 0 {
                let cached = self.cached_tokens() as f32;
                let total = usage.prompt_tokens as f32;
                return (cached / total) * 100.0;
            }
        }
        0.0
    }

    /// Check if prompt caching was used
    #[must_use]
    pub fn used_cache(&self) -> bool {
        self.cached_tokens() > 0
    }
}

/// Individual choice in the response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseChoice {
    /// Index of this choice in the choices array
    pub index: u32,
    /// The generated message content
    pub message: ResponseOutput,
    /// Reason why the generation finished
    pub finish_reason: Option<String>,
}

/// Different types of streaming events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamEvent {
    /// Response generation has started
    #[serde(rename = "response.started")]
    ResponseStarted {
        /// Unique identifier for the response
        response_id: String,
    },
    /// Incremental content received
    #[serde(rename = "response.delta")]
    ResponseDelta {
        /// Unique identifier for the response
        response_id: String,
        /// The content delta/fragment
        delta: String,
    },
    /// Response generation completed successfully
    #[serde(rename = "response.completed")]
    ResponseCompleted {
        /// Unique identifier for the response
        response_id: String,
        /// The complete response
        response: ResponseResult,
    },
    /// Response generation failed
    #[serde(rename = "response.failed")]
    ResponseFailed {
        /// Unique identifier for the response
        response_id: String,
        /// Error message describing the failure
        error: String,
    },
    /// Function call started
    #[serde(rename = "response.function_call.started")]
    FunctionCallStarted {
        /// Unique identifier for the response
        response_id: String,
        /// The function call ID
        call_id: String,
        /// The function name
        function_name: String,
    },
    /// Function call arguments delta
    #[serde(rename = "response.function_call.arguments.delta")]
    FunctionCallArgumentsDelta {
        /// Unique identifier for the response
        response_id: String,
        /// The function call ID
        call_id: String,
        /// Arguments delta/fragment
        delta: String,
    },
    /// Function call completed
    #[serde(rename = "response.function_call.completed")]
    FunctionCallCompleted {
        /// Unique identifier for the response
        response_id: String,
        /// The completed function call
        function_call: FunctionCall,
    },
    /// Output item added
    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        /// Unique identifier for the response
        response_id: String,
        /// The output item
        item: ResponseOutput,
    },
}

/// Streaming response chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    /// Unique identifier for the response
    pub id: String,
    /// Object type (usually "stream.chunk")
    pub object: String,
    /// Unix timestamp when the chunk was created
    pub created: u64,
    /// Model used to generate the response
    pub model: String,
    /// List of streaming choices
    pub choices: Vec<StreamChoice>,
}

/// Choice in streaming chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChoice {
    /// Index of this choice in the choices array
    pub index: u32,
    /// The incremental content delta
    pub delta: StreamDelta,
    /// Reason why the generation finished (if complete)
    pub finish_reason: Option<String>,
}

/// Delta content in streaming chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDelta {
    /// Incremental text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Role of the message (if starting a new message)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<MessageRole>,
    /// Tool calls being streamed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallDelta>>,
}

/// Delta for tool calls in streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Index of the tool call
    pub index: u32,
    /// Tool call ID (if starting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Tool call type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Function call delta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionCallDelta>,
}

/// Delta for function calls in streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallDelta {
    /// Function name (if starting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Arguments delta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
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

/// Supported image formats for validation
#[derive(Debug, Clone, PartialEq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Gif,
    Webp,
}

impl ImageFormat {
    /// Detect image format from data URL or file extension
    #[must_use]
    pub fn from_data_url(data_url: &str) -> Option<Self> {
        if data_url.starts_with("data:image/jpeg") || data_url.starts_with("data:image/jpg") {
            Some(ImageFormat::Jpeg)
        } else if data_url.starts_with("data:image/png") {
            Some(ImageFormat::Png)
        } else if data_url.starts_with("data:image/gif") {
            Some(ImageFormat::Gif)
        } else if data_url.starts_with("data:image/webp") {
            Some(ImageFormat::Webp)
        } else {
            None
        }
    }

    /// Detect image format from URL extension
    #[must_use]
    pub fn from_url(url: &str) -> Option<Self> {
        let url_lower = url.to_lowercase();
        if url_lower.ends_with(".jpg") || url_lower.ends_with(".jpeg") {
            Some(ImageFormat::Jpeg)
        } else if url_lower.ends_with(".png") {
            Some(ImageFormat::Png)
        } else if url_lower.ends_with(".gif") {
            Some(ImageFormat::Gif)
        } else if url_lower.ends_with(".webp") {
            Some(ImageFormat::Webp)
        } else {
            None
        }
    }

    /// Get MIME type for the format
    #[must_use]
    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Png => "image/png",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Webp => "image/webp",
        }
    }
}

/// Image utilities for encoding and validation
pub struct ImageUtils;

impl ImageUtils {
    /// Encode image bytes to base64 data URL
    #[must_use]
    pub fn encode_to_data_url(image_data: &[u8], format: ImageFormat) -> String {
        let base64_data = general_purpose::STANDARD.encode(image_data);
        format!("data:{};base64,{}", format.mime_type(), base64_data)
    }

    /// Decode base64 data URL to image bytes
    pub fn decode_from_data_url(data_url: &str) -> Result<Vec<u8>, String> {
        if !data_url.starts_with("data:image/") {
            return Err("Invalid data URL format".to_string());
        }

        let parts: Vec<&str> = data_url.split(',').collect();
        if parts.len() != 2 {
            return Err("Invalid data URL structure".to_string());
        }

        general_purpose::STANDARD
            .decode(parts[1])
            .map_err(|e| format!("Base64 decode error: {e}"))
    }

    /// Validate image format from URL or data URL
    pub fn validate_format(url: &str) -> Result<ImageFormat, String> {
        if url.starts_with("data:image/") {
            ImageFormat::from_data_url(url)
                .ok_or_else(|| "Unsupported image format in data URL".to_string())
        } else {
            ImageFormat::from_url(url).ok_or_else(|| "Unsupported image format in URL".to_string())
        }
    }

    /// Estimate token usage for image based on detail level
    #[must_use]
    pub fn estimate_tokens(detail: &ImageDetail) -> u32 {
        match detail {
            ImageDetail::Low => 85,   // Low detail uses 85 tokens
            ImageDetail::High => 170, // High detail can use up to 170 tokens per 512x512 tile
            ImageDetail::Auto => 85,  // Default to low estimate
        }
    }
}

impl MessageContent {
    /// Create text content
    pub fn text(text: impl Into<String>) -> Self {
        MessageContent::Text { text: text.into() }
    }

    /// Create image content from URL
    pub fn image_url(url: impl Into<String>) -> Self {
        MessageContent::Image {
            image_url: ImageUrl {
                url: url.into(),
                detail: None,
            },
        }
    }

    /// Create image content from URL with detail level
    pub fn image_url_with_detail(url: impl Into<String>, detail: ImageDetail) -> Self {
        MessageContent::Image {
            image_url: ImageUrl {
                url: url.into(),
                detail: Some(detail),
            },
        }
    }

    /// Create image content from bytes and format
    #[must_use]
    pub fn image_from_bytes(image_data: &[u8], format: ImageFormat) -> Self {
        let data_url = ImageUtils::encode_to_data_url(image_data, format);
        MessageContent::Image {
            image_url: ImageUrl {
                url: data_url,
                detail: None,
            },
        }
    }

    /// Create image content from bytes, format, and detail level
    #[must_use]
    pub fn image_from_bytes_with_detail(
        image_data: &[u8],
        format: ImageFormat,
        detail: ImageDetail,
    ) -> Self {
        let data_url = ImageUtils::encode_to_data_url(image_data, format);
        MessageContent::Image {
            image_url: ImageUrl {
                url: data_url,
                detail: Some(detail),
            },
        }
    }
}

impl Message {
    /// Create a new user message with text content
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Text(content.into()),
        }
    }

    /// Create a new assistant message with text content
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: MessageContentInput::Text(content.into()),
        }
    }

    /// Create a new developer message (high-priority instructions)
    pub fn developer(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Developer,
            content: MessageContentInput::Text(content.into()),
        }
    }

    /// Create a new system message (legacy, use developer for new code)
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: MessageContentInput::Text(content.into()),
        }
    }

    /// Create a user message with multimodal content
    #[must_use]
    pub fn user_with_content(content: Vec<MessageContent>) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(content),
        }
    }

    /// Create an assistant message with multimodal content
    #[must_use]
    pub fn assistant_with_content(content: Vec<MessageContent>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: MessageContentInput::Array(content),
        }
    }

    /// Create a user message with text and image
    pub fn user_with_image(text: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(vec![
                MessageContent::text(text),
                MessageContent::image_url(image_url),
            ]),
        }
    }

    /// Create a user message with text and image with detail level
    pub fn user_with_image_detail(
        text: impl Into<String>,
        image_url: impl Into<String>,
        detail: ImageDetail,
    ) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(vec![
                MessageContent::text(text),
                MessageContent::image_url_with_detail(image_url, detail),
            ]),
        }
    }

    /// Create a user message with text and multiple images
    pub fn user_with_images(text: impl Into<String>, image_urls: Vec<String>) -> Self {
        let mut content = vec![MessageContent::text(text)];
        for url in image_urls {
            content.push(MessageContent::image_url(url));
        }
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(content),
        }
    }

    /// Create a user message from image bytes
    pub fn user_with_image_bytes(
        text: impl Into<String>,
        image_data: &[u8],
        format: ImageFormat,
    ) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageContentInput::Array(vec![
                MessageContent::text(text),
                MessageContent::image_from_bytes(image_data, format),
            ]),
        }
    }

    /// Get estimated token count including images
    #[must_use]
    pub fn estimate_tokens(&self) -> u32 {
        match &self.content {
            MessageContentInput::Text(text) => {
                // Rough estimate: 1 token per 4 characters
                (text.len() as f32 / 4.0).ceil() as u32
            }
            MessageContentInput::Array(contents) => {
                let mut total = 0;
                for content in contents {
                    match content {
                        MessageContent::Text { text } => {
                            total += (text.len() as f32 / 4.0).ceil() as u32;
                        }
                        MessageContent::Image { image_url } => {
                            let detail = image_url.detail.as_ref().unwrap_or(&ImageDetail::Auto);
                            total += ImageUtils::estimate_tokens(detail);
                        }
                    }
                }
                total
            }
        }
    }

    /// Check if message contains images
    #[must_use]
    pub fn has_images(&self) -> bool {
        match &self.content {
            MessageContentInput::Text(_) => false,
            MessageContentInput::Array(contents) => contents
                .iter()
                .any(|c| matches!(c, MessageContent::Image { .. })),
        }
    }

    /// Get text content only (concatenated if multimodal)
    #[must_use]
    pub fn text_content(&self) -> String {
        match &self.content {
            MessageContentInput::Text(text) => text.clone(),
            MessageContentInput::Array(contents) => contents
                .iter()
                .filter_map(|c| match c {
                    MessageContent::Text { text } => Some(text.as_str()),
                    MessageContent::Image { .. } => None,
                })
                .collect::<Vec<_>>()
                .join(" "),
        }
    }

    /// Get image URLs from the message
    #[must_use]
    pub fn image_urls(&self) -> Vec<&str> {
        match &self.content {
            MessageContentInput::Text(_) => vec![],
            MessageContentInput::Array(contents) => contents
                .iter()
                .filter_map(|c| match c {
                    MessageContent::Text { .. } => None,
                    MessageContent::Image { image_url } => Some(image_url.url.as_str()),
                })
                .collect(),
        }
    }
}

impl ResponseFormat {
    /// Create a JSON object response format
    #[must_use]
    pub fn json_object() -> Self {
        ResponseFormat::JsonObject
    }

    /// Create a JSON schema response format
    pub fn json_schema(name: impl Into<String>, schema: serde_json::Value) -> Self {
        ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema,
                strict: false,
            },
            strict: false,
        }
    }

    /// Create a strict JSON schema response format
    pub fn strict_json_schema(name: impl Into<String>, schema: serde_json::Value) -> Self {
        ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema,
                strict: true,
            },
            strict: true,
        }
    }

    /// Create from schema builder
    pub fn from_schema_builder(
        name: impl Into<String>,
        builder: crate::schema::SchemaBuilder,
    ) -> Self {
        let schema = builder.build();
        ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema: schema.to_value(),
                strict: false,
            },
            strict: false,
        }
    }

    /// Create strict format from schema builder
    pub fn strict_from_schema_builder(
        name: impl Into<String>,
        builder: crate::schema::SchemaBuilder,
    ) -> Self {
        let schema = builder.build();
        ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema: schema.to_value(),
                strict: true,
            },
            strict: true,
        }
    }

    /// Check if this is a structured output format
    #[must_use]
    pub fn is_structured(&self) -> bool {
        matches!(
            self,
            ResponseFormat::JsonObject | ResponseFormat::JsonSchema { .. }
        )
    }

    /// Check if this format requires schema validation
    #[must_use]
    pub fn requires_schema_validation(&self) -> bool {
        matches!(self, ResponseFormat::JsonSchema { .. })
    }

    /// Get the schema if available
    #[must_use]
    pub fn schema(&self) -> Option<&JsonSchemaSpec> {
        match self {
            ResponseFormat::JsonSchema { json_schema, .. } => Some(json_schema),
            _ => None,
        }
    }
}

impl JsonSchemaSpec {
    /// Create a new JSON schema specification
    pub fn new(name: impl Into<String>, schema: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            description: None,
            schema,
            strict: false,
        }
    }

    /// Create a strict JSON schema specification
    pub fn strict(name: impl Into<String>, schema: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            description: None,
            schema,
            strict: true,
        }
    }

    /// Create with description
    pub fn with_description(
        name: impl Into<String>,
        description: impl Into<String>,
        schema: serde_json::Value,
    ) -> Self {
        Self {
            name: name.into(),
            description: Some(description.into()),
            schema,
            strict: false,
        }
    }

    /// Set description
    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set strict mode
    #[must_use]
    pub fn set_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Validate data against this schema
    #[must_use]
    pub fn validate(&self, data: &serde_json::Value) -> SchemaValidationResult {
        let json_schema = JsonSchema::new(self.schema.clone());
        match json_schema.validate(data) {
            Ok(()) => SchemaValidationResult {
                is_valid: true,
                errors: vec![],
                data: Some(data.clone()),
            },
            Err(e) => SchemaValidationResult {
                is_valid: false,
                errors: vec![e.to_string()],
                data: Some(data.clone()),
            },
        }
    }

    /// Convert to `JsonSchema` for validation
    #[must_use]
    pub fn to_json_schema(&self) -> JsonSchema {
        JsonSchema::new(self.schema.clone())
    }
}

/// Schema generation utilities
pub struct SchemaUtils;

impl SchemaUtils {
    /// Generate a basic object schema with properties
    #[must_use]
    pub fn object_schema(properties: &[(&str, &str)]) -> serde_json::Value {
        let mut props = serde_json::Map::new();
        let mut required = Vec::new();

        for (name, type_name) in properties {
            props.insert(
                (*name).to_string(),
                serde_json::json!({ "type": type_name }),
            );
            required.push((*name).to_string());
        }

        serde_json::json!({
            "type": "object",
            "properties": props,
            "required": required,
            "additionalProperties": false
        })
    }

    /// Generate an array schema
    #[must_use]
    pub fn array_schema(item_type: &str) -> serde_json::Value {
        serde_json::json!({
            "type": "array",
            "items": { "type": item_type }
        })
    }

    /// Generate an enum schema
    #[must_use]
    pub fn enum_schema(values: &[&str]) -> serde_json::Value {
        serde_json::json!({
            "type": "string",
            "enum": values
        })
    }

    /// Generate a union schema (anyOf)
    #[must_use]
    pub fn union_schema(schemas: &[serde_json::Value]) -> serde_json::Value {
        serde_json::json!({
            "anyOf": schemas
        })
    }
}

/// Helper trait for creating schema-validated structured outputs
pub trait StructuredOutput: serde::Serialize + serde::de::DeserializeOwned {
    /// Get the JSON schema for this type
    fn json_schema() -> serde_json::Value;

    /// Get the schema name
    fn schema_name() -> &'static str;

    /// Create a response format for this type
    #[must_use]
    fn response_format() -> ResponseFormat {
        ResponseFormat::json_schema(Self::schema_name(), Self::json_schema())
    }

    /// Create a strict response format for this type
    #[must_use]
    fn strict_response_format() -> ResponseFormat {
        ResponseFormat::strict_json_schema(Self::schema_name(), Self::json_schema())
    }

    /// Validate and parse from JSON value
    fn from_json_value(value: &serde_json::Value) -> Result<Self, String> {
        let schema = JsonSchema::new(Self::json_schema());
        schema.validate(value).map_err(|e| e.to_string())?;
        serde_json::from_value(value.clone()).map_err(|e| e.to_string())
    }
}
