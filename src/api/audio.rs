//! # Audio API
//!
//! This module provides access to OpenAI's Audio API for text-to-speech,
//! speech-to-text transcription, and translation.

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::api::shared_utilities::{
    AudioResponseHandler, FormBuilder, MultipartRequestExecutor, RequestValidator,
};
use crate::error::{OpenAIError, Result};
use crate::models::audio::{
    AudioFormat, AudioModels, AudioSpeechRequest, AudioSpeechResponse, AudioTranscriptionRequest,
    AudioTranscriptionResponse, AudioTranslationRequest, AudioTranslationResponse, Voice,
};
#[cfg(test)]
use crate::models::audio::{
    SpeechBuilder, TranscriptionBuilder, TranscriptionFormat, TranslationBuilder,
};
use bytes::Bytes;
use std::path::Path;
use tokio::fs;
use tokio_stream::StreamExt;

/// Audio API client
pub struct AudioApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for AudioApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl AudioApi {
    /// Create speech from text using text-to-speech
    pub async fn create_speech(&self, request: &AudioSpeechRequest) -> Result<AudioSpeechResponse> {
        let (audio_data, content_type) = self
            .http_client
            .post_bytes_with_content_type("/v1/audio/speech", request)
            .await?;

        let content_type = if content_type == "application/octet-stream" {
            "audio/mpeg".to_string()
        } else {
            content_type
        };

        Ok(AudioSpeechResponse::new(audio_data, content_type))
    }

    /// Create speech with streaming response
    pub async fn create_speech_stream(
        &self,
        request: &AudioSpeechRequest,
    ) -> Result<impl tokio_stream::Stream<Item = Result<Bytes>>> {
        let response = self
            .http_client
            .post_stream("/v1/audio/speech", request)
            .await?;

        let stream = response
            .bytes_stream()
            .map(|chunk| chunk.map_err(crate::request_err!(to_string)));

        Ok(stream)
    }

    /// Transcribe audio to text
    pub async fn create_transcription(
        &self,
        request: &AudioTranscriptionRequest,
        file_data: Vec<u8>,
    ) -> Result<AudioTranscriptionResponse> {
        // Validate inputs
        RequestValidator::validate_file_not_empty(&file_data)?;
        RequestValidator::validate_required_string(&request.model, "model")?;
        RequestValidator::validate_temperature(request.temperature)?;

        // Build form using shared utilities
        let form = FormBuilder::build_transcription_form(
            file_data,
            request.file.clone(),
            request.model.clone(),
            request.language.as_ref(),
            request.prompt.as_ref(),
            request.response_format.as_ref(),
            request.temperature,
            request.timestamp_granularities.as_ref(),
        )?;

        // Send request using shared utilities
        let response = MultipartRequestExecutor::send_multipart_request(
            &self.http_client,
            "/v1/audio/transcriptions",
            form,
        )
        .await?;

        // Handle response using shared utilities
        AudioResponseHandler::handle_flexible_response(response, |text| {
            AudioTranscriptionResponse {
                text,
                language: None,
                duration: None,
                words: None,
                segments: None,
            }
        })
        .await
    }

    /// Transcribe audio from file path
    pub async fn transcribe_file(
        &self,
        file_path: impl AsRef<Path>,
        request: &AudioTranscriptionRequest,
    ) -> Result<AudioTranscriptionResponse> {
        let file_data = crate::helpers::read_bytes(file_path).await?;

        self.create_transcription(request, file_data).await
    }

    /// Translate audio to English text
    pub async fn create_translation(
        &self,
        request: &AudioTranslationRequest,
        file_data: Vec<u8>,
    ) -> Result<AudioTranslationResponse> {
        // Validate inputs
        RequestValidator::validate_file_not_empty(&file_data)?;
        RequestValidator::validate_required_string(&request.model, "model")?;
        RequestValidator::validate_temperature(request.temperature)?;

        // Build form using shared utilities
        let form = FormBuilder::build_translation_form(
            file_data,
            request.file.clone(),
            request.model.clone(),
            request.prompt.as_ref(),
            request.response_format.as_ref(),
            request.temperature,
        )?;

        // Send request using shared utilities
        let response = MultipartRequestExecutor::send_multipart_request(
            &self.http_client,
            "/v1/audio/translations",
            form,
        )
        .await?;

        // Handle response using shared utilities
        AudioResponseHandler::handle_flexible_response(response, |text| AudioTranslationResponse {
            text,
            duration: None,
            segments: None,
        })
        .await
    }

    /// Translate audio from file path
    pub async fn translate_file(
        &self,
        file_path: impl AsRef<Path>,
        request: &AudioTranslationRequest,
    ) -> Result<AudioTranslationResponse> {
        let file_data = crate::helpers::read_bytes(file_path).await?;

        self.create_translation(request, file_data).await
    }

    // Convenience methods

    /// Generate speech with simple parameters
    pub async fn generate_speech(
        &self,
        text: impl Into<String>,
        voice: Voice,
        model: Option<&str>,
    ) -> Result<AudioSpeechResponse> {
        let model = model.unwrap_or(AudioModels::TTS_1);
        let request = AudioSpeechRequest::new(model, text, voice);
        self.create_speech(&request).await
    }

    /// Generate speech and save to file
    pub async fn generate_speech_to_file(
        &self,
        text: impl Into<String>,
        voice: Voice,
        output_path: impl AsRef<Path>,
        model: Option<&str>,
        format: Option<AudioFormat>,
    ) -> Result<()> {
        let model = model.unwrap_or(AudioModels::TTS_1);
        let mut request = AudioSpeechRequest::new(model, text, voice);

        if let Some(fmt) = format {
            request.response_format = Some(fmt);
        }

        let response = self.create_speech(&request).await?;
        response
            .save_to_file(output_path)
            .await
            .map_err(crate::file_err!("Failed to save audio: {}"))?;

        Ok(())
    }

    /// Quick transcription with minimal parameters
    pub async fn transcribe(
        &self,
        file_path: impl AsRef<Path>,
        language: Option<&str>,
    ) -> Result<String> {
        let file_name = file_path
            .as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.mp3")
            .to_string();

        let mut request = AudioTranscriptionRequest::new(file_name, AudioModels::WHISPER_1);

        if let Some(lang) = language {
            request.language = Some(lang.to_string());
        }

        let response = self.transcribe_file(file_path, &request).await?;
        Ok(response.text)
    }

    /// Quick translation with minimal parameters
    pub async fn translate(&self, file_path: impl AsRef<Path>) -> Result<String> {
        let file_name = file_path
            .as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.mp3")
            .to_string();

        let request = AudioTranslationRequest::new(file_name, AudioModels::WHISPER_1);
        let response = self.translate_file(file_path, &request).await?;
        Ok(response.text)
    }

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

    /// Get the API key (for testing purposes)
    #[cfg(test)]
    pub fn api_key(&self) -> &str {
        self.http_client.api_key()
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
    fn test_audio_api_creation() {
        let api = AudioApi::new("test-key".to_string()).unwrap();
        assert_eq!(api.api_key(), "test-key");
    }

    #[test]
    fn test_empty_api_key() {
        let result = AudioApi::new("".to_string());
        assert!(result.is_err());
    }

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

    #[tokio::test]
    async fn test_speech_request_serialization() {
        let request = AudioSpeechRequest::new("tts-1", "Hello world", Voice::Alloy)
            .with_format(AudioFormat::Mp3)
            .with_speed(1.0);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"tts-1\""));
        assert!(json.contains("\"input\":\"Hello world\""));
        assert!(json.contains("\"voice\":\"alloy\""));
        assert!(json.contains("\"response_format\":\"mp3\""));
        assert!(json.contains("\"speed\":1.0"));
    }

    #[tokio::test]
    async fn test_transcription_request_creation() {
        let request = AudioTranscriptionRequest::new("test.mp3", "whisper-1")
            .with_language("en")
            .with_format(TranscriptionFormat::VerboseJson)
            .with_word_timestamps();

        assert_eq!(request.file, "test.mp3");
        assert_eq!(request.model, "whisper-1");
        assert_eq!(request.language, Some("en".to_string()));
        assert_eq!(
            request.response_format,
            Some(TranscriptionFormat::VerboseJson)
        );
        assert!(request.timestamp_granularities.is_some());
    }

    #[tokio::test]
    async fn test_translation_request_creation() {
        let request = AudioTranslationRequest::new("spanish.mp3", "whisper-1")
            .with_prompt("Translate this Spanish")
            .with_format(TranscriptionFormat::Json);

        assert_eq!(request.file, "spanish.mp3");
        assert_eq!(request.model, "whisper-1");
        assert_eq!(request.prompt, Some("Translate this Spanish".to_string()));
        assert_eq!(request.response_format, Some(TranscriptionFormat::Json));
    }

    #[test]
    fn test_response_parsing() {
        let json = r#"{"text":"Hello world","language":"en","duration":2.5}"#;
        let response: AudioTranscriptionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.text, "Hello world");
        assert_eq!(response.language, Some("en".to_string()));
        assert_eq!(response.duration, Some(2.5));
        assert!(response.has_metadata());
    }

    #[test]
    fn test_builder_patterns() {
        // Test SpeechBuilder
        let speech_req = SpeechBuilder::tts_1_hd("Test", Voice::Nova)
            .mp3()
            .speed(1.25)
            .build();
        assert_eq!(speech_req.model, AudioModels::TTS_1_HD);
        assert_eq!(speech_req.response_format, Some(AudioFormat::Mp3));

        // Test TranscriptionBuilder
        let transcription_req = TranscriptionBuilder::whisper("file.wav")
            .language("en")
            .verbose_json()
            .word_timestamps()
            .build();
        assert_eq!(transcription_req.model, AudioModels::WHISPER_1);
        assert_eq!(
            transcription_req.response_format,
            Some(TranscriptionFormat::VerboseJson)
        );

        // Test TranslationBuilder
        let translation_req = TranslationBuilder::whisper("file.mp3")
            .prompt("Translate carefully")
            .text()
            .build();
        assert_eq!(translation_req.model, AudioModels::WHISPER_1);
        assert_eq!(
            translation_req.response_format,
            Some(TranscriptionFormat::Text)
        );
    }
}
