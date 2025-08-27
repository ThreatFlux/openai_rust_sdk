//! # Audio Utilities
//!
//! Helper functions and utilities for the Audio API.

use super::types::*;
use crate::api::audio::AudioApi;

impl AudioApi {
    /// Get supported audio formats for input
    #[must_use]
    pub fn supported_input_formats() -> Vec<&'static str> {
        vec![
            "flac", "m4a", "mp3", "mp4", "mpeg", "mpga", "oga", "ogg", "wav", "webm",
        ]
    }

    /// Get supported audio formats for output
    #[must_use]
    pub fn supported_output_formats() -> Vec<AudioFormat> {
        vec![
            AudioFormat::Mp3,
            AudioFormat::Opus,
            AudioFormat::Aac,
            AudioFormat::Flac,
            AudioFormat::Wav,
            AudioFormat::Pcm,
        ]
    }

    /// Get available voices
    #[must_use]
    pub fn available_voices() -> Vec<Voice> {
        vec![
            Voice::Alloy,
            Voice::Echo,
            Voice::Fable,
            Voice::Onyx,
            Voice::Nova,
            Voice::Shimmer,
        ]
    }

    /// Validate audio file format
    pub fn is_supported_format(file_path: impl AsRef<Path>) -> bool {
        if let Some(extension) = file_path.as_ref().extension() {
            if let Some(ext_str) = extension.to_str() {
                return Self::supported_input_formats().contains(&ext_str.to_lowercase().as_str());
            }
        }
        false
    }
}

/// Audio utilities
pub struct AudioUtils;

impl AudioUtils {
    /// Estimate audio duration from file size (rough approximation)
    /// This is a very rough estimate based on typical bitrates
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn estimate_duration_from_size(file_size_bytes: u64, format: &AudioFormat) -> f64 {
        let bits_per_second = match format {
            AudioFormat::Mp3 | AudioFormat::Aac => 128_000.0, // 128 kbps
            AudioFormat::Opus => 64_000.0,                    // 64 kbps
            AudioFormat::Flac | AudioFormat::Wav | AudioFormat::Pcm => 1_411_000.0, // CD quality
        };

        (file_size_bytes as f64 * 8.0) / bits_per_second
    }

    /// Calculate estimated cost for text-to-speech
    /// Based on `OpenAI` pricing (as of 2024)
    #[must_use]
    pub fn estimate_tts_cost(text: &str, model: &str) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let chars = text.len() as f64;

        match model {
            "tts-1-hd" => chars * 0.00003, // $0.030 per 1K characters
            _ => chars * 0.000_015,        // tts-1 and default pricing
        }
    }

    /// Calculate estimated cost for transcription/translation
    /// Based on `OpenAI` pricing (as of 2024)
    #[must_use]
    pub fn estimate_whisper_cost(duration_seconds: f64) -> f64 {
        let minutes = duration_seconds / 60.0;
        minutes * 0.006 // $0.006 per minute
    }

    /// Get recommended voice for different use cases
    #[must_use]
    pub fn recommend_voice(use_case: &str) -> Voice {
        match use_case.to_lowercase().as_str() {
            "professional" | "business" | "corporate" => Voice::Onyx,
            "friendly" | "customer_service" | "welcoming" => Voice::Shimmer,
            "storytelling" | "narrative" | "audiobook" => Voice::Fable,
            "energetic" | "upbeat" | "marketing" => Voice::Nova,
            "deep" | "authoritative" | "announcement" => Voice::Echo,
            _ => Voice::Alloy, // Default balanced voice
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_formats() {
        let formats = AudioApi::supported_input_formats();
        assert!(formats.contains(&"mp3"));
        assert!(formats.contains(&"wav"));
        assert!(formats.contains(&"flac"));
    }

    #[test]
    fn test_format_validation() {
        assert!(AudioApi::is_supported_format("test.mp3"));
        assert!(AudioApi::is_supported_format("test.wav"));
        assert!(!AudioApi::is_supported_format("test.txt"));
        assert!(!AudioApi::is_supported_format("test"));
    }

    #[test]
    fn test_available_voices() {
        let voices = AudioApi::available_voices();
        assert_eq!(voices.len(), 6);
        assert!(voices.contains(&Voice::Alloy));
        assert!(voices.contains(&Voice::Nova));
    }

    #[test]
    fn test_duration_estimation() {
        let duration = AudioUtils::estimate_duration_from_size(1_000_000, &AudioFormat::Mp3);
        assert!(duration > 0.0);

        let flac_duration = AudioUtils::estimate_duration_from_size(1_000_000, &AudioFormat::Flac);
        assert!(flac_duration < duration); // FLAC should be shorter duration for same file size
    }

    #[test]
    fn test_cost_estimation() {
        let tts_cost = AudioUtils::estimate_tts_cost("Hello world", "tts-1");
        assert!(tts_cost > 0.0);

        let hd_cost = AudioUtils::estimate_tts_cost("Hello world", "tts-1-hd");
        assert!(hd_cost > tts_cost);

        let whisper_cost = AudioUtils::estimate_whisper_cost(60.0); // 1 minute
        assert!(whisper_cost > 0.0);
    }

    #[test]
    fn test_voice_recommendations() {
        assert_eq!(AudioUtils::recommend_voice("professional"), Voice::Onyx);
        assert_eq!(AudioUtils::recommend_voice("friendly"), Voice::Shimmer);
        assert_eq!(AudioUtils::recommend_voice("storytelling"), Voice::Fable);
        assert_eq!(AudioUtils::recommend_voice("energetic"), Voice::Nova);
        assert_eq!(AudioUtils::recommend_voice("deep"), Voice::Echo);
        assert_eq!(AudioUtils::recommend_voice("unknown"), Voice::Alloy);
    }
}
