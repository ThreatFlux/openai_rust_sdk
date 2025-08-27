//! # Audio Models
//!
//! Data structures for the OpenAI Audio API including text-to-speech,
//! speech-to-text transcription, and translation endpoints.

/// Audio API builders for fluent request creation
pub mod builders;
/// Audio model constants
pub mod models;
/// Audio API request structures
pub mod requests;
/// Audio API response structures  
pub mod responses;
/// Core audio types and enums
pub mod types;

// Re-export all public items to maintain API compatibility
pub use builders::*;
pub use models::*;
pub use requests::*;
pub use responses::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common_builder::{Builder, WithFormat, WithSpeed, WithTemperature};

    #[test]
    fn test_speech_request_creation() {
        let req = AudioSpeechRequest::new("tts-1", "Hello world", Voice::Alloy);
        assert_eq!(req.model, "tts-1");
        assert_eq!(req.input, "Hello world");
        matches!(req.voice, Voice::Alloy);
    }

    #[test]
    fn test_speech_builder() {
        let req = SpeechBuilder::tts_1_hd("Test speech", Voice::Nova)
            .format(AudioFormat::Mp3)
            .speed(1.5)
            .build();

        assert_eq!(req.model, AudioModels::TTS_1_HD);
        assert_eq!(req.input, "Test speech");
        matches!(req.voice, Voice::Nova);
        assert_eq!(req.response_format, Some(AudioFormat::Mp3));
        assert_eq!(req.speed, Some(1.5));
    }

    #[test]
    fn test_transcription_request_creation() {
        let req = AudioTranscriptionRequest::new("audio.mp3", "whisper-1");
        assert_eq!(req.file, "audio.mp3");
        assert_eq!(req.model, "whisper-1");
    }

    #[test]
    fn test_transcription_builder() {
        let req = TranscriptionBuilder::whisper("test.wav")
            .language("en")
            .verbose_json()
            .word_timestamps()
            .temperature(0.3)
            .build();

        assert_eq!(req.file, "test.wav");
        assert_eq!(req.model, AudioModels::WHISPER_1);
        assert_eq!(req.language, Some("en".to_string()));
        assert_eq!(req.response_format, Some(TranscriptionFormat::VerboseJson));
        assert_eq!(req.temperature, Some(0.3));
        assert!(req.timestamp_granularities.is_some());
    }

    #[test]
    fn test_translation_builder() {
        let req = TranslationBuilder::whisper("spanish.mp3")
            .prompt("Translate this Spanish audio")
            .json()
            .build();

        assert_eq!(req.file, "spanish.mp3");
        assert_eq!(req.model, AudioModels::WHISPER_1);
        assert_eq!(req.prompt, Some("Translate this Spanish audio".to_string()));
        assert_eq!(req.response_format, Some(TranscriptionFormat::Json));
    }

    #[test]
    fn test_voice_serialization() {
        assert_eq!(serde_json::to_string(&Voice::Alloy).unwrap(), "\"alloy\"");
        assert_eq!(serde_json::to_string(&Voice::Echo).unwrap(), "\"echo\"");
        assert_eq!(serde_json::to_string(&Voice::Fable).unwrap(), "\"fable\"");
        assert_eq!(serde_json::to_string(&Voice::Onyx).unwrap(), "\"onyx\"");
        assert_eq!(serde_json::to_string(&Voice::Nova).unwrap(), "\"nova\"");
        assert_eq!(
            serde_json::to_string(&Voice::Shimmer).unwrap(),
            "\"shimmer\""
        );
    }

    #[test]
    fn test_audio_format_serialization() {
        assert_eq!(serde_json::to_string(&AudioFormat::Mp3).unwrap(), "\"mp3\"");
        assert_eq!(
            serde_json::to_string(&AudioFormat::Opus).unwrap(),
            "\"opus\""
        );
        assert_eq!(serde_json::to_string(&AudioFormat::Aac).unwrap(), "\"aac\"");
        assert_eq!(
            serde_json::to_string(&AudioFormat::Flac).unwrap(),
            "\"flac\""
        );
    }

    #[test]
    fn test_speed_clamping() {
        let req = SpeechBuilder::tts_1("test", Voice::Alloy)
            .speed(10.0) // Should be clamped to 4.0
            .build();
        assert_eq!(req.speed, Some(4.0));

        let req2 = SpeechBuilder::tts_1("test", Voice::Alloy)
            .speed(0.1) // Should be clamped to 0.25
            .build();
        assert_eq!(req2.speed, Some(0.25));
    }

    #[test]
    fn test_temperature_clamping() {
        let req = TranscriptionBuilder::whisper("test.mp3")
            .temperature(2.0) // Should be clamped to 1.0
            .build();
        assert_eq!(req.temperature, Some(1.0));

        let req2 = TranscriptionBuilder::whisper("test.mp3")
            .temperature(-1.0) // Should be clamped to 0.0
            .build();
        assert_eq!(req2.temperature, Some(0.0));
    }

    #[test]
    fn test_audio_response_methods() {
        let response = AudioSpeechResponse::new(vec![1, 2, 3, 4], "audio/mpeg".to_string());
        assert_eq!(response.data(), &[1, 2, 3, 4]);
        assert_eq!(response.content_type(), "audio/mpeg");
    }

    #[test]
    fn test_transcription_response_methods() {
        let response = AudioTranscriptionResponse {
            text: "Hello world".to_string(),
            language: Some("en".to_string()),
            duration: Some(2.5),
            words: None,
            segments: None,
        };

        assert!(response.has_metadata());
        assert_eq!(response.word_count(), 2);
        assert_eq!(response.duration(), Some(2.5));
    }
}
