//! # WebRTC Connection Setup
//!
//! WebRTC connection creation and configuration.

use crate::error::{OpenAIError, Result};
use crate::models::realtime_audio::{
    RealtimeEvent, RealtimeSessionConfig, RealtimeSessionResponse, WebRtcConnectionState,
};
use log::info;
use std::sync::Arc;
use tokio::sync::mpsc;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_OPUS};
use webrtc::api::APIBuilder;
use webrtc::data_channel::data_channel_init::RTCDataChannelInit;
use webrtc::data_channel::RTCDataChannel;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability;
use webrtc::track::track_local::track_local_static_sample::TrackLocalStaticSample;
use webrtc::track::track_local::TrackLocal;

use super::super::{
    client::RealtimeAudioApi, session::RealtimeSession, vad::VoiceActivityDetector,
};
use crate::models::realtime_audio::AudioBuffer;

/// Type alias for event channel pair
pub type EventChannels = (
    mpsc::UnboundedSender<RealtimeEvent>,
    mpsc::UnboundedReceiver<RealtimeEvent>,
);

/// Type alias for audio channel pair  
pub type AudioChannels = (
    mpsc::UnboundedSender<AudioBuffer>,
    mpsc::UnboundedReceiver<AudioBuffer>,
);

impl RealtimeAudioApi {
    /// Create WebRTC session from response
    pub(crate) async fn create_webrtc_session(
        &self,
        response: RealtimeSessionResponse,
        config: Option<RealtimeSessionConfig>,
    ) -> Result<Arc<RealtimeSession>> {
        let api = self.create_webrtc_api().await?;
        let peer_connection = self.create_peer_connection(api).await?;
        let (event_channels, audio_channels) = self.create_communication_channels();

        let session = self.build_session(
            response.clone(),
            &peer_connection,
            event_channels,
            audio_channels,
            config,
        );

        self.setup_webrtc_connection(&session, &response).await?;
        Ok(session)
    }

    /// Creates and configures the WebRTC API
    pub(crate) async fn create_webrtc_api(&self) -> Result<webrtc::api::API> {
        let mut media_engine = MediaEngine::default();
        media_engine
            .register_default_codecs()
            .map_err(crate::invalid_request_err!("Failed to register codecs: {}"))?;

        let mut registry = Registry::new();
        registry = register_default_interceptors(registry, &mut media_engine).map_err(|e| {
            OpenAIError::InvalidRequest(format!("Failed to register interceptors: {e}"))
        })?;

        Ok(APIBuilder::new()
            .with_media_engine(media_engine)
            .with_interceptor_registry(registry)
            .build())
    }

    /// Creates a WebRTC peer connection
    pub(crate) async fn create_peer_connection(
        &self,
        api: webrtc::api::API,
    ) -> Result<Arc<RTCPeerConnection>> {
        let rtc_config = RTCConfiguration {
            ice_servers: self.config.ice_servers.clone(),
            ..Default::default()
        };

        let peer_connection = api.new_peer_connection(rtc_config).await.map_err(|e| {
            OpenAIError::InvalidRequest(format!("Failed to create peer connection: {e}"))
        })?;

        Ok(Arc::new(peer_connection))
    }

    /// Creates communication channels for events and audio
    pub(crate) fn create_communication_channels(&self) -> (EventChannels, AudioChannels) {
        let event_channels = mpsc::unbounded_channel();
        let audio_channels = mpsc::unbounded_channel();
        (event_channels, audio_channels)
    }

    /// Builds the RealtimeSession struct
    pub(crate) fn build_session(
        &self,
        response: RealtimeSessionResponse,
        peer_connection: &Arc<RTCPeerConnection>,
        event_channels: EventChannels,
        audio_channels: AudioChannels,
        config: Option<RealtimeSessionConfig>,
    ) -> Arc<RealtimeSession> {
        let (event_sender, event_receiver) = event_channels;
        let (audio_sender, audio_receiver) = audio_channels;

        Arc::new(RealtimeSession::new(
            response.id,
            peer_connection.clone(),
            event_sender,
            event_receiver,
            audio_sender,
            audio_receiver,
            config.unwrap_or_default(),
            VoiceActivityDetector::new(self.config.vad_config.clone(), self.config.sample_rate),
        ))
    }

    /// Connect to WebRTC endpoint using ephemeral key
    pub async fn connect_webrtc(&self, session: &Arc<RealtimeSession>) -> Result<()> {
        // This would implement the actual WebRTC connection establishment
        // For now, we'll simulate the connection process

        let peer_connection = session.peer_connection();

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

        session
            .set_connection_state(WebRtcConnectionState::Connected)
            .await;

        let mut session_stats = session.stats_mutex().lock().await;
        session_stats.connected_at = Some(chrono::Utc::now());

        info!("WebRTC connection established for session: {}", session.id);

        Ok(())
    }

    /// Maps WebRTC connection state to internal state
    #[allow(dead_code)]
    pub(crate) fn map_connection_state(state: RTCPeerConnectionState) -> WebRtcConnectionState {
        map_webrtc_connection_state(state)
    }

    /// Creates an audio track for Opus codec
    pub(crate) fn create_audio_track(&self) -> Arc<TrackLocalStaticSample> {
        Arc::new(TrackLocalStaticSample::new(
            RTCRtpCodecCapability {
                mime_type: MIME_TYPE_OPUS.to_owned(),
                ..Default::default()
            },
            "audio".to_owned(),
            "realtime-audio".to_owned(),
        ))
    }
}

/// Maps WebRTC connection state to internal state
pub(crate) fn map_webrtc_connection_state(state: RTCPeerConnectionState) -> WebRtcConnectionState {
    match state {
        RTCPeerConnectionState::New => WebRtcConnectionState::New,
        RTCPeerConnectionState::Connecting => WebRtcConnectionState::Connecting,
        RTCPeerConnectionState::Connected => WebRtcConnectionState::Connected,
        RTCPeerConnectionState::Disconnected => WebRtcConnectionState::Disconnected,
        RTCPeerConnectionState::Failed => WebRtcConnectionState::Failed,
        RTCPeerConnectionState::Closed => WebRtcConnectionState::Closed,
        _ => WebRtcConnectionState::New,
    }
}
