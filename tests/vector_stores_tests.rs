#![allow(clippy::pedantic, clippy::nursery)]
//! Comprehensive tests for the Vector Stores API
//!
//! This module contains unit tests and integration tests for the Vector Stores API,
//! covering all major functionality including CRUD operations, file management,
//! batch operations, and error handling.

mod common;

use openai_rust_sdk::api::{common::ApiClientConstructors, vector_stores::VectorStoresApi};
use openai_rust_sdk::models::vector_stores::{
    ChunkingStrategy, ExpirationPolicy, FileCounts, ListVectorStoreFilesParams,
    ListVectorStoresParams, VectorStoreDeleteResponse, VectorStoreFileBatchRequest,
    VectorStoreFileBatchStatus, VectorStoreFileDeleteResponse, VectorStoreFileStatus,
    VectorStoreRequest, VectorStoreStatus,
};
use std::collections::HashMap;

use common::{
    assert_chunking_strategy_static, assert_json_contains, create_test_metadata,
    create_test_vector_store, create_test_vector_store_file_batch_request,
    create_test_vector_store_file_request, create_vector_store_file_with_status,
    create_vector_store_with_status, test_serialization_round_trip,
};

#[test]
fn test_vector_stores_api_creation() {
    let api = VectorStoresApi::new("test-api-key").unwrap();
    assert_eq!(api.api_key(), "test-api-key");
    assert_eq!(api.base_url(), "https://api.openai.com");
}

#[test]
fn test_vector_stores_api_custom_base_url() {
    let api = VectorStoresApi::new_with_base_url("test-key", "https://custom.openai.com").unwrap();
    assert_eq!(api.api_key(), "test-key");
    assert_eq!(api.base_url(), "https://custom.openai.com");
}

#[test]
fn test_vector_store_status_enum() {
    assert_eq!(VectorStoreStatus::InProgress.to_string(), "in_progress");
    assert_eq!(VectorStoreStatus::Completed.to_string(), "completed");
    assert_eq!(VectorStoreStatus::Failed.to_string(), "failed");
    assert_eq!(VectorStoreStatus::Cancelled.to_string(), "cancelled");
    assert_eq!(VectorStoreStatus::Expired.to_string(), "expired");
}

#[test]
fn test_vector_store_file_status_enum() {
    assert_eq!(VectorStoreFileStatus::InProgress.to_string(), "in_progress");
    assert_eq!(VectorStoreFileStatus::Completed.to_string(), "completed");
    assert_eq!(VectorStoreFileStatus::Cancelled.to_string(), "cancelled");
    assert_eq!(VectorStoreFileStatus::Failed.to_string(), "failed");
}

#[test]
fn test_vector_store_file_batch_status_enum() {
    assert_eq!(
        VectorStoreFileBatchStatus::InProgress.to_string(),
        "in_progress"
    );
    assert_eq!(
        VectorStoreFileBatchStatus::Completed.to_string(),
        "completed"
    );
    assert_eq!(
        VectorStoreFileBatchStatus::Cancelled.to_string(),
        "cancelled"
    );
    assert_eq!(VectorStoreFileBatchStatus::Failed.to_string(), "failed");
}

#[test]
fn test_expiration_policy_creation() {
    let policy = ExpirationPolicy::new_days(30);
    assert_eq!(policy.anchor, "last_active_at");
    assert_eq!(policy.days, 30);

    let custom_policy = ExpirationPolicy::new_with_anchor("created_at", 7);
    assert_eq!(custom_policy.anchor, "created_at");
    assert_eq!(custom_policy.days, 7);
}

#[test]
fn test_file_counts_methods() {
    let mut counts = FileCounts::new();
    assert_eq!(counts.total, 0);
    assert!(counts.is_processing_complete());
    assert_eq!(counts.completion_percentage(), 100.0);
    assert_eq!(counts.failure_percentage(), 0.0);

    counts.total = 100;
    counts.completed = 70;
    counts.failed = 20;
    counts.in_progress = 10;

    assert!(!counts.is_processing_complete());
    assert_eq!(counts.completion_percentage(), 70.0);
    assert_eq!(counts.failure_percentage(), 20.0);
}

#[test]
fn test_chunking_strategy_creation() {
    let auto_strategy = ChunkingStrategy::auto();
    assert_eq!(auto_strategy, ChunkingStrategy::Auto);

    let static_strategy = ChunkingStrategy::static_chunking(1024, 100);
    assert_chunking_strategy_static(&static_strategy, 1024, 100);
}

#[test]
fn test_vector_store_request_builder() {
    let request = VectorStoreRequest::builder()
        .name("Test Knowledge Base")
        .add_file_id("file-123")
        .add_file_id("file-456")
        .expires_after(ExpirationPolicy::new_days(30))
        .chunking_strategy(ChunkingStrategy::static_chunking(512, 50))
        .add_metadata("environment", "test")
        .add_metadata("version", "1.0")
        .build();

    assert_eq!(request.name, Some("Test Knowledge Base".to_string()));
    assert_eq!(
        request.file_ids,
        Some(vec!["file-123".to_string(), "file-456".to_string()])
    );
    assert!(request.expires_after.is_some());
    assert!(request.chunking_strategy.is_some());
    assert!(request.metadata.is_some());

    let metadata = request.metadata.unwrap();
    assert_eq!(metadata.get("environment"), Some(&"test".to_string()));
    assert_eq!(metadata.get("version"), Some(&"1.0".to_string()));
}

#[test]
fn test_vector_store_request_fluent_interface() {
    let mut metadata = HashMap::new();
    metadata.insert("project".to_string(), "test".to_string());

    let request = VectorStoreRequest::new()
        .with_name("Fluent Test")
        .with_file_ids(vec!["file-789".to_string()])
        .with_expires_after(ExpirationPolicy::new_days(60))
        .with_chunking_strategy(ChunkingStrategy::auto())
        .with_metadata(metadata)
        .add_metadata("additional", "value");

    assert_eq!(request.name, Some("Fluent Test".to_string()));
    assert_eq!(request.file_ids, Some(vec!["file-789".to_string()]));
    assert!(request.expires_after.is_some());
    assert_eq!(request.chunking_strategy, Some(ChunkingStrategy::Auto));
    assert!(request.metadata.is_some());

    let meta = request.metadata.unwrap();
    assert_eq!(meta.len(), 2);
    assert_eq!(meta.get("project"), Some(&"test".to_string()));
    assert_eq!(meta.get("additional"), Some(&"value".to_string()));
}

#[test]
fn test_vector_store_file_request() {
    let request = create_test_vector_store_file_request();

    assert_eq!(request.file_id, "file-test123");
    assert!(request.chunking_strategy.is_some());

    if let Some(strategy) = &request.chunking_strategy {
        assert_chunking_strategy_static(strategy, 256, 25);
    }
}

#[test]
fn test_vector_store_file_batch_request() {
    let request = create_test_vector_store_file_batch_request();

    let expected_file_ids = vec![
        "file-1".to_string(),
        "file-2".to_string(),
        "file-3".to_string(),
    ];

    assert_eq!(request.file_ids, expected_file_ids);
    assert_eq!(request.chunking_strategy, Some(ChunkingStrategy::Auto));
}

// Generate comprehensive parameter tests using a macro
macro_rules! test_list_params {
    ($test_name:ident, $param_type:ty, {
        $($method:ident($value:expr) => ($key:literal, $expected:literal)),*
    }, expected_count: $count:expr) => {
        #[test]
        fn $test_name() {
            let params = <$param_type>::new()
                $(.$method($value))*;

            let query_params = params.to_query_params();
            assert_eq!(query_params.len(), $count);
            $(
                assert!(query_params.contains(&($key.to_string(), $expected.to_string())));
            )*
        }
    };
}

test_list_params!(test_list_vector_stores_params, ListVectorStoresParams, {
    with_limit(25) => ("limit", "25"),
    with_order("desc") => ("order", "desc"),
    with_after("vs-abc123") => ("after", "vs-abc123"),
    with_before("vs-xyz789") => ("before", "vs-xyz789")
}, expected_count: 4);

test_list_params!(test_list_vector_store_files_params, ListVectorStoreFilesParams, {
    with_limit(50) => ("limit", "50"),
    with_order("asc") => ("order", "asc"),
    with_after("file-123") => ("after", "file-123")
}, expected_count: 3);

#[test]
fn test_list_vector_store_files_params_with_filter() {
    let params = ListVectorStoreFilesParams::new().with_filter(VectorStoreFileStatus::Completed);

    let query_params = params.to_query_params();
    assert!(query_params.contains(&("filter".to_string(), "completed".to_string())));
}

#[test]
fn test_vector_store_methods() {
    let mut vector_store = create_test_vector_store();
    // Update for this specific test
    vector_store.usage_bytes = 2048;

    assert!(vector_store.is_ready());
    assert!(!vector_store.is_processing());
    assert!(!vector_store.has_failed());
    assert!(!vector_store.has_expired());
    assert_eq!(vector_store.usage_human_readable(), "2.0 KB");
    assert!(!vector_store.expires_soon());

    // Test different statuses
    vector_store.status = VectorStoreStatus::InProgress;
    assert!(!vector_store.is_ready());
    assert!(vector_store.is_processing());

    vector_store.status = VectorStoreStatus::Failed;
    assert!(vector_store.has_failed());

    vector_store.status = VectorStoreStatus::Expired;
    assert!(vector_store.has_expired());

    // Test human readable sizes
    vector_store.usage_bytes = 512;
    assert_eq!(vector_store.usage_human_readable(), "512 B");

    vector_store.usage_bytes = 1_048_576; // 1 MB
    assert_eq!(vector_store.usage_human_readable(), "1.0 MB");

    vector_store.usage_bytes = 1_073_741_824; // 1 GB
    assert_eq!(vector_store.usage_human_readable(), "1.0 GB");
}

#[test]
fn test_vector_store_expires_soon() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut vector_store = create_test_vector_store();
    vector_store.created_at = now - 3600;
    vector_store.usage_bytes = 1024;
    vector_store.file_counts = FileCounts::new();
    vector_store.expires_after = None;
    vector_store.expires_at = Some(now + 3600); // Expires in 1 hour (should trigger expires_soon)
    vector_store.last_active_at = Some(now);

    assert!(vector_store.expires_soon());

    vector_store.expires_at = Some(now + 48 * 3600); // Expires in 2 days
    assert!(!vector_store.expires_soon());

    vector_store.expires_at = None;
    assert!(!vector_store.expires_soon());
}

#[test]
fn test_list_responses_filtering() {
    use openai_rust_sdk::models::vector_stores::{
        ListVectorStoreFilesResponse, ListVectorStoresResponse,
    };

    // Test ListVectorStoresResponse filtering
    let store1 = create_vector_store_with_status(VectorStoreStatus::Completed, 1024);
    let store2 = create_vector_store_with_status(VectorStoreStatus::InProgress, 2048);

    let stores_response = ListVectorStoresResponse {
        object: "list".to_string(),
        data: vec![store1, store2],
        first_id: Some("vs-custom".to_string()),
        last_id: Some("vs-custom".to_string()),
        has_more: false,
    };

    assert_eq!(stores_response.total_usage_bytes(), 3072);
    assert_eq!(stores_response.ready_stores().len(), 1);
    assert_eq!(stores_response.processing_stores().len(), 1);
    assert_eq!(
        stores_response
            .by_status(&VectorStoreStatus::Completed)
            .len(),
        1
    );

    // Test ListVectorStoreFilesResponse filtering
    let file1 = create_vector_store_file_with_status(VectorStoreFileStatus::Completed, 512);
    let file2 = create_vector_store_file_with_status(VectorStoreFileStatus::Failed, 256);

    let files_response = ListVectorStoreFilesResponse {
        object: "list".to_string(),
        data: vec![file1, file2],
        first_id: Some("file-custom".to_string()),
        last_id: Some("file-custom".to_string()),
        has_more: false,
    };

    assert_eq!(files_response.total_usage_bytes(), 768);
    assert_eq!(files_response.completed_files().len(), 1);
    assert_eq!(files_response.failed_files().len(), 1);
    assert_eq!(
        files_response
            .by_status(&VectorStoreFileStatus::Completed)
            .len(),
        1
    );
}

#[test]
fn test_delete_responses() {
    let success_store = VectorStoreDeleteResponse::success("vs-123".to_string());
    assert!(success_store.deleted);
    assert_eq!(success_store.id, "vs-123");
    assert_eq!(success_store.object, "vector_store.deleted");

    let failure_store = VectorStoreDeleteResponse::failure("vs-456".to_string());
    assert!(!failure_store.deleted);
    assert_eq!(failure_store.id, "vs-456");

    let success_file = VectorStoreFileDeleteResponse::success("file-789".to_string());
    assert!(success_file.deleted);
    assert_eq!(success_file.id, "file-789");
    assert_eq!(success_file.object, "vector_store.file.deleted");

    let failure_file = VectorStoreFileDeleteResponse::failure("file-101".to_string());
    assert!(!failure_file.deleted);
    assert_eq!(failure_file.id, "file-101");
}

#[test]
fn test_vector_store_request_serialization() {
    let request = VectorStoreRequest::builder()
        .name("Serialization Test")
        .add_file_id("file-test")
        .expires_after(ExpirationPolicy::new_days(14))
        .chunking_strategy(ChunkingStrategy::static_chunking(800, 80))
        .add_metadata("test", "value")
        .build();

    // Test that the request can be serialized to JSON
    let json = serde_json::to_string(&request).unwrap();
    assert_json_contains(&json, &["Serialization Test", "file-test"]);

    // Test that it can be deserialized back
    let deserialized: VectorStoreRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, request.name);
    assert_eq!(deserialized.file_ids, request.file_ids);
}

#[test]
fn test_file_batch_request_serialization() {
    let request = create_test_vector_store_file_batch_request();

    let json = serde_json::to_string(&request).unwrap();
    assert_json_contains(&json, &["file-1", "file-2", "file-3"]);

    let deserialized: VectorStoreFileBatchRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.file_ids, request.file_ids);
    assert_eq!(deserialized.chunking_strategy, request.chunking_strategy);
}

#[test]
fn test_empty_list_responses() {
    use openai_rust_sdk::models::vector_stores::{
        ListVectorStoreFilesResponse, ListVectorStoresResponse,
    };

    let empty_stores = ListVectorStoresResponse::empty();
    assert_eq!(empty_stores.object, "list");
    assert!(empty_stores.data.is_empty());
    assert!(!empty_stores.has_more);
    assert_eq!(empty_stores.total_usage_bytes(), 0);
    assert!(empty_stores.ready_stores().is_empty());

    let empty_files = ListVectorStoreFilesResponse::empty();
    assert_eq!(empty_files.object, "list");
    assert!(empty_files.data.is_empty());
    assert!(!empty_files.has_more);
    assert_eq!(empty_files.total_usage_bytes(), 0);
    assert!(empty_files.completed_files().is_empty());
}

// Generate error handling tests using a macro
macro_rules! test_param_edge_cases {
    ($($test_name:ident: $param_type:ty => $test_body:block),*) => {
        $(
            #[test]
            fn $test_name() $test_body
        )*
    };
}

test_param_edge_cases! {
    test_empty_params: ListVectorStoresParams => {
        let empty_params = ListVectorStoresParams::new();
        let query_params = empty_params.to_query_params();
        assert!(query_params.is_empty());
    },
    test_zero_limit_params: ListVectorStoresParams => {
        let params = ListVectorStoresParams::new().with_limit(0);
        let query_params = params.to_query_params();
        assert!(query_params.contains(&("limit".to_string(), "0".to_string())));
    }
}

#[test]
fn test_chunking_strategy_serialization() {
    let auto_strategy = ChunkingStrategy::auto();
    let auto_json = serde_json::to_string(&auto_strategy).unwrap();
    assert_json_contains(&auto_json, &["\"type\":\"auto\""]);
    test_serialization_round_trip(&auto_strategy);

    let static_strategy = ChunkingStrategy::static_chunking(512, 64);
    let static_json = serde_json::to_string(&static_strategy).unwrap();
    assert_json_contains(
        &static_json,
        &[
            "\"type\":\"static\"",
            "\"max_chunk_size_tokens\":512",
            "\"chunk_overlap_tokens\":64",
        ],
    );
    test_serialization_round_trip(&static_strategy);
}

#[test]
fn test_metadata_handling() {
    let mut metadata = create_test_metadata();
    metadata.insert("key1".to_string(), "value1".to_string());

    let request = VectorStoreRequest::builder()
        .name("Metadata Test")
        .metadata(metadata)
        .add_metadata("key3", "value3")
        .build();

    let final_metadata = request.metadata.unwrap();
    assert_eq!(final_metadata.len(), 4); // environment, version, key1, key3
    assert_eq!(final_metadata.get("environment"), Some(&"test".to_string()));
    assert_eq!(final_metadata.get("version"), Some(&"1.0".to_string()));
    assert_eq!(final_metadata.get("key1"), Some(&"value1".to_string()));
    assert_eq!(final_metadata.get("key3"), Some(&"value3".to_string()));
}

// Note: The following tests would be integration tests that require actual API access
// They are commented out but show the pattern for testing actual API calls

/*
#[tokio::test]
#[ignore] // Requires API key
async fn test_create_vector_store_integration() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api = VectorStoresApi::new(&api_key).unwrap();

    let request = VectorStoreRequest::builder()
        .name("Integration Test Store")
        .expires_after(ExpirationPolicy::new_days(1))
        .build();

    let result = api.create_vector_store(request).await;
    assert!(result.is_ok());

    let vector_store = result.unwrap();
    assert_eq!(vector_store.name, Some("Integration Test Store".to_string()));

    // Clean up
    let _ = api.delete_vector_store(&vector_store.id).await;
}

#[tokio::test]
#[ignore] // Requires API key
async fn test_list_vector_stores_integration() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api = VectorStoresApi::new(&api_key).unwrap();

    let params = ListVectorStoresParams::new().with_limit(5);
    let result = api.list_vector_stores(Some(params)).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.object, "list");
}

#[tokio::test]
#[ignore] // Requires API key and file IDs
async fn test_vector_store_file_batch_integration() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api = VectorStoresApi::new(&api_key).unwrap();

    // Create a vector store first
    let request = VectorStoreRequest::builder()
        .name("Batch Test Store")
        .build();
    let vector_store = api.create_vector_store(request).await.unwrap();

    // Add files in batch (would need actual file IDs)
    let file_ids = vec!["file-123".to_string(), "file-456".to_string()];
    let batch_result = api.create_vector_store_file_batch(&vector_store.id, file_ids).await;

    // Clean up
    let _ = api.delete_vector_store(&vector_store.id).await;
}
*/
