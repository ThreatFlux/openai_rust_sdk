use crate::api::custom_tools::CustomToolsApi;
use crate::constants::endpoints;
use crate::error::{OpenAIError, Result};
use crate::models::functions::{FunctionCall, FunctionCallOutput, Tool, ToolChoice};
use crate::models::responses::ResponseRequest;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Core function calling API for `OpenAI`
#[derive(Debug, Clone)]
pub struct FunctionsApi {
    /// HTTP client for API requests
    client: Client,
    /// API key for authentication
    api_key: String,
    /// Base URL for API requests
    base_url: String,
    /// Custom tools registry
    custom_tools: CustomToolsApi,
    /// Active conversation state
    conversation_state: ConversationState,
}

/// State management for function calling conversations
#[derive(Debug, Clone, Default)]
pub struct ConversationState {
    /// Pending function calls awaiting results
    pending_calls: HashMap<String, FunctionCall>,
    /// Completed function calls with results
    completed_calls: HashMap<String, FunctionCallOutput>,
    /// Function call history for context
    call_history: Vec<FunctionCallEvent>,
}

/// Events in the function calling lifecycle
#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum FunctionCallEvent {
    /// A function call was initiated
    CallInitiated {
        /// When the call was initiated
        timestamp: u64,
        /// The function call details
        call: FunctionCall,
    },
    /// A function call was completed
    CallCompleted {
        /// When the call was completed
        timestamp: u64,
        /// The call ID
        call_id: String,
        /// The function result
        output: FunctionCallOutput,
    },
    /// A function call failed
    CallFailed {
        /// When the call failed
        timestamp: u64,
        /// The call ID
        call_id: String,
        /// Error message
        error: String,
    },
}

/// Helper struct for extracting tool call data with reduced complexity
#[derive(Debug)]
struct ToolCallExtractor<'a> {
    call: &'a Value,
}

impl<'a> ToolCallExtractor<'a> {
    /// Create a new extractor for a tool call
    fn new(call: &'a Value) -> Self {
        Self { call }
    }

    /// Extract all required fields in a single operation
    fn extract_all(&self) -> Result<(&'a str, &'a str, &'a str)> {
        let id = self.extract_id()?;
        let function = self.extract_function()?;
        let name = self.extract_name(function)?;
        let arguments = self.extract_arguments(function)?;
        Ok((id, name, arguments))
    }

    /// Extract tool call ID
    fn extract_id(&self) -> Result<&'a str> {
        self.call
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenAIError::parsing("Tool call missing id"))
    }

    /// Extract function object
    fn extract_function(&self) -> Result<&'a Value> {
        self.call
            .get("function")
            .ok_or_else(|| OpenAIError::parsing("Tool call missing function"))
    }

    /// Extract function name
    fn extract_name(&self, function: &'a Value) -> Result<&'a str> {
        function
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenAIError::parsing("Function missing name"))
    }

    /// Extract function arguments
    fn extract_arguments(&self, function: &'a Value) -> Result<&'a str> {
        function
            .get("arguments")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenAIError::parsing("Function missing arguments"))
    }
}

/// Configuration for function calling
#[derive(Debug, Clone, Default)]
pub struct FunctionConfig {
    /// Tools available for calling
    pub tools: Vec<Tool>,
    /// Tool choice strategy
    pub tool_choice: Option<ToolChoice>,
    /// Whether to allow parallel function calls
    pub parallel_function_calls: Option<bool>,
    /// Whether to use strict mode for schema adherence
    pub strict_mode: Option<bool>,
    /// Maximum number of function calls per response
    pub max_function_calls: Option<u32>,
}

impl FunctionsApi {
    /// Create a new functions API
    pub fn new(api_key: &str) -> Result<Self> {
        Self::with_base_url(api_key, "https://api.openai.com")
    }

    /// Create a new functions API with custom base URL
    pub fn with_base_url(api_key: &str, base_url: &str) -> Result<Self> {
        if api_key.trim().is_empty() {
            return Err(OpenAIError::authentication("API key cannot be empty"));
        }

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(crate::network_err!("Failed to create HTTP client: {}"))?;

        Ok(Self {
            client,
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            custom_tools: CustomToolsApi::new(),
            conversation_state: ConversationState::default(),
        })
    }

    /// Create a chat completion with function calling support
    pub async fn create_function_response(
        &mut self,
        request: &ResponseRequest,
        config: &FunctionConfig,
    ) -> Result<FunctionResponseResult> {
        let mut payload = self.build_payload(request)?;

        // Add function calling configuration
        self.add_function_config(&mut payload, config)?;

        let response = self.send_request(&payload).await?;
        let result = self.parse_function_response(response).await?;

        // Update conversation state with any function calls
        self.update_conversation_state(&result);

        Ok(result)
    }

    /// Submit function call results and continue the conversation
    pub async fn submit_function_results(
        &mut self,
        results: Vec<FunctionCallOutput>,
        original_request: &ResponseRequest,
        config: &FunctionConfig,
    ) -> Result<FunctionResponseResult> {
        // Mark function calls as completed
        for result in &results {
            self.conversation_state
                .completed_calls
                .insert(result.call_id.clone(), result.clone());

            self.conversation_state
                .call_history
                .push(FunctionCallEvent::CallCompleted {
                    timestamp: self.current_timestamp(),
                    call_id: result.call_id.clone(),
                    output: result.clone(),
                });
        }

        // Create a new request with function results
        let mut request_with_results = original_request.clone();
        self.add_function_results_to_request(&mut request_with_results, &results)?;

        // Make another API call with the results
        self.create_function_response(&request_with_results, config)
            .await
    }

    /// Get pending function calls that need to be executed
    #[must_use]
    pub fn get_pending_calls(&self) -> Vec<&FunctionCall> {
        self.conversation_state.pending_calls.values().collect()
    }

    /// Check if there are any pending function calls
    #[must_use]
    pub fn has_pending_calls(&self) -> bool {
        !self.conversation_state.pending_calls.is_empty()
    }

    /// Get the conversation history
    #[must_use]
    pub fn get_call_history(&self) -> &[FunctionCallEvent] {
        &self.conversation_state.call_history
    }

    /// Clear the conversation state
    pub fn clear_state(&mut self) {
        self.conversation_state = ConversationState::default();
    }

    /// Register a custom tool
    pub fn register_custom_tool(
        &mut self,
        tool: crate::models::functions::CustomTool,
    ) -> Result<()> {
        self.custom_tools.register_tool(tool)
    }

    /// Validate function call arguments against the schema
    pub fn validate_function_call(&self, call: &FunctionCall, tools: &[Tool]) -> Result<()> {
        // Find the tool definition
        let tool = tools
            .iter()
            .find(|t| t.name() == call.name)
            .ok_or_else(|| OpenAIError::validation(format!("Unknown function: {}", call.name)))?;

        match tool {
            Tool::Function { function: _ } => {
                // Parse arguments as JSON
                let args: Value = serde_json::from_str(&call.arguments)
                    .map_err(crate::validation_err!("Invalid JSON arguments: {}"))?;

                // TODO: Add proper JSON schema validation here
                // For now, just check that it's a valid JSON object
                if !args.is_object() {
                    return Err(OpenAIError::validation(
                        "Function arguments must be a JSON object",
                    ));
                }

                Ok(())
            }
            Tool::Custom { custom_tool } => {
                // Validate with custom tool grammar if available
                self.custom_tools
                    .validate_input(&custom_tool.name, &call.arguments)?;
                Ok(())
            }
        }
    }

    /// Execute a function call (placeholder - actual execution is application-specific)
    pub async fn execute_function_call(&self, call: &FunctionCall) -> Result<FunctionCallOutput> {
        // This is a placeholder implementation
        // In practice, applications would provide their own function executors

        match call.name.as_str() {
            "get_weather" => {
                let args: Value = call.arguments_json()?;
                let location = args
                    .get("location")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");

                let result = json!({
                    "location": location,
                    "temperature": 22,
                    "unit": "celsius",
                    "description": "Sunny"
                });

                Ok(FunctionCallOutput::from_json(&call.call_id, &result)?)
            }
            "get_time" => {
                let result = json!({
                    "current_time": chrono::Utc::now().to_rfc3339(),
                    "timezone": "UTC"
                });

                Ok(FunctionCallOutput::from_json(&call.call_id, &result)?)
            }
            _ => Err(OpenAIError::invalid_request(format!(
                "Unknown function: {}",
                call.name
            ))),
        }
    }

    /// Build the API request payload
    fn build_payload(&self, request: &ResponseRequest) -> Result<Value> {
        let mut payload = json!({
            "model": request.model,
            "messages": self.convert_input_to_messages(&request.input)?,
        });

        if let Some(temp) = request.temperature {
            payload["temperature"] = json!(temp);
        }
        if let Some(max_tokens) = request.max_tokens {
            payload["max_tokens"] = json!(max_tokens);
        }
        if let Some(stream) = request.stream {
            payload["stream"] = json!(stream);
        }

        Ok(payload)
    }

    /// Add function calling configuration to the payload
    fn add_function_config(&self, payload: &mut Value, config: &FunctionConfig) -> Result<()> {
        if !config.tools.is_empty() {
            payload["tools"] = json!(self.serialize_tools(&config.tools)?);
        }

        if let Some(ref tool_choice) = config.tool_choice {
            payload["tool_choice"] = json!(tool_choice);
        }

        if let Some(parallel) = config.parallel_function_calls {
            payload["parallel_tool_calls"] = json!(parallel);
        }

        Ok(())
    }

    /// Convert tools to API format
    #[allow(clippy::unused_self)]
    #[allow(clippy::unnecessary_wraps)]
    fn serialize_tools(&self, tools: &[Tool]) -> Result<Vec<Value>> {
        let mut serialized = Vec::new();

        for tool in tools {
            match tool {
                Tool::Function { function } => {
                    serialized.push(json!({
                        "type": "function",
                        "function": {
                            "name": function.name,
                            "description": function.description,
                            "parameters": function.parameters,
                            "strict": function.strict
                        }
                    }));
                }
                Tool::Custom { custom_tool } => {
                    // Convert custom tool to function format
                    serialized.push(json!({
                        "type": "function",
                        "function": {
                            "name": custom_tool.name,
                            "description": custom_tool.description,
                            "parameters": {
                                "type": "object",
                                "properties": {},
                                "required": []
                            }
                        }
                    }));
                }
            }
        }

        Ok(serialized)
    }

    /// Convert input to messages format
    #[allow(clippy::unused_self)]
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::match_same_arms)]
    fn convert_input_to_messages(
        &self,
        input: &crate::models::responses::ResponseInput,
    ) -> Result<Vec<Value>> {
        match input {
            crate::models::responses::ResponseInput::Text(text) => Ok(vec![json!({
                "role": "user",
                "content": text
            })]),
            crate::models::responses::ResponseInput::Messages(messages) => Ok(messages
                .iter()
                .map(|msg| {
                    json!({
                        "role": match msg.role {
                            crate::models::responses::MessageRole::User => "user",
                            crate::models::responses::MessageRole::Assistant => "assistant",
                            crate::models::responses::MessageRole::Developer => "system",
                            crate::models::responses::MessageRole::System => "system",
                        },
                        "content": msg.content
                    })
                })
                .collect()),
        }
    }

    /// Send the API request
    async fn send_request(&self, payload: &Value) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoints::CHAT_COMPLETIONS);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(payload)
            .send()
            .await
            .map_err(crate::network_err!("Request failed: {}"))?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::api_error(status_code, &error_text));
        }

        let json_response: Value = response
            .json()
            .await
            .map_err(crate::parsing_err!("Failed to parse response: {}"))?;

        Ok(json_response)
    }

    /// Parse the function response from the API
    async fn parse_function_response(&self, response: Value) -> Result<FunctionResponseResult> {
        let choices = response
            .get("choices")
            .and_then(|v| v.as_array())
            .ok_or_else(|| OpenAIError::parsing("No choices in response"))?;

        let first_choice = choices
            .first()
            .ok_or_else(|| OpenAIError::parsing("No choices in response"))?;

        let message = first_choice
            .get("message")
            .ok_or_else(|| OpenAIError::parsing("No message in choice"))?;

        let content = message
            .get("content")
            .and_then(|v| v.as_str())
            .map(std::string::ToString::to_string);

        let tool_calls = message
            .get("tool_calls")
            .and_then(|v| v.as_array())
            .map(|calls| self.parse_tool_calls(calls))
            .transpose()?
            .unwrap_or_default();

        Ok(FunctionResponseResult {
            content,
            function_calls: tool_calls,
            response,
        })
    }

    /// Parse tool calls from the API response using functional approach
    #[allow(clippy::unused_self)]
    fn parse_tool_calls(&self, tool_calls: &[Value]) -> Result<Vec<FunctionCall>> {
        tool_calls
            .iter()
            .map(|call| self.parse_single_tool_call(call))
            .collect()
    }

    /// Parse a single tool call from the API response using structured extraction
    fn parse_single_tool_call(&self, call: &Value) -> Result<FunctionCall> {
        let extractor = ToolCallExtractor::new(call);
        let (id, name, arguments) = extractor.extract_all()?;
        Ok(FunctionCall::new(id, name, arguments))
    }

    /// Extract tool call ID with validation
    fn extract_tool_call_id<'a>(&self, call: &'a Value) -> Result<&'a str> {
        call.get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenAIError::parsing("Tool call missing id"))
    }

    /// Extract function object with validation
    fn extract_function_object<'a>(&self, call: &'a Value) -> Result<&'a Value> {
        call.get("function")
            .ok_or_else(|| OpenAIError::parsing("Tool call missing function"))
    }

    /// Extract function name with validation
    fn extract_function_name<'a>(&self, function: &'a Value) -> Result<&'a str> {
        function
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenAIError::parsing("Function missing name"))
    }

    /// Extract function arguments with validation
    fn extract_function_arguments<'a>(&self, function: &'a Value) -> Result<&'a str> {
        function
            .get("arguments")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenAIError::parsing("Function missing arguments"))
    }

    /// Update conversation state with function calls
    fn update_conversation_state(&mut self, result: &FunctionResponseResult) {
        let timestamp = self.current_timestamp();

        for call in &result.function_calls {
            self.conversation_state
                .pending_calls
                .insert(call.call_id.clone(), call.clone());

            self.conversation_state
                .call_history
                .push(FunctionCallEvent::CallInitiated {
                    timestamp,
                    call: call.clone(),
                });
        }
    }

    /// Add function results to a request for continuation
    #[allow(clippy::match_same_arms)]
    fn add_function_results_to_request(
        &self,
        request: &mut ResponseRequest,
        results: &[FunctionCallOutput],
    ) -> Result<()> {
        // Convert current input to messages
        let mut messages = self.convert_input_to_messages(&request.input)?;

        // Add function result messages
        for result in results {
            messages.push(json!({
                "role": "tool",
                "tool_call_id": result.call_id,
                "content": result.output
            }));
        }

        // Update the request input
        request.input = crate::models::responses::ResponseInput::Messages(
            messages
                .into_iter()
                .map(|msg| crate::models::responses::Message {
                    role: match msg.get("role").and_then(|v| v.as_str()).unwrap_or("user") {
                        "user" => crate::models::responses::MessageRole::User,
                        "assistant" => crate::models::responses::MessageRole::Assistant,
                        "system" => crate::models::responses::MessageRole::Developer,
                        _ => crate::models::responses::MessageRole::User,
                    },
                    content: crate::models::responses::MessageContentInput::Text(
                        msg.get("content")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    ),
                })
                .collect(),
        );

        Ok(())
    }

    /// Get current timestamp
    #[allow(clippy::unused_self)]
    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Get API key
    #[must_use]
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

/// Result of a function calling response
#[derive(Debug, Clone)]
pub struct FunctionResponseResult {
    /// Text content from the response
    pub content: Option<String>,
    /// Function calls made by the model
    pub function_calls: Vec<FunctionCall>,
    /// Raw response for additional data
    pub response: Value,
}

impl FunctionConfig {
    /// Create a new function config
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add tools to the configuration
    #[must_use]
    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = tools;
        self
    }

    /// Set tool choice strategy
    #[must_use]
    pub fn with_tool_choice(mut self, choice: ToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    /// Enable/disable parallel function calls
    #[must_use]
    pub const fn with_parallel_calls(mut self, enabled: bool) -> Self {
        self.parallel_function_calls = Some(enabled);
        self
    }

    /// Enable/disable strict mode
    #[must_use]
    pub const fn with_strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = Some(enabled);
        self
    }

    /// Set maximum function calls per response
    #[must_use]
    pub const fn with_max_calls(mut self, max: u32) -> Self {
        self.max_function_calls = Some(max);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::functions::FunctionTool;

    #[test]
    fn test_function_config() {
        let config = FunctionConfig::new()
            .with_parallel_calls(true)
            .with_strict_mode(true)
            .with_max_calls(5);

        assert_eq!(config.parallel_function_calls, Some(true));
        assert_eq!(config.strict_mode, Some(true));
        assert_eq!(config.max_function_calls, Some(5));
    }

    #[test]
    fn test_conversation_state() {
        let mut state = ConversationState::default();
        assert!(state.pending_calls.is_empty());
        assert!(state.completed_calls.is_empty());
        assert!(state.call_history.is_empty());

        let call = FunctionCall::new("call-1", "test_fn", "{}");
        state.pending_calls.insert("call-1".to_string(), call);
        assert_eq!(state.pending_calls.len(), 1);
    }

    #[test]
    fn test_function_call_validation() {
        let api = FunctionsApi::new("test-key").unwrap();

        let tool = Tool::function(FunctionTool::simple("test_fn", "Test function"));
        let call = FunctionCall::new("call-1", "test_fn", "{}");

        let result = api.validate_function_call(&call, &[tool]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_serialize_tools() {
        let api = FunctionsApi::new("test-key").unwrap();

        let tools = vec![Tool::function(FunctionTool::new(
            "get_weather",
            "Get weather information",
            json!({
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                },
                "required": ["location"]
            }),
        ))];

        let serialized = api.serialize_tools(&tools).unwrap();
        assert_eq!(serialized.len(), 1);
        assert_eq!(serialized[0]["type"], "function");
        assert_eq!(serialized[0]["function"]["name"], "get_weather");
    }

    #[test]
    fn test_parse_tool_calls() {
        let api = FunctionsApi::new("test-key").unwrap();

        // Test data simulating OpenAI API response
        let tool_calls = vec![
            json!({
                "id": "call_1",
                "type": "function",
                "function": {
                    "name": "get_weather",
                    "arguments": r#"{"location": "New York"}"#
                }
            }),
            json!({
                "id": "call_2",
                "type": "function",
                "function": {
                    "name": "get_time",
                    "arguments": r#"{"timezone": "UTC"}"#
                }
            }),
        ];

        let result = api.parse_tool_calls(&tool_calls).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].call_id, "call_1");
        assert_eq!(result[0].name, "get_weather");
        assert_eq!(result[0].arguments, r#"{"location": "New York"}"#);

        assert_eq!(result[1].call_id, "call_2");
        assert_eq!(result[1].name, "get_time");
        assert_eq!(result[1].arguments, r#"{"timezone": "UTC"}"#);
    }

    #[test]
    fn test_tool_call_extractor() {
        let call_data = json!({
            "id": "call_test",
            "type": "function",
            "function": {
                "name": "test_function",
                "arguments": r#"{"param": "value"}"#
            }
        });

        let extractor = ToolCallExtractor::new(&call_data);
        let (id, name, arguments) = extractor.extract_all().unwrap();

        assert_eq!(id, "call_test");
        assert_eq!(name, "test_function");
        assert_eq!(arguments, r#"{"param": "value"}"#);
    }
}
