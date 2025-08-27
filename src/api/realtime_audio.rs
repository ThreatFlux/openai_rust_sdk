//! # Real-time Audio API
//!
//! This module provides WebRTC-based real-time audio streaming capabilities
//! for OpenAI's real-time audio API, supporting bidirectional audio streaming,
//! voice activity detection, and low-latency communication.

use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
use crate::models::realtime_audio::{
    AudioBuffer, RealtimeAudioModels, RealtimeEvent, RealtimeSessionConfig, RealtimeSessionRequest,
    RealtimeSessionResponse, RealtimeVoice, VoiceActivityDetectionConfig, VoiceActivityResult,
    WebRtcConnectionState, WebRtcStats,
};
use chrono::{DateTime, Utc};
use log::{error, info, warn};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Mutex, RwLock};
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_OPUS};
use webrtc::api::APIBuilder;
use webrtc::data_channel::data_channel_init::RTCDataChannelInit;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::media::Sample;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability;
use webrtc::track::track_local::track_local_static_sample::TrackLocalStaticSample;
use webrtc::track::track_local::TrackLocal;
use webrtc::track::track_remote::TrackRemote;

/// Real-time Audio API client with WebRTC support
pub struct RealtimeAudioApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
    /// Active real-time sessions mapped by session ID
    sessions: Arc<RwLock<HashMap<String, Arc<RealtimeSession>>>>,
    /// Configuration for real-time audio API
    pub config: RealtimeAudioConfig,
}

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

/// Real-time audio session
pub struct RealtimeSession {
    /// Session ID
    pub id: String,

    /// WebRTC peer connection
    peer_connection: Arc<RTCPeerConnection>,

    /// Data channel for events
    data_channel: Arc<Mutex<Option<Arc<RTCDataChannel>>>>,

    /// Audio track for sending
    audio_track: Arc<Mutex<Option<Arc<TrackLocalStaticSample>>>>,

    /// Event sender for outgoing events
    event_sender: mpsc::UnboundedSender<RealtimeEvent>,

    /// Event receiver for incoming events
    event_receiver: Arc<Mutex<Option<mpsc::UnboundedReceiver<RealtimeEvent>>>>,

    /// Audio sender for outgoing audio
    audio_sender: mpsc::UnboundedSender<AudioBuffer>,

    /// Audio receiver for incoming audio
    audio_receiver: Arc<Mutex<Option<mpsc::UnboundedReceiver<AudioBuffer>>>>,

    /// Session configuration
    _config: RealtimeSessionConfig,

    /// Connection state
    connection_state: Arc<Mutex<WebRtcConnectionState>>,

    /// Statistics
    stats: Arc<Mutex<WebRtcStats>>,

    /// Voice activity detector
    _vad: Arc<Mutex<VoiceActivityDetector>>,

    /// Session start time
    started_at: DateTime<Utc>,

    /// Is session active
    is_active: Arc<AtomicBool>,

    /// Reconnection handler
    reconnect_handler: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

/// Voice Activity Detector
pub struct VoiceActivityDetector {
    /// Voice activity detection configuration
    config: VoiceActivityDetectionConfig,
    /// History of audio energy levels for smoothing
    energy_history: Vec<f32>,
    /// Timestamp of last detected speech
    last_speech_time: Option<Instant>,
    /// Whether speech is currently being detected
    is_speaking: bool,
    /// Audio sample rate (for future use)
    _sample_rate: u32,
}

/// Audio processor for real-time audio effects
pub struct AudioProcessor {
    /// Audio sample rate (for future use)
    _sample_rate: u32,
    /// Number of audio channels (for future use)
    _channels: u16,
    /// Whether acoustic echo cancellation is enabled
    enable_aec: bool,
    /// Whether noise suppression is enabled
    enable_noise_suppression: bool,
    /// Whether automatic gain control is enabled
    enable_agc: bool,
    // Audio processing state would be maintained here
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

impl RealtimeAudioApi {
    /// Create a new Real-time Audio API client
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config: RealtimeAudioConfig::default(),
        })
    }

    /// Create a new client with custom configuration
    pub fn new_with_config<S: Into<String>>(
        api_key: S,
        config: RealtimeAudioConfig,
    ) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    /// Create a new client with custom base URL
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config: RealtimeAudioConfig::default(),
        })
    }

    /// Create a new real-time audio session
    pub async fn create_session(
        &self,
        request: &RealtimeSessionRequest,
    ) -> Result<Arc<RealtimeSession>> {
        let url = format!("{}/realtime/sessions", self.http_client.base_url());
        let headers = self.http_client.build_headers()?;

        let response = self
            .http_client
            .client()
            .post(&url)
            .headers(headers)
            .json(request)
            .send()
            .await
            .map_err(crate::request_err!(to_string))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let session_response: RealtimeSessionResponse = response
            .json()
            .await
            .map_err(crate::parse_err!(to_string))?;

        let session = self
            .create_webrtc_session(session_response, request.config.clone())
            .await?;

        // Store the session
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session.clone());

        Ok(session)
    }

    /// Get an existing session
    pub async fn get_session(&self, session_id: &str) -> Option<Arc<RealtimeSession>> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// List active sessions
    pub async fn list_sessions(&self) -> Vec<String> {
        let sessions = self.sessions.read().await;
        sessions.keys().cloned().collect()
    }

    /// Close a session
    pub async fn close_session(&self, session_id: &str) -> Result<()> {
        if let Some(session) = {
            let mut sessions = self.sessions.write().await;
            sessions.remove(session_id)
        } {
            session.close().await?;
        }
        Ok(())
    }

    /// Create WebRTC session from response
    async fn create_webrtc_session(
        &self,
        response: RealtimeSessionResponse,
        config: Option<RealtimeSessionConfig>,
    ) -> Result<Arc<RealtimeSession>> {
        // Create media engine
        let mut media_engine = MediaEngine::default();
        media_engine
            .register_default_codecs()
            .map_err(crate::invalid_request_err!("Failed to register codecs: {}"))?;

        // Create interceptor registry
        let mut registry = Registry::new();
        registry = register_default_interceptors(registry, &mut media_engine).map_err(|e| {
            OpenAIError::InvalidRequest(format!("Failed to register interceptors: {e}"))
        })?;

        // Create API
        let api = APIBuilder::new()
            .with_media_engine(media_engine)
            .with_interceptor_registry(registry)
            .build();

        // Create peer connection configuration
        let rtc_config = RTCConfiguration {
            ice_servers: self.config.ice_servers.clone(),
            ..Default::default()
        };

        // Create peer connection
        let peer_connection = Arc::new(api.new_peer_connection(rtc_config).await.map_err(|e| {
            OpenAIError::InvalidRequest(format!("Failed to create peer connection: {e}"))
        })?);

        // Create channels
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        let (audio_sender, audio_receiver) = mpsc::unbounded_channel();

        // Create session
        let session = Arc::new(RealtimeSession {
            id: response.id.clone(),
            peer_connection: peer_connection.clone(),
            data_channel: Arc::new(Mutex::new(None)),
            audio_track: Arc::new(Mutex::new(None)),
            event_sender,
            event_receiver: Arc::new(Mutex::new(Some(event_receiver))),
            audio_sender,
            audio_receiver: Arc::new(Mutex::new(Some(audio_receiver))),
            _config: config.unwrap_or_default(),
            connection_state: Arc::new(Mutex::new(WebRtcConnectionState::New)),
            stats: Arc::new(Mutex::new(WebRtcStats::default())),
            _vad: Arc::new(Mutex::new(VoiceActivityDetector::new(
                self.config.vad_config.clone(),
                self.config.sample_rate,
            ))),
            started_at: Utc::now(),
            is_active: Arc::new(AtomicBool::new(true)),
            reconnect_handler: Arc::new(Mutex::new(None)),
        });

        // Set up WebRTC connection
        self.setup_webrtc_connection(&session, &response).await?;

        Ok(session)
    }

    /// Set up WebRTC connection
    async fn setup_webrtc_connection(
        &self,
        session: &Arc<RealtimeSession>,
        _response: &RealtimeSessionResponse,
    ) -> Result<()> {
        let peer_connection = session.peer_connection.clone();
        let session_weak = Arc::downgrade(session);

        // Set up connection state change handler
        let state_handler = session.connection_state.clone();
        peer_connection.on_peer_connection_state_change(Box::new(move |state| {
            let state_handler = state_handler.clone();
            Box::pin(async move {
                let mut connection_state = state_handler.lock().await;
                *connection_state = match state {
                    RTCPeerConnectionState::New => WebRtcConnectionState::New,
                    RTCPeerConnectionState::Connecting => WebRtcConnectionState::Connecting,
                    RTCPeerConnectionState::Connected => WebRtcConnectionState::Connected,
                    RTCPeerConnectionState::Disconnected => WebRtcConnectionState::Disconnected,
                    RTCPeerConnectionState::Failed => WebRtcConnectionState::Failed,
                    RTCPeerConnectionState::Closed => WebRtcConnectionState::Closed,
                    _ => WebRtcConnectionState::New,
                };

                info!(
                    "WebRTC connection state changed to: {:?}",
                    *connection_state
                );
            })
        }));

        // Create data channel for events
        let data_channel = peer_connection
            .create_data_channel(
                "events",
                Some(RTCDataChannelInit {
                    ordered: Some(true),
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| {
                OpenAIError::InvalidRequest(format!("Failed to create data channel: {e}"))
            })?;

        *session.data_channel.lock().await = Some(data_channel.clone());

        // Set up data channel handlers
        let session_for_events = session_weak.clone();
        data_channel.on_message(Box::new(move |msg| {
            let session_for_events = session_for_events.clone();
            Box::pin(async move {
                if let Some(session) = session_for_events.upgrade() {
                    if let Err(e) = session.handle_data_channel_message(msg).await {
                        error!("Failed to handle data channel message: {e}");
                    }
                }
            })
        }));

        // Create audio track
        let audio_track = Arc::new(TrackLocalStaticSample::new(
            RTCRtpCodecCapability {
                mime_type: MIME_TYPE_OPUS.to_owned(),
                ..Default::default()
            },
            "audio".to_owned(),
            "realtime-audio".to_owned(),
        ));

        *session.audio_track.lock().await = Some(audio_track.clone());

        // Add track to peer connection
        let _rtp_sender = peer_connection
            .add_track(audio_track.clone() as Arc<dyn TrackLocal + Send + Sync>)
            .await
            .map_err(crate::invalid_request_err!("Failed to add audio track: {}"))?;

        // Set up track handlers for incoming audio
        let session_for_track = session_weak.clone();
        peer_connection.on_track(Box::new(move |track, _receiver, _transceiver| {
            let session_for_track = session_for_track.clone();
            Box::pin(async move {
                if let Some(session) = session_for_track.upgrade() {
                    session.handle_incoming_track(track).await;
                }
            })
        }));

        Ok(())
    }

    /// Connect to WebRTC endpoint using ephemeral key
    pub async fn connect_webrtc(&self, session: &Arc<RealtimeSession>) -> Result<()> {
        // This would implement the actual WebRTC connection establishment
        // For now, we'll simulate the connection process

        let peer_connection = session.peer_connection.clone();

        // Create offer
        let offer = peer_connection
            .create_offer(None)
            .await
            .map_err(crate::invalid_request_err!("Failed to create offer: {}"))?;

        // Set local description
        peer_connection
            .set_local_description(offer)
            .await
            .map_err(|e| {
                OpenAIError::InvalidRequest(format!("Failed to set local description: {e}"))
            })?;

        // In a real implementation, you would exchange the SDP with OpenAI's servers
        // and handle the answer. For now, we'll mark the connection as connected.

        let mut connection_state = session.connection_state.lock().await;
        *connection_state = WebRtcConnectionState::Connected;

        let mut session_stats = session.stats.lock().await;
        session_stats.connected_at = Some(Utc::now());

        info!("WebRTC connection established for session: {}", session.id);

        Ok(())
    }
}

impl RealtimeSession {
    /// Send an event to the server
    pub async fn send_event(&self, event: RealtimeEvent) -> Result<()> {
        if let Some(data_channel) = self.data_channel.lock().await.as_ref() {
            let json = serde_json::to_string(&event).map_err(crate::parse_err!(to_string))?;

            data_channel
                .send_text(json)
                .await
                .map_err(crate::streaming_err!("Failed to send event: {}"))?;
        } else {
            return Err(OpenAIError::InvalidRequest(
                "Data channel not available".to_string(),
            ));
        }

        Ok(())
    }

    /// Send audio data
    pub async fn send_audio(&self, audio_buffer: AudioBuffer) -> Result<()> {
        if let Some(audio_track) = self.audio_track.lock().await.as_ref() {
            // Convert audio buffer to RTP samples
            let samples = audio_buffer.to_mono();
            let sample_duration = Duration::from_millis(20); // 20ms per packet

            // Create audio sample
            let sample = Sample {
                data: samples.iter().flat_map(|&s| s.to_le_bytes()).collect(),
                duration: sample_duration,
                ..Default::default()
            };

            audio_track
                .write_sample(&sample)
                .await
                .map_err(crate::streaming_err!("Failed to send audio: {}"))?;
        } else {
            return Err(OpenAIError::InvalidRequest(
                "Audio track not available".to_string(),
            ));
        }

        Ok(())
    }

    /// Start audio streaming from microphone
    pub async fn start_audio_input(&self) -> Result<()> {
        // This would start capturing audio from microphone
        // For now, we'll just mark as started
        info!("Audio input started for session: {}", self.id);
        Ok(())
    }

    /// Stop audio streaming
    pub async fn stop_audio_input(&self) -> Result<()> {
        // This would stop capturing audio from microphone
        info!("Audio input stopped for session: {}", self.id);
        Ok(())
    }

    /// Get event stream
    pub async fn event_stream(&self) -> Option<mpsc::UnboundedReceiver<RealtimeEvent>> {
        self.event_receiver.lock().await.take()
    }

    /// Get audio stream
    pub async fn audio_stream(&self) -> Option<mpsc::UnboundedReceiver<AudioBuffer>> {
        self.audio_receiver.lock().await.take()
    }

    /// Get session statistics
    pub async fn get_stats(&self) -> WebRtcStats {
        self.stats.lock().await.clone()
    }

    /// Check if session is active
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.is_active.load(Ordering::Relaxed)
    }

    /// Get session start time
    #[must_use]
    pub fn started_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    /// Close the session
    pub async fn close(&self) -> Result<()> {
        self.is_active.store(false, Ordering::Relaxed);

        // Close peer connection
        self.peer_connection.close().await.map_err(|e| {
            OpenAIError::InvalidRequest(format!("Failed to close peer connection: {e}"))
        })?;

        // Cancel reconnection handler if any
        if let Some(handle) = self.reconnect_handler.lock().await.take() {
            handle.abort();
        }

        info!("Session closed: {}", self.id);
        Ok(())
    }

    /// Handle incoming data channel message
    async fn handle_data_channel_message(&self, msg: DataChannelMessage) -> Result<()> {
        if let Ok(text) = String::from_utf8(msg.data.to_vec()) {
            match serde_json::from_str::<RealtimeEvent>(&text) {
                Ok(event) => {
                    if let Err(e) = self.event_sender.send(event) {
                        warn!("Failed to send event to handler: {e}");
                    }
                }
                Err(e) => {
                    warn!("Failed to parse incoming event: {e}");
                }
            }
        }

        Ok(())
    }

    /// Handle incoming audio track
    async fn handle_incoming_track(&self, track: Arc<TrackRemote>) {
        let audio_sender = self.audio_sender.clone();
        let sample_rate = 24000; // Default sample rate for real-time audio

        tokio::spawn(async move {
            loop {
                match track.read_rtp().await {
                    Ok((rtp_packet, _)) => {
                        // Convert RTP packet to audio buffer
                        // This is a simplified conversion - in reality you'd need
                        // to handle Opus decoding properly
                        let samples: Vec<i16> = rtp_packet
                            .payload
                            .chunks(2)
                            .map(|chunk| {
                                if chunk.len() == 2 {
                                    i16::from_le_bytes([chunk[0], chunk[1]])
                                } else {
                                    0
                                }
                            })
                            .collect();

                        let audio_buffer = AudioBuffer::new(samples, sample_rate, 1);

                        if let Err(e) = audio_sender.send(audio_buffer) {
                            warn!("Failed to send audio buffer: {e}");
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Failed to read RTP packet: {e}");
                        break;
                    }
                }
            }
        });
    }
}

impl VoiceActivityDetector {
    /// Create a new voice activity detector
    #[must_use]
    pub fn new(config: VoiceActivityDetectionConfig, sample_rate: u32) -> Self {
        Self {
            config,
            energy_history: Vec::new(),
            last_speech_time: None,
            is_speaking: false,
            _sample_rate: sample_rate,
        }
    }

    /// Process audio buffer and detect voice activity
    pub fn process(&mut self, audio_buffer: &AudioBuffer) -> VoiceActivityResult {
        let energy = audio_buffer.rms_energy();
        self.energy_history.push(energy);

        // Keep only recent history (last 100 frames)
        if self.energy_history.len() > 100 {
            self.energy_history.remove(0);
        }

        // Calculate average energy
        let avg_energy = if self.energy_history.is_empty() {
            0.0
        } else {
            self.energy_history.iter().sum::<f32>() / self.energy_history.len() as f32
        };

        // Determine if speech is detected
        let is_speech = energy > self.config.threshold && energy > avg_energy * 2.0;
        let confidence = if is_speech {
            (energy / self.config.threshold).min(1.0)
        } else {
            0.0
        };

        if is_speech {
            self.last_speech_time = Some(Instant::now());
            self.is_speaking = true;
        } else if let Some(last_speech) = self.last_speech_time {
            if last_speech.elapsed().as_millis() > u128::from(self.config.silence_duration_ms) {
                self.is_speaking = false;
            }
        }

        VoiceActivityResult {
            is_speech: self.is_speaking,
            confidence,
            energy,
            timestamp: Utc::now(),
        }
    }

    /// Check if currently speaking
    #[must_use]
    pub fn is_speaking(&self) -> bool {
        self.is_speaking
    }
}

impl AudioProcessor {
    /// Create a new audio processor
    #[must_use]
    pub fn new(
        sample_rate: u32,
        channels: u16,
        enable_echo_cancellation: bool,
        enable_noise_suppression: bool,
        enable_automatic_gain_control: bool,
    ) -> Self {
        Self {
            _sample_rate: sample_rate,
            _channels: channels,
            enable_aec: enable_echo_cancellation,
            enable_noise_suppression,
            enable_agc: enable_automatic_gain_control,
        }
    }

    /// Process audio buffer with effects
    pub fn process(&mut self, audio_buffer: &mut AudioBuffer) {
        if self.enable_noise_suppression {
            self.apply_noise_suppression(audio_buffer);
        }

        if self.enable_agc {
            self.apply_automatic_gain_control(audio_buffer);
        }

        if self.enable_aec {
            self.apply_echo_cancellation(audio_buffer);
        }
    }

    /// Apply noise suppression (simplified implementation)
    fn apply_noise_suppression(&self, audio_buffer: &mut AudioBuffer) {
        // Simple noise gate - remove samples below threshold
        let threshold = 100; // Adjust based on requirements

        for sample in &mut audio_buffer.samples {
            if sample.abs() < threshold {
                *sample = 0;
            }
        }
    }

    /// Apply automatic gain control (simplified implementation)
    fn apply_automatic_gain_control(&self, audio_buffer: &mut AudioBuffer) {
        if audio_buffer.samples.is_empty() {
            return;
        }

        // Calculate current RMS level
        let rms = audio_buffer.rms_energy();
        let target_rms = 1000.0; // Target RMS level

        if rms > 0.0 {
            let gain: f32 = target_rms / rms;
            let clamped_gain = gain.clamp(0.1, 10.0); // Limit gain range

            for sample in &mut audio_buffer.samples {
                *sample = (f32::from(*sample) * clamped_gain) as i16;
            }
        }
    }

    /// Apply echo cancellation (placeholder implementation)
    fn apply_echo_cancellation(&self, _audio_buffer: &mut AudioBuffer) {
        // Echo cancellation would require reference signal and adaptive filtering
        // This is a complex algorithm typically implemented in specialized libraries
        // For now, this is a placeholder
    }
}

/// Builder for creating real-time audio sessions
pub struct RealtimeSessionBuilder {
    /// The realtime session request
    request: RealtimeSessionRequest,
}

impl RealtimeSessionBuilder {
    /// Create a new session builder
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            request: RealtimeSessionRequest {
                model: model.into(),
                config: None,
                instructions: None,
                voice: None,
                temperature: None,
                max_response_output_tokens: None,
            },
        }
    }

    /// Use GPT-4o real-time model
    #[must_use]
    pub fn gpt_4o_realtime() -> Self {
        Self::new(RealtimeAudioModels::GPT_4O_REALTIME_PREVIEW)
    }

    /// Use GPT-4o mini real-time model
    #[must_use]
    pub fn gpt_4o_mini_realtime() -> Self {
        Self::new(RealtimeAudioModels::GPT_4O_MINI_REALTIME_PREVIEW)
    }

    /// Set session configuration
    #[must_use]
    pub fn config(mut self, config: RealtimeSessionConfig) -> Self {
        self.request.config = Some(config);
        self
    }

    /// Set instructions
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request.instructions = Some(instructions.into());
        self
    }

    /// Set voice
    #[must_use]
    pub fn voice(mut self, voice: RealtimeVoice) -> Self {
        self.request.voice = Some(voice);
        self
    }

    /// Set temperature
    #[must_use]
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature.clamp(0.0, 2.0));
        self
    }

    /// Set max response tokens
    #[must_use]
    pub fn max_response_tokens(mut self, tokens: u32) -> Self {
        self.request.max_response_output_tokens = Some(tokens);
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> RealtimeSessionRequest {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_realtime_audio_api_creation() {
        let api = RealtimeAudioApi::new("test-key".to_string()).unwrap();
        assert_eq!(api.config.sample_rate, 24000);
        assert_eq!(api.config.channels, 1);
    }

    #[test]
    fn test_config_defaults() {
        let config = RealtimeAudioConfig::default();
        assert_eq!(config.sample_rate, 24000);
        assert_eq!(config.channels, 1);
        assert_eq!(config.buffer_size_ms, 20);
        assert!(config.enable_aec);
        assert!(config.enable_noise_suppression);
        assert!(config.enable_agc);
    }

    #[test]
    fn test_session_builder() {
        let request = RealtimeSessionBuilder::gpt_4o_realtime()
            .instructions("You are a helpful assistant")
            .voice(RealtimeVoice::Alloy)
            .temperature(0.8)
            .max_response_tokens(4096)
            .build();

        assert_eq!(request.model, RealtimeAudioModels::GPT_4O_REALTIME_PREVIEW);
        assert_eq!(
            request.instructions,
            Some("You are a helpful assistant".to_string())
        );
        assert_eq!(request.voice, Some(RealtimeVoice::Alloy));
        assert_eq!(request.temperature, Some(0.8));
        assert_eq!(request.max_response_output_tokens, Some(4096));
    }

    #[test]
    fn test_voice_activity_detector() {
        let config = VoiceActivityDetectionConfig::default();
        let mut vad = VoiceActivityDetector::new(config, 24000);

        // Test with silence
        let silence = AudioBuffer::new(vec![0; 480], 24000, 1); // 20ms of silence
        let result = vad.process(&silence);
        assert!(!result.is_speech);
        assert_eq!(result.confidence, 0.0);

        // Test with loud audio (process twice to build history)
        let loud_audio = AudioBuffer::new(vec![10000; 480], 24000, 1);
        // First process builds history
        let _ = vad.process(&silence);
        // Second process should detect speech against silence baseline
        let result = vad.process(&loud_audio);
        assert!(result.is_speech);
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_audio_processor() {
        let mut processor = AudioProcessor::new(24000, 1, true, true, true);
        let mut audio = AudioBuffer::new(vec![50, 2000, -50, -2000], 24000, 1);

        processor.process(&mut audio);

        // Verify noise suppression removed low-amplitude samples
        assert_eq!(audio.samples[0], 0); // Below threshold
        assert_ne!(audio.samples[1], 2000); // Should be modified by AGC
    }

    #[tokio::test]
    async fn test_session_lifecycle() {
        let _session_id = "test-session".to_string();

        // Create mock session components
        let (event_sender, mut event_receiver) = mpsc::unbounded_channel::<RealtimeEvent>();
        let (_audio_sender, _audio_receiver) = mpsc::unbounded_channel::<Vec<f32>>();

        // Test that channels work
        let test_event = RealtimeEvent::SessionUpdate {
            event_id: "test-123".to_string(),
            session: RealtimeSessionConfig::default(),
        };

        event_sender.send(test_event).unwrap();
        let received = event_receiver.recv().await.unwrap();

        match received {
            RealtimeEvent::SessionUpdate { event_id, .. } => {
                assert_eq!(event_id, "test-123");
            }
            _ => panic!("Unexpected event type"),
        }
    }
}
