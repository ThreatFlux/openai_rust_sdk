//! Main StreamingApi client for OpenAI streaming operations

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::api::responses::ResponsesApi;
use crate::constants::endpoints;
use crate::error::{ApiErrorResponse, OpenAIError, Result};
use crate::models::functions::{Tool, ToolChoice};
use crate::models::responses::{Message, ResponseRequest};
use eventsource_stream::Eventsource;
use futures::StreamExt as FuturesStreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use super::helpers::{collect_stream_response, process_stream_event};
use super::processor::FunctionStreamProcessor;
use super::types::{FunctionStream, ResponseStream};

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

        let url = format!(
            "{}{}",
            self.responses_api.base_url(),
            endpoints::CHAT_COMPLETIONS
        );

        // Convert to OpenAI format and send request
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

        // Check response status
        let status = response.status();
        if !status.is_success() {
            let error_response: ApiErrorResponse = response.json().await?;
            return Err(OpenAIError::from_api_response(
                status.as_u16(),
                error_response,
            ));
        }

        // Convert the response to a stream with event processing
        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(|event_result| async move { process_stream_event(event_result) });

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
    pub async fn collect_stream_response(stream: ResponseStream) -> Result<String> {
        collect_stream_response(stream).await
    }

    /// Create a channel-based stream for easier handling
    pub async fn create_channel_stream(
        &self,
        request: &ResponseRequest,
    ) -> Result<UnboundedReceiverStream<Result<crate::models::responses::StreamChunk>>> {
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

impl ApiClientConstructors for StreamingApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        let responses_api = ResponsesApi::from_http_client(http_client);
        Self { responses_api }
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
