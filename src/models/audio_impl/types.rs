//! # Audio API Types
//!
//! Core types and enums for the OpenAI Audio API including voices,
//! audio formats, and transcription options.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Available voices for text-to-speech
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    /// Alloy voice - balanced and natural
    Alloy,
    /// Echo voice - deep and resonant
    Echo,
    /// Fable voice - expressive and storytelling
    Fable,
    /// Onyx voice - authoritative and deep
    Onyx,
    /// Nova voice - bright and energetic
    Nova,
    /// Shimmer voice - warm and friendly
    Shimmer,
}

crate::impl_enum_display! {
    Voice {
        Alloy => "alloy",
        Echo => "echo",
        Fable => "fable",
        Onyx => "onyx",
        Nova => "nova",
        Shimmer => "shimmer",
    }
}

/// Audio output formats
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    /// MP3 audio format (default)
    Mp3,
    /// Opus audio format - optimized for internet streaming
    Opus,
    /// AAC audio format - optimized for digital audio compression
    Aac,
    /// FLAC audio format - lossless compression
    Flac,
    /// WAV audio format - uncompressed
    Wav,
    /// PCM audio format - raw audio data
    Pcm,
}

crate::impl_enum_display! {
    AudioFormat {
        Mp3 => "mp3",
        Opus => "opus",
        Aac => "aac",
        Flac => "flac",
        Wav => "wav",
        Pcm => "pcm",
    }
}

/// Transcription output formats
#[derive(Debug, Clone, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptionFormat {
    /// JSON format with metadata (default)
    Json,
    /// Plain text format
    Text,
    /// `SubRip` (SRT) subtitle format
    Srt,
    /// `WebVTT` subtitle format
    Vtt,
    /// Verbose JSON with word-level timestamps
    #[serde(rename = "verbose_json")]
    VerboseJson,
}

/// Timestamp granularity for transcriptions
#[derive(Debug, Clone, Ser, De)]
#[serde(rename_all = "lowercase")]
pub enum TimestampGranularity {
    /// Word-level timestamps
    Word,
    /// Segment-level timestamps
    Segment,
}
