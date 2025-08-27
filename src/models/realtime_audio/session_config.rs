//! Session configuration types and enums for real-time audio
//!
//! Contains data structures for configuring real-time audio sessions,
//! including audio formats, voice options, detection settings, and tools.

use crate::{De, Ser};
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};

/// Request for creating a real-time audio session
#[derive(Debug, Clone, Ser, De)]
pub struct RealtimeSessionRequest {
    /// The model to use for real-time audio
    pub model: String,

    /// Configuration for the audio session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<RealtimeSessionConfig>,

    /// Instructions for the AI assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Voice to use for responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,

    /// Temperature for response generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Maximum response tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<u32>,
}

/// Configuration for real-time audio session
#[derive(Debug, Clone, Ser, De)]
pub struct RealtimeSessionConfig {
    /// Audio input configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<RealtimeAudioFormat>,

    /// Audio output configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<RealtimeAudioFormat>,

    /// Voice activity detection settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_activity_detection: Option<VoiceActivityDetectionConfig>,

    /// Turn detection settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<TurnDetectionConfig>,

    /// Tools available to the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RealtimeTool>>,

    /// Tool choice configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    /// Modalities supported in the session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<RealtimeModality>>,
}

/// Audio formats supported for real-time streaming
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeAudioFormat {
    /// Raw PCM 16-bit 24kHz mono
    #[serde(rename = "pcm16")]
    Pcm16,
    /// G.711 µ-law encoding
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    /// G.711 A-law encoding
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}

/// Voice options for real-time audio responses
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeVoice {
    /// Alloy voice - balanced and natural
    Alloy,
    /// Echo voice - deep and resonant
    Echo,
    /// Fable voice - expressive and storytelling
    Fable,
    /// Onyx voice - authoritative and deep
    Onyx,
    /// Nova voice - bright and energetic
    Nova,
    /// Shimmer voice - warm and friendly
    Shimmer,
}

/// Voice activity detection configuration
#[derive(Debug, Clone, Ser, De)]
pub struct VoiceActivityDetectionConfig {
    /// Threshold for voice activity detection
    pub threshold: f32,

    /// Prefix padding in milliseconds
    pub prefix_padding_ms: u32,

    /// Silence duration to detect end of speech
    pub silence_duration_ms: u32,
}

/// Turn detection configuration
#[derive(Debug, Clone, Ser, De)]
pub struct TurnDetectionConfig {
    /// Type of turn detection
    #[serde(rename = "type")]
    pub detection_type: TurnDetectionType,

    /// Threshold for turn detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f32>,

    /// Prefix padding in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_padding_ms: Option<u32>,

    /// Silence duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silence_duration_ms: Option<u32>,
}

/// Types of turn detection
#[derive(Debug, Clone, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum TurnDetectionType {
    /// Server-side voice activity detection
    ServerVad,
    /// No turn detection
    None,
}

/// Modalities supported in real-time sessions
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeModality {
    /// Text modality
    Text,
    /// Audio modality
    Audio,
}

/// Tool definition for real-time sessions
#[derive(Debug, Clone, Ser, De)]
pub struct RealtimeTool {
    /// Type of the tool
    #[serde(rename = "type")]
    pub tool_type: String,

    /// Name of the function
    pub name: String,

    /// Description of the function
    pub description: String,

    /// Parameters schema
    pub parameters: serde_json::Value,
}

/// Response from session creation
#[derive(Debug, Clone, Ser, De)]
pub struct RealtimeSessionResponse {
    /// Unique session identifier
    pub id: String,

    /// Session object type
    pub object: String,

    /// Current session status
    pub status: SessionStatus,

    /// Ephemeral API key for WebRTC connection
    pub ephemeral_key: String,

    /// WebRTC connection URL
    pub webrtc_url: String,

    /// Session configuration
    pub config: RealtimeSessionConfig,

    /// Expiration time of the session
    pub expires_at: DateTime<Utc>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Session status
#[derive(Debug, Clone, Ser, De)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    /// Session is active and ready
    Active,
    /// Session is connecting
    Connecting,
    /// Session is disconnected
    Disconnected,
    /// Session has expired
    Expired,
    /// Session encountered an error
    Error,
}

/// Response configuration
#[derive(Debug, Clone, Ser, De)]
pub struct ResponseConfig {
    /// Modalities for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<RealtimeModality>>,

    /// Instructions for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Voice to use for audio responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,

    /// Output audio format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<RealtimeAudioFormat>,

    /// Tools available for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RealtimeTool>>,

    /// Tool choice configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    /// Temperature for response generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Maximum response output tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<u32>,
}

/// Input audio transcription configuration
#[derive(Debug, Clone, Ser, De)]
pub struct InputAudioTranscriptionConfig {
    /// Whether transcription is enabled
    pub enabled: bool,

    /// Model to use for transcription
    pub model: String,
}

/// Real-time audio models
pub struct RealtimeAudioModels;

impl RealtimeAudioModels {
    /// GPT-4o real-time preview model
    pub const GPT_4O_REALTIME_PREVIEW: &'static str = "gpt-4o-realtime-preview";

    /// GPT-4o mini real-time preview model
    pub const GPT_4O_MINI_REALTIME_PREVIEW: &'static str = "gpt-4o-mini-realtime-preview";
}

impl Default for RealtimeSessionConfig {
    fn default() -> Self {
        Self {
            input_audio_format: Some(RealtimeAudioFormat::Pcm16),
            output_audio_format: Some(RealtimeAudioFormat::Pcm16),
            voice_activity_detection: Some(VoiceActivityDetectionConfig::default()),
            turn_detection: Some(TurnDetectionConfig {
                detection_type: TurnDetectionType::ServerVad,
                threshold: Some(0.5),
                prefix_padding_ms: Some(300),
                silence_duration_ms: Some(200),
            }),
            tools: None,
            tool_choice: Some("auto".to_string()),
            modalities: Some(vec![RealtimeModality::Text, RealtimeModality::Audio]),
        }
    }
}

impl Default for VoiceActivityDetectionConfig {
    fn default() -> Self {
        Self {
            threshold: 0.5,
            prefix_padding_ms: 300,
            silence_duration_ms: 200,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_request_creation() {
        let request = RealtimeSessionRequest {
            model: RealtimeAudioModels::GPT_4O_REALTIME_PREVIEW.to_string(),
            config: Some(RealtimeSessionConfig::default()),
            instructions: Some("You are a helpful assistant.".to_string()),
            voice: Some(RealtimeVoice::Alloy),
            temperature: Some(0.8),
            max_response_output_tokens: Some(4096),
        };

        assert_eq!(request.model, "gpt-4o-realtime-preview");
        assert!(request.config.is_some());
        assert_eq!(request.voice, Some(RealtimeVoice::Alloy));
    }

    #[test]
    fn test_voice_activity_detection_config() {
        let config = VoiceActivityDetectionConfig::default();
        assert_eq!(config.threshold, 0.5);
        assert_eq!(config.prefix_padding_ms, 300);
        assert_eq!(config.silence_duration_ms, 200);
    }

    #[test]
    fn test_audio_format_serialization() {
        assert_eq!(
            serde_json::to_string(&RealtimeAudioFormat::Pcm16).unwrap(),
            "\"pcm16\""
        );
        assert_eq!(
            serde_json::to_string(&RealtimeAudioFormat::G711Ulaw).unwrap(),
            "\"g711_ulaw\""
        );
    }

    #[test]
    fn test_voice_serialization() {
        assert_eq!(
            serde_json::to_string(&RealtimeVoice::Alloy).unwrap(),
            "\"alloy\""
        );
        assert_eq!(
            serde_json::to_string(&RealtimeVoice::Nova).unwrap(),
            "\"nova\""
        );
    }

    #[test]
    fn test_session_config_default() {
        let config = RealtimeSessionConfig::default();
        assert!(config.input_audio_format.is_some());
        assert!(config.output_audio_format.is_some());
        assert!(config.voice_activity_detection.is_some());
        assert!(config.turn_detection.is_some());
        assert_eq!(config.tool_choice, Some("auto".to_string()));
    }
}
