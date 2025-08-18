//! # Real-time Audio Models
//!
//! Data structures for OpenAI's real-time audio API with WebRTC support,
//! including session management, events, and audio streaming.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Request for creating a real-time audio session
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceActivityDetectionConfig {
    /// Threshold for voice activity detection
    pub threshold: f32,

    /// Prefix padding in milliseconds
    pub prefix_padding_ms: u32,

    /// Silence duration to detect end of speech
    pub silence_duration_ms: u32,
}

/// Turn detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TurnDetectionType {
    /// Server-side voice activity detection
    ServerVad,
    /// No turn detection
    None,
}

/// Modalities supported in real-time sessions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeModality {
    /// Text modality
    Text,
    /// Audio modality
    Audio,
}

/// Tool definition for real-time sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Real-time event types
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Conversation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationItem {
    /// Unique item identifier
    pub id: Option<String>,

    /// Object type
    pub object: String,

    /// Item type
    #[serde(rename = "type")]
    pub item_type: ConversationItemType,

    /// Item status
    pub status: ConversationItemStatus,

    /// Item role
    pub role: ConversationRole,

    /// Item content
    pub content: Vec<ContentPart>,
}

/// Types of conversation items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationItemType {
    /// Message item
    Message,
    /// Function call item
    FunctionCall,
    /// Function call output item
    FunctionCallOutput,
}

/// Status of conversation items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationItemStatus {
    /// Item is completed
    Completed,
    /// Item is in progress
    InProgress,
    /// Item is incomplete
    Incomplete,
}

/// Roles in conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConversationRole {
    /// User role
    User,
    /// Assistant role
    Assistant,
    /// System role
    System,
}

/// Content parts of conversation items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    /// Text content
    #[serde(rename = "text")]
    Text {
        /// Text content
        text: String,
    },

    /// Audio content
    #[serde(rename = "audio")]
    Audio {
        /// Base64-encoded audio data
        audio: Option<String>,
        /// Audio transcript
        transcript: Option<String>,
    },

    /// Input audio content
    #[serde(rename = "input_audio")]
    InputAudio {
        /// Base64-encoded audio data
        audio: Option<String>,
        /// Audio transcript
        transcript: Option<String>,
    },

    /// Function call content
    #[serde(rename = "function_call")]
    FunctionCall {
        /// Call ID
        call_id: String,
        /// Function name
        name: String,
        /// Function arguments (JSON string)
        arguments: String,
    },

    /// Function call output content
    #[serde(rename = "function_call_output")]
    FunctionCallOutput {
        /// Call ID
        call_id: String,
        /// Function output
        output: String,
    },
}

/// Response configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Error information for real-time events
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationInfo {
    /// Conversation ID
    pub id: String,

    /// Object type
    pub object: String,
}

/// Response information
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStatusDetails {
    /// Reason for the status
    #[serde(rename = "type")]
    pub status_type: String,

    /// Reason description
    pub reason: Option<String>,
}

/// Response usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDetails {
    /// Cached tokens
    pub cached_tokens: Option<u32>,

    /// Text tokens
    pub text_tokens: Option<u32>,

    /// Audio tokens
    pub audio_tokens: Option<u32>,
}

/// Input audio transcription configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputAudioTranscriptionConfig {
    /// Whether transcription is enabled
    pub enabled: bool,

    /// Model to use for transcription
    pub model: String,
}

/// WebRTC connection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebRtcConnectionState {
    /// Connection is new
    New,
    /// Connection is connecting
    Connecting,
    /// Connection is connected
    Connected,
    /// Connection is disconnected
    Disconnected,
    /// Connection failed
    Failed,
    /// Connection is closed
    Closed,
}

/// WebRTC peer connection statistics
#[derive(Debug, Clone)]
pub struct WebRtcStats {
    /// Connection state
    pub connection_state: WebRtcConnectionState,

    /// Audio bytes sent
    pub audio_bytes_sent: u64,

    /// Audio bytes received
    pub audio_bytes_received: u64,

    /// Audio packets sent
    pub audio_packets_sent: u64,

    /// Audio packets received
    pub audio_packets_received: u64,

    /// Audio packets lost
    pub audio_packets_lost: u64,

    /// Round trip time in milliseconds
    pub round_trip_time_ms: Option<f64>,

    /// Jitter in milliseconds
    pub jitter_ms: Option<f64>,

    /// Connected timestamp
    pub connected_at: Option<DateTime<Utc>>,
}

/// Voice activity detection result
#[derive(Debug, Clone)]
pub struct VoiceActivityResult {
    /// Whether voice activity is detected
    pub is_speech: bool,

    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,

    /// Audio energy level
    pub energy: f32,

    /// Timestamp of the detection
    pub timestamp: DateTime<Utc>,
}

/// Audio buffer for real-time processing
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    /// Audio samples (PCM 16-bit)
    pub samples: Vec<i16>,

    /// Sample rate in Hz
    pub sample_rate: u32,

    /// Number of channels
    pub channels: u16,

    /// Timestamp when buffer was created
    pub timestamp: DateTime<Utc>,
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
            voice_activity_detection: Some(VoiceActivityDetectionConfig {
                threshold: 0.5,
                prefix_padding_ms: 300,
                silence_duration_ms: 200,
            }),
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

impl Default for WebRtcStats {
    fn default() -> Self {
        Self {
            connection_state: WebRtcConnectionState::New,
            audio_bytes_sent: 0,
            audio_bytes_received: 0,
            audio_packets_sent: 0,
            audio_packets_received: 0,
            audio_packets_lost: 0,
            round_trip_time_ms: None,
            jitter_ms: None,
            connected_at: None,
        }
    }
}

impl AudioBuffer {
    /// Create a new audio buffer
    #[must_use]
    pub fn new(samples: Vec<i16>, sample_rate: u32, channels: u16) -> Self {
        Self {
            samples,
            sample_rate,
            channels,
            timestamp: Utc::now(),
        }
    }

    /// Get the duration of the audio buffer in seconds
    #[must_use]
    pub fn duration_seconds(&self) -> f64 {
        self.samples.len() as f64 / (f64::from(self.sample_rate) * f64::from(self.channels))
    }

    /// Get the number of frames in the buffer
    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.samples.len() / self.channels as usize
    }

    /// Convert to mono if stereo
    #[must_use]
    pub fn to_mono(&self) -> Vec<i16> {
        if self.channels == 1 {
            self.samples.clone()
        } else {
            self.samples
                .chunks(self.channels as usize)
                .map(|frame| {
                    let sum: i32 = frame.iter().map(|&s| i32::from(s)).sum();
                    (sum / frame.len() as i32) as i16
                })
                .collect()
        }
    }

    /// Get RMS (Root Mean Square) energy level
    #[must_use]
    pub fn rms_energy(&self) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }

        let sum_squares: f64 = self.samples.iter().map(|&s| f64::from(s).powi(2)).sum();

        (sum_squares / self.samples.len() as f64).sqrt() as f32
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
    fn test_audio_buffer_operations() {
        let samples = vec![100, -200, 300, -400];
        let buffer = AudioBuffer::new(samples, 44100, 2);

        assert_eq!(buffer.frame_count(), 2);
        assert!(buffer.duration_seconds() > 0.0);

        let mono = buffer.to_mono();
        assert_eq!(mono.len(), 2);
        assert_eq!(mono[0], -50); // (100 + (-200)) / 2
        assert_eq!(mono[1], -50); // (300 + (-400)) / 2

        let energy = buffer.rms_energy();
        assert!(energy > 0.0);
    }

    #[test]
    fn test_voice_activity_detection_config() {
        let config = VoiceActivityDetectionConfig::default();
        assert_eq!(config.threshold, 0.5);
        assert_eq!(config.prefix_padding_ms, 300);
        assert_eq!(config.silence_duration_ms, 200);
    }

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
    fn test_conversation_item_creation() {
        let item = ConversationItem {
            id: Some("item-123".to_string()),
            object: "realtime.item".to_string(),
            item_type: ConversationItemType::Message,
            status: ConversationItemStatus::Completed,
            role: ConversationRole::User,
            content: vec![ContentPart::Text {
                text: "Hello, world!".to_string(),
            }],
        };

        assert_eq!(item.id, Some("item-123".to_string()));
        assert_eq!(item.role, ConversationRole::User);
        assert_eq!(item.content.len(), 1);
    }

    #[test]
    fn test_webrtc_stats_default() {
        let stats = WebRtcStats::default();
        assert_eq!(stats.connection_state, WebRtcConnectionState::New);
        assert_eq!(stats.audio_bytes_sent, 0);
        assert_eq!(stats.audio_packets_lost, 0);
        assert!(stats.connected_at.is_none());
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
}
