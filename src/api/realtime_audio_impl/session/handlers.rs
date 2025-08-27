//! # Session Event and Audio Handlers
//!
//! Event and audio handling functionality for real-time sessions.

use crate::error::{OpenAIError, Result};
use crate::models::realtime_audio::{AudioBuffer, RealtimeEvent};
use log::warn;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::media::Sample;
use webrtc::track::track_remote::TrackRemote;

use super::types::RealtimeSession;

impl RealtimeSession {
    /// Send an event to the server
    pub async fn send_event(&self, event: RealtimeEvent) -> Result<()> {
        if let Some(data_channel) = self.data_channel().await {
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
        if let Some(audio_track) = self.audio_track().await {
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
        log::info!("Audio input started for session: {}", self.id);
        Ok(())
    }

    /// Stop audio streaming
    pub async fn stop_audio_input(&self) -> Result<()> {
        // This would stop capturing audio from microphone
        log::info!("Audio input stopped for session: {}", self.id);
        Ok(())
    }

    /// Handle incoming data channel message
    pub async fn handle_data_channel_message(&self, msg: DataChannelMessage) -> Result<()> {
        if let Ok(text) = String::from_utf8(msg.data.to_vec()) {
            match serde_json::from_str::<RealtimeEvent>(&text) {
                Ok(event) => {
                    if let Err(e) = self.event_sender().send(event) {
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
    pub async fn handle_incoming_track(&self, track: Arc<TrackRemote>) {
        let audio_sender = self.audio_sender().clone();
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
                        log::error!("Failed to read RTP packet: {e}");
                        break;
                    }
                }
            }
        });
    }
}
