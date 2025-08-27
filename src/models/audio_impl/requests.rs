//! # Audio API Requests
//!
//! Request structures for the OpenAI Audio API including text-to-speech,
//! speech-to-text transcription, and translation endpoints.

use super::types::{AudioFormat, TimestampGranularity, TranscriptionFormat, Voice};
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Request for text-to-speech audio generation
#[derive(Debug, Clone, Ser, De)]
pub struct AudioSpeechRequest {
    /// The model to use for generating audio (e.g., "tts-1", "tts-1-hd")
    pub model: String,

    /// The text to convert to speech
    pub input: String,

    /// The voice to use for speech generation
    pub voice: Voice,

    /// The format to return the audio in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<AudioFormat>,

    /// The speed of the generated audio (0.25 to 4.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

/// Request for speech-to-text transcription
#[derive(Debug, Clone, Ser, De)]
pub struct AudioTranscriptionRequest {
    /// The audio file to transcribe (file name for multipart upload)
    #[serde(skip_serializing)]
    pub file: String,

    /// The model to use for transcription (e.g., "whisper-1")
    pub model: String,

    /// The language of the input audio (ISO-639-1 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// An optional text to guide the model's style or continue a previous audio segment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// The format of the transcript output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<TranscriptionFormat>,

    /// The sampling temperature (0 to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Timestamp granularities to populate for the transcription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
}

/// Request for speech-to-text translation
#[derive(Debug, Clone, Ser, De)]
pub struct AudioTranslationRequest {
    /// The audio file to translate (file name for multipart upload)
    #[serde(skip_serializing)]
    pub file: String,

    /// The model to use for translation (e.g., "whisper-1")
    pub model: String,

    /// An optional text to guide the model's style or continue a previous audio segment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// The format of the transcript output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<TranscriptionFormat>,

    /// The sampling temperature (0 to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

impl AudioSpeechRequest {
    /// Create a new speech request
    pub fn new(model: impl Into<String>, input: impl Into<String>, voice: Voice) -> Self {
        Self {
            model: model.into(),
            input: input.into(),
            voice,
            response_format: None,
            speed: None,
        }
    }

    /// Set the audio format
    #[must_use]
    pub fn with_format(mut self, format: AudioFormat) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set the speech speed (0.25 to 4.0)
    #[must_use]
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = Some(speed.clamp(0.25, 4.0));
        self
    }
}

impl AudioTranscriptionRequest {
    /// Create a new transcription request
    pub fn new(file: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            model: model.into(),
            language: None,
            prompt: None,
            response_format: None,
            temperature: None,
            timestamp_granularities: None,
        }
    }

    /// Set the language of the audio
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Set a prompt to guide the transcription
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Set the response format
    #[must_use]
    pub fn with_format(mut self, format: TranscriptionFormat) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set the temperature for transcription
    #[must_use]
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature.clamp(0.0, 1.0));
        self
    }

    /// Enable word-level timestamps
    #[must_use]
    pub fn with_word_timestamps(mut self) -> Self {
        self.timestamp_granularities = Some(vec![TimestampGranularity::Word]);
        self
    }

    /// Enable segment-level timestamps
    #[must_use]
    pub fn with_segment_timestamps(mut self) -> Self {
        self.timestamp_granularities = Some(vec![TimestampGranularity::Segment]);
        self
    }

    /// Enable both word and segment timestamps
    #[must_use]
    pub fn with_all_timestamps(mut self) -> Self {
        self.timestamp_granularities = Some(vec![
            TimestampGranularity::Word,
            TimestampGranularity::Segment,
        ]);
        self
    }
}

impl AudioTranslationRequest {
    /// Create a new translation request
    pub fn new(file: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            model: model.into(),
            prompt: None,
            response_format: None,
            temperature: None,
        }
    }

    /// Set a prompt to guide the translation
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Set the response format
    #[must_use]
    pub fn with_format(mut self, format: TranscriptionFormat) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set the temperature for translation
    #[must_use]
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature.clamp(0.0, 1.0));
        self
    }
}
