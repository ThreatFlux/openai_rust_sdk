//! Tests for batch serialization and deserialization

use crate::batch_tests::{
    create_sample_batch, create_sample_batch_list, create_sample_file_response,
};
use openai_rust_sdk::api::batch::{CreateBatchRequest, FileUploadResponse};

#[cfg(test)]
mod batch_serialization_tests {
    use super::*;

    #[test]
    fn test_batch_serialization() {
        let batch = create_sample_batch();
        let json = serde_json::to_string(&batch).unwrap();
        assert!(json.contains("batch_test123"));
        assert!(json.contains("in_progress"));

        let deserialized: openai_rust_sdk::api::batch::Batch = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, batch.id);
        assert_eq!(deserialized.status, batch.status);
    }

    #[test]
    fn test_file_upload_response_serialization() {
        let response = create_sample_file_response();
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("file-test123"));
        assert!(json.contains("batch"));

        let deserialized: FileUploadResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, response.id);
        assert_eq!(deserialized.purpose, response.purpose);
    }

    #[test]
    fn test_batch_list_serialization() {
        let batch_list = create_sample_batch_list();
        let json = serde_json::to_string(&batch_list).unwrap();
        assert!(json.contains("list"));

        let deserialized: openai_rust_sdk::api::batch::BatchList =
            serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.data.len(), batch_list.data.len());
        assert_eq!(deserialized.has_more, batch_list.has_more);
    }

    #[test]
    fn test_create_batch_request_serialization() {
        let request = CreateBatchRequest {
            input_file_id: "file-123".to_string(),
            endpoint: "/v1/chat/completions".to_string(),
            completion_window: "24h".to_string(),
            metadata: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("file-123"));
        assert!(json.contains("/v1/chat/completions"));
        assert!(!json.contains("metadata")); // Should be omitted when None

        // Note: CreateBatchRequest only derives Serialize, not Deserialize
        // so we can't test round-trip serialization
    }
}
