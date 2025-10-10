use crate::api::functions::{FunctionConfig, FunctionResponseResult, FunctionsApi};
use crate::api::responses_v2::{
    DeleteResponseAck, ListResponsesParams as ResponsesListParams, ResponseInputItemList,
    ResponseList, ResponsesApiV2, ResponsesEventStream,
};
use crate::api::streaming::ResponseStream;
use crate::api::streaming::StreamingApi;
use crate::api::{
    common::{ApiClientConstructors, StandardListParams},
    ResponsesApi,
};
use crate::error::Result;
use crate::models::functions::{FunctionCall, FunctionCallOutput, Tool, ToolChoice};
use crate::models::responses::{
    Message, MessageRole, ResponseInput, ResponseRequest, ResponseResult,
};
use crate::models::responses_v2::{
    from_legacy_request, to_legacy_response, CreateResponseRequest, Instructions, ResponseObject,
};

/// Main `OpenAI` client that provides access to all APIs
#[derive(Clone)]
pub struct OpenAIClient {
    /// API client for non-streaming responses
    responses_api: ResponsesApi,
    /// Modern Responses API client
    responses_api_v2: ResponsesApiV2,
    /// API client for streaming responses
    streaming_api: StreamingApi,
    /// API client for function calling
    functions_api: FunctionsApi,
}

/// State management for function calling conversations
struct ConversationState {
    /// The current messages in the conversation
    current_messages: Vec<Message>,
    /// Results from function calls
    results: Vec<FunctionResponseResult>,
    /// Current iteration count
    iteration_count: u32,
    /// Maximum allowed iterations
    max_iterations: u32,
}

impl ConversationState {
    /// Create a new conversation state
    fn new(messages: Vec<Message>, max_iterations: Option<u32>) -> Self {
        Self {
            current_messages: messages,
            results: Vec::new(),
            iteration_count: 0,
            max_iterations: max_iterations.unwrap_or(10),
        }
    }

    /// Check if the conversation should continue
    fn should_continue(&self) -> bool {
        self.iteration_count < self.max_iterations
    }

    /// Increment the iteration counter
    fn increment_iteration(&mut self) {
        self.iteration_count += 1;
    }

    /// Add a function result
    fn add_result(&mut self, result: FunctionResponseResult) {
        self.results.push(result);
    }
}

impl OpenAIClient {
    /// Create a new `OpenAI` client with an API key
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        let responses_api = ResponsesApi::new(&api_key)?;
        let responses_api_v2 = ResponsesApiV2::new(&api_key)?;
        let streaming_api = StreamingApi::new(&api_key)?;
        let functions_api = FunctionsApi::new(&api_key)?;

        Ok(Self {
            responses_api,
            responses_api_v2,
            streaming_api,
            functions_api,
        })
    }

    /// Create a new `OpenAI` client with custom base URL
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        let base_url = base_url.into();
        let responses_api = ResponsesApi::with_base_url(&api_key, &base_url)?;
        let responses_api_v2 = ResponsesApiV2::new_with_base_url(&api_key, &base_url)?;
        let streaming_api = StreamingApi::with_base_url(&api_key, &base_url)?;
        let functions_api = FunctionsApi::with_base_url(&api_key, &base_url)?;

        Ok(Self {
            responses_api,
            responses_api_v2,
            streaming_api,
            functions_api,
        })
    }

    /// Create a response using the responses API
    pub async fn create_response(&self, request: &ResponseRequest) -> Result<ResponseResult> {
        let modern_request = from_legacy_request(request);
        let response = self
            .responses_api_v2
            .create_response(&modern_request)
            .await?;
        Ok(to_legacy_response(&response))
    }

    /// Create a streaming response
    pub async fn create_response_stream(
        &self,
        request: &ResponseRequest,
    ) -> Result<ResponseStream> {
        self.streaming_api.create_response_stream(request).await
    }

    // Convenience methods for common operations

    /// Generate text from a simple prompt
    pub async fn generate_text(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<String> {
        let request = CreateResponseRequest::new_text(model, prompt);
        let response = self.create_response_v2(&request).await?;
        Ok(response.output_text())
    }

    /// Generate text with streaming
    pub async fn generate_text_stream(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<ResponseStream> {
        self.streaming_api.create_text_stream(model, prompt).await
    }

    /// Generate text with instructions
    pub async fn generate_with_instructions(
        &self,
        model: impl Into<String>,
        input: impl Into<String>,
        instructions: impl Into<String>,
    ) -> Result<String> {
        let request = CreateResponseRequest::new_text(model, input)
            .with_instructions(Instructions::Text(instructions.into()));
        let response = self.create_response_v2(&request).await?;
        Ok(response.output_text())
    }

    /// Generate text with instructions and streaming
    pub async fn generate_with_instructions_stream(
        &self,
        model: impl Into<String>,
        input: impl Into<String>,
        instructions: impl Into<String>,
    ) -> Result<ResponseStream> {
        self.streaming_api
            .create_instructed_stream(model, input, instructions)
            .await
    }

    /// Create a chat completion from messages
    pub async fn create_chat_completion(
        &self,
        model: impl Into<String>,
        messages: Vec<Message>,
    ) -> Result<String> {
        let request = CreateResponseRequest::new_messages(model, messages);
        let response = self.create_response_v2(&request).await?;
        Ok(response.output_text())
    }

    /// Create a streaming chat completion
    pub async fn create_chat_completion_stream(
        &self,
        model: impl Into<String>,
        messages: Vec<Message>,
    ) -> Result<ResponseStream> {
        self.streaming_api.create_chat_stream(model, messages).await
    }

    /// Build a conversation and get response
    pub async fn chat(
        &self,
        model: impl Into<String>,
        conversation: ChatBuilder,
    ) -> Result<String> {
        let messages = conversation.build();
        self.create_chat_completion(model, messages).await
    }

    /// Build a conversation and get streaming response
    pub async fn chat_stream(
        &self,
        model: impl Into<String>,
        conversation: ChatBuilder,
    ) -> Result<ResponseStream> {
        let messages = conversation.build();
        self.create_chat_completion_stream(model, messages).await
    }

    /// Create a custom response with all parameters
    pub async fn create_custom_response(
        &self,
        model: impl Into<String>,
        input: ResponseInput,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        instructions: Option<String>,
    ) -> Result<ResponseResult> {
        let model_str = model.into();
        let mut legacy_request = match input {
            ResponseInput::Text(text) => ResponseRequest::new_text(model_str.clone(), text),
            ResponseInput::Messages(messages) => {
                ResponseRequest::new_messages(model_str.clone(), messages)
            }
        };

        if let Some(temp) = temperature {
            legacy_request = legacy_request.with_temperature(temp);
        }
        if let Some(tokens) = max_tokens {
            legacy_request = legacy_request.with_max_tokens(tokens);
        }
        if let Some(instr) = instructions {
            legacy_request = legacy_request.with_instructions(instr);
        }

        let modern_request = from_legacy_request(&legacy_request);
        let response = self
            .responses_api_v2
            .create_response(&modern_request)
            .await?;
        Ok(to_legacy_response(&response))
    }

    /// Get access to the responses API
    #[must_use]
    pub fn responses(&self) -> &ResponsesApi {
        &self.responses_api
    }

    /// Get access to the modern Responses API
    #[must_use]
    pub fn responses_v2(&self) -> &ResponsesApiV2 {
        &self.responses_api_v2
    }

    /// Create a response using the modern Responses API
    pub async fn create_response_v2(
        &self,
        request: &CreateResponseRequest,
    ) -> Result<ResponseObject> {
        self.responses_api_v2.create_response(request).await
    }

    /// Stream a response using the modern Responses API
    pub async fn stream_response_v2(
        &self,
        request: &CreateResponseRequest,
    ) -> Result<ResponsesEventStream> {
        self.responses_api_v2.stream_response(request).await
    }

    /// Retrieve a response by ID using the modern Responses API
    pub async fn retrieve_response_v2(&self, response_id: &str) -> Result<ResponseObject> {
        self.responses_api_v2
            .retrieve_response(response_id, None)
            .await
    }

    /// Delete a stored response using the modern Responses API
    pub async fn delete_response_v2(&self, response_id: &str) -> Result<DeleteResponseAck> {
        self.responses_api_v2.delete_response(response_id).await
    }

    /// Cancel a background response request using the modern Responses API
    pub async fn cancel_response_v2(&self, response_id: &str) -> Result<ResponseObject> {
        self.responses_api_v2.cancel_response(response_id).await
    }

    /// List responses for the current project using the modern Responses API
    pub async fn list_responses_v2(&self, params: &ResponsesListParams) -> Result<ResponseList> {
        self.responses_api_v2.list_responses(params).await
    }

    /// List input items for a particular response using the modern Responses API
    pub async fn list_response_input_items_v2(
        &self,
        response_id: &str,
        params: &StandardListParams,
    ) -> Result<ResponseInputItemList> {
        self.responses_api_v2
            .list_response_input_items(response_id, params)
            .await
    }

    /// Get access to the streaming API
    #[must_use]
    pub fn streaming(&self) -> &StreamingApi {
        &self.streaming_api
    }

    /// Get access to the functions API
    #[must_use]
    pub fn functions(&self) -> &FunctionsApi {
        &self.functions_api
    }

    // Function calling methods

    /// Create a response with function calling support
    pub async fn create_function_response(
        &self,
        request: &ResponseRequest,
        config: &FunctionConfig,
    ) -> Result<FunctionResponseResult> {
        // Clone the functions API to make it mutable for this call
        let mut functions_api = self.functions_api.clone();
        functions_api
            .create_function_response(request, config)
            .await
    }

    /// Submit function call results and continue the conversation
    pub async fn submit_function_results(
        &self,
        results: Vec<FunctionCallOutput>,
        original_request: &ResponseRequest,
        config: &FunctionConfig,
    ) -> Result<FunctionResponseResult> {
        let mut functions_api = self.functions_api.clone();
        functions_api
            .submit_function_results(results, original_request, config)
            .await
    }

    /// Process a single iteration of the conversation
    async fn process_conversation_iteration(
        &self,
        model: &str,
        tools: &[Tool],
        state: &mut ConversationState,
    ) -> Result<bool> {
        let request =
            ResponseRequest::new_messages(model.to_string(), state.current_messages.clone());
        let config = FunctionConfig::new().with_tools(tools.to_vec());

        let result = self.create_function_response(&request, &config).await?;
        state.add_result(result.clone());
        state.increment_iteration();

        let has_function_calls = !result.function_calls.is_empty();
        if !has_function_calls {
            return Ok(false);
        }

        let continuation = self
            .execute_and_continue_conversation(&result, &request, &config)
            .await?;
        state.add_result(continuation.clone());

        self.update_conversation_messages(&mut state.current_messages, &continuation);
        Ok(!continuation.function_calls.is_empty())
    }

    /// Execute a complete function calling conversation
    pub async fn function_conversation(
        &self,
        model: impl Into<String> + Clone,
        messages: Vec<Message>,
        tools: Vec<Tool>,
        max_iterations: Option<u32>,
    ) -> Result<Vec<FunctionResponseResult>> {
        let mut conversation_state = ConversationState::new(messages, max_iterations);
        let model_str = model.into();

        while conversation_state.should_continue() {
            let iteration_result = self
                .process_conversation_iteration(&model_str, &tools, &mut conversation_state)
                .await?;

            if !iteration_result {
                break;
            }
        }

        Ok(conversation_state.results)
    }

    /// Execute function calls and continue the conversation
    async fn execute_and_continue_conversation(
        &self,
        result: &FunctionResponseResult,
        request: &ResponseRequest,
        config: &FunctionConfig,
    ) -> Result<FunctionResponseResult> {
        // Execute function calls (this is application-specific)
        let function_results = self.execute_function_calls(&result.function_calls).await?;

        // Submit results and continue
        self.submit_function_results(function_results, request, config)
            .await
    }

    /// Execute multiple function calls
    async fn execute_function_calls(
        &self,
        function_calls: &[FunctionCall],
    ) -> Result<Vec<FunctionCallOutput>> {
        let mut function_results = Vec::new();
        for call in function_calls {
            // In practice, applications would implement their own function execution
            let output = self.functions_api.execute_function_call(call).await?;
            function_results.push(output);
        }
        Ok(function_results)
    }

    /// Update conversation messages with assistant response
    fn update_conversation_messages(
        &self,
        current_messages: &mut Vec<Message>,
        continuation: &FunctionResponseResult,
    ) {
        // Add the response to the conversation
        if let Some(content) = &continuation.content {
            current_messages.push(Message::assistant(content.clone()));
        }
    }

    /// Create a simple function call
    pub async fn call_function(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
        tools: Vec<Tool>,
        tool_choice: Option<ToolChoice>,
    ) -> Result<FunctionResponseResult> {
        let request = ResponseRequest::new_text(model, prompt).with_tools(tools.clone());

        let mut config = FunctionConfig::new().with_tools(tools);
        if let Some(choice) = tool_choice {
            config = config.with_tool_choice(choice);
        }

        self.create_function_response(&request, &config).await
    }

    /// Create a chat completion with optional tools
    pub async fn chat_with_tools(
        &self,
        model: impl Into<String>,
        conversation: ChatBuilder,
        tools: Option<Vec<Tool>>,
        tool_choice: Option<ToolChoice>,
    ) -> Result<FunctionResponseResult> {
        let messages = conversation.build();
        let mut request = ResponseRequest::new_messages(model, messages);

        let mut config = FunctionConfig::new();

        if let Some(tools) = tools {
            request = request.with_tools(tools.clone());
            config = config.with_tools(tools);
        }

        if let Some(choice) = tool_choice {
            request = request.with_tool_choice(choice.clone());
            config = config.with_tool_choice(choice);
        }

        self.create_function_response(&request, &config).await
    }

    /// Execute a function call with automatic result handling
    pub async fn execute_function_with_result(
        &self,
        call: &FunctionCall,
    ) -> Result<FunctionCallOutput> {
        self.functions_api.execute_function_call(call).await
    }
}

/// Builder for constructing conversations
#[derive(Debug, Clone, Default)]
pub struct ChatBuilder {
    /// Collection of conversation messages
    messages: Vec<Message>,
}

impl ChatBuilder {
    /// Create a new chat builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a user message
    pub fn user(mut self, content: impl Into<String>) -> Self {
        self.messages.push(Message::user(content));
        self
    }

    /// Add an assistant message
    pub fn assistant(mut self, content: impl Into<String>) -> Self {
        self.messages.push(Message::assistant(content));
        self
    }

    /// Add a developer/system message
    pub fn developer(mut self, content: impl Into<String>) -> Self {
        self.messages.push(Message::developer(content));
        self
    }

    /// Add a custom message
    pub fn message(mut self, role: MessageRole, content: impl Into<String>) -> Self {
        self.messages.push(Message {
            role,
            content: crate::models::responses::MessageContentInput::Text(content.into()),
        });
        self
    }

    /// Add multiple messages
    #[must_use]
    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.messages.extend(messages);
        self
    }

    /// Build the final message list
    #[must_use]
    pub fn build(self) -> Vec<Message> {
        self.messages
    }

    /// Get the current message count
    #[must_use]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if the builder is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

/// Convenience function to create a client from environment variables
pub fn from_env() -> Result<OpenAIClient> {
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
        crate::error::OpenAIError::authentication("OPENAI_API_KEY environment variable not set")
    })?;

    let base_url = match std::env::var("OPENAI_BASE_URL") {
        Ok(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Err(crate::error::OpenAIError::invalid_request(
                    "OPENAI_BASE_URL cannot be empty",
                ));
            }
            Some(trimmed.to_string())
        }
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(_)) => {
            return Err(crate::error::OpenAIError::invalid_request(
                "OPENAI_BASE_URL contains invalid unicode characters",
            ));
        }
    };

    match base_url {
        Some(url) => OpenAIClient::with_base_url(api_key, url),
        None => OpenAIClient::new(api_key),
    }
}

/// Convenience function to create a client with custom base URL from environment
pub fn from_env_with_base_url(base_url: impl Into<String>) -> Result<OpenAIClient> {
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
        crate::error::OpenAIError::authentication("OPENAI_API_KEY environment variable not set")
    })?;
    OpenAIClient::with_base_url(api_key, base_url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_client_creation() {
        let client = OpenAIClient::new("test-key").unwrap();
        assert_eq!(client.responses().api_key(), "test-key");
    }

    #[test]
    fn test_chat_builder() {
        let conversation = ChatBuilder::new()
            .developer("You are a helpful assistant")
            .user("Hello")
            .assistant("Hi there!")
            .user("How are you?");

        let messages = conversation.build();
        assert_eq!(messages.len(), 4);
        assert_eq!(messages[0].role, MessageRole::Developer);
        assert_eq!(messages[1].role, MessageRole::User);
        assert_eq!(messages[2].role, MessageRole::Assistant);
        assert_eq!(messages[3].role, MessageRole::User);
    }

    #[test]
    fn test_chat_builder_methods() {
        let mut builder = ChatBuilder::new();
        assert_eq!(builder.len(), 0);
        assert!(builder.is_empty());

        builder = builder.user("Test");
        assert_eq!(builder.len(), 1);
        assert!(!builder.is_empty());
    }

    #[test]
    fn test_empty_api_key() {
        let result = OpenAIClient::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_env_with_custom_base_url() {
        let _guard = ENV_LOCK.lock().expect("lock poisoned");

        std::env::set_var("OPENAI_API_KEY", "test-key");
        std::env::set_var("OPENAI_BASE_URL", "https://example-proxy.test/v1");

        let client = from_env().expect("client creation failed");
        assert_eq!(
            client.responses().base_url(),
            "https://example-proxy.test/v1"
        );

        std::env::remove_var("OPENAI_API_KEY");
        std::env::remove_var("OPENAI_BASE_URL");
    }
}
