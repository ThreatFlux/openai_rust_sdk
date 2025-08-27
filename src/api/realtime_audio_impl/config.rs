//! # Real-time Audio Configuration
//!
//! Configuration structures for the Real-time Audio API.

use crate::models::realtime_audio::VoiceActivityDetectionConfig;
use std::time::Duration;
use webrtc::ice_transport::ice_server::RTCIceServer;

/// Configuration for real-time audio API
#[derive(Debug, Clone)]
pub struct RealtimeAudioConfig {
    /// ICE servers for WebRTC connection
    pub ice_servers: Vec<RTCIceServer>,

    /// Audio sample rate (default: 24000 Hz)
    pub sample_rate: u32,

    /// Audio channels (default: 1 for mono)
    pub channels: u16,

    /// Audio buffer size in milliseconds (default: 20ms)
    pub buffer_size_ms: u32,

    /// Voice activity detection configuration
    pub vad_config: VoiceActivityDetectionConfig,

    /// Connection timeout in seconds
    pub connection_timeout: Duration,

    /// Reconnection attempts
    pub max_reconnect_attempts: u32,

    /// Enable automatic echo cancellation
    pub enable_aec: bool,

    /// Enable noise suppression
    pub enable_noise_suppression: bool,

    /// Enable automatic gain control
    pub enable_agc: bool,
}

impl Default for RealtimeAudioConfig {
    fn default() -> Self {
        Self {
            ice_servers: vec![RTCIceServer {
                urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                ..Default::default()
            }],
            sample_rate: 24000,
            channels: 1,
            buffer_size_ms: 20,
            vad_config: VoiceActivityDetectionConfig::default(),
            connection_timeout: Duration::from_secs(30),
            max_reconnect_attempts: 3,
            enable_aec: true,
            enable_noise_suppression: true,
            enable_agc: true,
        }
    }
}
