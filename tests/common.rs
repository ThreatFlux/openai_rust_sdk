//! Common test utilities and helpers
//!
//! This module provides reusable test fixtures, helper functions, and assertion
//! utilities to reduce code duplication across all test files in the `OpenAI` SDK test suite.
//!
//! ## Features
//! - API client creation helpers
//! - Mock response builders and test fixtures
//! - Assertion utilities for common response types
//! - Error testing utilities
//! - Serialization testing helpers
//! - Metadata and parameter object builders

#![allow(dead_code)] // Test utility functions may not all be used yet

use openai_rust_sdk::api::common::ApiClientConstructors;
use openai_rust_sdk::error::OpenAIError;
use openai_rust_sdk::models::assistants::{AssistantRequest, AssistantTool};
use openai_rust_sdk::models::fine_tuning::{FineTuningJobRequest, Hyperparameters};
use openai_rust_sdk::models::functions::FunctionTool;
use openai_rust_sdk::models::responses::{
    Message, MessageContentInput, MessageRole, ResponseRequest,
};
use openai_rust_sdk::models::runs::{RunRequest, ThreadMessage};
use openai_rust_sdk::models::vector_stores::{
    ChunkingStrategy, ExpirationPolicy, FileCounts, VectorStore, VectorStoreFile,
    VectorStoreFileBatchRequest, VectorStoreFileRequest, VectorStoreFileStatus, VectorStoreStatus,
};
use std::collections::HashMap;

// =============================================================================
// API CLIENT CREATION HELPERS
// =============================================================================

/// Mock API key for testing across all test files
pub const TEST_API_KEY: &str = "sk-test123456789abcdef";

/// Create a test API client for any API type that implements `ApiClientConstructors`
///
/// # Example
/// ```rust
/// use openai_rust_sdk::api::runs::RunsApi;
/// let api = create_test_api_client::<RunsApi>();
/// ```
pub fn create_test_api_client<T>() -> T
where
    T: ApiClientConstructors,
{
    T::new(TEST_API_KEY).expect("Failed to create test API client")
}

/// Create a test API client with custom base URL
///
/// # Example
/// ```rust
/// use openai_rust_sdk::api::runs::RunsApi;
/// let api = create_test_api_client_with_url::<RunsApi>("https://custom.openai.com");
/// ```
pub fn create_test_api_client_with_url<T>(base_url: &str) -> T
where
    T: ApiClientConstructors,
{
    T::new_with_base_url(TEST_API_KEY, base_url)
        .expect("Failed to create test API client with custom URL")
}

// =============================================================================
// SERIALIZATION TESTING UTILITIES
// =============================================================================

/// Helper function for testing serialization round-trip for types that implement `PartialEq`
pub fn test_serialization_round_trip<T>(item: &T)
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + std::fmt::Debug + PartialEq,
{
    let json = serde_json::to_string(item).expect("Should serialize to JSON");
    let deserialized: T = serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(
        *item, deserialized,
        "Serialization round-trip should preserve equality"
    );
}

/// Test serialization and verify JSON contains expected fields
pub fn test_serialization_contains<T>(item: &T, expected_fields: &[&str])
where
    T: serde::Serialize,
{
    let json = serde_json::to_string(item).expect("Should serialize to JSON");
    for field in expected_fields {
        assert!(
            json.contains(field),
            "JSON should contain '{field}', but got: {json}"
        );
    }
}

/// Test serialization for types that don't implement `PartialEq`
pub fn test_serialization_only<T>(item: &T)
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + std::fmt::Debug,
{
    let json = serde_json::to_string(item).expect("Should serialize to JSON");
    let _deserialized: T = serde_json::from_str(&json).expect("Should deserialize from JSON");
    // We can't compare equality for types without PartialEq, but we verify serialization works
}

/// Helper function to test JSON serialization contains expected content
pub fn assert_json_contains(json: &str, expected_content: &[&str]) {
    for content in expected_content {
        assert!(
            json.contains(content),
            "JSON should contain '{content}', but got: {json}"
        );
    }
}

// =============================================================================
// MOCK DATA GENERATORS
// =============================================================================

/// Helper function to create a `HashMap` with test metadata
pub fn create_test_metadata() -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    metadata.insert("environment".to_string(), "test".to_string());
    metadata.insert("version".to_string(), "1.0".to_string());
    metadata
}

/// Create test metadata with custom key-value pairs
pub fn create_custom_metadata(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

/// Create a test function tool for testing across multiple APIs
pub fn create_test_function_tool(name: &str, description: &str) -> AssistantTool {
    let function = FunctionTool {
        name: name.to_string(),
        description: description.to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "input": {
                    "type": "string",
                    "description": "The input parameter"
                }
            },
            "required": ["input"]
        }),
        strict: Some(true),
    };
    AssistantTool::function(function)
}

/// Create a test thread message for runs API testing
pub fn create_test_thread_message(role: &str, content: &str) -> ThreadMessage {
    ThreadMessage {
        role: role.to_string(),
        content: content.to_string(),
        file_ids: None,
        metadata: None,
    }
}

/// Create a test message for streaming/response testing
pub fn create_test_message(role: MessageRole, content: &str) -> Message {
    Message {
        role,
        content: MessageContentInput::Text(content.to_string()),
    }
}

// =============================================================================
// REQUEST BUILDERS AND TEST FIXTURES
// =============================================================================

/// Create a test `RunRequest` with minimal required fields
pub fn create_minimal_run_request(assistant_id: &str) -> RunRequest {
    RunRequest::builder()
        .assistant_id(assistant_id)
        .build()
        .expect("Failed to build minimal run request")
}

/// Create a full-featured `RunRequest` for testing
pub fn create_full_run_request(assistant_id: &str, model: &str) -> RunRequest {
    RunRequest::builder()
        .assistant_id(assistant_id)
        .model(model)
        .instructions("Test instructions")
        .tool(AssistantTool::CodeInterpreter)
        .file_id("file-test123")
        .metadata_pair("test", "true")
        .build()
        .expect("Failed to build full run request")
}

/// Create a test `AssistantRequest` with minimal required fields
pub fn create_minimal_assistant_request(model: &str) -> AssistantRequest {
    AssistantRequest::builder()
        .model(model)
        .build()
        .expect("Failed to build minimal assistant request")
}

/// Create a full-featured `AssistantRequest` for testing
pub fn create_full_assistant_request(model: &str, name: &str) -> AssistantRequest {
    AssistantRequest::builder()
        .model(model)
        .name(name)
        .description("A test assistant")
        .instructions("You are a helpful test assistant.")
        .tool(AssistantTool::code_interpreter())
        .tool(create_test_function_tool(
            "test_function",
            "A test function",
        ))
        .file_id("file-123")
        .metadata_pair("test", "true")
        .build()
        .expect("Failed to build full assistant request")
}

/// Create a test `FineTuningJobRequest` with minimal required fields
pub fn create_minimal_fine_tuning_request(
    training_file: &str,
    model: &str,
) -> FineTuningJobRequest {
    FineTuningJobRequest::new(training_file, model)
}

/// Create a full-featured `FineTuningJobRequest` for testing
pub fn create_full_fine_tuning_request(training_file: &str, model: &str) -> FineTuningJobRequest {
    FineTuningJobRequest::builder()
        .training_file(training_file)
        .model(model)
        .validation_file("file-val123")
        .hyperparameters(
            Hyperparameters::builder()
                .n_epochs(3)
                .batch_size(16)
                .learning_rate_multiplier(0.1)
                .build(),
        )
        .suffix("test-model")
        .metadata_entry("test", "true")
        .build()
        .expect("Failed to build full fine-tuning request")
}

/// Create a test `ResponseRequest` for streaming tests
pub fn create_streaming_response_request(model: &str, prompt: &str) -> ResponseRequest {
    ResponseRequest::new_text(model, prompt).with_streaming(true)
}

// =============================================================================
// ERROR TESTING UTILITIES
// =============================================================================

/// Test that an API constructor fails with empty API key
pub fn assert_api_creation_fails_with_empty_key<T, F>(constructor: F)
where
    T: std::fmt::Debug,
    F: Fn(&str) -> Result<T, OpenAIError>,
{
    let result = constructor("");
    assert!(result.is_err(), "Expected error with empty API key");
    let error = result.unwrap_err();
    match error {
        OpenAIError::Authentication(msg) => {
            assert!(
                msg.contains("API key") || msg.contains("empty"),
                "Error should mention API key or empty: {msg}"
            );
        }
        _ => panic!("Expected authentication error, got: {error:?}"),
    }
}

/// Test that an API constructor fails with whitespace-only API key
pub fn assert_api_creation_fails_with_whitespace_key<T, F>(constructor: F)
where
    T: std::fmt::Debug,
    F: Fn(&str) -> Result<T, OpenAIError>,
{
    let result = constructor("   ");
    assert!(result.is_err(), "Expected error with whitespace API key");
    match result.unwrap_err() {
        OpenAIError::Authentication(_) => {} // Expected
        other => panic!("Expected authentication error, got: {other:?}"),
    }
}

/// Assert that a builder fails with a specific error message
pub fn assert_builder_fails_with_message<T, E>(result: Result<T, E>, expected_message: &str)
where
    E: std::fmt::Display,
    T: std::fmt::Debug,
{
    assert!(result.is_err(), "Expected builder to fail");
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains(expected_message),
        "Error message '{error_msg}' should contain '{expected_message}'"
    );
}

// =============================================================================
// ASSERTION HELPERS
// =============================================================================

/// Assert that a serialized JSON contains all expected fields
pub fn assert_json_has_fields<T>(item: &T, expected_fields: &[&str])
where
    T: serde::Serialize,
{
    let json = serde_json::to_string(item).expect("Should serialize to JSON");
    for field in expected_fields {
        assert!(
            json.contains(field),
            "JSON should contain field '{field}': {json}"
        );
    }
}

/// Assert that a request has the expected metadata entries
#[allow(clippy::implicit_hasher)]
pub fn assert_has_metadata_entries(
    metadata: Option<&HashMap<String, String>>,
    expected: &[(&str, &str)],
) {
    assert!(metadata.is_some(), "Expected metadata to be present");
    let metadata = metadata.unwrap();
    for (key, value) in expected {
        assert_eq!(
            metadata.get(*key),
            Some(&(*value).to_string()),
            "Expected metadata key '{key}' to have value '{value}'",
        );
    }
}

/// Assert that a list parameter has expected pagination values
pub fn assert_pagination_params<T>(
    params: &T,
    limit_getter: impl Fn(&T) -> Option<i32>,
    after_getter: impl Fn(&T) -> Option<String>,
    before_getter: impl Fn(&T) -> Option<String>,
    expected_limit: Option<i32>,
    expected_after: Option<&str>,
    expected_before: Option<&str>,
) {
    assert_eq!(limit_getter(params), expected_limit, "Limit mismatch");
    assert_eq!(
        after_getter(params).as_deref(),
        expected_after,
        "After cursor mismatch"
    );
    assert_eq!(
        before_getter(params).as_deref(),
        expected_before,
        "Before cursor mismatch"
    );
}

// =============================================================================
// VECTOR STORE SPECIFIC HELPERS (EXISTING)
// =============================================================================

/// Test fixture for creating a basic `VectorStore` with common test values
pub fn create_test_vector_store() -> VectorStore {
    VectorStore {
        id: "vs-test123".to_string(),
        object: "vector_store".to_string(),
        created_at: 1_640_995_200,
        name: Some("Test Store".to_string()),
        usage_bytes: 1024,
        file_counts: FileCounts {
            in_progress: 0,
            completed: 5,
            failed: 1,
            cancelled: 0,
            total: 6,
        },
        status: VectorStoreStatus::Completed,
        expires_after: Some(ExpirationPolicy::new_days(30)),
        expires_at: Some(2_000_000_000), // Future timestamp (2033)
        last_active_at: Some(1_640_995_200),
        metadata: HashMap::new(),
    }
}

/// Test fixture for creating a `VectorStore` with custom properties
pub fn create_vector_store_with_status(status: VectorStoreStatus, usage_bytes: u64) -> VectorStore {
    VectorStore {
        id: "vs-custom".to_string(),
        object: "vector_store".to_string(),
        created_at: 1_640_995_200,
        name: Some("Custom Store".to_string()),
        usage_bytes,
        file_counts: FileCounts::new(),
        status,
        expires_after: None,
        expires_at: None,
        last_active_at: Some(1_640_995_200),
        metadata: HashMap::new(),
    }
}

/// Test fixture for creating a `VectorStoreFile` with custom properties
pub fn create_vector_store_file_with_status(
    status: VectorStoreFileStatus,
    usage_bytes: u64,
) -> VectorStoreFile {
    VectorStoreFile {
        id: "file-custom".to_string(),
        object: "vector_store.file".to_string(),
        usage_bytes,
        created_at: 1_640_995_200,
        vector_store_id: "vs-test123".to_string(),
        status,
        last_error: None,
        chunking_strategy: None,
    }
}

/// Test fixture for creating a `VectorStoreFileRequest`
pub fn create_test_vector_store_file_request() -> VectorStoreFileRequest {
    VectorStoreFileRequest::new("file-test123")
        .with_chunking_strategy(ChunkingStrategy::static_chunking(256, 25))
}

/// Test fixture for creating a `VectorStoreFileBatchRequest`
pub fn create_test_vector_store_file_batch_request() -> VectorStoreFileBatchRequest {
    let file_ids = vec![
        "file-1".to_string(),
        "file-2".to_string(),
        "file-3".to_string(),
    ];
    VectorStoreFileBatchRequest::new(file_ids).with_chunking_strategy(ChunkingStrategy::auto())
}

/// Helper function to test chunking strategy variants
pub fn assert_chunking_strategy_static(
    strategy: &ChunkingStrategy,
    expected_chunk_size: u32,
    expected_overlap: u32,
) {
    if let ChunkingStrategy::Static {
        max_chunk_size_tokens,
        chunk_overlap_tokens,
    } = strategy
    {
        assert_eq!(*max_chunk_size_tokens, expected_chunk_size);
        assert_eq!(*chunk_overlap_tokens, expected_overlap);
    } else {
        panic!("Expected static chunking strategy, got: {strategy:?}");
    }
}
