//! # Real-time Audio API
//!
//! This module provides WebRTC-based real-time audio streaming capabilities
//! for OpenAI's real-time audio API, supporting bidirectional audio streaming,
//! voice activity detection, and low-latency communication.

/// Audio processing and effects
pub mod audio_processor;
/// Builder patterns for session creation
pub mod builders;
/// Core client functionality
pub mod client;
/// Configuration structures
pub mod config;
/// Session management
pub mod session;
/// Voice activity detection
pub mod vad;
/// WebRTC connection management
pub mod webrtc;

// Re-export all public items to maintain API compatibility
pub use audio_processor::*;
pub use builders::*;
pub use client::*;
pub use config::*;
pub use session::*;
pub use vad::*;
pub use webrtc::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::realtime_audio::{
        RealtimeEvent, RealtimeSessionConfig, RealtimeVoice, VoiceActivityDetectionConfig,
    };
    use tokio::sync::mpsc;

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

        assert_eq!(
            request.model,
            crate::models::realtime_audio::RealtimeAudioModels::GPT_4O_REALTIME_PREVIEW
        );
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
        let silence = crate::models::realtime_audio::AudioBuffer::new(vec![0; 480], 24000, 1); // 20ms of silence
        let result = vad.process(&silence);
        assert!(!result.is_speech);
        assert_eq!(result.confidence, 0.0);

        // Test with loud audio (process twice to build history)
        let loud_audio =
            crate::models::realtime_audio::AudioBuffer::new(vec![10000; 480], 24000, 1);
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
        let mut audio =
            crate::models::realtime_audio::AudioBuffer::new(vec![50, 2000, -50, -2000], 24000, 1);

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
        let (_audio_sender, _audio_receiver) =
            mpsc::unbounded_channel::<crate::models::realtime_audio::AudioBuffer>();

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
