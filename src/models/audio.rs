//! # Audio Models
//!
//! Data structures for the OpenAI Audio API including text-to-speech,
//! speech-to-text transcription, and translation endpoints.

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

impl std::fmt::Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Voice::Alloy => write!(f, "alloy"),
            Voice::Echo => write!(f, "echo"),
            Voice::Fable => write!(f, "fable"),
            Voice::Onyx => write!(f, "onyx"),
            Voice::Nova => write!(f, "nova"),
            Voice::Shimmer => write!(f, "shimmer"),
        }
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

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioFormat::Mp3 => write!(f, "mp3"),
            AudioFormat::Opus => write!(f, "opus"),
            AudioFormat::Aac => write!(f, "aac"),
            AudioFormat::Flac => write!(f, "flac"),
            AudioFormat::Wav => write!(f, "wav"),
            AudioFormat::Pcm => write!(f, "pcm"),
        }
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
        self.request.timestamp_granularities = Some(vec![TimestampGranularity::Word]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_request_creation() {
        let req = AudioSpeechRequest::new("tts-1", "Hello world", Voice::Alloy);
        assert_eq!(req.model, "tts-1");
        assert_eq!(req.input, "Hello world");
        matches!(req.voice, Voice::Alloy);
    }

    #[test]
    fn test_speech_builder() {
        let req = SpeechBuilder::tts_1_hd("Test speech", Voice::Nova)
            .format(AudioFormat::Mp3)
            .speed(1.5)
            .build();

        assert_eq!(req.model, AudioModels::TTS_1_HD);
        assert_eq!(req.input, "Test speech");
        matches!(req.voice, Voice::Nova);
        assert_eq!(req.response_format, Some(AudioFormat::Mp3));
        assert_eq!(req.speed, Some(1.5));
    }

    #[test]
    fn test_transcription_request_creation() {
        let req = AudioTranscriptionRequest::new("audio.mp3", "whisper-1");
        assert_eq!(req.file, "audio.mp3");
        assert_eq!(req.model, "whisper-1");
    }

    #[test]
    fn test_transcription_builder() {
        let req = TranscriptionBuilder::whisper("test.wav")
            .language("en")
            .verbose_json()
            .word_timestamps()
            .temperature(0.3)
            .build();

        assert_eq!(req.file, "test.wav");
        assert_eq!(req.model, AudioModels::WHISPER_1);
        assert_eq!(req.language, Some("en".to_string()));
        assert_eq!(req.response_format, Some(TranscriptionFormat::VerboseJson));
        assert_eq!(req.temperature, Some(0.3));
        assert!(req.timestamp_granularities.is_some());
    }

    #[test]
    fn test_translation_builder() {
        let req = TranslationBuilder::whisper("spanish.mp3")
            .prompt("Translate this Spanish audio")
            .json()
            .build();

        assert_eq!(req.file, "spanish.mp3");
        assert_eq!(req.model, AudioModels::WHISPER_1);
        assert_eq!(req.prompt, Some("Translate this Spanish audio".to_string()));
        assert_eq!(req.response_format, Some(TranscriptionFormat::Json));
    }

    #[test]
    fn test_voice_serialization() {
        assert_eq!(serde_json::to_string(&Voice::Alloy).unwrap(), "\"alloy\"");
        assert_eq!(serde_json::to_string(&Voice::Echo).unwrap(), "\"echo\"");
        assert_eq!(serde_json::to_string(&Voice::Fable).unwrap(), "\"fable\"");
        assert_eq!(serde_json::to_string(&Voice::Onyx).unwrap(), "\"onyx\"");
        assert_eq!(serde_json::to_string(&Voice::Nova).unwrap(), "\"nova\"");
        assert_eq!(
            serde_json::to_string(&Voice::Shimmer).unwrap(),
            "\"shimmer\""
        );
    }

    #[test]
    fn test_audio_format_serialization() {
        assert_eq!(serde_json::to_string(&AudioFormat::Mp3).unwrap(), "\"mp3\"");
        assert_eq!(
            serde_json::to_string(&AudioFormat::Opus).unwrap(),
            "\"opus\""
        );
        assert_eq!(serde_json::to_string(&AudioFormat::Aac).unwrap(), "\"aac\"");
        assert_eq!(
            serde_json::to_string(&AudioFormat::Flac).unwrap(),
            "\"flac\""
        );
    }

    #[test]
    fn test_speed_clamping() {
        let req = SpeechBuilder::tts_1("test", Voice::Alloy)
            .speed(10.0) // Should be clamped to 4.0
            .build();
        assert_eq!(req.speed, Some(4.0));

        let req2 = SpeechBuilder::tts_1("test", Voice::Alloy)
            .speed(0.1) // Should be clamped to 0.25
            .build();
        assert_eq!(req2.speed, Some(0.25));
    }

    #[test]
    fn test_temperature_clamping() {
        let req = TranscriptionBuilder::whisper("test.mp3")
            .temperature(2.0) // Should be clamped to 1.0
            .build();
        assert_eq!(req.temperature, Some(1.0));

        let req2 = TranscriptionBuilder::whisper("test.mp3")
            .temperature(-1.0) // Should be clamped to 0.0
            .build();
        assert_eq!(req2.temperature, Some(0.0));
    }

    #[test]
    fn test_audio_response_methods() {
        let response = AudioSpeechResponse::new(vec![1, 2, 3, 4], "audio/mpeg".to_string());
        assert_eq!(response.data(), &[1, 2, 3, 4]);
        assert_eq!(response.content_type(), "audio/mpeg");
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
