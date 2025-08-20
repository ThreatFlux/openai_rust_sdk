#![allow(clippy::pedantic, clippy::nursery)]
//! Integration tests validating all major `OpenAI` APIs compile and have proper structure
//!
//! This test suite ensures that all implemented APIs:
//! - Can be instantiated
//! - Have the expected methods
//! - Models can be serialized/deserialized
//! - Examples compile

use openai_rust_sdk::{
    api::{
        audio::AudioApi, embeddings::EmbeddingsApi, files::FilesApi, images::ImagesApi,
        models::ModelsApi, moderations::ModerationsApi,
    },
    models::{
        audio::{AudioFormat, AudioSpeechRequest, AudioTranscriptionRequest, Voice},
        embeddings::{EmbeddingInput, EmbeddingRequest, EncodingFormat},
        files::{FilePurpose, FileUploadRequest},
        images::{ImageGenerationRequest, ImageQuality, ImageSize},
        moderations::{ModerationInput, ModerationRequest},
    },
};

#[test]
fn test_audio_api_structure() {
    // Test API can be created
    let _api = AudioApi::new("test-key").unwrap();

    // Test request models can be created
    let speech_request = AudioSpeechRequest::new("tts-1", "Test text", Voice::Alloy);
    assert_eq!(speech_request.input, "Test text");
    assert_eq!(speech_request.voice, Voice::Alloy);

    let transcription_request = AudioTranscriptionRequest::new("audio.mp3", "whisper-1");
    assert_eq!(transcription_request.model, "whisper-1");

    // Test serialization
    let json = serde_json::to_string(&speech_request).unwrap();
    assert!(json.contains("\"input\":\"Test text\""));

    // Test voice variants
    let voices = vec![
        Voice::Alloy,
        Voice::Echo,
        Voice::Fable,
        Voice::Onyx,
        Voice::Nova,
        Voice::Shimmer,
    ];
    for voice in voices {
        assert!(!voice.to_string().is_empty());
    }

    // Test audio formats
    let formats = vec![
        AudioFormat::Mp3,
        AudioFormat::Opus,
        AudioFormat::Aac,
        AudioFormat::Flac,
        AudioFormat::Wav,
        AudioFormat::Pcm,
    ];
    for format in formats {
        assert!(!format.to_string().is_empty());
    }
}

#[test]
fn test_images_api_structure() {
    // Test API can be created
    let _api = ImagesApi::new("test-key").unwrap();

    // Test request models
    let generation_request = ImageGenerationRequest::new("dall-e-3", "A beautiful sunset");
    assert_eq!(generation_request.prompt, "A beautiful sunset");
    assert_eq!(generation_request.model, "dall-e-3");

    // Test with builder pattern
    let request = ImageGenerationRequest::new("dall-e-3", "Test")
        .with_size(ImageSize::Size1024x1024)
        .with_quality(ImageQuality::Hd)
        .with_n(2);
    assert_eq!(request.size, Some(ImageSize::Size1024x1024));
    assert_eq!(request.quality, Some(ImageQuality::Hd));
    assert_eq!(request.n, Some(2));

    // Test serialization
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"prompt\":\"Test\""));
}

#[test]
fn test_files_api_structure() {
    // Test API can be created
    let _api = FilesApi::new("test-key").unwrap();

    // Test request models
    let upload_request = FileUploadRequest::new(
        b"content".to_vec(),
        "test.jsonl".to_string(),
        FilePurpose::FineTune,
    );
    assert_eq!(upload_request.filename, "test.jsonl");
    assert_eq!(upload_request.purpose, FilePurpose::FineTune);

    // Test all file purposes
    let purposes = FilePurpose::all();
    assert!(purposes.len() >= 6);
    assert!(purposes.contains(&FilePurpose::FineTune));
    assert!(purposes.contains(&FilePurpose::Assistants));
    assert!(purposes.contains(&FilePurpose::Batch));

    // Test validation
    assert!(upload_request.validate().is_ok());

    // Test MIME type detection
    assert_eq!(upload_request.mime_type(), "application/jsonl");
}

#[test]
fn test_embeddings_api_structure() {
    // Test API can be created
    let _api = EmbeddingsApi::new("test-key").unwrap();

    // Test request models
    let request = EmbeddingRequest::new("text-embedding-3-small", "test text");
    assert_eq!(request.model, "text-embedding-3-small");

    // Test with optional parameters
    let request = request
        .with_encoding_format(EncodingFormat::Base64)
        .with_dimensions(512);
    assert_eq!(request.encoding_format, Some(EncodingFormat::Base64));
    assert_eq!(request.dimensions, Some(512));

    // Test different input types
    let _single_input = EmbeddingInput::String("text".to_string());
    let _batch_input = EmbeddingInput::StringArray(vec!["text1".to_string(), "text2".to_string()]);
    let _token_input = EmbeddingInput::TokenArray(vec![vec![1, 2, 3]]);

    // Test serialization
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("text-embedding-3-small"));
}

#[test]
fn test_models_api_structure() {
    // Test API can be created
    let _api = ModelsApi::new("test-key").unwrap();

    // API exists and can be instantiated
    // Test passes if no panic occurs during API creation
}

#[test]
fn test_moderations_api_structure() {
    // Test API can be created
    let _api = ModerationsApi::new("test-key").unwrap();

    // Test request models
    let request = ModerationRequest::new("Test content");
    assert_eq!(
        request.input,
        ModerationInput::String("Test content".to_string())
    );

    // Test serialization
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"input\":\"Test content\""));
}

#[test]
fn test_api_interoperability() {
    // Test that all APIs can be created with the same pattern
    let api_key = "test-key";

    let _audio = AudioApi::new(api_key).unwrap();
    let _images = ImagesApi::new(api_key).unwrap();
    let _files = FilesApi::new(api_key).unwrap();
    let _embeddings = EmbeddingsApi::new(api_key).unwrap();
    let _models = ModelsApi::new(api_key).unwrap();
    let _moderations = ModerationsApi::new(api_key).unwrap();

    // Test custom base URL support
    let base_url = "https://custom.api.com";
    let _audio = AudioApi::new_with_base_url(api_key, base_url).unwrap();
    let _images = ImagesApi::new_with_base_url(api_key, base_url).unwrap();
    let _files = FilesApi::new_with_base_url(api_key, base_url).unwrap();
    let _embeddings = EmbeddingsApi::new_with_base_url(api_key, base_url).unwrap();
    let _models = ModelsApi::with_base_url(api_key, base_url).unwrap();
    let _moderations = ModerationsApi::new_with_base_url(api_key, base_url).unwrap();
}

#[test]
fn test_error_handling_consistency() {
    use openai_rust_sdk::error::OpenAIError;

    // Test that all APIs use the same error type
    fn accepts_error(_e: OpenAIError) {}

    // All API methods should return Result<T, OpenAIError>
    let error = OpenAIError::InvalidRequest("Test error".to_string());
    accepts_error(error);

    let error = OpenAIError::ApiError {
        status: 400,
        message: "Bad request".to_string(),
    };
    accepts_error(error);

    let error = OpenAIError::RequestError("Network error".to_string());
    accepts_error(error);

    let error = OpenAIError::ParseError("JSON error".to_string());
    accepts_error(error);
}

#[test]
fn test_model_serialization_consistency() {
    // Test that all request models serialize to valid JSON

    // Audio
    let audio_req = AudioSpeechRequest::new("tts-1", "text", Voice::Alloy);
    let audio_json = serde_json::to_value(&audio_req).unwrap();
    assert!(audio_json.is_object());

    // Images
    let image_req = ImageGenerationRequest::new("prompt", "dall-e-3");
    let image_json = serde_json::to_value(&image_req).unwrap();
    assert!(image_json.is_object());

    // Embeddings
    let embed_req = EmbeddingRequest::new("text-embedding-3-small", "text");
    let embed_json = serde_json::to_value(&embed_req).unwrap();
    assert!(embed_json.is_object());

    // Moderations
    let mod_req = ModerationRequest::new("content");
    let mod_json = serde_json::to_value(&mod_req).unwrap();
    assert!(mod_json.is_object());
}

#[test]
fn test_all_apis_compile() {
    // This test simply ensures all APIs and their dependencies compile correctly
    // The fact that this test compiles means all the APIs are properly integrated

    use openai_rust_sdk::api::{
        audio::AudioApi, embeddings::EmbeddingsApi, files::FilesApi, images::ImagesApi,
        models::ModelsApi, moderations::ModerationsApi,
    };

    // Test that APIs can be created
    let _audio = AudioApi::new("key").unwrap();
    let _embeddings = EmbeddingsApi::new("key").unwrap();
    let _files = FilesApi::new("key").unwrap();
    let _images = ImagesApi::new("key").unwrap();
    let _models = ModelsApi::new("key").unwrap();
    let _moderations = ModerationsApi::new("key").unwrap();

    // If this compiles, all APIs are properly integrated
    // Test passes if no panic occurs during API creation
}
