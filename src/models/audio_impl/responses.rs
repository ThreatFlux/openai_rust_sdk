//! # Audio API Responses
//!
//! Response structures for the OpenAI Audio API including speech generation,
//! transcription, and translation responses.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Response from speech generation endpoint
#[derive(Debug, Clone)]
pub struct AudioSpeechResponse {
    /// The generated audio data
    pub audio_data: Vec<u8>,
    /// The content type of the audio
    pub content_type: String,
}

/// Response from transcription endpoint
#[derive(Debug, Clone, Ser, De)]
pub struct AudioTranscriptionResponse {
    /// The transcribed text
    pub text: String,

    /// Language of the input audio (`verbose_json` format only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Duration of the input audio in seconds (`verbose_json` format only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Extracted words with timestamps (`verbose_json` format only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<TranscriptionWord>>,

    /// Segments of the transcription (`verbose_json` format only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<TranscriptionSegment>>,
}

/// Response from translation endpoint  
#[derive(Debug, Clone, Ser, De)]
pub struct AudioTranslationResponse {
    /// The translated text (always in English)
    pub text: String,

    /// Duration of the input audio in seconds (`verbose_json` format only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Segments of the translation (`verbose_json` format only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<TranscriptionSegment>>,
}

/// Word-level transcription data
#[derive(Debug, Clone, Ser, De)]
pub struct TranscriptionWord {
    /// The transcribed word
    pub word: String,

    /// Start time of the word in seconds
    pub start: f64,

    /// End time of the word in seconds
    pub end: f64,
}

/// Segment-level transcription data
#[derive(Debug, Clone, Ser, De)]
pub struct TranscriptionSegment {
    /// Unique identifier for the segment
    pub id: u32,

    /// Seek offset for the segment
    pub seek: u32,

    /// Start time of the segment in seconds
    pub start: f64,

    /// End time of the segment in seconds
    pub end: f64,

    /// Text content of the segment
    pub text: String,

    /// Array of token IDs for the text content
    pub tokens: Vec<u32>,

    /// Temperature used for this segment
    pub temperature: f64,

    /// Average log probability of the segment
    pub avg_logprob: f64,

    /// Compression ratio of the segment
    pub compression_ratio: f64,

    /// Probability of no speech in the segment
    pub no_speech_prob: f64,
}

impl AudioSpeechResponse {
    /// Create a new speech response
    #[must_use]
    pub fn new(audio_data: Vec<u8>, content_type: String) -> Self {
        Self {
            audio_data,
            content_type,
        }
    }

    /// Get the audio data
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.audio_data
    }

    /// Get the content type
    #[must_use]
    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    /// Save the audio to a file
    pub async fn save_to_file(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), std::io::Error> {
        tokio::fs::write(path, &self.audio_data).await
    }
}

impl AudioTranscriptionResponse {
    /// Check if this is a verbose JSON response with metadata
    #[must_use]
    pub fn has_metadata(&self) -> bool {
        self.language.is_some()
            || self.duration.is_some()
            || self.words.is_some()
            || self.segments.is_some()
    }

    /// Get the word count
    #[must_use]
    pub fn word_count(&self) -> usize {
        self.words
            .as_ref()
            .map_or_else(|| self.text.split_whitespace().count(), std::vec::Vec::len)
    }

    /// Get the duration in seconds
    #[must_use]
    pub fn duration(&self) -> Option<f64> {
        self.duration
    }

    /// Get segments if available
    #[must_use]
    pub fn segments(&self) -> Option<&[TranscriptionSegment]> {
        self.segments.as_deref()
    }

    /// Get words if available
    #[must_use]
    pub fn words(&self) -> Option<&[TranscriptionWord]> {
        self.words.as_deref()
    }
}

impl AudioTranslationResponse {
    /// Get the duration in seconds
    #[must_use]
    pub fn duration(&self) -> Option<f64> {
        self.duration
    }

    /// Get segments if available
    #[must_use]
    pub fn segments(&self) -> Option<&[TranscriptionSegment]> {
        self.segments.as_deref()
    }

    /// Get the word count
    #[must_use]
    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
}
