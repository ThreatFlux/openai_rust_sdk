use crate::api::functions::{FunctionConfig, FunctionResponseResult, FunctionsApi};
use crate::api::streaming::ResponseStream;
use crate::api::{common::ApiClientConstructors, ResponsesApi, StreamingApi};
use crate::error::Result;
use crate::models::functions::{FunctionCall, FunctionCallOutput, Tool, ToolChoice};
use crate::models::responses::{
    Message, MessageRole, ResponseInput, ResponseRequest, ResponseResult,
};

/// Main `OpenAI` client that provides access to all APIs
#[derive(Clone)]
pub struct OpenAIClient {
    /// API client for non-streaming responses
    responses_api: ResponsesApi,
    /// API client for streaming responses
    streaming_api: StreamingApi,
    /// API client for function calling
    functions_api: FunctionsApi,
}

impl OpenAIClient {
    /// Create a new `OpenAI` client with an API key
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        let responses_api = ResponsesApi::new(&api_key)?;
        let streaming_api = StreamingApi::new(&api_key)?;
        let functions_api = FunctionsApi::new(&api_key)?;

        Ok(Self {
            responses_api,
            streaming_api,
            functions_api,
        })
    }

    /// Create a new `OpenAI` client with custom base URL
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        let base_url = base_url.into();
        let responses_api = ResponsesApi::with_base_url(&api_key, &base_url)?;
        let streaming_api = StreamingApi::with_base_url(&api_key, &base_url)?;
        let functions_api = FunctionsApi::with_base_url(&api_key, &base_url)?;

        Ok(Self {
            responses_api,
            streaming_api,
            functions_api,
        })
    }

    /// Create a response using the responses API
    pub async fn create_response(&self, request: &ResponseRequest) -> Result<ResponseResult> {
        self.responses_api.create_response(request).await
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
        self.responses_api.create_text_response(model, prompt).await
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
        self.responses_api
            .create_instructed_response(model, input, instructions)
            .await
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
        self.responses_api
            .create_chat_response(model, messages)
            .await
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
        self.responses_api
            .create_custom_response(model, input, temperature, max_tokens, instructions)
            .await
    }

    /// Get access to the responses API
    #[must_use]
    pub fn responses(&self) -> &ResponsesApi {
        &self.responses_api
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

    /// Execute a complete function calling conversation
    pub async fn function_conversation(
        &self,
        model: impl Into<String> + Clone,
        messages: Vec<Message>,
        tools: Vec<Tool>,
        max_iterations: Option<u32>,
    ) -> Result<Vec<FunctionResponseResult>> {
        let mut conversation_results = Vec::new();
        let mut current_messages = messages;
        let max_iter = max_iterations.unwrap_or(10);

        for _iteration in 0..max_iter {
            let request =
                ResponseRequest::new_messages(model.clone().into(), current_messages.clone());
            let config = FunctionConfig::new().with_tools(tools.clone());

            let result = self.create_function_response(&request, &config).await?;
            conversation_results.push(result.clone());

            // If no function calls, we're done
            if result.function_calls.is_empty() {
                break;
            }

            // Execute function calls (this is application-specific)
            let mut function_results = Vec::new();
            for call in &result.function_calls {
                // In practice, applications would implement their own function execution
                let output = self.functions_api.execute_function_call(call).await?;
                function_results.push(output);
            }

            // Submit results and continue
            let continuation = self
                .submit_function_results(function_results, &request, &config)
                .await?;
            conversation_results.push(continuation.clone());

            // Add the response to the conversation
            if let Some(content) = continuation.content {
                current_messages.push(Message::assistant(content));
            }

            // If no more function calls, we're done
            if continuation.function_calls.is_empty() {
                break;
            }
        }

        Ok(conversation_results)
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

/// Convenience function to create a client from environment variable
pub fn from_env() -> Result<OpenAIClient> {
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
        crate::error::OpenAIError::authentication("OPENAI_API_KEY environment variable not set")
    })?;
    OpenAIClient::new(api_key)
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
}
