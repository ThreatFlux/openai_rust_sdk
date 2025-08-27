//! # Audio Transcription (Speech-to-Text)
//!
//! This module provides speech-to-text transcription functionality for the Audio API.

use super::types::*;
use crate::api::audio::AudioApi;

impl AudioApi {
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
            self.http_client(),
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use crate::models::audio::{TranscriptionBuilder, TranscriptionFormat};

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

    #[test]
    fn test_transcription_builder() {
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
    }

    #[test]
    fn test_transcription_response_parsing() {
        let json = r#"{"text":"Hello world","language":"en","duration":2.5}"#;
        let response: AudioTranscriptionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.text, "Hello world");
        assert_eq!(response.language, Some("en".to_string()));
        assert_eq!(response.duration, Some(2.5));
        assert!(response.has_metadata());
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
