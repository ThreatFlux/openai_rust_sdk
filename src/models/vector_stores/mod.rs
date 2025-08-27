//! # Vector Stores API Models
//!
//! This module provides data structures for OpenAI's Vector Stores API, which allows you to
//! store and manage large amounts of file-based data in a vector database optimized for RAG.
//!
//! ## Overview
//!
//! Vector stores are used to store file data in a format optimized for retrieval-augmented generation (RAG).
//! They automatically process uploaded files, chunk the content, and create embeddings that can be
//! searched efficiently. Vector stores integrate seamlessly with the Assistants API.
//!
//! ## Key Features
//!
//! - **Automatic Processing**: Files are automatically chunked and embedded
//! - **Efficient Retrieval**: Optimized for similarity search and retrieval
//! - **File Management**: Support for attaching, listing, and removing files
//! - **Batch Operations**: Upload multiple files simultaneously
//! - **Expiration Policies**: Automatic cleanup with configurable expiration
//! - **Chunking Strategies**: Configurable text chunking for optimal embedding
//!
//! ## Module Structure
//!
//! This module is organized into focused sub-modules for better maintainability:
//!
//! - `status_types` - Status enums for vector stores, files, and batches
//! - `common_types` - Shared utility types like expiration policies and chunking strategies
//! - `store_types` - Main vector store types and request builders
//! - `file_types` - Vector store file associations and operations
//! - `batch_types` - Batch operations for multiple files
//! - `request_types` - Request parameter builders for API operations
//! - `response_types` - Response types and list operations
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::vector_stores::{VectorStoreRequest, ExpirationPolicy, ChunkingStrategy};
//!
//! // Create a vector store with expiration policy
//! let request = VectorStoreRequest::builder()
//!     .name("Knowledge Base")
//!     .expires_after(ExpirationPolicy::new_days(30))
//!     .build();
//! ```

pub mod batch_types;
pub mod common_types;
pub mod file_types;
pub mod request_types;
pub mod response_types;
pub mod status_types;
pub mod store_types;

// Re-export all public types for backward compatibility
pub use batch_types::*;
pub use common_types::*;
pub use file_types::*;
pub use request_types::*;
// Alias to avoid conflict with realtime_audio module
pub use response_types::{
    ListVectorStoreFilesResponse, ListVectorStoresResponse, VectorStoreDeleteResponse,
};
pub use status_types::*;
pub use store_types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_vector_store_status_display() {
        assert_eq!(VectorStoreStatus::InProgress.to_string(), "in_progress");
        assert_eq!(VectorStoreStatus::Completed.to_string(), "completed");
        assert_eq!(VectorStoreStatus::Failed.to_string(), "failed");
        assert_eq!(VectorStoreStatus::Cancelled.to_string(), "cancelled");
        assert_eq!(VectorStoreStatus::Expired.to_string(), "expired");
    }

    #[test]
    fn test_expiration_policy() {
        let policy = ExpirationPolicy::new_days(30);
        assert_eq!(policy.anchor, "last_active_at");
        assert_eq!(policy.days, 30);

        let custom_policy = ExpirationPolicy::new_with_anchor("created_at", 7);
        assert_eq!(custom_policy.anchor, "created_at");
        assert_eq!(custom_policy.days, 7);
    }

    #[test]
    fn test_file_counts() {
        let mut counts = FileCounts::new();
        assert_eq!(counts.total, 0);
        assert!(counts.is_processing_complete());
        assert_eq!(counts.completion_percentage(), 100.0);

        counts.total = 10;
        counts.completed = 7;
        counts.failed = 2;
        counts.in_progress = 1;

        assert!(!counts.is_processing_complete());
        assert_eq!(counts.completion_percentage(), 70.0);
        assert_eq!(counts.failure_percentage(), 20.0);
    }

    #[test]
    fn test_chunking_strategy() {
        let auto_strategy = ChunkingStrategy::auto();
        assert_eq!(auto_strategy, ChunkingStrategy::Auto);

        let static_strategy = ChunkingStrategy::static_chunking(512, 50);
        if let ChunkingStrategy::Static {
            max_chunk_size_tokens,
            chunk_overlap_tokens,
        } = static_strategy
        {
            assert_eq!(max_chunk_size_tokens, 512);
            assert_eq!(chunk_overlap_tokens, 50);
        } else {
            panic!("Expected static chunking strategy");
        }
    }

    #[test]
    fn test_vector_store_request_builder() {
        let request = VectorStoreRequest::builder()
            .name("Test Store")
            .add_file_id("file-123")
            .add_file_id("file-456")
            .expires_after(ExpirationPolicy::new_days(30))
            .chunking_strategy(ChunkingStrategy::static_chunking(512, 50))
            .add_metadata("environment", "test")
            .add_metadata("version", "1.0")
            .build();

        assert_eq!(request.name, Some("Test Store".to_string()));
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
    fn test_vector_store_file_request() {
        let request = VectorStoreFileRequest::new("file-123")
            .with_chunking_strategy(ChunkingStrategy::static_chunking(256, 25));

        assert_eq!(request.file_id, "file-123");
        assert!(request.chunking_strategy.is_some());
    }

    #[test]
    fn test_vector_store_file_batch_request() {
        let request =
            VectorStoreFileBatchRequest::new(vec!["file-1".to_string(), "file-2".to_string()])
                .with_chunking_strategy(ChunkingStrategy::auto());

        assert_eq!(request.file_ids.len(), 2);
        assert_eq!(request.chunking_strategy, Some(ChunkingStrategy::Auto));
    }

    #[test]
    fn test_list_vector_stores_params() {
        let params = ListVectorStoresParams::new()
            .with_limit(50)
            .with_order("desc")
            .with_after("vs-123");

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&("limit".to_string(), "50".to_string())));
        assert!(query_params.contains(&("order".to_string(), "desc".to_string())));
        assert!(query_params.contains(&("after".to_string(), "vs-123".to_string())));
    }

    #[test]
    fn test_list_vector_store_files_params() {
        let params = ListVectorStoreFilesParams::new()
            .with_limit(25)
            .with_filter(VectorStoreFileStatus::Completed)
            .with_order("asc");

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&("limit".to_string(), "25".to_string())));
        assert!(query_params.contains(&("filter".to_string(), "completed".to_string())));
        assert!(query_params.contains(&("order".to_string(), "asc".to_string())));
    }

    #[test]
    fn test_vector_store_status_methods() {
        let mut store = VectorStore {
            id: "vs-123".to_string(),
            object: "vector_store".to_string(),
            created_at: 1_640_995_200,
            name: Some("Test Store".to_string()),
            usage_bytes: 1024,
            file_counts: FileCounts::new(),
            status: VectorStoreStatus::Completed,
            expires_after: None,
            expires_at: None,
            last_active_at: None,
            metadata: HashMap::new(),
        };

        assert!(store.is_ready());
        assert!(!store.is_processing());
        assert!(!store.has_failed());
        assert!(!store.has_expired());
        assert_eq!(store.usage_human_readable(), "1.0 KB");
        assert!(!store.expires_soon());

        store.status = VectorStoreStatus::InProgress;
        assert!(!store.is_ready());
        assert!(store.is_processing());

        store.status = VectorStoreStatus::Failed;
        assert!(store.has_failed());

        store.status = VectorStoreStatus::Expired;
        assert!(store.has_expired());
    }

    #[test]
    fn test_list_responses_filtering() {
        let stores = vec![
            VectorStore {
                id: "vs-1".to_string(),
                object: "vector_store".to_string(),
                created_at: 1_640_995_200,
                name: Some("Store 1".to_string()),
                usage_bytes: 500,
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
                usage_bytes: 1500,
                file_counts: FileCounts::new(),
                status: VectorStoreStatus::InProgress,
                expires_after: None,
                expires_at: None,
                last_active_at: None,
                metadata: HashMap::new(),
            },
        ];

        let response = ListVectorStoresResponse {
            object: "list".to_string(),
            data: stores,
            first_id: None,
            last_id: None,
            has_more: false,
        };

        assert_eq!(response.total_usage_bytes(), 2000);
        assert_eq!(response.ready_stores().len(), 1);
        assert_eq!(response.processing_stores().len(), 1);
        assert_eq!(response.by_status(&VectorStoreStatus::Completed).len(), 1);
    }

    #[test]
    fn test_delete_responses() {
        let success = VectorStoreDeleteResponse::success("vs-123".to_string());
        assert!(success.deleted);
        assert_eq!(success.id, "vs-123");
        assert_eq!(success.object, "vector_store.deleted");

        let failure = VectorStoreDeleteResponse::failure("vs-456".to_string());
        assert!(!failure.deleted);
        assert_eq!(failure.id, "vs-456");

        let file_success = VectorStoreFileDeleteResponse::success("file-123".to_string());
        assert!(file_success.deleted);
        assert_eq!(file_success.object, "vector_store.file.deleted");
    }
}
