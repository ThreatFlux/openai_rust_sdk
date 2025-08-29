//! # Speech Generation (Text-to-Speech)
//!
//! This module provides text-to-speech functionality for the Audio API.

use super::types::{
    ApiClientConstructors, AudioFormat, AudioModels, AudioSpeechRequest, AudioSpeechResponse,
    Bytes, Path, Result, StreamExt, Voice,
};
use crate::api::audio::AudioApi;

impl AudioApi {
    /// Create speech from text using text-to-speech
    pub async fn create_speech(&self, request: &AudioSpeechRequest) -> Result<AudioSpeechResponse> {
        let (audio_data, content_type) = self
            .http_client()
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
            .http_client()
            .post_stream("/v1/audio/speech", request)
            .await?;

        let stream = response
            .bytes_stream()
            .map(|chunk| chunk.map_err(crate::request_err!(to_string)));

        Ok(stream)
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use crate::models::audio::SpeechBuilder;
    use crate::models::common_builder::{Builder, WithSpeed};

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

    #[test]
    fn test_speech_builder() {
        let speech_req = SpeechBuilder::tts_1_hd("Test", Voice::Nova)
            .mp3()
            .speed(1.25)
            .build();
        assert_eq!(speech_req.model, AudioModels::TTS_1_HD);
        assert_eq!(speech_req.response_format, Some(AudioFormat::Mp3));
    }
}
