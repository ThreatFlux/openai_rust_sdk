use crate::error::Result;
use crate::models::functions::{FunctionCall, FunctionCallOutput};
use crate::models::responses::ResponseRequest;
use serde_json::{json, Value};

use super::client::{FunctionCallEvent, FunctionsApi};

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

impl FunctionsApi {
    /// Update conversation state with function calls
    pub(crate) fn update_conversation_state(&mut self, result: &FunctionResponseResult) {
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
    pub(crate) fn add_function_results_to_request(
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
    pub(crate) fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}
