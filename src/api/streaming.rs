use crate::api::common::ApiClientConstructors;
use crate::api::responses::ResponsesApi;
use crate::error::{ApiErrorResponse, OpenAIError, Result};
use crate::models::functions::{FunctionCall, Tool, ToolChoice};
use crate::models::responses::{
    Message, ResponseChoice, ResponseOutput, ResponseRequest, ResponseResult, StreamChunk, Usage,
};
use eventsource_stream::Eventsource;
use futures::Stream;
use futures::StreamExt as FuturesStreamExt;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

/// Streaming response from `OpenAI` API
pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>;

/// Streaming response with function calling support
pub type FunctionStream = Pin<Box<dyn Stream<Item = Result<FunctionStreamEvent>> + Send>>;

/// Streaming API client (extends `ResponsesApi`)
#[derive(Clone)]
pub struct StreamingApi {
    /// Underlying responses API client
    responses_api: ResponsesApi,
}

impl StreamingApi {
    /// Create a new `StreamingApi` client
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let responses_api = ResponsesApi::new(api_key)?;
        Ok(Self { responses_api })
    }

    /// Create a new `StreamingApi` client with custom base URL
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        let responses_api = ResponsesApi::with_base_url(api_key, base_url)?;
        Ok(Self { responses_api })
    }

    /// Create a streaming response
    pub async fn create_response_stream(
        &self,
        request: &ResponseRequest,
    ) -> Result<ResponseStream> {
        // Create a streaming version of the request
        let mut streaming_request = request.clone();
        streaming_request.stream = Some(true);

        let url = format!("{}/v1/chat/completions", self.responses_api.base_url());

        // Convert to OpenAI format
        let mut openai_request = self.responses_api.to_openai_format(&streaming_request)?;
        openai_request["stream"] = serde_json::json!(true);

        let response = self
            .responses_api
            .client()
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.responses_api.api_key()),
            )
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
            .json(&openai_request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_response: ApiErrorResponse = response.json().await?;
            return Err(OpenAIError::from_api_response(
                status.as_u16(),
                error_response,
            ));
        }

        // Convert the response to a stream
        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(|event_result| async move {
                match event_result {
                    Ok(event) => {
                        if event.data == "[DONE]" {
                            return None; // End of stream
                        }

                        match serde_json::from_str::<StreamChunk>(&event.data) {
                            Ok(chunk) => Some(Ok(chunk)),
                            Err(e) => Some(Err(OpenAIError::streaming(format!(
                                "Failed to parse chunk: {e}"
                            )))),
                        }
                    }
                    Err(e) => Some(Err(OpenAIError::streaming(format!("Stream error: {e}")))),
                }
            });

        Ok(Box::pin(stream))
    }

    /// Create a simple text streaming response
    pub async fn create_text_stream(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<ResponseStream> {
        let request = ResponseRequest::new_text(model, prompt).with_streaming(true);
        self.create_response_stream(&request).await
    }

    /// Create a streaming chat response
    pub async fn create_chat_stream(
        &self,
        model: impl Into<String>,
        messages: Vec<Message>,
    ) -> Result<ResponseStream> {
        let request = ResponseRequest::new_messages(model, messages).with_streaming(true);
        self.create_response_stream(&request).await
    }

    /// Create a streaming response with instructions
    pub async fn create_instructed_stream(
        &self,
        model: impl Into<String>,
        input: impl Into<String>,
        instructions: impl Into<String>,
    ) -> Result<ResponseStream> {
        let request = ResponseRequest::new_text(model, input)
            .with_instructions(instructions)
            .with_streaming(true);
        self.create_response_stream(&request).await
    }

    /// Collect all chunks from a stream into a single response
    pub async fn collect_stream_response(mut stream: ResponseStream) -> Result<String> {
        let mut content = String::new();

        while let Some(chunk_result) = FuturesStreamExt::next(&mut stream).await {
            let chunk = chunk_result?;

            for choice in chunk.choices {
                if let Some(ref delta_content) = choice.delta.content {
                    content.push_str(delta_content);
                }

                // Check if we're done
                if choice.finish_reason.is_some() {
                    break;
                }
            }
        }

        Ok(content)
    }

    /// Create a channel-based stream for easier handling
    pub async fn create_channel_stream(
        &self,
        request: &ResponseRequest,
    ) -> Result<UnboundedReceiverStream<Result<StreamChunk>>> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut stream = self.create_response_stream(request).await?;

        tokio::spawn(async move {
            while let Some(chunk_result) = FuturesStreamExt::next(&mut stream).await {
                if tx.send(chunk_result).is_err() {
                    break; // Receiver dropped
                }
            }
        });

        Ok(UnboundedReceiverStream::new(rx))
    }

    /// Create a streaming response with function calling support
    pub async fn create_function_stream(
        &self,
        request: &ResponseRequest,
        tools: Vec<Tool>,
        tool_choice: Option<ToolChoice>,
    ) -> Result<FunctionStream> {
        let mut streaming_request = request.clone();
        streaming_request.stream = Some(true);
        streaming_request.tools = Some(tools);
        if let Some(choice) = tool_choice {
            streaming_request.tool_choice = Some(choice);
        }

        let stream = self.create_response_stream(&streaming_request).await?;
        Ok(FunctionStreamProcessor::into_function_stream(stream))
    }

    /// Create a simple function call stream
    pub async fn call_function_stream(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
        tools: Vec<Tool>,
        tool_choice: Option<ToolChoice>,
    ) -> Result<FunctionStream> {
        let request = ResponseRequest::new_text(model, prompt);
        self.create_function_stream(&request, tools, tool_choice)
            .await
    }

    /// Get access to the underlying `ResponsesApi`
    #[must_use]
    pub fn responses_api(&self) -> &ResponsesApi {
        &self.responses_api
    }
}

/// Helper trait for streaming operations
pub trait ResponseStreamExt {
    /// Collect content from stream chunks
    fn collect_content(self) -> Pin<Box<dyn futures::Future<Output = Result<String>> + Send>>;
}

impl ResponseStreamExt for ResponseStream {
    fn collect_content(self) -> Pin<Box<dyn futures::Future<Output = Result<String>> + Send>> {
        Box::pin(StreamingApi::collect_stream_response(self))
    }
}

/// Stream event types for fine-grained control
#[derive(Debug, Clone)]
pub enum StreamEventType {
    /// Response stream has started
    Started {
        /// Unique identifier for the response
        response_id: String,
    },
    /// Incremental content delta received
    Delta {
        /// The content fragment
        content: String,
    },
    /// Response stream has completed successfully
    Completed {
        /// The complete response result
        response: ResponseResult,
    },
    /// Error occurred during streaming
    Error {
        /// Error message describing what went wrong
        message: String,
    },
}

/// Convert stream chunks to events
#[must_use]
pub fn chunk_to_events(chunk: StreamChunk) -> Vec<StreamEventType> {
    let mut events = Vec::new();

    for choice in chunk.choices {
        if let Some(content) = choice.delta.content {
            events.push(StreamEventType::Delta { content });
        }

        if choice.finish_reason.is_some() {
            // This is a simplified completion event
            // In a real implementation, you'd construct the full ResponseResult
            events.push(StreamEventType::Completed {
                response: ResponseResult {
                    id: Some(chunk.id.clone()),
                    object: chunk.object.clone(),
                    created: chunk.created,
                    model: chunk.model.clone(),
                    choices: vec![ResponseChoice {
                        index: choice.index,
                        message: ResponseOutput {
                            content: Some(String::new()), // Would need to accumulate
                            tool_calls: None,
                            function_calls: None,
                            structured_data: None,
                            schema_validation: None,
                        },
                        finish_reason: choice.finish_reason.clone(),
                    }],
                    usage: Some(Usage {
                        prompt_tokens: 0, // Would need actual values
                        completion_tokens: 0,
                        total_tokens: 0,
                        prompt_tokens_details: None,
                        completion_tokens_details: None,
                    }),
                },
            });
        }
    }

    events
}

/// Events for function calling streams
#[derive(Debug, Clone)]
pub enum FunctionStreamEvent {
    /// Regular content delta
    ContentDelta {
        /// The content text being streamed
        content: String,
    },
    /// Function call started
    FunctionCallStarted {
        /// Unique identifier for this function call
        call_id: String,
        /// Name of the function being called
        function_name: String,
    },
    /// Function call arguments delta
    FunctionCallArgumentsDelta {
        /// Unique identifier for this function call
        call_id: String,
        /// Incremental arguments data for the function call
        arguments_delta: String,
    },
    /// Function call completed
    FunctionCallCompleted {
        /// The completed function call
        call: FunctionCall,
    },
    /// Stream completed
    Completed {
        /// The final response result
        response: ResponseResult,
    },
    /// Error occurred
    Error {
        /// Error message describing what went wrong
        message: String,
    },
}

/// Function stream processor that accumulates function calls
pub struct FunctionStreamProcessor {
    /// Raw stream
    stream: ResponseStream,
    /// Accumulated function calls by index
    function_calls: std::collections::HashMap<u32, FunctionCallBuilder>,
}

/// Builder for accumulating function call deltas
#[derive(Debug, Clone)]
struct FunctionCallBuilder {
    /// ID of the function call
    call_id: Option<String>,
    /// Name of the function being called
    name: Option<String>,
    /// JSON arguments for the function call
    arguments: String,
}

impl FunctionStreamProcessor {
    /// Create a new function stream from a response stream
    #[must_use]
    pub fn into_function_stream(stream: ResponseStream) -> FunctionStream {
        let processor = Self {
            stream,
            function_calls: std::collections::HashMap::new(),
        };

        Box::pin(async_stream::stream! {
            let mut processor = processor;
            while let Some(chunk_result) = FuturesStreamExt::next(&mut processor.stream).await {
                let chunk = match chunk_result {
                    Ok(chunk) => chunk,
                    Err(e) => {
                        yield Ok(FunctionStreamEvent::Error {
                            message: e.to_string(),
                        });
                        break;
                    }
                };

                for choice in &chunk.choices {
                    if let Some(ref content) = choice.delta.content {
                        yield Ok(FunctionStreamEvent::ContentDelta { content: content.clone() });
                    }

                    if let Some(ref tool_calls) = choice.delta.tool_calls {
                        for event in Self::process_tool_calls(&mut processor.function_calls, tool_calls.clone()) {
                            yield Ok(event);
                        }
                    }

                    if choice.finish_reason.is_some() {
                        for event in Self::handle_completion(&mut processor.function_calls, &chunk, choice) {
                            yield Ok(event);
                        }
                        break;
                    }
                }
            }
        })
    }

    /// Process tool calls from stream delta
    fn process_tool_calls(
        function_calls: &mut std::collections::HashMap<u32, FunctionCallBuilder>,
        tool_calls: Vec<crate::models::responses::ToolCallDelta>,
    ) -> Vec<FunctionStreamEvent> {
        let mut events = Vec::new();

        for tool_call in tool_calls {
            let builder =
                function_calls
                    .entry(tool_call.index)
                    .or_insert_with(|| FunctionCallBuilder {
                        call_id: None,
                        name: None,
                        arguments: String::new(),
                    });

            if let Some(id) = tool_call.id {
                builder.call_id = Some(id);
            }

            if let Some(function) = tool_call.function {
                events.extend(Self::process_function_delta(builder, function));
            }
        }

        events
    }

    /// Process function call delta from stream
    fn process_function_delta(
        builder: &mut FunctionCallBuilder,
        function: crate::models::responses::FunctionCallDelta,
    ) -> Vec<FunctionStreamEvent> {
        let mut events = Vec::new();

        if let Some(name) = function.name {
            builder.name = Some(name.clone());

            if let Some(call_id) = &builder.call_id {
                events.push(FunctionStreamEvent::FunctionCallStarted {
                    call_id: call_id.clone(),
                    function_name: name,
                });
            }
        }

        if let Some(args_delta) = function.arguments {
            builder.arguments.push_str(&args_delta);

            if let Some(call_id) = &builder.call_id {
                events.push(FunctionStreamEvent::FunctionCallArgumentsDelta {
                    call_id: call_id.clone(),
                    arguments_delta: args_delta,
                });
            }
        }

        events
    }

    /// Handle stream completion
    fn handle_completion(
        function_calls: &mut std::collections::HashMap<u32, FunctionCallBuilder>,
        chunk: &StreamChunk,
        choice: &crate::models::responses::StreamChoice,
    ) -> Vec<FunctionStreamEvent> {
        let mut events = Vec::new();

        for (_, builder) in function_calls.drain() {
            if let Some(call) = builder.build() {
                events.push(FunctionStreamEvent::FunctionCallCompleted { call });
            }
        }

        let response = Self::create_completion_response(chunk, choice);
        events.push(FunctionStreamEvent::Completed { response });

        events
    }

    /// Create completion response from stream
    fn create_completion_response(
        chunk: &StreamChunk,
        choice: &crate::models::responses::StreamChoice,
    ) -> ResponseResult {
        ResponseResult {
            id: Some(chunk.id.clone()),
            object: chunk.object.clone(),
            created: chunk.created,
            model: chunk.model.clone(),
            choices: vec![ResponseChoice {
                index: choice.index,
                message: ResponseOutput {
                    content: None,
                    tool_calls: None,
                    function_calls: None,
                    structured_data: None,
                    schema_validation: None,
                },
                finish_reason: choice.finish_reason.clone(),
            }],
            usage: Some(Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
                prompt_tokens_details: None,
                completion_tokens_details: None,
            }),
        }
    }
}

impl FunctionCallBuilder {
    /// Build the final function call if complete
    fn build(self) -> Option<FunctionCall> {
        match (self.call_id, self.name) {
            (Some(call_id), Some(name)) => Some(FunctionCall::new(call_id, name, self.arguments)),
            _ => None,
        }
    }
}

/// Implement the new function for `FunctionStream`
impl FunctionStreamProcessor {
    /// Create a function stream from a response stream
    #[must_use]
    pub fn from_response_stream(stream: ResponseStream) -> FunctionStream {
        Self::into_function_stream(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_api_creation() {
        let api = StreamingApi::new("test-key").unwrap();
        assert_eq!(api.responses_api().api_key(), "test-key");
    }

    #[tokio::test]
    async fn test_stream_request_configuration() {
        let request = ResponseRequest::new_text("gpt-4", "Hello").with_streaming(true);

        assert_eq!(request.stream, Some(true));
    }
}
