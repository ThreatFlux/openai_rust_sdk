//! # WebRTC Setup and Configuration
//!
//! WebRTC session setup, track management, and event handlers.

use crate::error::{OpenAIError, Result};
use crate::models::realtime_audio::RealtimeSessionResponse;
use std::sync::Arc;
use webrtc::data_channel::{DataChannel, DataChannelEvent, RTCDataChannelInit};
use webrtc::media_stream::track_local::{TrackLocal, static_sample::TrackLocalStaticSample};

use super::super::{client::RealtimeAudioApi, session::RealtimeSession};

impl RealtimeAudioApi {
    /// Set up WebRTC connection
    pub(crate) async fn setup_webrtc_connection(
        &self,
        session: &Arc<RealtimeSession>,
        _response: &RealtimeSessionResponse,
    ) -> Result<()> {
        let session_weak = Arc::downgrade(session);

        self.setup_data_channel(session, &session_weak).await?;
        self.setup_audio_track(session).await?;

        Ok(())
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
                    ordered: true,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| {
                OpenAIError::InvalidRequest(format!("Failed to create data channel: {e}"))
            })?;

        session.set_data_channel(data_channel.clone()).await;
        self.setup_data_channel_handlers(data_channel, session_weak);

        Ok(())
    }

    /// Sets up data channel message handlers
    fn setup_data_channel_handlers(
        &self,
        data_channel: Arc<dyn DataChannel>,
        session_weak: &std::sync::Weak<RealtimeSession>,
    ) {
        let session_for_events = session_weak.clone();
        tokio::spawn(async move {
            while let Some(event) = data_channel.poll().await {
                match event {
                    DataChannelEvent::OnMessage(msg) => {
                        if let Some(session) = session_for_events.upgrade()
                            && let Err(e) = session.handle_data_channel_message(msg).await
                        {
                            log::error!("Failed to handle data channel message: {e}");
                        }
                    }
                    DataChannelEvent::OnClose => break,
                    _ => {}
                }
            }
        });
    }

    /// Sets up audio track for sending audio
    async fn setup_audio_track(&self, session: &Arc<RealtimeSession>) -> Result<()> {
        let audio_track = self.create_audio_track()?;
        session.set_audio_track(audio_track.clone()).await;

        self.add_audio_track_to_connection(session, audio_track)
            .await?;

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
            .add_track(audio_track as Arc<dyn TrackLocal>)
            .await
            .map_err(crate::invalid_request_err!("Failed to add audio track: {}"))?;
        Ok(())
    }
}
