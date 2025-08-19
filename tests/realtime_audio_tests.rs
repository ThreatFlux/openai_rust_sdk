//! # Real-time Audio API Tests
//!
//! Comprehensive tests for the WebRTC-based real-time audio API

use openai_rust_sdk::api::realtime_audio::{
    AudioProcessor, RealtimeAudioApi, RealtimeAudioConfig, RealtimeSessionBuilder,
    VoiceActivityDetector,
};
use openai_rust_sdk::models::realtime_audio::*;

#[tokio::test]
async fn test_realtime_audio_api_creation() {
    let api = RealtimeAudioApi::new("test-api-key").unwrap();

    // Test that the API is created with default config
    assert_eq!(api.config.sample_rate, 24000);
    assert_eq!(api.config.channels, 1);
    assert_eq!(api.config.buffer_size_ms, 20);
    assert!(api.config.enable_aec);
    assert!(api.config.enable_noise_suppression);
    assert!(api.config.enable_agc);
}

#[tokio::test]
async fn test_custom_config_creation() {
    let custom_config = RealtimeAudioConfig {
        sample_rate: 48000,
        channels: 2,
        buffer_size_ms: 10,
        enable_aec: false,
        enable_noise_suppression: false,
        enable_agc: false,
        ..Default::default()
    };

    let api = RealtimeAudioApi::new_with_config("test-key", custom_config.clone()).unwrap();
    assert_eq!(api.config.sample_rate, 48000);
    assert_eq!(api.config.channels, 2);
    assert_eq!(api.config.buffer_size_ms, 10);
    assert!(!api.config.enable_aec);
}

#[test]
fn test_session_request_builder() {
    let request = RealtimeSessionBuilder::gpt_4o_realtime()
        .instructions("Test instructions")
        .voice(RealtimeVoice::Nova)
        .temperature(0.9)
        .max_response_tokens(2048)
        .build();

    assert_eq!(request.model, RealtimeAudioModels::GPT_4O_REALTIME_PREVIEW);
    assert_eq!(request.instructions, Some("Test instructions".to_string()));
    assert_eq!(request.voice, Some(RealtimeVoice::Nova));
    assert_eq!(request.temperature, Some(0.9));
    assert_eq!(request.max_response_output_tokens, Some(2048));
}

#[test]
fn test_session_config_defaults() {
    let config = RealtimeSessionConfig::default();

    assert_eq!(config.input_audio_format, Some(RealtimeAudioFormat::Pcm16));
    assert_eq!(config.output_audio_format, Some(RealtimeAudioFormat::Pcm16));
    assert!(config.voice_activity_detection.is_some());
    assert!(config.turn_detection.is_some());
    assert_eq!(
        config.modalities,
        Some(vec![RealtimeModality::Text, RealtimeModality::Audio])
    );
}

#[test]
fn test_audio_buffer_operations() {
    // Test mono audio buffer
    let mono_samples = vec![100, -200, 300, -400, 500];
    let mono_buffer = AudioBuffer::new(mono_samples.clone(), 24000, 1);

    assert_eq!(mono_buffer.samples, mono_samples);
    assert_eq!(mono_buffer.sample_rate, 24000);
    assert_eq!(mono_buffer.channels, 1);
    assert_eq!(mono_buffer.frame_count(), 5);
    assert!(mono_buffer.duration_seconds() > 0.0);

    let mono_converted = mono_buffer.to_mono();
    assert_eq!(mono_converted, mono_samples);

    let energy = mono_buffer.rms_energy();
    assert!(energy > 0.0);

    // Test stereo audio buffer
    let stereo_samples = vec![100, -100, 200, -200, 300, -300];
    let stereo_buffer = AudioBuffer::new(stereo_samples, 24000, 2);

    assert_eq!(stereo_buffer.channels, 2);
    assert_eq!(stereo_buffer.frame_count(), 3);

    let stereo_to_mono = stereo_buffer.to_mono();
    assert_eq!(stereo_to_mono.len(), 3);
    assert_eq!(stereo_to_mono[0], 0); // (100 + (-100)) / 2
    assert_eq!(stereo_to_mono[1], 0); // (200 + (-200)) / 2
    assert_eq!(stereo_to_mono[2], 0); // (300 + (-300)) / 2
}

#[test]
fn test_voice_activity_detector() {
    let config = VoiceActivityDetectionConfig {
        threshold: 0.5,
        prefix_padding_ms: 300,
        silence_duration_ms: 500,
    };

    let mut vad = VoiceActivityDetector::new(config, 24000);

    // Test with silence
    let silence = AudioBuffer::new(vec![0; 480], 24000, 1);
    let result = vad.process(&silence);
    assert!(!result.is_speech);
    assert_eq!(result.confidence, 0.0);

    // Test with speech-like audio (process silence first to build baseline)
    let _ = vad.process(&silence);
    let speech = AudioBuffer::new(vec![10000; 480], 24000, 1);
    let result = vad.process(&speech);
    assert!(result.is_speech);
    assert!(result.confidence > 0.0);
    assert!(result.energy > 0.0);

    // Test state persistence
    assert!(vad.is_speaking());
}

#[test]
fn test_audio_processor() {
    let mut processor = AudioProcessor::new(24000, 1, true, true, true);
    let mut audio = AudioBuffer::new(vec![50, 2000, -50, -2000, 100], 24000, 1);

    let original_energy = audio.rms_energy();
    processor.process(&mut audio);

    // Verify that processing occurred
    // Note: The exact values depend on the processing algorithms
    assert_eq!(audio.samples[0], 0); // Should be noise-gated (below threshold)
    assert_ne!(audio.samples[1], 2000); // Should be modified by AGC
    assert_eq!(audio.samples[2], 0); // Should be noise-gated

    // Energy should have changed due to processing
    let processed_energy = audio.rms_energy();
    assert_ne!(original_energy, processed_energy);
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

    let deserialized: RealtimeEvent = serde_json::from_str(&json).unwrap();
    match deserialized {
        RealtimeEvent::SessionUpdate { event_id, .. } => {
            assert_eq!(event_id, "test-123");
        }
        _ => panic!("Unexpected event type"),
    }
}

#[test]
fn test_conversation_item_creation() {
    let item = ConversationItem {
        id: Some("item-123".to_string()),
        object: "realtime.item".to_string(),
        item_type: ConversationItemType::Message,
        status: ConversationItemStatus::Completed,
        role: ConversationRole::User,
        content: vec![
            ContentPart::Text {
                text: "Hello, world!".to_string(),
            },
            ContentPart::Audio {
                audio: Some("base64-encoded-data".to_string()),
                transcript: Some("Hello, world!".to_string()),
            },
        ],
    };

    assert_eq!(item.id, Some("item-123".to_string()));
    assert_eq!(item.role, ConversationRole::User);
    assert_eq!(item.content.len(), 2);

    match &item.content[0] {
        ContentPart::Text { text } => {
            assert_eq!(text, "Hello, world!");
        }
        _ => panic!("Expected text content"),
    }

    match &item.content[1] {
        ContentPart::Audio { audio, transcript } => {
            assert_eq!(audio, &Some("base64-encoded-data".to_string()));
            assert_eq!(transcript, &Some("Hello, world!".to_string()));
        }
        _ => panic!("Expected audio content"),
    }
}

#[test]
fn test_webrtc_stats_default() {
    let stats = WebRtcStats::default();

    assert_eq!(stats.connection_state, WebRtcConnectionState::New);
    assert_eq!(stats.audio_bytes_sent, 0);
    assert_eq!(stats.audio_bytes_received, 0);
    assert_eq!(stats.audio_packets_sent, 0);
    assert_eq!(stats.audio_packets_received, 0);
    assert_eq!(stats.audio_packets_lost, 0);
    assert!(stats.round_trip_time_ms.is_none());
    assert!(stats.jitter_ms.is_none());
    assert!(stats.connected_at.is_none());
}

#[test]
fn test_voice_and_format_serialization() {
    // Test voice serialization
    let voices = [
        (RealtimeVoice::Alloy, "\"alloy\""),
        (RealtimeVoice::Echo, "\"echo\""),
        (RealtimeVoice::Fable, "\"fable\""),
        (RealtimeVoice::Onyx, "\"onyx\""),
        (RealtimeVoice::Nova, "\"nova\""),
        (RealtimeVoice::Shimmer, "\"shimmer\""),
    ];

    for (voice, expected) in voices {
        let json = serde_json::to_string(&voice).unwrap();
        assert_eq!(json, expected);
    }

    // Test audio format serialization
    let formats = [
        (RealtimeAudioFormat::Pcm16, "\"pcm16\""),
        (RealtimeAudioFormat::G711Ulaw, "\"g711_ulaw\""),
        (RealtimeAudioFormat::G711Alaw, "\"g711_alaw\""),
    ];

    for (format, expected) in formats {
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, expected);
    }
}

#[test]
fn test_response_config_builder() {
    let config = ResponseConfig {
        modalities: Some(vec![RealtimeModality::Audio]),
        instructions: Some("Respond briefly".to_string()),
        voice: Some(RealtimeVoice::Echo),
        output_audio_format: Some(RealtimeAudioFormat::Pcm16),
        tools: None,
        tool_choice: Some("none".to_string()),
        temperature: Some(0.7),
        max_response_output_tokens: Some(1024),
    };

    assert_eq!(config.modalities, Some(vec![RealtimeModality::Audio]));
    assert_eq!(config.voice, Some(RealtimeVoice::Echo));
    assert_eq!(config.temperature, Some(0.7));
    assert_eq!(config.max_response_output_tokens, Some(1024));
}

#[test]
fn test_error_handling() {
    let error = RealtimeError {
        error_type: "invalid_request".to_string(),
        code: Some("400".to_string()),
        message: "Invalid session configuration".to_string(),
        param: Some("config.voice_activity_detection".to_string()),
        event_id: Some("evt-123".to_string()),
    };

    assert_eq!(error.error_type, "invalid_request");
    assert_eq!(error.code, Some("400".to_string()));
    assert!(error.message.contains("Invalid session"));
    assert_eq!(
        error.param,
        Some("config.voice_activity_detection".to_string())
    );
    assert_eq!(error.event_id, Some("evt-123".to_string()));
}

#[tokio::test]
async fn test_session_state_transitions() {
    let states = [
        WebRtcConnectionState::New,
        WebRtcConnectionState::Connecting,
        WebRtcConnectionState::Connected,
        WebRtcConnectionState::Disconnected,
        WebRtcConnectionState::Failed,
        WebRtcConnectionState::Closed,
    ];

    for state in states {
        // Test that all states are properly handled
        match state {
            WebRtcConnectionState::New => assert_eq!(state, WebRtcConnectionState::New),
            WebRtcConnectionState::Connecting => {
                assert_eq!(state, WebRtcConnectionState::Connecting)
            }
            WebRtcConnectionState::Connected => assert_eq!(state, WebRtcConnectionState::Connected),
            WebRtcConnectionState::Disconnected => {
                assert_eq!(state, WebRtcConnectionState::Disconnected)
            }
            WebRtcConnectionState::Failed => assert_eq!(state, WebRtcConnectionState::Failed),
            WebRtcConnectionState::Closed => assert_eq!(state, WebRtcConnectionState::Closed),
        }
    }
}

#[test]
fn test_audio_models_constants() {
    assert_eq!(
        RealtimeAudioModels::GPT_4O_REALTIME_PREVIEW,
        "gpt-4o-realtime-preview"
    );
    assert_eq!(
        RealtimeAudioModels::GPT_4O_MINI_REALTIME_PREVIEW,
        "gpt-4o-mini-realtime-preview"
    );
}

#[test]
fn test_vad_config_defaults() {
    let config = VoiceActivityDetectionConfig::default();

    assert_eq!(config.threshold, 0.5);
    assert_eq!(config.prefix_padding_ms, 300);
    assert_eq!(config.silence_duration_ms, 200);
}

// Integration test helper functions
mod test_helpers {
    use super::*;

    pub fn create_test_audio_buffer(
        duration_ms: u32,
        sample_rate: u32,
        amplitude: i16,
    ) -> AudioBuffer {
        let samples_count = (duration_ms * sample_rate / 1000) as usize;
        let samples = vec![amplitude; samples_count];
        AudioBuffer::new(samples, sample_rate, 1)
    }

    pub fn create_test_session_config() -> RealtimeSessionConfig {
        RealtimeSessionConfig {
            input_audio_format: Some(RealtimeAudioFormat::Pcm16),
            output_audio_format: Some(RealtimeAudioFormat::Pcm16),
            voice_activity_detection: Some(VoiceActivityDetectionConfig::default()),
            turn_detection: Some(TurnDetectionConfig {
                detection_type: TurnDetectionType::ServerVad,
                threshold: Some(0.5),
                prefix_padding_ms: Some(300),
                silence_duration_ms: Some(500),
            }),
            tools: None,
            tool_choice: Some("auto".to_string()),
            modalities: Some(vec![RealtimeModality::Audio]),
        }
    }
}

#[test]
fn test_helper_functions() {
    use test_helpers::*;

    let audio = create_test_audio_buffer(100, 24000, 1000);
    assert_eq!(audio.sample_rate, 24000);
    assert_eq!(audio.channels, 1);
    assert!(audio.samples.iter().all(|&s| s == 1000));

    let config = create_test_session_config();
    assert_eq!(config.input_audio_format, Some(RealtimeAudioFormat::Pcm16));
    assert!(config.voice_activity_detection.is_some());
}
