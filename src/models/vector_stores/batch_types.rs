//! Batch operation types for vector store files
//!
//! This module contains types and functions for managing batch operations
//! on multiple files within vector stores.

use crate::models::vector_stores::common_types::{ChunkingStrategy, FileCounts};
use crate::models::vector_stores::status_types::VectorStoreFileBatchStatus;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// A vector store file batch represents a batch operation on multiple files
#[derive(Debug, Clone, Ser, De)]
pub struct VectorStoreFileBatch {
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always "`vector_store.files_batch`"
    pub object: String,
    /// The Unix timestamp (in seconds) for when the vector store files batch was created
    pub created_at: u64,
    /// The ID of the vector store that the files are being added to
    pub vector_store_id: String,
    /// The status of the vector store files batch
    pub status: VectorStoreFileBatchStatus,
    /// The file counts for different processing states
    pub file_counts: FileCounts,
}

impl VectorStoreFileBatch {
    /// Check if the batch processing is complete
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self.status, VectorStoreFileBatchStatus::Completed)
    }

    /// Check if the batch processing is in progress
    #[must_use]
    pub fn is_processing(&self) -> bool {
        matches!(self.status, VectorStoreFileBatchStatus::InProgress)
    }

    /// Check if the batch processing has failed
    #[must_use]
    pub fn has_failed(&self) -> bool {
        matches!(self.status, VectorStoreFileBatchStatus::Failed)
    }

    /// Check if the batch processing was cancelled
    #[must_use]
    pub fn was_cancelled(&self) -> bool {
        matches!(self.status, VectorStoreFileBatchStatus::Cancelled)
    }

    /// Get the creation date as a formatted string
    #[must_use]
    pub fn created_at_formatted(&self) -> String {
        crate::models::vector_stores::common_types::utils::format_timestamp(self.created_at)
    }

    /// Get the progress percentage of the batch operation
    #[must_use]
    pub fn progress_percentage(&self) -> f64 {
        self.file_counts.completion_percentage()
    }

    /// Check if all files in the batch have been processed
    #[must_use]
    pub fn all_files_processed(&self) -> bool {
        self.file_counts.is_processing_complete()
    }
}

/// Request to create a vector store file batch
#[derive(Debug, Clone, Ser, De)]
pub struct VectorStoreFileBatchRequest {
    /// A list of file IDs to be added to the vector store
    pub file_ids: Vec<String>,
    /// The chunking strategy used to chunk the file(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<ChunkingStrategy>,
}

impl VectorStoreFileBatchRequest {
    /// Create a new vector store file batch request
    #[must_use]
    pub fn new(file_ids: Vec<String>) -> Self {
        Self {
            file_ids,
            chunking_strategy: None,
        }
    }

    /// Set the chunking strategy for the files
    #[must_use]
    pub fn with_chunking_strategy(mut self, chunking_strategy: ChunkingStrategy) -> Self {
        self.chunking_strategy = Some(chunking_strategy);
        self
    }

    /// Add a file ID to the batch request
    pub fn add_file_id(mut self, file_id: impl Into<String>) -> Self {
        self.file_ids.push(file_id.into());
        self
    }

    /// Get the number of files in the batch
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.file_ids.len()
    }

    /// Check if the batch request is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.file_ids.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_store_file_batch_status_methods() {
        let mut batch = VectorStoreFileBatch {
            id: "batch-123".to_string(),
            object: "vector_store.files_batch".to_string(),
            created_at: 1_640_995_200,
            vector_store_id: "vs-456".to_string(),
            status: VectorStoreFileBatchStatus::Completed,
            file_counts: FileCounts {
                in_progress: 0,
                completed: 5,
                failed: 1,
                cancelled: 0,
                total: 6,
            },
        };

        assert!(batch.is_completed());
        assert!(!batch.is_processing());
        assert!(!batch.has_failed());
        assert!(!batch.was_cancelled());
        assert!(batch.all_files_processed());
        assert!((batch.progress_percentage() - 83.33).abs() < 0.01); // 5/6 * 100

        // Test different statuses
        batch.status = VectorStoreFileBatchStatus::InProgress;
        assert!(!batch.is_completed());
        assert!(batch.is_processing());

        batch.status = VectorStoreFileBatchStatus::Failed;
        assert!(batch.has_failed());

        batch.status = VectorStoreFileBatchStatus::Cancelled;
        assert!(batch.was_cancelled());
    }

    #[test]
    fn test_vector_store_file_batch_with_processing_files() {
        let batch = VectorStoreFileBatch {
            id: "batch-123".to_string(),
            object: "vector_store.files_batch".to_string(),
            created_at: 1_640_995_200,
            vector_store_id: "vs-456".to_string(),
            status: VectorStoreFileBatchStatus::InProgress,
            file_counts: FileCounts {
                in_progress: 2,
                completed: 3,
                failed: 0,
                cancelled: 0,
                total: 5,
            },
        };

        assert!(batch.is_processing());
        assert!(!batch.all_files_processed());
        assert_eq!(batch.progress_percentage(), 60.0); // 3/5 * 100
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
        assert_eq!(request.file_count(), 3);
        assert!(!request.is_empty());
    }

    #[test]
    fn test_vector_store_file_batch_request_builder_pattern() {
        let request = VectorStoreFileBatchRequest::new(vec!["file-1".to_string()])
            .add_file_id("file-2")
            .add_file_id("file-3")
            .with_chunking_strategy(ChunkingStrategy::static_chunking(512, 64));

        assert_eq!(request.file_count(), 3);
        assert_eq!(request.file_ids[0], "file-1");
        assert_eq!(request.file_ids[1], "file-2");
        assert_eq!(request.file_ids[2], "file-3");

        if let Some(ChunkingStrategy::Static {
            max_chunk_size_tokens,
            chunk_overlap_tokens,
        }) = request.chunking_strategy
        {
            assert_eq!(max_chunk_size_tokens, 512);
            assert_eq!(chunk_overlap_tokens, 64);
        } else {
            panic!("Expected static chunking strategy");
        }
    }

    #[test]
    fn test_empty_batch_request() {
        let request = VectorStoreFileBatchRequest::new(vec![]);
        assert!(request.is_empty());
        assert_eq!(request.file_count(), 0);

        let request_with_file = request.add_file_id("file-1");
        assert!(!request_with_file.is_empty());
        assert_eq!(request_with_file.file_count(), 1);
    }

    #[test]
    fn test_batch_serialization() {
        let batch = VectorStoreFileBatch {
            id: "batch-123".to_string(),
            object: "vector_store.files_batch".to_string(),
            created_at: 1_640_995_200,
            vector_store_id: "vs-456".to_string(),
            status: VectorStoreFileBatchStatus::Completed,
            file_counts: FileCounts {
                in_progress: 0,
                completed: 5,
                failed: 0,
                cancelled: 0,
                total: 5,
            },
        };

        let json = serde_json::to_string(&batch).unwrap();
        assert!(json.contains("batch-123"));
        assert!(json.contains("vs-456"));
        assert!(json.contains("completed"));

        let deserialized: VectorStoreFileBatch = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, batch.id);
        assert_eq!(deserialized.vector_store_id, batch.vector_store_id);
        assert_eq!(deserialized.status, batch.status);
    }

    #[test]
    fn test_batch_request_serialization() {
        let request =
            VectorStoreFileBatchRequest::new(vec!["file-1".to_string(), "file-2".to_string()])
                .with_chunking_strategy(ChunkingStrategy::auto());

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("file-1"));
        assert!(json.contains("file-2"));
        assert!(json.contains("auto"));

        let deserialized: VectorStoreFileBatchRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.file_ids, request.file_ids);
        assert_eq!(deserialized.chunking_strategy, request.chunking_strategy);
    }
}
