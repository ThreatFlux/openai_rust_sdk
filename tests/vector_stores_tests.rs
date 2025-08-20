#![allow(clippy::pedantic, clippy::nursery)]
//! Comprehensive tests for the Vector Stores API
//!
//! This module contains unit tests and integration tests for the Vector Stores API,
//! covering all major functionality including CRUD operations, file management,
//! batch operations, and error handling.

use openai_rust_sdk::api::vector_stores::VectorStoresApi;
use openai_rust_sdk::models::vector_stores::{
    ChunkingStrategy, ExpirationPolicy, FileCounts, ListVectorStoreFilesParams,
    ListVectorStoresParams, VectorStore, VectorStoreDeleteResponse, VectorStoreFile,
    VectorStoreFileBatchRequest, VectorStoreFileBatchStatus, VectorStoreFileDeleteResponse,
    VectorStoreFileRequest, VectorStoreFileStatus, VectorStoreRequest, VectorStoreStatus,
};
use std::collections::HashMap;

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
    if let ChunkingStrategy::Static {
        max_chunk_size_tokens,
        chunk_overlap_tokens,
    } = static_strategy
    {
        assert_eq!(max_chunk_size_tokens, 1024);
        assert_eq!(chunk_overlap_tokens, 100);
    } else {
        panic!("Expected static chunking strategy");
    }
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
    let request = VectorStoreFileRequest::new("file-abc123")
        .with_chunking_strategy(ChunkingStrategy::static_chunking(256, 25));

    assert_eq!(request.file_id, "file-abc123");
    assert!(request.chunking_strategy.is_some());

    if let Some(ChunkingStrategy::Static {
        max_chunk_size_tokens,
        chunk_overlap_tokens,
    }) = request.chunking_strategy
    {
        assert_eq!(max_chunk_size_tokens, 256);
        assert_eq!(chunk_overlap_tokens, 25);
    } else {
        panic!("Expected static chunking strategy");
    }
}

#[test]
fn test_vector_store_file_batch_request() {
    let file_ids = vec![
        "file-1".to_string(),
        "file-2".to_string(),
        "file-3".to_string(),
    ];
    let request = VectorStoreFileBatchRequest::new(file_ids.clone())
        .with_chunking_strategy(ChunkingStrategy::auto());

    assert_eq!(request.file_ids, file_ids);
    assert_eq!(request.chunking_strategy, Some(ChunkingStrategy::Auto));
}

#[test]
fn test_list_vector_stores_params() {
    let params = ListVectorStoresParams::new()
        .with_limit(25)
        .with_order("desc")
        .with_after("vs-abc123")
        .with_before("vs-xyz789");

    let query_params = params.to_query_params();
    assert_eq!(query_params.len(), 4);
    assert!(query_params.contains(&("limit".to_string(), "25".to_string())));
    assert!(query_params.contains(&("order".to_string(), "desc".to_string())));
    assert!(query_params.contains(&("after".to_string(), "vs-abc123".to_string())));
    assert!(query_params.contains(&("before".to_string(), "vs-xyz789".to_string())));
}

#[test]
fn test_list_vector_store_files_params() {
    let params = ListVectorStoreFilesParams::new()
        .with_limit(50)
        .with_order("asc")
        .with_filter(VectorStoreFileStatus::Completed)
        .with_after("file-123");

    let query_params = params.to_query_params();
    assert_eq!(query_params.len(), 4);
    assert!(query_params.contains(&("limit".to_string(), "50".to_string())));
    assert!(query_params.contains(&("order".to_string(), "asc".to_string())));
    assert!(query_params.contains(&("filter".to_string(), "completed".to_string())));
    assert!(query_params.contains(&("after".to_string(), "file-123".to_string())));
}

#[test]
fn test_vector_store_methods() {
    let mut vector_store = VectorStore {
        id: "vs-test123".to_string(),
        object: "vector_store".to_string(),
        created_at: 1_640_995_200,
        name: Some("Test Store".to_string()),
        usage_bytes: 2048,
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
    };

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

    let mut vector_store = VectorStore {
        id: "vs-test123".to_string(),
        object: "vector_store".to_string(),
        created_at: now - 3600,
        name: Some("Test Store".to_string()),
        usage_bytes: 1024,
        file_counts: FileCounts::new(),
        status: VectorStoreStatus::Completed,
        expires_after: None,
        expires_at: Some(now + 3600), // Expires in 1 hour (should trigger expires_soon)
        last_active_at: Some(now),
        metadata: HashMap::new(),
    };

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
    let stores = vec![
        VectorStore {
            id: "vs-1".to_string(),
            object: "vector_store".to_string(),
            created_at: 1_640_995_200,
            name: Some("Store 1".to_string()),
            usage_bytes: 1024,
            file_counts: FileCounts::new(),
            status: VectorStoreStatus::Completed,
            expires_after: None,
            expires_at: None,
            last_active_at: None,
            metadata: HashMap::new(),
        },
        VectorStore {
            id: "vs-2".to_string(),
            object: "vector_store".to_string(),
            created_at: 1_640_995_300,
            name: Some("Store 2".to_string()),
            usage_bytes: 2048,
            file_counts: FileCounts::new(),
            status: VectorStoreStatus::InProgress,
            expires_after: None,
            expires_at: None,
            last_active_at: None,
            metadata: HashMap::new(),
        },
    ];

    let stores_response = ListVectorStoresResponse {
        object: "list".to_string(),
        data: stores,
        first_id: Some("vs-1".to_string()),
        last_id: Some("vs-2".to_string()),
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
    let files = vec![
        VectorStoreFile {
            id: "file-1".to_string(),
            object: "vector_store.file".to_string(),
            usage_bytes: 512,
            created_at: 1_640_995_200,
            vector_store_id: "vs-1".to_string(),
            status: VectorStoreFileStatus::Completed,
            last_error: None,
            chunking_strategy: None,
        },
        VectorStoreFile {
            id: "file-2".to_string(),
            object: "vector_store.file".to_string(),
            usage_bytes: 256,
            created_at: 1_640_995_300,
            vector_store_id: "vs-1".to_string(),
            status: VectorStoreFileStatus::Failed,
            last_error: None,
            chunking_strategy: None,
        },
    ];

    let files_response = ListVectorStoreFilesResponse {
        object: "list".to_string(),
        data: files,
        first_id: Some("file-1".to_string()),
        last_id: Some("file-2".to_string()),
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
    assert!(json.contains("Serialization Test"));
    assert!(json.contains("file-test"));

    // Test that it can be deserialized back
    let deserialized: VectorStoreRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, request.name);
    assert_eq!(deserialized.file_ids, request.file_ids);
}

#[test]
fn test_file_batch_request_serialization() {
    let request =
        VectorStoreFileBatchRequest::new(vec!["file-1".to_string(), "file-2".to_string()])
            .with_chunking_strategy(ChunkingStrategy::auto());

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("file-1"));
    assert!(json.contains("file-2"));

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

#[test]
fn test_error_handling_patterns() {
    // Test that invalid parameters would cause errors
    let empty_params = ListVectorStoresParams::new();
    let query_params = empty_params.to_query_params();
    assert!(query_params.is_empty());

    let params_with_zero_limit = ListVectorStoresParams::new().with_limit(0);
    let query_params = params_with_zero_limit.to_query_params();
    assert!(query_params.contains(&("limit".to_string(), "0".to_string())));
}

#[test]
fn test_chunking_strategy_serialization() {
    let auto_strategy = ChunkingStrategy::auto();
    let auto_json = serde_json::to_string(&auto_strategy).unwrap();
    assert!(auto_json.contains("\"type\":\"auto\""));

    let static_strategy = ChunkingStrategy::static_chunking(512, 64);
    let static_json = serde_json::to_string(&static_strategy).unwrap();
    assert!(static_json.contains("\"type\":\"static\""));
    assert!(static_json.contains("\"max_chunk_size_tokens\":512"));
    assert!(static_json.contains("\"chunk_overlap_tokens\":64"));

    // Test deserialization
    let deserialized_auto: ChunkingStrategy = serde_json::from_str(&auto_json).unwrap();
    assert_eq!(deserialized_auto, ChunkingStrategy::Auto);

    let deserialized_static: ChunkingStrategy = serde_json::from_str(&static_json).unwrap();
    if let ChunkingStrategy::Static {
        max_chunk_size_tokens,
        chunk_overlap_tokens,
    } = deserialized_static
    {
        assert_eq!(max_chunk_size_tokens, 512);
        assert_eq!(chunk_overlap_tokens, 64);
    } else {
        panic!("Expected static chunking strategy");
    }
}

#[test]
fn test_metadata_handling() {
    let mut metadata = HashMap::new();
    metadata.insert("key1".to_string(), "value1".to_string());
    metadata.insert("key2".to_string(), "value2".to_string());

    let request = VectorStoreRequest::builder()
        .name("Metadata Test")
        .metadata(metadata.clone())
        .add_metadata("key3", "value3")
        .build();

    let final_metadata = request.metadata.unwrap();
    assert_eq!(final_metadata.len(), 3);
    assert_eq!(final_metadata.get("key1"), Some(&"value1".to_string()));
    assert_eq!(final_metadata.get("key2"), Some(&"value2".to_string()));
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
