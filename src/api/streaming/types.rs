//! Stream types and event definitions for the OpenAI Streaming API

use crate::error::Result;
use crate::models::functions::FunctionCall;
use crate::models::responses::{ResponseResult, StreamChunk};
use futures::Stream;
use std::pin::Pin;

/// Streaming response from `OpenAI` API
pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>;

/// Streaming response with function calling support
pub type FunctionStream = Pin<Box<dyn Stream<Item = Result<FunctionStreamEvent>> + Send>>;

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

/// Builder for accumulating function call deltas
#[derive(Debug, Clone)]
pub(crate) struct FunctionCallBuilder {
    /// ID of the function call
    pub call_id: Option<String>,
    /// Name of the function being called
    pub name: Option<String>,
    /// JSON arguments for the function call
    pub arguments: String,
}

impl FunctionCallBuilder {
    /// Create a new function call builder
    pub fn new() -> Self {
        Self {
            call_id: None,
            name: None,
            arguments: String::new(),
        }
    }

    /// Build the final function call if complete
    pub fn build(self) -> Option<FunctionCall> {
        match (self.call_id, self.name) {
            (Some(call_id), Some(name)) => Some(FunctionCall::new(call_id, name, self.arguments)),
            _ => None,
        }
    }
}

impl Default for FunctionCallBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Processing state for function streams
#[derive(Debug, Clone)]
pub enum StreamProcessingState {
    /// Processing active stream chunks
    Processing,
    /// Stream has completed or encountered error
    Completed,
}
