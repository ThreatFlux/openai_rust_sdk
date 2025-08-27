//! # Audio API Builders
//!
//! Builder patterns for creating audio API requests with fluent interfaces.

use super::models::AudioModels;
use super::requests::{AudioSpeechRequest, AudioTranscriptionRequest, AudioTranslationRequest};
use super::types::{AudioFormat, TranscriptionFormat, Voice};

/// Builder for creating speech requests
pub struct SpeechBuilder {
    /// The audio speech request being built
    request: AudioSpeechRequest,
}

impl SpeechBuilder {
    /// Create a new speech builder
    pub fn new(model: impl Into<String>, input: impl Into<String>, voice: Voice) -> Self {
        Self {
            request: AudioSpeechRequest::new(model, input, voice),
        }
    }

    /// Use the standard TTS model
    pub fn tts_1(input: impl Into<String>, voice: Voice) -> Self {
        Self::new(AudioModels::TTS_1, input, voice)
    }

    /// Use the high-definition TTS model
    pub fn tts_1_hd(input: impl Into<String>, voice: Voice) -> Self {
        Self::new(AudioModels::TTS_1_HD, input, voice)
    }

    /// Set the audio format
    #[must_use]
    pub fn format(mut self, format: AudioFormat) -> Self {
        self.request.response_format = Some(format);
        self
    }

    /// Set the speech speed
    #[must_use]
    pub fn speed(mut self, speed: f32) -> Self {
        self.request.speed = Some(speed.clamp(0.25, 4.0));
        self
    }

    /// Use MP3 format
    #[must_use]
    pub fn mp3(mut self) -> Self {
        self.request.response_format = Some(AudioFormat::Mp3);
        self
    }

    /// Use Opus format
    #[must_use]
    pub fn opus(mut self) -> Self {
        self.request.response_format = Some(AudioFormat::Opus);
        self
    }

    /// Use AAC format
    #[must_use]
    pub fn aac(mut self) -> Self {
        self.request.response_format = Some(AudioFormat::Aac);
        self
    }

    /// Use FLAC format
    #[must_use]
    pub fn flac(mut self) -> Self {
        self.request.response_format = Some(AudioFormat::Flac);
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> AudioSpeechRequest {
        self.request
    }
}

/// Builder for creating transcription requests
pub struct TranscriptionBuilder {
    /// The audio transcription request being built
    request: AudioTranscriptionRequest,
}

impl TranscriptionBuilder {
    /// Create a new transcription builder
    pub fn new(file: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            request: AudioTranscriptionRequest::new(file, model),
        }
    }

    /// Use the Whisper model
    pub fn whisper(file: impl Into<String>) -> Self {
        Self::new(file, AudioModels::WHISPER_1)
    }

    /// Set the language
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.request.language = Some(language.into());
        self
    }

    /// Set a prompt
    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.request.prompt = Some(prompt.into());
        self
    }

    /// Use JSON format
    #[must_use]
    pub fn json(mut self) -> Self {
        self.request.response_format = Some(TranscriptionFormat::Json);
        self
    }

    /// Use verbose JSON format with timestamps
    #[must_use]
    pub fn verbose_json(mut self) -> Self {
        self.request.response_format = Some(TranscriptionFormat::VerboseJson);
        self
    }

    /// Use plain text format
    #[must_use]
    pub fn text(mut self) -> Self {
        self.request.response_format = Some(TranscriptionFormat::Text);
        self
    }

    /// Use SRT subtitle format
    #[must_use]
    pub fn srt(mut self) -> Self {
        self.request.response_format = Some(TranscriptionFormat::Srt);
        self
    }

    /// Use `WebVTT` subtitle format
    #[must_use]
    pub fn vtt(mut self) -> Self {
        self.request.response_format = Some(TranscriptionFormat::Vtt);
        self
    }

    /// Set temperature
    #[must_use]
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature.clamp(0.0, 1.0));
        self
    }

    /// Enable word timestamps
    #[must_use]
    pub fn word_timestamps(mut self) -> Self {
        self.request.timestamp_granularities = Some(vec![super::types::TimestampGranularity::Word]);
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> AudioTranscriptionRequest {
        self.request
    }
}

/// Builder for creating translation requests
pub struct TranslationBuilder {
    /// The audio translation request being built
    request: AudioTranslationRequest,
}

impl TranslationBuilder {
    /// Create a new translation builder
    pub fn new(file: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            request: AudioTranslationRequest::new(file, model),
        }
    }

    /// Use the Whisper model
    pub fn whisper(file: impl Into<String>) -> Self {
        Self::new(file, AudioModels::WHISPER_1)
    }

    /// Set a prompt
    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.request.prompt = Some(prompt.into());
        self
    }

    /// Use JSON format
    #[must_use]
    pub fn json(mut self) -> Self {
        self.request.response_format = Some(TranscriptionFormat::Json);
        self
    }

    /// Use verbose JSON format
    #[must_use]
    pub fn verbose_json(mut self) -> Self {
        self.request.response_format = Some(TranscriptionFormat::VerboseJson);
        self
    }

    /// Use plain text format
    #[must_use]
    pub fn text(mut self) -> Self {
        self.request.response_format = Some(TranscriptionFormat::Text);
        self
    }

    /// Set temperature
    #[must_use]
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature.clamp(0.0, 1.0));
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> AudioTranslationRequest {
        self.request
    }
}
