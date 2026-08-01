//! # WebRTC Connection Setup
//!
//! WebRTC connection creation and configuration.

use crate::error::{OpenAIError, Result};
use crate::models::realtime_audio::{
    RealtimeEvent, RealtimeSessionConfig, RealtimeSessionResponse, WebRtcConnectionState,
};
use log::info;
use rtc::peer_connection::configuration::media_engine::MIME_TYPE_OPUS;
use rtc::rtp_transceiver::rtp_sender::{
    RTCRtpCodec, RTCRtpCodingParameters, RTCRtpEncodingParameters, RtpCodecKind,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use webrtc::media_stream::MediaStreamTrack;
use webrtc::media_stream::track_local::static_sample::TrackLocalStaticSample;
use webrtc::media_stream::track_remote::TrackRemote;
use webrtc::peer_connection::{
    MediaEngine, PeerConnection, PeerConnectionBuilder, PeerConnectionEventHandler,
    RTCConfigurationBuilder, RTCPeerConnectionState, Registry, register_default_interceptors,
};

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
        let (peer_connection, handler) = Box::pin(self.create_peer_connection()).await?;
        let (event_channels, audio_channels) = self.create_communication_channels();

        let session = self.build_session(
            response.clone(),
            &peer_connection,
            event_channels,
            audio_channels,
            config,
        );
        handler.set_session(Arc::downgrade(&session)).await;

        self.setup_webrtc_connection(&session, &response).await?;
        Ok(session)
    }

    /// Creates and configures a WebRTC peer connection.
    pub(crate) async fn create_peer_connection(
        &self,
    ) -> Result<(Arc<dyn PeerConnection>, Arc<RealtimePeerConnectionHandler>)> {
        let mut media_engine = MediaEngine::default();
        media_engine
            .register_default_codecs()
            .map_err(crate::invalid_request_err!("Failed to register codecs: {}"))?;

        let registry =
            register_default_interceptors(Registry::new(), &mut media_engine).map_err(|e| {
                OpenAIError::InvalidRequest(format!("Failed to register interceptors: {e}"))
            })?;

        let handler = Arc::new(RealtimePeerConnectionHandler::new());
        let rtc_config = RTCConfigurationBuilder::default()
            .with_ice_servers(self.config.ice_servers.clone())
            .build();
        let peer_connection = Box::pin(
            PeerConnectionBuilder::new()
                .with_configuration(rtc_config)
                .with_media_engine(media_engine)
                .with_interceptor_registry(registry)
                .with_handler(handler.clone())
                .with_udp_addrs(vec!["0.0.0.0:0"])
                .build(),
        )
        .await
        .map_err(|e| {
            OpenAIError::InvalidRequest(format!("Failed to create peer connection: {e}"))
        })?;

        Ok((Arc::new(peer_connection), handler))
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
        peer_connection: &Arc<dyn PeerConnection>,
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
    pub(crate) fn create_audio_track(&self) -> Result<Arc<TrackLocalStaticSample>> {
        let track = MediaStreamTrack::new(
            "realtime-audio".to_owned(),
            "audio".to_owned(),
            "realtime-audio".to_owned(),
            RtpCodecKind::Audio,
            vec![RTCRtpEncodingParameters {
                rtp_coding_parameters: RTCRtpCodingParameters {
                    ssrc: Some(123_456),
                    ..Default::default()
                },
                codec: RTCRtpCodec {
                    mime_type: MIME_TYPE_OPUS.to_owned(),
                    clock_rate: 48_000,
                    channels: 2,
                    ..Default::default()
                },
                ..Default::default()
            }],
        );

        TrackLocalStaticSample::new(track)
            .map(Arc::new)
            .map_err(crate::invalid_request_err!(
                "Failed to create audio track: {}"
            ))
    }
}

/// Bridges WebRTC callbacks to the owning realtime session.
pub(crate) struct RealtimePeerConnectionHandler {
    /// Weak reference to the session receiving WebRTC callbacks.
    session: Arc<Mutex<Option<std::sync::Weak<RealtimeSession>>>>,
}

impl RealtimePeerConnectionHandler {
    /// Create a handler before the session is built.
    fn new() -> Self {
        Self {
            session: Arc::new(Mutex::new(None)),
        }
    }

    /// Attach the session after the peer connection has been constructed.
    async fn set_session(&self, session: std::sync::Weak<RealtimeSession>) {
        *self.session.lock().await = Some(session);
    }
}

#[async_trait::async_trait]
impl PeerConnectionEventHandler for RealtimePeerConnectionHandler {
    async fn on_connection_state_change(&self, state: RTCPeerConnectionState) {
        if let Some(session) = self
            .session
            .lock()
            .await
            .as_ref()
            .and_then(|weak| weak.upgrade())
        {
            session
                .set_connection_state(map_webrtc_connection_state(state))
                .await;
        }
    }

    async fn on_track(&self, track: Arc<dyn TrackRemote>) {
        if let Some(session) = self
            .session
            .lock()
            .await
            .as_ref()
            .and_then(|weak| weak.upgrade())
        {
            session.handle_incoming_track(track).await;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::realtime_audio::{RealtimeSessionConfig, SessionStatus};
    use chrono::Utc;

    fn test_session_response() -> RealtimeSessionResponse {
        let now = Utc::now();
        RealtimeSessionResponse {
            id: "realtime-test".to_owned(),
            object: "realtime.session".to_owned(),
            status: SessionStatus::Active,
            ephemeral_key: "ephemeral-test".to_owned(),
            webrtc_url: "https://example.test/realtime".to_owned(),
            config: RealtimeSessionConfig::default(),
            expires_at: now,
            created_at: now,
        }
    }

    #[test]
    fn maps_all_peer_connection_states() {
        assert_eq!(
            map_webrtc_connection_state(RTCPeerConnectionState::New),
            WebRtcConnectionState::New
        );
        assert_eq!(
            map_webrtc_connection_state(RTCPeerConnectionState::Connecting),
            WebRtcConnectionState::Connecting
        );
        assert_eq!(
            map_webrtc_connection_state(RTCPeerConnectionState::Connected),
            WebRtcConnectionState::Connected
        );
        assert_eq!(
            map_webrtc_connection_state(RTCPeerConnectionState::Disconnected),
            WebRtcConnectionState::Disconnected
        );
        assert_eq!(
            map_webrtc_connection_state(RTCPeerConnectionState::Failed),
            WebRtcConnectionState::Failed
        );
        assert_eq!(
            map_webrtc_connection_state(RTCPeerConnectionState::Closed),
            WebRtcConnectionState::Closed
        );
    }

    #[tokio::test]
    async fn creates_and_connects_local_webrtc_session() {
        let api = RealtimeAudioApi::new("test-key").expect("API");
        let session = api
            .create_webrtc_session(test_session_response(), None)
            .await
            .expect("local WebRTC session");

        assert!(session.data_channel().await.is_some());
        assert!(session.audio_track().await.is_some());
        assert_eq!(session.connection_state().await, WebRtcConnectionState::New);

        api.connect_webrtc(&session).await.expect("local offer");
        assert_eq!(
            session.connection_state().await,
            WebRtcConnectionState::Connected
        );

        session.close().await.expect("close session");
        assert!(!session.is_active());
    }

    #[tokio::test]
    async fn handler_updates_attached_session_state() {
        let api = RealtimeAudioApi::new("test-key").expect("API");
        let (peer_connection, handler) = api.create_peer_connection().await.expect("peer");
        let (event_channels, audio_channels) = api.create_communication_channels();
        let session = api.build_session(
            test_session_response(),
            &peer_connection,
            event_channels,
            audio_channels,
            None,
        );

        handler.set_session(Arc::downgrade(&session)).await;
        handler
            .on_connection_state_change(RTCPeerConnectionState::Failed)
            .await;
        assert_eq!(
            session.connection_state().await,
            WebRtcConnectionState::Failed
        );

        session.close().await.expect("close session");
    }
}
