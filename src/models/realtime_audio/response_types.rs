//! Response and error types for real-time audio API
//!
//! Contains data structures for API responses, error handling,
//! session information, and usage statistics.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

use super::conversation_types::ConversationItem;
use super::session_config::{
    InputAudioTranscriptionConfig, RealtimeAudioFormat, RealtimeModality, RealtimeTool,
    RealtimeVoice, TurnDetectionConfig,
};

/// Error information for real-time events
#[derive(Debug, Clone, Ser, De)]
pub struct RealtimeError {
    /// Error type
    #[serde(rename = "type")]
    pub error_type: String,

    /// Error code
    pub code: Option<String>,

    /// Error message
    pub message: String,

    /// Parameter that caused the error
    pub param: Option<String>,

    /// Event ID that caused the error
    pub event_id: Option<String>,
}

/// Session information
#[derive(Debug, Clone, Ser, De)]
pub struct RealtimeSessionInfo {
    /// Session ID
    pub id: String,

    /// Object type
    pub object: String,

    /// Model being used
    pub model: String,

    /// Session modalities
    pub modalities: Vec<RealtimeModality>,

    /// Session instructions
    pub instructions: String,

    /// Voice configuration
    pub voice: RealtimeVoice,

    /// Input audio format
    pub input_audio_format: RealtimeAudioFormat,

    /// Output audio format
    pub output_audio_format: RealtimeAudioFormat,

    /// Input audio transcription configuration
    pub input_audio_transcription: Option<InputAudioTranscriptionConfig>,

    /// Turn detection configuration
    pub turn_detection: TurnDetectionConfig,

    /// Tools available in the session
    pub tools: Vec<RealtimeTool>,

    /// Tool choice configuration
    pub tool_choice: String,

    /// Temperature setting
    pub temperature: f32,

    /// Maximum response output tokens
    pub max_response_output_tokens: Option<u32>,
}

/// Conversation information
#[derive(Debug, Clone, Ser, De)]
pub struct ConversationInfo {
    /// Conversation ID
    pub id: String,

    /// Object type
    pub object: String,
}

/// Response information
#[derive(Debug, Clone, Ser, De)]
pub struct ResponseInfo {
    /// Response ID
    pub id: String,

    /// Object type
    pub object: String,

    /// Response status
    pub status: ResponseStatus,

    /// Status details
    pub status_details: Option<ResponseStatusDetails>,

    /// Output items
    pub output: Vec<ConversationItem>,

    /// Usage information
    pub usage: Option<ResponseUsage>,
}

/// Response status
#[derive(Debug, Clone, Ser, De)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    /// Response is in progress
    InProgress,
    /// Response is completed
    Completed,
    /// Response was cancelled
    Cancelled,
    /// Response failed
    Failed,
    /// Response is incomplete
    Incomplete,
}

/// Response status details
#[derive(Debug, Clone, Ser, De)]
pub struct ResponseStatusDetails {
    /// Reason for the status
    #[serde(rename = "type")]
    pub status_type: String,

    /// Reason description
    pub reason: Option<String>,
}

/// Response usage information
#[derive(Debug, Clone, Ser, De)]
pub struct ResponseUsage {
    /// Total tokens used
    pub total_tokens: u32,

    /// Input tokens used
    pub input_tokens: u32,

    /// Output tokens used
    pub output_tokens: u32,

    /// Input token details
    pub input_token_details: Option<TokenDetails>,

    /// Output token details
    pub output_token_details: Option<TokenDetails>,
}

/// Token usage details
#[derive(Debug, Clone, Ser, De)]
pub struct TokenDetails {
    /// Cached tokens
    pub cached_tokens: Option<u32>,

    /// Text tokens
    pub text_tokens: Option<u32>,

    /// Audio tokens
    pub audio_tokens: Option<u32>,
}

impl RealtimeError {
    /// Create a new error
    #[must_use]
    pub fn new(error_type: String, message: String) -> Self {
        Self {
            error_type,
            code: None,
            message,
            param: None,
            event_id: None,
        }
    }

    /// Create a new error with all fields
    #[must_use]
    pub fn new_detailed(
        error_type: String,
        code: Option<String>,
        message: String,
        param: Option<String>,
        event_id: Option<String>,
    ) -> Self {
        Self {
            error_type,
            code,
            message,
            param,
            event_id,
        }
    }

    /// Check if this is a validation error
    #[must_use]
    pub fn is_validation_error(&self) -> bool {
        self.error_type == "invalid_request_error" || self.error_type == "validation_error"
    }

    /// Check if this is a rate limit error
    #[must_use]
    pub fn is_rate_limit_error(&self) -> bool {
        self.error_type == "rate_limit_exceeded"
    }

    /// Check if this is an authentication error
    #[must_use]
    pub fn is_auth_error(&self) -> bool {
        self.error_type == "authentication_error" || self.error_type == "permission_denied"
    }

    /// Check if this is a server error
    #[must_use]
    pub fn is_server_error(&self) -> bool {
        self.error_type == "internal_error" || self.error_type == "server_error"
    }
}

impl ResponseUsage {
    /// Create a new usage record
    #[must_use]
    pub fn new(input_tokens: u32, output_tokens: u32) -> Self {
        Self {
            total_tokens: input_tokens + output_tokens,
            input_tokens,
            output_tokens,
            input_token_details: None,
            output_token_details: None,
        }
    }

    /// Get the cost ratio (output tokens / input tokens)
    #[must_use]
    pub fn cost_ratio(&self) -> f64 {
        if self.input_tokens == 0 {
            return 0.0;
        }
        f64::from(self.output_tokens) / f64::from(self.input_tokens)
    }

    /// Get the percentage of tokens that were cached
    #[must_use]
    pub fn cache_hit_rate(&self) -> f64 {
        let cached_input = self
            .input_token_details
            .as_ref()
            .and_then(|d| d.cached_tokens)
            .unwrap_or(0);
        let cached_output = self
            .output_token_details
            .as_ref()
            .and_then(|d| d.cached_tokens)
            .unwrap_or(0);

        let total_cached = cached_input + cached_output;
        if self.total_tokens == 0 {
            return 0.0;
        }

        f64::from(total_cached) / f64::from(self.total_tokens)
    }

    /// Check if the response used primarily audio tokens
    #[must_use]
    pub fn is_audio_heavy(&self) -> bool {
        let input_audio = self
            .input_token_details
            .as_ref()
            .and_then(|d| d.audio_tokens)
            .unwrap_or(0);
        let output_audio = self
            .output_token_details
            .as_ref()
            .and_then(|d| d.audio_tokens)
            .unwrap_or(0);

        let total_audio = input_audio + output_audio;
        total_audio > self.total_tokens / 2
    }
}

impl ResponseStatus {
    /// Check if the response is complete
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Check if the response is in progress
    #[must_use]
    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    /// Check if the response failed or was cancelled
    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed | Self::Cancelled | Self::Incomplete)
    }
}

impl ConversationInfo {
    /// Create a new conversation info
    #[must_use]
    pub fn new(id: String) -> Self {
        Self {
            id,
            object: "realtime.conversation".to_string(),
        }
    }
}

impl ResponseInfo {
    /// Create a new response info
    #[must_use]
    pub fn new(id: String, status: ResponseStatus) -> Self {
        Self {
            id,
            object: "realtime.response".to_string(),
            status,
            status_details: None,
            output: Vec::new(),
            usage: None,
        }
    }

    /// Check if the response has any output
    #[must_use]
    pub fn has_output(&self) -> bool {
        !self.output.is_empty()
    }

    /// Get the number of output items
    #[must_use]
    pub fn output_count(&self) -> usize {
        self.output.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_realtime_error_creation() {
        let error = RealtimeError::new(
            "invalid_request_error".to_string(),
            "Invalid audio format".to_string(),
        );

        assert_eq!(error.error_type, "invalid_request_error");
        assert_eq!(error.message, "Invalid audio format");
        assert!(error.is_validation_error());
        assert!(!error.is_rate_limit_error());
    }

    #[test]
    fn test_realtime_error_detailed() {
        let error = RealtimeError::new_detailed(
            "rate_limit_exceeded".to_string(),
            Some("RATE_LIMIT".to_string()),
            "Too many requests".to_string(),
            Some("audio_format".to_string()),
            Some("event-123".to_string()),
        );

        assert_eq!(error.error_type, "rate_limit_exceeded");
        assert_eq!(error.code, Some("RATE_LIMIT".to_string()));
        assert!(error.is_rate_limit_error());
        assert!(!error.is_validation_error());
    }

    #[test]
    fn test_response_usage_calculations() {
        let usage = ResponseUsage::new(100, 50);

        assert_eq!(usage.total_tokens, 150);
        assert_eq!(usage.input_tokens, 100);
        assert_eq!(usage.output_tokens, 50);
        assert_eq!(usage.cost_ratio(), 0.5);
    }

    #[test]
    fn test_response_usage_with_cache() {
        let mut usage = ResponseUsage::new(100, 50);
        usage.input_token_details = Some(TokenDetails {
            cached_tokens: Some(30),
            text_tokens: Some(70),
            audio_tokens: None,
        });

        assert_eq!(usage.cache_hit_rate(), 0.2); // 30 cached / 150 total
    }

    #[test]
    fn test_response_usage_audio_heavy() {
        let mut usage = ResponseUsage::new(100, 50);
        usage.input_token_details = Some(TokenDetails {
            cached_tokens: None,
            text_tokens: Some(20),
            audio_tokens: Some(80),
        });
        usage.output_token_details = Some(TokenDetails {
            cached_tokens: None,
            text_tokens: Some(10),
            audio_tokens: Some(40),
        });

        assert!(usage.is_audio_heavy()); // 120 audio tokens > 75 (half of 150)
    }

    #[test]
    fn test_response_status_checks() {
        assert!(ResponseStatus::Completed.is_complete());
        assert!(!ResponseStatus::InProgress.is_complete());

        assert!(ResponseStatus::InProgress.is_in_progress());
        assert!(!ResponseStatus::Completed.is_in_progress());

        assert!(ResponseStatus::Failed.is_failed());
        assert!(ResponseStatus::Cancelled.is_failed());
        assert!(!ResponseStatus::Completed.is_failed());
    }

    #[test]
    fn test_conversation_info_creation() {
        let info = ConversationInfo::new("conv-123".to_string());

        assert_eq!(info.id, "conv-123");
        assert_eq!(info.object, "realtime.conversation");
    }

    #[test]
    fn test_response_info_creation() {
        let info = ResponseInfo::new("resp-123".to_string(), ResponseStatus::Completed);

        assert_eq!(info.id, "resp-123");
        assert_eq!(info.object, "realtime.response");
        assert!(info.status.is_complete());
        assert!(!info.has_output());
        assert_eq!(info.output_count(), 0);
    }

    #[test]
    fn test_error_type_classification() {
        let auth_error = RealtimeError::new(
            "authentication_error".to_string(),
            "Invalid key".to_string(),
        );
        let server_error =
            RealtimeError::new("internal_error".to_string(), "Server error".to_string());

        assert!(auth_error.is_auth_error());
        assert!(!auth_error.is_server_error());

        assert!(server_error.is_server_error());
        assert!(!server_error.is_auth_error());
    }

    #[test]
    fn test_response_status_serialization() {
        assert_eq!(
            serde_json::to_string(&ResponseStatus::InProgress).unwrap(),
            "\"inprogress\""
        );
        assert_eq!(
            serde_json::to_string(&ResponseStatus::Completed).unwrap(),
            "\"completed\""
        );
        assert_eq!(
            serde_json::to_string(&ResponseStatus::Failed).unwrap(),
            "\"failed\""
        );
    }
}
