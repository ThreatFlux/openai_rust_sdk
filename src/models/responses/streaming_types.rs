use crate::models::functions::FunctionCall;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

use super::{
    message_types::MessageRole,
    response_types::{ResponseOutput, ResponseResult},
};

/// Different types of streaming events
#[derive(Debug, Clone, Ser, De)]
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
#[derive(Debug, Clone, Ser, De)]
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
#[derive(Debug, Clone, Ser, De)]
pub struct StreamChoice {
    /// Index of this choice in the choices array
    pub index: u32,
    /// The incremental content delta
    pub delta: StreamDelta,
    /// Reason why the generation finished (if complete)
    pub finish_reason: Option<String>,
}

/// Delta content in streaming chunk
#[derive(Debug, Clone, Ser, De)]
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
#[derive(Debug, Clone, Ser, De)]
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
#[derive(Debug, Clone, Ser, De)]
pub struct FunctionCallDelta {
    /// Function name (if starting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Arguments delta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}
