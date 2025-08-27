//! Tests for batch metadata handling

use crate::batch_tests::create_sample_batch;
use openai_rust_sdk::api::batch::CreateBatchRequest;
use serde_json::json;

#[cfg(test)]
mod batch_metadata_tests {
    use super::*;

    #[test]
    fn test_batch_with_metadata() {
        let mut batch = create_sample_batch();
        batch.metadata = Some(json!({
            "project": "malware_analysis",
            "version": "1.0",
            "priority": "high"
        }));

        assert!(batch.metadata.is_some());
        let metadata = batch.metadata.unwrap();
        assert_eq!(metadata["project"], "malware_analysis");
        assert_eq!(metadata["version"], "1.0");
        assert_eq!(metadata["priority"], "high");
    }

    #[test]
    fn test_batch_without_metadata() {
        let mut batch = create_sample_batch();
        batch.metadata = None;

        assert!(batch.metadata.is_none());
    }

    #[test]
    fn test_metadata_serialization() {
        let metadata = json!({
            "key1": "value1",
            "key2": 42,
            "key3": true
        });

        let request = CreateBatchRequest {
            input_file_id: "file-123".to_string(),
            endpoint: "/v1/chat/completions".to_string(),
            completion_window: "24h".to_string(),
            metadata: Some(metadata.clone()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("key1"));
        assert!(json.contains("value1"));
        assert!(json.contains("42"));
        assert!(json.contains("true"));
    }
}
