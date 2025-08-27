//! # Model Enumerations
//!
//! Enumerations for categorizing models and their capabilities.

/// Types of completions a model supports
#[derive(Debug, Clone, PartialEq)]
pub enum CompletionType {
    /// Text completion (legacy)
    Text,
    /// Chat completion
    Chat,
    /// Code completion
    Code,
    /// Image generation
    Image,
    /// Audio processing (speech-to-text, text-to-speech)
    Audio,
    /// Embeddings
    Embeddings,
    /// Moderation
    Moderation,
}

/// Model family categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelFamily {
    /// GPT-4 series models
    GPT4,
    /// GPT-4 Turbo models
    GPT4Turbo,
    /// GPT-4o series models
    GPT4o,
    /// GPT-3.5 series models
    GPT35,
    /// DALL-E image generation models
    #[allow(clippy::upper_case_acronyms)]
    DALLE,
    /// Whisper audio models
    Whisper,
    /// TTS (Text-to-Speech) models
    #[allow(clippy::upper_case_acronyms)]
    TTS,
    /// Embedding models
    Embeddings,
    /// Moderation models
    Moderation,
    /// Unknown or unclassified
    Unknown,
}

/// Model tier/quality classification
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ModelTier {
    /// Legacy models (deprecated or older)
    Legacy,
    /// Standard quality models
    Standard,
    /// Premium/high-quality models
    Premium,
    /// Cutting-edge/experimental models
    Experimental,
}

/// Internal enum to categorize model types for capability determination
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ModelType {
    /// GPT-4o series models
    Gpt4o,
    /// GPT-4 Turbo models
    Gpt4Turbo,
    /// GPT-4 models
    Gpt4,
    /// GPT-3.5 models
    Gpt35,
    /// DALL-E image generation models
    Dalle,
    /// Whisper audio models
    Whisper,
    /// TTS (Text-to-Speech) models
    Tts,
    /// Embedding models
    Embedding,
    /// Moderation models
    Moderation,
    /// Legacy/unknown models
    Legacy,
}
