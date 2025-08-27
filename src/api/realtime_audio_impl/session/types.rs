//! # Session Types
//!
//! Core types and structures for real-time audio sessions.

use crate::error::Result;
use crate::models::realtime_audio::{
    AudioBuffer, RealtimeEvent, RealtimeSessionConfig, WebRtcConnectionState, WebRtcStats,
};
use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use webrtc::data_channel::RTCDataChannel;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::track::track_local::track_local_static_sample::TrackLocalStaticSample;

use super::super::vad::VoiceActivityDetector;

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

impl RealtimeSession {
    /// Create a new realtime session
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        peer_connection: Arc<RTCPeerConnection>,
        event_sender: mpsc::UnboundedSender<RealtimeEvent>,
        event_receiver: mpsc::UnboundedReceiver<RealtimeEvent>,
        audio_sender: mpsc::UnboundedSender<AudioBuffer>,
        audio_receiver: mpsc::UnboundedReceiver<AudioBuffer>,
        config: RealtimeSessionConfig,
        vad: VoiceActivityDetector,
    ) -> Self {
        Self {
            id,
            peer_connection,
            data_channel: Arc::new(Mutex::new(None)),
            audio_track: Arc::new(Mutex::new(None)),
            event_sender,
            event_receiver: Arc::new(Mutex::new(Some(event_receiver))),
            audio_sender,
            audio_receiver: Arc::new(Mutex::new(Some(audio_receiver))),
            _config: config,
            connection_state: Arc::new(Mutex::new(WebRtcConnectionState::New)),
            stats: Arc::new(Mutex::new(WebRtcStats::default())),
            _vad: Arc::new(Mutex::new(vad)),
            started_at: Utc::now(),
            is_active: Arc::new(AtomicBool::new(true)),
            reconnect_handler: Arc::new(Mutex::new(None)),
        }
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
            crate::error::OpenAIError::InvalidRequest(format!(
                "Failed to close peer connection: {e}"
            ))
        })?;

        // Cancel reconnection handler if any
        if let Some(handle) = self.reconnect_handler.lock().await.take() {
            handle.abort();
        }

        log::info!("Session closed: {}", self.id);
        Ok(())
    }

    /// Set the data channel
    pub async fn set_data_channel(&self, data_channel: Arc<RTCDataChannel>) {
        *self.data_channel.lock().await = Some(data_channel);
    }

    /// Set the audio track
    pub async fn set_audio_track(&self, audio_track: Arc<TrackLocalStaticSample>) {
        *self.audio_track.lock().await = Some(audio_track);
    }

    /// Get the connection state
    pub async fn connection_state(&self) -> WebRtcConnectionState {
        *self.connection_state.lock().await
    }

    /// Set the connection state
    pub async fn set_connection_state(&self, state: WebRtcConnectionState) {
        *self.connection_state.lock().await = state;
    }

    /// Get the peer connection
    pub fn peer_connection(&self) -> &Arc<RTCPeerConnection> {
        &self.peer_connection
    }

    /// Get the stats mutex for internal use
    pub(crate) fn stats_mutex(&self) -> &Arc<Mutex<WebRtcStats>> {
        &self.stats
    }

    /// Get the data channel (for internal use)
    pub(crate) async fn data_channel(&self) -> Option<Arc<RTCDataChannel>> {
        self.data_channel.lock().await.clone()
    }

    /// Get the audio track (for internal use)
    pub(crate) async fn audio_track(&self) -> Option<Arc<TrackLocalStaticSample>> {
        self.audio_track.lock().await.clone()
    }

    /// Get the event sender (for internal use)
    pub(crate) fn event_sender(&self) -> &mpsc::UnboundedSender<RealtimeEvent> {
        &self.event_sender
    }

    /// Get the audio sender (for internal use)
    pub(crate) fn audio_sender(&self) -> &mpsc::UnboundedSender<AudioBuffer> {
        &self.audio_sender
    }
}
