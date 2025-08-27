//! Utility functions for streaming operations

use crate::error::{OpenAIError, Result};
use crate::models::responses::{
    ResponseChoice, ResponseOutput, ResponseResult, StreamChunk, Usage,
};
use serde::Serialize;

use super::types::{ResponseStream, StreamEventType};

/// Streaming request helper to consolidate JSON streaming setup patterns
///
/// This helper function takes any serializable request and converts it to a JSON value
/// with the "stream": true field added, reducing code duplication across streaming APIs.
///
/// # Arguments
///
/// * `request` - Any type that implements `Serialize`
///
/// # Returns
///
/// * `Result<serde_json::Value>` - The JSON representation with streaming enabled
///
/// # Examples
///
/// ```rust
/// # use serde::Serialize;
/// # use openai_rust_sdk::api::streaming::utilities::to_streaming_json;
/// #
/// #[derive(Serialize)]
/// struct MyRequest {
///     assistant_id: String,
///     instructions: String,
/// }
///
/// # fn main() -> Result<(), openai_rust_sdk::error::OpenAIError> {
/// let request = MyRequest {
///     assistant_id: "asst_123".to_string(),
///     instructions: "Help me".to_string(),
/// };
///
/// let streaming_json = to_streaming_json(&request)?;
/// assert_eq!(streaming_json["stream"], serde_json::Value::Bool(true));
/// # Ok(())
/// # }
/// ```
pub fn to_streaming_json<T: Serialize>(request: &T) -> Result<serde_json::Value> {
    let mut request_json = serde_json::to_value(request).map_err(OpenAIError::Json)?;
    request_json["stream"] = serde_json::Value::Bool(true);
    Ok(request_json)
}

/// Process individual stream events
pub fn process_stream_event(
    event_result: std::result::Result<
        eventsource_stream::Event,
        eventsource_stream::EventStreamError<reqwest::Error>,
    >,
) -> Option<Result<StreamChunk>> {
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
