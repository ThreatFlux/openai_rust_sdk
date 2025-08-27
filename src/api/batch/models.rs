//! Request and response models for the OpenAI Batch API

use crate::{De, Ser};
use serde::{Deserialize, Serialize};

use super::types::{BatchRequestCounts, BatchStatus};

/// File upload response from `OpenAI` Files API
#[derive(Debug, Clone, Ser, De)]
pub struct FileUploadResponse {
    /// Unique identifier for the uploaded file
    pub id: String,
    /// Type of object (always "file")
    pub object: String,
    /// Size of the file in bytes
    pub bytes: u64,
    /// Unix timestamp of when the file was created
    pub created_at: u64,
    /// Name of the uploaded file
    pub filename: String,
    /// Purpose of the file upload (always "batch" for batch API)
    pub purpose: String,
}

/// Complete batch object returned by `OpenAI`
#[derive(Debug, Clone, Ser, De)]
pub struct Batch {
    /// Unique identifier for the batch
    pub id: String,
    /// Type of object (always "batch")
    pub object: String,
    /// API endpoint used for the batch
    pub endpoint: String,
    /// Any errors that occurred during batch processing
    pub errors: Option<serde_json::Value>,
    /// ID of the input file
    pub input_file_id: String,
    /// Completion window (e.g., "24h")
    pub completion_window: String,
    /// Current status of the batch
    pub status: BatchStatus,
    /// ID of the output file (available when completed)
    pub output_file_id: Option<String>,
    /// ID of the error file (if any errors occurred)
    pub error_file_id: Option<String>,
    /// Unix timestamp of when the batch was created
    pub created_at: u64,
    /// Unix timestamp of when the batch started processing
    pub in_progress_at: Option<u64>,
    /// Unix timestamp of when the batch expires
    pub expires_at: u64,
    /// Unix timestamp of when the batch completed
    pub completed_at: Option<u64>,
    /// Unix timestamp of when the batch failed
    pub failed_at: Option<u64>,
    /// Unix timestamp of when the batch expired
    pub expired_at: Option<u64>,
    /// Request counts and statistics
    pub request_counts: BatchRequestCounts,
    /// Optional metadata for the batch
    pub metadata: Option<serde_json::Value>,
}

/// Request to create a new batch
#[derive(Debug, Clone, Ser)]
pub struct CreateBatchRequest {
    /// ID of the uploaded input file
    pub input_file_id: String,
    /// API endpoint to use for the batch
    pub endpoint: String,
    /// Completion window (currently only "24h" is supported)
    pub completion_window: String,
    /// Optional metadata for the batch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl CreateBatchRequest {
    /// Create a new batch request
    pub fn new(input_file_id: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            input_file_id: input_file_id.into(),
            endpoint: endpoint.into(),
            completion_window: "24h".to_string(),
            metadata: None,
        }
    }

    /// Set metadata for the batch
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// List of batches response
#[derive(Debug, Clone, Ser, De)]
pub struct BatchList {
    /// Type of object (always "list")
    pub object: String,
    /// Array of batch objects
    pub data: Vec<Batch>,
    /// Whether there are more results available
    pub has_more: bool,
    /// ID of the first item in the list
    pub first_id: Option<String>,
    /// ID of the last item in the list
    pub last_id: Option<String>,
}
