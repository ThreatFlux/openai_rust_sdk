//! Function stream processing logic for handling tool calls and function execution

use crate::error::Result;
use crate::models::functions::FunctionCall;
use crate::models::responses::{
    ResponseChoice, ResponseOutput, ResponseResult, StreamChunk, Usage,
};
use futures::StreamExt as FuturesStreamExt;
use std::collections::HashMap;

use super::function_state::FunctionStreamState;
use super::types::{FunctionCallBuilder, FunctionStream, FunctionStreamEvent, ResponseStream};

/// Function stream processor that accumulates function calls
pub struct FunctionStreamProcessor {
    /// Raw stream
    stream: ResponseStream,
    /// Accumulated function calls by index
    function_calls: HashMap<u32, FunctionCallBuilder>,
}

impl FunctionStreamProcessor {
    /// Get mutable reference to function calls
    pub(crate) fn function_calls_mut(&mut self) -> &mut HashMap<u32, FunctionCallBuilder> {
        &mut self.function_calls
    }
    /// Create a new function stream from a response stream
    #[must_use]
    pub fn into_function_stream(stream: ResponseStream) -> FunctionStream {
        let processor = Self {
            stream,
            function_calls: HashMap::new(),
        };

        Box::pin(async_stream::stream! {
            let mut processor = processor;
            let mut stream_state = FunctionStreamState::new();

            while let Some(chunk_result) = FuturesStreamExt::next(&mut processor.stream).await {
                if let Some(events) = stream_state.process_chunk(&mut processor, chunk_result) {
                    for event in events {
                        yield Ok(event);
                    }
                } else {
                    break;
                }
            }
        })
    }

    /// Create a function stream from a response stream
    #[must_use]
    pub fn from_response_stream(stream: ResponseStream) -> FunctionStream {
        Self::into_function_stream(stream)
    }

    /// Process content delta from choice
    pub(crate) fn process_content_delta(
        choice: &crate::models::responses::StreamChoice,
    ) -> Option<Vec<FunctionStreamEvent>> {
        if let Some(content) = &choice.delta.content {
            Some(vec![FunctionStreamEvent::ContentDelta {
                content: content.clone(),
            }])
        } else {
            None
        }
    }

    /// Process tool calls from choice
    pub(crate) fn process_choice_tool_calls(
        function_calls: &mut HashMap<u32, FunctionCallBuilder>,
        choice: &crate::models::responses::StreamChoice,
    ) -> Option<Vec<FunctionStreamEvent>> {
        choice
            .delta
            .tool_calls
            .as_ref()
            .map(|tool_calls| Self::process_tool_calls(function_calls, tool_calls.clone()))
    }

    /// Process choice completion
    pub(crate) fn process_choice_completion(
        function_calls: &mut HashMap<u32, FunctionCallBuilder>,
        chunk: &StreamChunk,
        choice: &crate::models::responses::StreamChoice,
    ) -> Option<Vec<FunctionStreamEvent>> {
        if choice.finish_reason.is_some() {
            Some(Self::handle_completion(function_calls, chunk, choice))
        } else {
            None
        }
    }

    /// Process tool calls from stream delta
    fn process_tool_calls(
        function_calls: &mut HashMap<u32, FunctionCallBuilder>,
        tool_calls: Vec<crate::models::responses::ToolCallDelta>,
    ) -> Vec<FunctionStreamEvent> {
        let mut events = Vec::new();

        for tool_call in tool_calls {
            let builder = function_calls
                .entry(tool_call.index)
                .or_insert_with(FunctionCallBuilder::default);

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
        function_calls: &mut HashMap<u32, FunctionCallBuilder>,
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
