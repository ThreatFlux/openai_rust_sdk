//! Event types for real-time audio communication
//!
//! Contains all WebSocket event types used for bidirectional communication
//! in real-time audio sessions, including client events, server events,
//! and their associated data structures.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

use super::conversation_types::{ContentPart, ConversationItem};
use super::response_types::{ConversationInfo, RealtimeError, RealtimeSessionInfo, ResponseInfo};
use super::session_config::{RealtimeSessionConfig, ResponseConfig};

/// Real-time event types for WebSocket communication
#[derive(Debug, Clone, Ser, De)]
#[serde(tag = "type")]
pub enum RealtimeEvent {
    /// Session configuration update
    #[serde(rename = "session.update")]
    SessionUpdate {
        /// Event ID
        event_id: String,
        /// Session configuration
        session: RealtimeSessionConfig,
    },

    /// Input audio buffer append
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend {
        /// Event ID
        event_id: String,
        /// Base64-encoded audio data
        audio: String,
    },

    /// Input audio buffer commit
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit {
        /// Event ID
        event_id: String,
    },

    /// Input audio buffer clear
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear {
        /// Event ID
        event_id: String,
    },

    /// Conversation item create
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate {
        /// Event ID
        event_id: String,
        /// Previous item ID
        previous_item_id: Option<String>,
        /// Item to create
        item: ConversationItem,
    },

    /// Conversation item truncate
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate {
        /// Event ID
        event_id: String,
        /// Item ID to truncate
        item_id: String,
        /// Content index to truncate at
        content_index: u32,
        /// Audio end time in milliseconds
        audio_end_ms: u32,
    },

    /// Conversation item delete
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete {
        /// Event ID
        event_id: String,
        /// Item ID to delete
        item_id: String,
    },

    /// Response create
    #[serde(rename = "response.create")]
    ResponseCreate {
        /// Event ID
        event_id: String,
        /// Response configuration
        response: ResponseConfig,
    },

    /// Response cancel
    #[serde(rename = "response.cancel")]
    ResponseCancel {
        /// Event ID
        event_id: String,
    },

    /// Error event
    #[serde(rename = "error")]
    Error {
        /// Event ID
        event_id: String,
        /// Error details
        error: RealtimeError,
    },

    /// Session created event
    #[serde(rename = "session.created")]
    SessionCreated {
        /// Event ID
        event_id: String,
        /// Session information
        session: RealtimeSessionInfo,
    },

    /// Session updated event
    #[serde(rename = "session.updated")]
    SessionUpdated {
        /// Event ID
        event_id: String,
        /// Session information
        session: RealtimeSessionInfo,
    },

    /// Conversation created event
    #[serde(rename = "conversation.created")]
    ConversationCreated {
        /// Event ID
        event_id: String,
        /// Conversation information
        conversation: ConversationInfo,
    },

    /// Input audio buffer committed event
    #[serde(rename = "input_audio_buffer.committed")]
    InputAudioBufferCommitted {
        /// Event ID
        event_id: String,
        /// Previous item ID
        previous_item_id: Option<String>,
        /// Item ID that was created
        item_id: String,
    },

    /// Input audio buffer cleared event
    #[serde(rename = "input_audio_buffer.cleared")]
    InputAudioBufferCleared {
        /// Event ID
        event_id: String,
    },

    /// Input audio buffer speech started event
    #[serde(rename = "input_audio_buffer.speech_started")]
    InputAudioBufferSpeechStarted {
        /// Event ID
        event_id: String,
        /// Audio start time in milliseconds
        audio_start_ms: u32,
        /// Item ID
        item_id: String,
    },

    /// Input audio buffer speech stopped event
    #[serde(rename = "input_audio_buffer.speech_stopped")]
    InputAudioBufferSpeechStopped {
        /// Event ID
        event_id: String,
        /// Audio end time in milliseconds
        audio_end_ms: u32,
        /// Item ID
        item_id: String,
    },

    /// Conversation item created event
    #[serde(rename = "conversation.item.created")]
    ConversationItemCreated {
        /// Event ID
        event_id: String,
        /// Previous item ID
        previous_item_id: Option<String>,
        /// Created item
        item: ConversationItem,
    },

    /// Conversation item input audio transcription completed
    #[serde(rename = "conversation.item.input_audio_transcription.completed")]
    ConversationItemInputAudioTranscriptionCompleted {
        /// Event ID
        event_id: String,
        /// Item ID
        item_id: String,
        /// Content index
        content_index: u32,
        /// Transcribed text
        transcript: String,
    },

    /// Conversation item input audio transcription failed
    #[serde(rename = "conversation.item.input_audio_transcription.failed")]
    ConversationItemInputAudioTranscriptionFailed {
        /// Event ID
        event_id: String,
        /// Item ID
        item_id: String,
        /// Content index
        content_index: u32,
        /// Error details
        error: RealtimeError,
    },

    /// Conversation item truncated event
    #[serde(rename = "conversation.item.truncated")]
    ConversationItemTruncated {
        /// Event ID
        event_id: String,
        /// Item ID
        item_id: String,
        /// Content index
        content_index: u32,
        /// Audio end time in milliseconds
        audio_end_ms: u32,
    },

    /// Conversation item deleted event
    #[serde(rename = "conversation.item.deleted")]
    ConversationItemDeleted {
        /// Event ID
        event_id: String,
        /// Item ID
        item_id: String,
    },

    /// Response created event
    #[serde(rename = "response.created")]
    ResponseCreated {
        /// Event ID
        event_id: String,
        /// Response information
        response: ResponseInfo,
    },

    /// Response done event
    #[serde(rename = "response.done")]
    ResponseDone {
        /// Event ID
        event_id: String,
        /// Response information
        response: ResponseInfo,
    },

    /// Response output item added
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Output item index
        output_index: u32,
        /// Output item
        item: ConversationItem,
    },

    /// Response output item done
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Output item index
        output_index: u32,
        /// Output item
        item: ConversationItem,
    },

    /// Response content part added
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
        /// Content part
        part: ContentPart,
    },

    /// Response content part done
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
        /// Content part
        part: ContentPart,
    },

    /// Response text delta
    #[serde(rename = "response.text.delta")]
    ResponseTextDelta {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
        /// Text delta
        delta: String,
    },

    /// Response text done
    #[serde(rename = "response.text.done")]
    ResponseTextDone {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
        /// Complete text
        text: String,
    },

    /// Response audio transcript delta
    #[serde(rename = "response.audio_transcript.delta")]
    ResponseAudioTranscriptDelta {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
        /// Transcript delta
        delta: String,
    },

    /// Response audio transcript done
    #[serde(rename = "response.audio_transcript.done")]
    ResponseAudioTranscriptDone {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
        /// Complete transcript
        transcript: String,
    },

    /// Response audio delta
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDelta {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
        /// Base64-encoded audio delta
        delta: String,
    },

    /// Response audio done
    #[serde(rename = "response.audio.done")]
    ResponseAudioDone {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
    },

    /// Response function call arguments delta
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
        /// Call ID
        call_id: String,
        /// Arguments delta
        delta: String,
    },

    /// Response function call arguments done
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone {
        /// Event ID
        event_id: String,
        /// Response ID
        response_id: String,
        /// Item ID
        item_id: String,
        /// Output item index
        output_index: u32,
        /// Content part index
        content_index: u32,
        /// Call ID
        call_id: String,
        /// Complete arguments
        arguments: String,
    },
}

impl RealtimeEvent {
    /// Get the event ID from any event variant
    #[must_use]
    pub fn event_id(&self) -> &str {
        match self {
            Self::SessionUpdate { event_id, .. } => event_id,
            Self::InputAudioBufferAppend { event_id, .. } => event_id,
            Self::InputAudioBufferCommit { event_id, .. } => event_id,
            Self::InputAudioBufferClear { event_id, .. } => event_id,
            Self::ConversationItemCreate { event_id, .. } => event_id,
            Self::ConversationItemTruncate { event_id, .. } => event_id,
            Self::ConversationItemDelete { event_id, .. } => event_id,
            Self::ResponseCreate { event_id, .. } => event_id,
            Self::ResponseCancel { event_id, .. } => event_id,
            Self::Error { event_id, .. } => event_id,
            Self::SessionCreated { event_id, .. } => event_id,
            Self::SessionUpdated { event_id, .. } => event_id,
            Self::ConversationCreated { event_id, .. } => event_id,
            Self::InputAudioBufferCommitted { event_id, .. } => event_id,
            Self::InputAudioBufferCleared { event_id, .. } => event_id,
            Self::InputAudioBufferSpeechStarted { event_id, .. } => event_id,
            Self::InputAudioBufferSpeechStopped { event_id, .. } => event_id,
            Self::ConversationItemCreated { event_id, .. } => event_id,
            Self::ConversationItemInputAudioTranscriptionCompleted { event_id, .. } => event_id,
            Self::ConversationItemInputAudioTranscriptionFailed { event_id, .. } => event_id,
            Self::ConversationItemTruncated { event_id, .. } => event_id,
            Self::ConversationItemDeleted { event_id, .. } => event_id,
            Self::ResponseCreated { event_id, .. } => event_id,
            Self::ResponseDone { event_id, .. } => event_id,
            Self::ResponseOutputItemAdded { event_id, .. } => event_id,
            Self::ResponseOutputItemDone { event_id, .. } => event_id,
            Self::ResponseContentPartAdded { event_id, .. } => event_id,
            Self::ResponseContentPartDone { event_id, .. } => event_id,
            Self::ResponseTextDelta { event_id, .. } => event_id,
            Self::ResponseTextDone { event_id, .. } => event_id,
            Self::ResponseAudioTranscriptDelta { event_id, .. } => event_id,
            Self::ResponseAudioTranscriptDone { event_id, .. } => event_id,
            Self::ResponseAudioDelta { event_id, .. } => event_id,
            Self::ResponseAudioDone { event_id, .. } => event_id,
            Self::ResponseFunctionCallArgumentsDelta { event_id, .. } => event_id,
            Self::ResponseFunctionCallArgumentsDone { event_id, .. } => event_id,
        }
    }

    /// Check if this is a client-to-server event
    #[must_use]
    pub fn is_client_event(&self) -> bool {
        matches!(
            self,
            Self::SessionUpdate { .. }
                | Self::InputAudioBufferAppend { .. }
                | Self::InputAudioBufferCommit { .. }
                | Self::InputAudioBufferClear { .. }
                | Self::ConversationItemCreate { .. }
                | Self::ConversationItemTruncate { .. }
                | Self::ConversationItemDelete { .. }
                | Self::ResponseCreate { .. }
                | Self::ResponseCancel { .. }
        )
    }

    /// Check if this is a server-to-client event
    #[must_use]
    pub fn is_server_event(&self) -> bool {
        !self.is_client_event()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::realtime_audio::session_config::RealtimeSessionConfig;

    #[test]
    fn test_realtime_event_serialization() {
        let event = RealtimeEvent::SessionUpdate {
            event_id: "test-123".to_string(),
            session: RealtimeSessionConfig::default(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("session.update"));
        assert!(json.contains("test-123"));
    }

    #[test]
    fn test_event_id_extraction() {
        let event = RealtimeEvent::SessionUpdate {
            event_id: "test-123".to_string(),
            session: RealtimeSessionConfig::default(),
        };

        assert_eq!(event.event_id(), "test-123");
    }

    #[test]
    fn test_client_event_classification() {
        let client_event = RealtimeEvent::SessionUpdate {
            event_id: "test-123".to_string(),
            session: RealtimeSessionConfig::default(),
        };

        assert!(client_event.is_client_event());
        assert!(!client_event.is_server_event());
    }

    #[test]
    fn test_input_audio_buffer_append_event() {
        let event = RealtimeEvent::InputAudioBufferAppend {
            event_id: "audio-123".to_string(),
            audio: "base64-encoded-data".to_string(),
        };

        assert_eq!(event.event_id(), "audio-123");
        assert!(event.is_client_event());
    }

    #[test]
    fn test_response_create_event() {
        use crate::models::realtime_audio::session_config::ResponseConfig;

        let event = RealtimeEvent::ResponseCreate {
            event_id: "response-123".to_string(),
            response: ResponseConfig {
                modalities: None,
                instructions: None,
                voice: None,
                output_audio_format: None,
                tools: None,
                tool_choice: None,
                temperature: None,
                max_response_output_tokens: None,
            },
        };

        assert_eq!(event.event_id(), "response-123");
        assert!(event.is_client_event());
    }
}
