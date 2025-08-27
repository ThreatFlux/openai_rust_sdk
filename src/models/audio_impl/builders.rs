//! # Audio API Builders
//!
//! Builder patterns for creating audio API requests with fluent interfaces.

use super::models::AudioModels;
use super::requests::{AudioSpeechRequest, AudioTranscriptionRequest, AudioTranslationRequest};
use super::types::{AudioFormat, TranscriptionFormat, Voice};
use crate::models::common_builder::{Builder, WithFormat, WithSpeed, WithTemperature};
use crate::{impl_builder, impl_with_format, impl_with_speed, impl_with_temperature, impl_audio_format_methods, impl_transcription_format_methods};

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
}

// Apply common builder traits
impl_builder!(SpeechBuilder, AudioSpeechRequest, request);
impl_with_format!(SpeechBuilder, AudioSpeechRequest, request, AudioFormat);
impl_with_speed!(SpeechBuilder, AudioSpeechRequest, request, (0.25, 4.0));

// Generate audio format convenience methods
impl_audio_format_methods!(SpeechBuilder, request);

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

    /// Enable word timestamps
    #[must_use]
    pub fn word_timestamps(mut self) -> Self {
        self.request.timestamp_granularities = Some(vec![super::types::TimestampGranularity::Word]);
        self
    }
}

// Apply common builder traits
impl_builder!(TranscriptionBuilder, AudioTranscriptionRequest, request);
impl_with_format!(TranscriptionBuilder, AudioTranscriptionRequest, request, TranscriptionFormat);
impl_with_temperature!(TranscriptionBuilder, AudioTranscriptionRequest, request);

// Generate transcription format convenience methods
impl_transcription_format_methods!(TranscriptionBuilder, request);

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
}

// Apply common builder traits
impl_builder!(TranslationBuilder, AudioTranslationRequest, request);
impl_with_format!(TranslationBuilder, AudioTranslationRequest, request, TranscriptionFormat);
impl_with_temperature!(TranslationBuilder, AudioTranslationRequest, request);

// Generate transcription format convenience methods (works for translation too)
impl_transcription_format_methods!(TranslationBuilder, request);
