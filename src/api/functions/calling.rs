use crate::error::Result;
use crate::models::functions::{FunctionCall, FunctionCallOutput};
use crate::models::responses::ResponseRequest;
use serde_json::Value;

use super::client::{FunctionCallEvent, FunctionConfig, FunctionsApi};
use super::helpers::FunctionResponseResult;

impl FunctionsApi {
    /// Create a chat completion with function calling support
    pub async fn create_function_response(
        &mut self,
        request: &ResponseRequest,
        config: &FunctionConfig,
    ) -> Result<FunctionResponseResult> {
        let mut payload = self.build_payload(request)?;

        // Add function calling configuration
        self.add_function_config(&mut payload, config)?;

        let response = self.send_request(&payload).await?;
        let result = self.parse_function_response(response).await?;

        // Update conversation state with any function calls
        self.update_conversation_state(&result);

        Ok(result)
    }

    /// Submit function call results and continue the conversation
    pub async fn submit_function_results(
        &mut self,
        results: Vec<FunctionCallOutput>,
        original_request: &ResponseRequest,
        config: &FunctionConfig,
    ) -> Result<FunctionResponseResult> {
        // Mark function calls as completed
        for result in &results {
            self.conversation_state
                .completed_calls
                .insert(result.call_id.clone(), result.clone());

            self.conversation_state
                .call_history
                .push(FunctionCallEvent::CallCompleted {
                    timestamp: self.current_timestamp(),
                    call_id: result.call_id.clone(),
                    output: result.clone(),
                });
        }

        // Create a new request with function results
        let mut request_with_results = original_request.clone();
        self.add_function_results_to_request(&mut request_with_results, &results)?;

        // Make another API call with the results
        self.create_function_response(&request_with_results, config)
            .await
    }

    /// Get pending function calls that need to be executed
    #[must_use]
    pub fn get_pending_calls(&self) -> Vec<&FunctionCall> {
        self.conversation_state.pending_calls.values().collect()
    }

    /// Check if there are any pending function calls
    #[must_use]
    pub fn has_pending_calls(&self) -> bool {
        !self.conversation_state.pending_calls.is_empty()
    }

    /// Get the conversation history
    #[must_use]
    pub fn get_call_history(&self) -> &[FunctionCallEvent] {
        &self.conversation_state.call_history
    }

    /// Clear the conversation state
    pub fn clear_state(&mut self) {
        self.conversation_state = super::client::ConversationState::default();
    }
}
