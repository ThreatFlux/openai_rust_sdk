//! # Audio Translation
//!
//! This module provides audio translation functionality for the Audio API.

use super::types::*;
use crate::api::audio::AudioApi;

impl AudioApi {
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
            self.http_client(),
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use crate::models::audio::{TranscriptionFormat, TranslationBuilder};
    use crate::models::common_builder::Builder;

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
    fn test_translation_builder() {
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
