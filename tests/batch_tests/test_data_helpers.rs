//! Helper functions for creating test data

use openai_rust_sdk::api::batch::{
    Batch, BatchList, BatchRequestCounts, BatchStatus, FileUploadResponse,
};
use serde_json::json;

/// Helper functions for creating test data
pub fn create_sample_file_response() -> FileUploadResponse {
    FileUploadResponse {
        id: "file-test123".to_string(),
        object: "file".to_string(),
        bytes: 1024,
        created_at: 1_640_995_200,
        filename: "test_batch.jsonl".to_string(),
        purpose: "batch".to_string(),
    }
}

pub fn create_sample_batch() -> Batch {
    Batch {
        id: "batch_test123".to_string(),
        object: "batch".to_string(),
        endpoint: "/v1/chat/completions".to_string(),
        errors: None,
        input_file_id: "file-input123".to_string(),
        completion_window: "24h".to_string(),
        status: BatchStatus::InProgress,
        output_file_id: None,
        error_file_id: None,
        created_at: 1_640_995_200,
        in_progress_at: Some(1_640_995_300),
        expires_at: 1_641_081_600,
        completed_at: None,
        failed_at: None,
        expired_at: None,
        request_counts: BatchRequestCounts {
            total: 100,
            completed: 75,
            failed: 5,
        },
        metadata: Some(json!({"test": "true"})),
    }
}

pub fn create_completed_batch() -> Batch {
    let mut batch = create_sample_batch();
    batch.status = BatchStatus::Completed;
    batch.output_file_id = Some("file-output123".to_string());
    batch.completed_at = Some(1_641_000_000);
    batch.request_counts.completed = 100;
    batch.request_counts.failed = 0;
    batch
}

pub fn create_sample_batch_list() -> BatchList {
    BatchList {
        object: "list".to_string(),
        data: vec![create_sample_batch(), create_completed_batch()],
        has_more: false,
        first_id: Some("batch_test123".to_string()),
        last_id: Some("batch_completed123".to_string()),
    }
}
