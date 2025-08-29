#![allow(clippy::pedantic, clippy::nursery)]
//! Smoke tests to verify all APIs compile and are accessible
//!
//! This minimal test suite just ensures that:
//! - All API modules exist
//! - Core types can be imported
//! - Basic instantiation works

#[test]
fn test_audio_api_exists() {
    use openai_rust_sdk::api::{audio::AudioApi, common::ApiClientConstructors};
    use openai_rust_sdk::models::audio::Voice;

    // API can be created
    let _api = AudioApi::new("test-key").unwrap();

    // Basic types exist
    let _ = Voice::Alloy;
}

#[test]
fn test_images_api_exists() {
    use openai_rust_sdk::api::{common::ApiClientConstructors, images::ImagesApi};

    // API can be created
    let _api = ImagesApi::new("test-key").unwrap();
}

#[test]
fn test_files_api_exists() {
    use openai_rust_sdk::api::{common::ApiClientConstructors, files::FilesApi};
    use openai_rust_sdk::models::files::FilePurpose;

    // API can be created
    let _api = FilesApi::new("test-key").unwrap();

    // Core types exist
    let _ = FilePurpose::FineTune;
}

#[test]
fn test_embeddings_api_exists() {
    use openai_rust_sdk::api::{common::ApiClientConstructors, embeddings::EmbeddingsApi};

    // API can be created
    let _api = EmbeddingsApi::new("test-key").unwrap();
}

#[test]
fn test_models_api_exists() {
    use openai_rust_sdk::api::models::ModelsApi;

    // API can be created
    let _api = ModelsApi::new("test-key").unwrap();
}

#[test]
fn test_moderations_api_exists() {
    use openai_rust_sdk::api::{common::ApiClientConstructors, moderations::ModerationsApi};

    // API can be created
    let _api = ModerationsApi::new("test-key").unwrap();
}

#[test]
fn test_error_types_exist() {
    use openai_rust_sdk::error::OpenAIError;

    // Error types can be created
    let _err = OpenAIError::InvalidRequest("test".to_string());
    let _err = OpenAIError::ApiError {
        status: 400,
        message: "test".to_string(),
    };
    let _err = OpenAIError::RequestError("test".to_string());
    let _err = OpenAIError::ParseError("test".to_string());
}

#[test]
fn test_all_apis_imported_together() {
    // This ensures there are no naming conflicts when importing all APIs
    use openai_rust_sdk::api::{
        audio::AudioApi, common::ApiClientConstructors, embeddings::EmbeddingsApi, files::FilesApi,
        images::ImagesApi, models::ModelsApi, moderations::ModerationsApi,
    };

    // All can coexist in the same scope
    let _ = AudioApi::new("key");
    let _ = EmbeddingsApi::new("key");
    let _ = FilesApi::new("key");
    let _ = ImagesApi::new("key");
    let _ = ModelsApi::new("key");
    let _ = ModerationsApi::new("key");
}
