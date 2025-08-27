//! Status types for vector stores and related operations
//!
//! This module contains status enumerations used throughout the Vector Stores API,
//! including statuses for vector stores, files, and batch operations.

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for VectorStoreStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            VectorStoreStatus::InProgress => "in_progress",
            VectorStoreStatus::Completed => "completed",
            VectorStoreStatus::Failed => "failed",
            VectorStoreStatus::Cancelled => "cancelled",
            VectorStoreStatus::Expired => "expired",
        };
        write!(f, "{status}")
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

impl fmt::Display for VectorStoreFileStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            VectorStoreFileStatus::InProgress => "in_progress",
            VectorStoreFileStatus::Completed => "completed",
            VectorStoreFileStatus::Cancelled => "cancelled",
            VectorStoreFileStatus::Failed => "failed",
        };
        write!(f, "{status}")
    }
}

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

impl fmt::Display for VectorStoreFileBatchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            VectorStoreFileBatchStatus::InProgress => "in_progress",
            VectorStoreFileBatchStatus::Completed => "completed",
            VectorStoreFileBatchStatus::Cancelled => "cancelled",
            VectorStoreFileBatchStatus::Failed => "failed",
        };
        write!(f, "{status}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_store_status_display() {
        assert_eq!(VectorStoreStatus::InProgress.to_string(), "in_progress");
        assert_eq!(VectorStoreStatus::Completed.to_string(), "completed");
        assert_eq!(VectorStoreStatus::Failed.to_string(), "failed");
        assert_eq!(VectorStoreStatus::Cancelled.to_string(), "cancelled");
        assert_eq!(VectorStoreStatus::Expired.to_string(), "expired");
    }

    #[test]
    fn test_vector_store_file_status_display() {
        assert_eq!(VectorStoreFileStatus::InProgress.to_string(), "in_progress");
        assert_eq!(VectorStoreFileStatus::Completed.to_string(), "completed");
        assert_eq!(VectorStoreFileStatus::Cancelled.to_string(), "cancelled");
        assert_eq!(VectorStoreFileStatus::Failed.to_string(), "failed");
    }

    #[test]
    fn test_vector_store_file_batch_status_display() {
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
    fn test_status_serialization() {
        // Test serialization/deserialization
        let status = VectorStoreStatus::InProgress;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"in_progress\"");

        let deserialized: VectorStoreStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, status);

        let file_status = VectorStoreFileStatus::Completed;
        let file_json = serde_json::to_string(&file_status).unwrap();
        assert_eq!(file_json, "\"completed\"");

        let batch_status = VectorStoreFileBatchStatus::Failed;
        let batch_json = serde_json::to_string(&batch_status).unwrap();
        assert_eq!(batch_json, "\"failed\"");
    }
}
