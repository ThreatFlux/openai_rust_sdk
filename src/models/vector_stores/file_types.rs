//! Vector store file types and operations
//!
//! This module contains types and functions for managing individual files
//! within vector stores, including file associations and their processing status.

use crate::models::vector_stores::common_types::{ChunkingStrategy, VectorStoreFileError, StatusChecker};
use crate::models::vector_stores::status_types::VectorStoreFileStatus;
use crate::{De, Ser, impl_status_methods};
use serde::{self, Deserialize, Serialize};

/// A vector store file represents the association between a file and a vector store
#[derive(Debug, Clone, Ser, De)]
pub struct VectorStoreFile {
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always "`vector_store.file`"
    pub object: String,
    /// The total vector store usage in bytes
    pub usage_bytes: u64,
    /// The Unix timestamp (in seconds) for when the vector store file was created
    pub created_at: u64,
    /// The ID of the vector store that the file is attached to
    pub vector_store_id: String,
    /// The status of the vector store file
    pub status: VectorStoreFileStatus,
    /// The last error associated with this vector store file, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<VectorStoreFileError>,
    /// The strategy used to chunk the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<ChunkingStrategy>,
}

// Use macro to generate status checking methods
impl_status_methods!(VectorStoreFile, VectorStoreFileStatus, {
    is_completed => Completed,
    is_processing => InProgress,
    has_failed => Failed,
    was_cancelled => Cancelled,
});

impl VectorStoreFile {

    /// Get human-readable usage size
    #[must_use]
    pub fn usage_human_readable(&self) -> String {
        crate::models::vector_stores::common_types::utils::bytes_to_human_readable(self.usage_bytes)
    }

    /// Get the creation date as a formatted string
    #[must_use]
    pub fn created_at_formatted(&self) -> String {
        crate::models::vector_stores::common_types::utils::format_timestamp(self.created_at)
    }

    /// Get error details if the file processing failed
    #[must_use]
    pub fn error_details(&self) -> Option<(String, String)> {
        self.last_error
            .as_ref()
            .map(|error| (error.code.clone(), error.message.clone()))
    }
}

/// Request to create a vector store file association
#[derive(Debug, Clone, Ser, De)]
pub struct VectorStoreFileRequest {
    /// A file ID that the vector store should use
    pub file_id: String,
    /// The chunking strategy used to chunk the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<ChunkingStrategy>,
}

impl VectorStoreFileRequest {
    /// Create a new vector store file request
    pub fn new(file_id: impl Into<String>) -> Self {
        Self {
            file_id: file_id.into(),
            chunking_strategy: None,
        }
    }

    /// Set the chunking strategy for the file
    #[must_use]
    pub fn with_chunking_strategy(mut self, chunking_strategy: ChunkingStrategy) -> Self {
        self.chunking_strategy = Some(chunking_strategy);
        self
    }
}

/// Response from deleting a vector store file
#[derive(Debug, Clone, Ser, De)]
pub struct VectorStoreFileDeleteResponse {
    /// The ID of the deleted vector store file
    pub id: String,
    /// The object type, which is always "`vector_store.file.deleted`"
    pub object: String,
    /// Whether the vector store file was successfully deleted
    pub deleted: bool,
}

impl VectorStoreFileDeleteResponse {
    /// Create a successful delete response
    #[must_use]
    pub fn success(id: String) -> Self {
        Self {
            id,
            object: "vector_store.file.deleted".to_string(),
            deleted: true,
        }
    }

    /// Create a failed delete response
    #[must_use]
    pub fn failure(id: String) -> Self {
        Self {
            id,
            object: "vector_store.file.deleted".to_string(),
            deleted: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_store_file_status_methods() {
        let mut file = VectorStoreFile {
            id: "file-123".to_string(),
            object: "vector_store.file".to_string(),
            usage_bytes: 2048,
            created_at: 1_640_995_200,
            vector_store_id: "vs-456".to_string(),
            status: VectorStoreFileStatus::Completed,
            last_error: None,
            chunking_strategy: None,
        };

        assert!(file.is_completed());
        assert!(!file.is_processing());
        assert!(!file.has_failed());
        assert!(!file.was_cancelled());
        assert_eq!(file.usage_human_readable(), "2.0 KB");
        assert!(file.error_details().is_none());

        // Test different statuses
        file.status = VectorStoreFileStatus::InProgress;
        assert!(!file.is_completed());
        assert!(file.is_processing());

        file.status = VectorStoreFileStatus::Failed;
        assert!(file.has_failed());

        file.status = VectorStoreFileStatus::Cancelled;
        assert!(file.was_cancelled());
    }

    #[test]
    fn test_vector_store_file_with_error() {
        let file = VectorStoreFile {
            id: "file-123".to_string(),
            object: "vector_store.file".to_string(),
            usage_bytes: 1024,
            created_at: 1_640_995_200,
            vector_store_id: "vs-456".to_string(),
            status: VectorStoreFileStatus::Failed,
            last_error: Some(VectorStoreFileError {
                code: "processing_error".to_string(),
                message: "Failed to process the file".to_string(),
            }),
            chunking_strategy: None,
        };

        assert!(file.has_failed());
        let (code, message) = file.error_details().unwrap();
        assert_eq!(code, "processing_error");
        assert_eq!(message, "Failed to process the file");
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
    fn test_vector_store_file_delete_response() {
        let success = VectorStoreFileDeleteResponse::success("file-123".to_string());
        assert!(success.deleted);
        assert_eq!(success.id, "file-123");
        assert_eq!(success.object, "vector_store.file.deleted");

        let failure = VectorStoreFileDeleteResponse::failure("file-456".to_string());
        assert!(!failure.deleted);
        assert_eq!(failure.id, "file-456");
        assert_eq!(failure.object, "vector_store.file.deleted");
    }

    #[test]
    fn test_vector_store_file_serialization() {
        let file = VectorStoreFile {
            id: "file-123".to_string(),
            object: "vector_store.file".to_string(),
            usage_bytes: 1024,
            created_at: 1_640_995_200,
            vector_store_id: "vs-456".to_string(),
            status: VectorStoreFileStatus::Completed,
            last_error: None,
            chunking_strategy: Some(ChunkingStrategy::auto()),
        };

        let json = serde_json::to_string(&file).unwrap();
        assert!(json.contains("file-123"));
        assert!(json.contains("vs-456"));
        assert!(json.contains("completed"));

        let deserialized: VectorStoreFile = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, file.id);
        assert_eq!(deserialized.vector_store_id, file.vector_store_id);
        assert_eq!(deserialized.status, file.status);
    }

    #[test]
    fn test_file_request_serialization() {
        let request = VectorStoreFileRequest::new("file-test")
            .with_chunking_strategy(ChunkingStrategy::static_chunking(512, 64));

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("file-test"));
        assert!(json.contains("static"));
        assert!(json.contains("512"));

        let deserialized: VectorStoreFileRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.file_id, request.file_id);
        assert_eq!(deserialized.chunking_strategy, request.chunking_strategy);
    }
}
