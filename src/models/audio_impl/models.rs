//! # Audio Models
//!
//! Common audio models for the OpenAI Audio API.

/// Common audio models
pub struct AudioModels;

impl AudioModels {
    /// Text-to-speech model (standard quality)
    pub const TTS_1: &'static str = "tts-1";

    /// Text-to-speech model (high definition quality)
    pub const TTS_1_HD: &'static str = "tts-1-hd";

    /// Speech-to-text model (Whisper)
    pub const WHISPER_1: &'static str = "whisper-1";
}
