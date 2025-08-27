//! Tests for batch data structures

use crate::batch_tests::{
    create_sample_batch, create_sample_batch_list, create_sample_file_response,
};
use openai_rust_sdk::api::batch::{BatchRequestCounts, CreateBatchRequest};
use serde_json::json;

#[cfg(test)]
mod batch_data_structure_tests {
    use super::*;

    #[test]
    fn test_file_upload_response_creation() {
        let response = create_sample_file_response();
        assert_eq!(response.id, "file-test123");
        assert_eq!(response.object, "file");
        assert_eq!(response.bytes, 1024);
        assert_eq!(response.purpose, "batch");
    }

    #[test]
    fn test_batch_request_counts() {
        let counts = BatchRequestCounts {
            total: 100,
            completed: 80,
            failed: 15,
        };

        assert_eq!(counts.total, 100);
        assert_eq!(counts.completed, 80);
        assert_eq!(counts.failed, 15);
    }

    #[test]
    fn test_batch_creation() {
        let batch = create_sample_batch();
        assert_eq!(batch.id, "batch_test123");
        assert_eq!(
            batch.status,
            openai_rust_sdk::api::batch::BatchStatus::InProgress
        );
        assert_eq!(batch.endpoint, "/v1/chat/completions");
        assert_eq!(batch.request_counts.total, 100);
    }

    #[test]
    fn test_create_batch_request() {
        let request = CreateBatchRequest {
            input_file_id: "file-123".to_string(),
            endpoint: "/v1/chat/completions".to_string(),
            completion_window: "24h".to_string(),
            metadata: Some(json!({"key": "value"})),
        };

        assert_eq!(request.input_file_id, "file-123");
        assert_eq!(request.endpoint, "/v1/chat/completions");
        assert_eq!(request.completion_window, "24h");
        assert!(request.metadata.is_some());
    }

    #[test]
    fn test_batch_list_structure() {
        let batch_list = create_sample_batch_list();
        assert_eq!(batch_list.object, "list");
        assert_eq!(batch_list.data.len(), 2);
        assert!(!batch_list.has_more);
        assert_eq!(batch_list.first_id, Some("batch_test123".to_string()));
    }
}
