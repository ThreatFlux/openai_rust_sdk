//! Helper functions for streaming API to reduce complexity
#![allow(dead_code)]

use crate::api::streaming::FunctionStreamEvent;
use crate::models::functions::FunctionCall;
use crate::models::responses::ToolCallDelta;
use std::collections::HashMap;

/// Builder for accumulating function call deltas
#[derive(Debug, Clone)]
pub struct FunctionCallBuilder {
    /// ID of the function call
    pub call_id: Option<String>,
    /// Name of the function being called
    pub name: Option<String>,
    /// JSON arguments for the function call
    pub arguments: String,
}

/// Process a single tool call delta and emit events
pub async fn process_tool_call_delta(
    tool_call: ToolCallDelta,
    function_calls: &mut HashMap<u32, FunctionCallBuilder>,
) -> Vec<FunctionStreamEvent> {
    let mut events = Vec::new();

    let builder = function_calls
        .entry(tool_call.index)
        .or_insert_with(|| FunctionCallBuilder {
            call_id: None,
            name: None,
            arguments: String::new(),
        });

    // Update call ID
    if let Some(id) = tool_call.id {
        builder.call_id = Some(id);
    }

    // Process function details
    if let Some(function) = tool_call.function {
        if let Some(name) = function.name {
            builder.name = Some(name.clone());

            // Emit function call started event
            if let Some(call_id) = &builder.call_id {
                events.push(FunctionStreamEvent::FunctionCallStarted {
                    call_id: call_id.clone(),
                    function_name: name,
                });
            }
        }

        if let Some(args_delta) = function.arguments {
            builder.arguments.push_str(&args_delta);

            // Emit arguments delta event
            if let Some(call_id) = &builder.call_id {
                events.push(FunctionStreamEvent::FunctionCallArgumentsDelta {
                    call_id: call_id.clone(),
                    arguments_delta: args_delta,
                });
            }
        }
    }

    events
}

/// Build a function call from builder if complete
impl FunctionCallBuilder {
    pub fn build(self) -> Option<FunctionCall> {
        match (self.call_id, self.name) {
            (Some(call_id), Some(name)) => Some(FunctionCall {
                call_id,
                name,
                arguments: self.arguments,
            }),
            _ => None,
        }
    }
}

/// Process completed function calls
pub fn process_completed_calls(
    function_calls: &mut HashMap<u32, FunctionCallBuilder>,
) -> Vec<FunctionStreamEvent> {
    let mut events = Vec::new();

    for (_, builder) in function_calls.drain() {
        if let Some(call) = builder.build() {
            events.push(FunctionStreamEvent::FunctionCallCompleted { call });
        }
    }

    events
}
