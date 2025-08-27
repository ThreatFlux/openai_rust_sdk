//! State management for function stream processing

use crate::error::Result;
use crate::models::responses::StreamChunk;

use super::processor::FunctionStreamProcessor;
use super::types::{FunctionStreamEvent, StreamProcessingState};

/// State machine for processing function stream events
pub(crate) struct FunctionStreamState {
    /// Current processing state
    state: StreamProcessingState,
}

impl FunctionStreamState {
    /// Create a new stream state
    pub fn new() -> Self {
        Self {
            state: StreamProcessingState::Processing,
        }
    }

    /// Process a stream chunk and return events
    pub fn process_chunk(
        &mut self,
        processor: &mut FunctionStreamProcessor,
        chunk_result: Result<StreamChunk>,
    ) -> Option<Vec<FunctionStreamEvent>> {
        match self.state {
            StreamProcessingState::Processing => self.handle_active_chunk(processor, chunk_result),
            StreamProcessingState::Completed => None,
        }
    }

    /// Handle chunk processing in active state
    fn handle_active_chunk(
        &mut self,
        processor: &mut FunctionStreamProcessor,
        chunk_result: Result<StreamChunk>,
    ) -> Option<Vec<FunctionStreamEvent>> {
        let chunk = self.handle_chunk_result(chunk_result)?;

        let events = self.process_chunk_choices(processor, &chunk);
        if self.should_complete(&chunk) {
            self.state = StreamProcessingState::Completed;
        }

        Some(events)
    }

    /// Handle chunk result and error cases
    fn handle_chunk_result(&mut self, chunk_result: Result<StreamChunk>) -> Option<StreamChunk> {
        chunk_result.ok().or_else(|| {
            self.state = StreamProcessingState::Completed;
            None
        })
    }

    /// Process all choices in a chunk
    fn process_chunk_choices(
        &self,
        processor: &mut FunctionStreamProcessor,
        chunk: &StreamChunk,
    ) -> Vec<FunctionStreamEvent> {
        let mut all_events = Vec::new();

        for choice in &chunk.choices {
            all_events.extend(self.process_single_choice(processor, chunk, choice));
        }

        all_events
    }

    /// Process a single choice and generate events
    fn process_single_choice(
        &self,
        processor: &mut FunctionStreamProcessor,
        chunk: &StreamChunk,
        choice: &crate::models::responses::StreamChoice,
    ) -> Vec<FunctionStreamEvent> {
        let mut events = Vec::new();

        events.extend(FunctionStreamProcessor::process_content_delta(choice).unwrap_or_default());
        events.extend(
            FunctionStreamProcessor::process_choice_tool_calls(
                processor.function_calls_mut(),
                choice,
            )
            .unwrap_or_default(),
        );

        if choice.finish_reason.is_some() {
            events.extend(
                FunctionStreamProcessor::process_choice_completion(
                    processor.function_calls_mut(),
                    chunk,
                    choice,
                )
                .unwrap_or_default(),
            );
        }

        events
    }

    /// Check if stream should complete based on chunk
    fn should_complete(&self, chunk: &StreamChunk) -> bool {
        chunk
            .choices
            .iter()
            .any(|choice| choice.finish_reason.is_some())
    }
}

impl Default for FunctionStreamState {
    fn default() -> Self {
        Self::new()
    }
}
