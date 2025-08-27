//! # WebRTC Setup and Configuration
//!
//! WebRTC session setup, track management, and event handlers.

use crate::error::{OpenAIError, Result};
use crate::models::realtime_audio::RealtimeSessionResponse;
use log::info;
use std::sync::Arc;
use webrtc::data_channel::data_channel_init::RTCDataChannelInit;
use webrtc::data_channel::RTCDataChannel;
use webrtc::track::track_local::track_local_static_sample::TrackLocalStaticSample;
use webrtc::track::track_local::TrackLocal;

use super::super::{client::RealtimeAudioApi, session::RealtimeSession};
use super::connection::map_webrtc_connection_state;

impl RealtimeAudioApi {
    /// Set up WebRTC connection
    pub(crate) async fn setup_webrtc_connection(
        &self,
        session: &Arc<RealtimeSession>,
        _response: &RealtimeSessionResponse,
    ) -> Result<()> {
        let session_weak = Arc::downgrade(session);

        self.setup_connection_state_handler(session).await;
        self.setup_data_channel(session, &session_weak).await?;
        self.setup_audio_track(session, &session_weak).await?;

        Ok(())
    }

    /// Sets up the connection state change handler
    async fn setup_connection_state_handler(&self, session: &Arc<RealtimeSession>) {
        let session_clone = session.clone();
        let peer_connection = session.peer_connection();

        peer_connection.on_peer_connection_state_change(Box::new(move |state| {
            let session_clone = session_clone.clone();
            Box::pin(async move {
                let webrtc_state = map_webrtc_connection_state(state);
                session_clone.set_connection_state(webrtc_state).await;

                info!("WebRTC connection state changed to: {:?}", webrtc_state);
            })
        }));
    }

    /// Sets up the data channel for event communication
    async fn setup_data_channel(
        &self,
        session: &Arc<RealtimeSession>,
        session_weak: &std::sync::Weak<RealtimeSession>,
    ) -> Result<()> {
        let peer_connection = session.peer_connection();

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

        session.set_data_channel(data_channel.clone()).await;
        self.setup_data_channel_handlers(&data_channel, session_weak);

        Ok(())
    }

    /// Sets up data channel message handlers
    fn setup_data_channel_handlers(
        &self,
        data_channel: &Arc<RTCDataChannel>,
        session_weak: &std::sync::Weak<RealtimeSession>,
    ) {
        let session_for_events = session_weak.clone();
        data_channel.on_message(Box::new(move |msg| {
            let session_for_events = session_for_events.clone();
            Box::pin(async move {
                if let Some(session) = session_for_events.upgrade() {
                    if let Err(e) = session.handle_data_channel_message(msg).await {
                        log::error!("Failed to handle data channel message: {e}");
                    }
                }
            })
        }));
    }

    /// Sets up audio track for sending audio
    async fn setup_audio_track(
        &self,
        session: &Arc<RealtimeSession>,
        session_weak: &std::sync::Weak<RealtimeSession>,
    ) -> Result<()> {
        let audio_track = self.create_audio_track();
        session.set_audio_track(audio_track.clone()).await;

        self.add_audio_track_to_connection(session, audio_track)
            .await?;
        self.setup_incoming_track_handler(session, session_weak);

        Ok(())
    }

    /// Adds the audio track to the peer connection
    async fn add_audio_track_to_connection(
        &self,
        session: &Arc<RealtimeSession>,
        audio_track: Arc<TrackLocalStaticSample>,
    ) -> Result<()> {
        let peer_connection = session.peer_connection();
        let _rtp_sender = peer_connection
            .add_track(audio_track as Arc<dyn TrackLocal + Send + Sync>)
            .await
            .map_err(crate::invalid_request_err!("Failed to add audio track: {}"))?;
        Ok(())
    }

    /// Sets up handler for incoming audio tracks
    fn setup_incoming_track_handler(
        &self,
        session: &Arc<RealtimeSession>,
        session_weak: &std::sync::Weak<RealtimeSession>,
    ) {
        let peer_connection = session.peer_connection();
        let session_for_track = session_weak.clone();

        peer_connection.on_track(Box::new(move |track, _receiver, _transceiver| {
            let session_for_track = session_for_track.clone();
            Box::pin(async move {
                if let Some(session) = session_for_track.upgrade() {
                    session.handle_incoming_track(track).await;
                }
            })
        }));
    }
}
