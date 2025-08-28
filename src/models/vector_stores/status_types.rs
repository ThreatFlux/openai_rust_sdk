//! Status types for vector stores and related operations
//!
//! This module contains status enumerations used throughout the Vector Stores API,
//! including statuses for vector stores, files, and batch operations.

use crate::models::shared_traits::StatusEnum;
use crate::{impl_status_enum, De, Ser};
use serde::{self, Deserialize, Serialize};

/// Status of a vector store
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreStatus {
    /// Vector store is being created and set up
    InProgress,
    /// Vector store is ready for use
    Completed,
    /// Vector store creation failed
    Failed,
    /// Vector store has been cancelled
    Cancelled,
    /// Vector store has expired and is being cleaned up
    Expired,
}

impl_status_enum!(VectorStoreStatus, {
    terminal: [Completed, Failed, Cancelled, Expired],
    active: [InProgress],
    failed: [Failed],
    completed: [Completed],
});

impl VectorStoreStatus {
    /// Check if the vector store is ready for use
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Check if the vector store is currently processing
    pub fn is_processing(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    /// Check if the vector store has failed
    pub fn has_failed(&self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Check if the vector store has expired
    pub fn has_expired(&self) -> bool {
        matches!(self, Self::Expired)
    }
}

/// Status of a vector store file
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreFileStatus {
    /// File is being processed
    InProgress,
    /// File has been successfully processed
    Completed,
    /// File processing was cancelled
    Cancelled,
    /// File processing failed
    Failed,
}

impl_status_enum!(VectorStoreFileStatus, {
    terminal: [Completed, Cancelled, Failed],
    active: [InProgress],
    failed: [Failed],
    completed: [Completed],
});

/// Status of a vector store file batch
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreFileBatchStatus {
    /// Batch is being processed
    InProgress,
    /// Batch has been completed
    Completed,
    /// Batch was cancelled
    Cancelled,
    /// Batch failed
    Failed,
}

impl_status_enum!(VectorStoreFileBatchStatus, {
    terminal: [Completed, Cancelled, Failed],
    active: [InProgress],
    failed: [Failed],
    completed: [Completed],
});

#[cfg(test)]
mod tests {
    use super::*;
    
    // Generate comprehensive tests for all status enums using the shared macro
    // This eliminates the duplicated test code that was present before
    
    // Vector Store Status Tests
    #[test]
    fn test_vector_store_status_terminal_states() {
        assert!(VectorStoreStatus::Completed.is_terminal());
        assert!(VectorStoreStatus::Failed.is_terminal());
        assert!(VectorStoreStatus::Cancelled.is_terminal());
        assert!(VectorStoreStatus::Expired.is_terminal());
        assert!(!VectorStoreStatus::InProgress.is_terminal());
    }
    
    #[test]
    fn test_vector_store_status_active_states() {
        assert!(VectorStoreStatus::InProgress.is_active());
        assert!(!VectorStoreStatus::Completed.is_active());
        assert!(!VectorStoreStatus::Failed.is_active());
        assert!(!VectorStoreStatus::Cancelled.is_active());
        assert!(!VectorStoreStatus::Expired.is_active());
    }
    
    #[test]
    fn test_vector_store_status_display() {
        assert_eq!(VectorStoreStatus::InProgress.to_string(), "in_progress");
        assert_eq!(VectorStoreStatus::Completed.to_string(), "completed");
        assert_eq!(VectorStoreStatus::Failed.to_string(), "failed");
        assert_eq!(VectorStoreStatus::Cancelled.to_string(), "cancelled");
        assert_eq!(VectorStoreStatus::Expired.to_string(), "expired");
    }
    
    // Vector Store File Status Tests
    #[test]
    fn test_vector_store_file_status_terminal_states() {
        assert!(VectorStoreFileStatus::Completed.is_terminal());
        assert!(VectorStoreFileStatus::Cancelled.is_terminal());
        assert!(VectorStoreFileStatus::Failed.is_terminal());
        assert!(!VectorStoreFileStatus::InProgress.is_terminal());
    }
    
    #[test]
    fn test_vector_store_file_status_active_states() {
        assert!(VectorStoreFileStatus::InProgress.is_active());
        assert!(!VectorStoreFileStatus::Completed.is_active());
        assert!(!VectorStoreFileStatus::Cancelled.is_active());
        assert!(!VectorStoreFileStatus::Failed.is_active());
    }
    
    // Vector Store File Batch Status Tests  
    #[test]
    fn test_vector_store_file_batch_status_terminal_states() {
        assert!(VectorStoreFileBatchStatus::Completed.is_terminal());
        assert!(VectorStoreFileBatchStatus::Cancelled.is_terminal());
        assert!(VectorStoreFileBatchStatus::Failed.is_terminal());
        assert!(!VectorStoreFileBatchStatus::InProgress.is_terminal());
    }
    
    #[test]
    fn test_vector_store_file_batch_status_active_states() {
        assert!(VectorStoreFileBatchStatus::InProgress.is_active());
        assert!(!VectorStoreFileBatchStatus::Completed.is_active());
        assert!(!VectorStoreFileBatchStatus::Cancelled.is_active());
        assert!(!VectorStoreFileBatchStatus::Failed.is_active());
    }
    
    // Consolidated serialization test
    #[test]
    fn test_all_status_serialization() {
        // Vector Store Status
        let vs_status = VectorStoreStatus::InProgress;
        let vs_json = serde_json::to_string(&vs_status).unwrap();
        assert_eq!(vs_json, "\"in_progress\"");
        let vs_deserialized: VectorStoreStatus = serde_json::from_str(&vs_json).unwrap();
        assert_eq!(vs_deserialized, vs_status);

        // Vector Store File Status
        let file_status = VectorStoreFileStatus::Completed;
        let file_json = serde_json::to_string(&file_status).unwrap();
        assert_eq!(file_json, "\"completed\"");
        let file_deserialized: VectorStoreFileStatus = serde_json::from_str(&file_json).unwrap();
        assert_eq!(file_deserialized, file_status);

        // Vector Store File Batch Status
        let batch_status = VectorStoreFileBatchStatus::Failed;
        let batch_json = serde_json::to_string(&batch_status).unwrap();
        assert_eq!(batch_json, "\"failed\"");
        let batch_deserialized: VectorStoreFileBatchStatus = serde_json::from_str(&batch_json).unwrap();
        assert_eq!(batch_deserialized, batch_status);
    }
}
