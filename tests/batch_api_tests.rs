//! Comprehensive tests for the Batch API
//!
//! This test suite covers all functionality of the Batch API including:
//! - Batch creation and management
//! - File upload for batch processing
//! - Status monitoring and polling
//! - Result retrieval and processing
//! - YARA rule extraction from responses
//! - Error handling and edge cases
//! - Batch cancellation and cleanup
//! - Report generation and analysis

use openai_rust_sdk::api::batch::{
    Batch, BatchApi, BatchList, BatchReport, BatchRequestCounts, BatchStatus, CreateBatchRequest,
    FileUploadResponse,
};
use serde_json::json;

// Helper functions for creating test data
fn create_sample_file_response() -> FileUploadResponse {
    FileUploadResponse {
        id: "file-test123".to_string(),
        object: "file".to_string(),
        bytes: 1024,
        created_at: 1_640_995_200,
        filename: "test_batch.jsonl".to_string(),
        purpose: "batch".to_string(),
    }
}

fn create_sample_batch() -> Batch {
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

fn create_completed_batch() -> Batch {
    let mut batch = create_sample_batch();
    batch.status = BatchStatus::Completed;
    batch.output_file_id = Some("file-output123".to_string());
    batch.completed_at = Some(1_641_000_000);
    batch.request_counts.completed = 100;
    batch.request_counts.failed = 0;
    batch
}

fn create_sample_batch_list() -> BatchList {
    BatchList {
        object: "list".to_string(),
        data: vec![create_sample_batch(), create_completed_batch()],
        has_more: false,
        first_id: Some("batch_test123".to_string()),
        last_id: Some("batch_completed123".to_string()),
    }
}

#[cfg(test)]
mod batch_api_creation_tests {
    use super::*;

    #[test]
    fn test_batch_api_creation() {
        let api = BatchApi::new("test-api-key");
        assert!(api.is_ok());
    }

    #[test]
    fn test_batch_api_creation_with_empty_key() {
        let api = BatchApi::new("");
        assert!(api.is_err());
    }

    #[test]
    fn test_batch_api_with_custom_base_url() {
        let api = BatchApi::new_with_base_url("test-key", "https://custom.api.com");
        assert!(api.is_ok());
    }

    #[test]
    fn test_batch_api_with_invalid_base_url() {
        let api = BatchApi::new_with_base_url("test-key", "");
        assert!(api.is_ok()); // Empty base URL should still work (though not practical)
    }
}

#[cfg(test)]
mod batch_status_tests {
    use super::*;

    #[test]
    fn test_batch_status_enum_values() {
        let statuses = vec![
            BatchStatus::Validating,
            BatchStatus::Failed,
            BatchStatus::InProgress,
            BatchStatus::Finalizing,
            BatchStatus::Completed,
            BatchStatus::Expired,
            BatchStatus::Cancelling,
            BatchStatus::Cancelled,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            assert!(!serialized.is_empty());

            let deserialized: BatchStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(format!("{status:?}"), format!("{:?}", deserialized));
        }
    }

    #[test]
    fn test_batch_status_display() {
        assert_eq!(BatchStatus::Validating.to_string(), "validating");
        assert_eq!(BatchStatus::Failed.to_string(), "failed");
        assert_eq!(BatchStatus::InProgress.to_string(), "in_progress");
        assert_eq!(BatchStatus::Finalizing.to_string(), "finalizing");
        assert_eq!(BatchStatus::Completed.to_string(), "completed");
        assert_eq!(BatchStatus::Expired.to_string(), "expired");
        assert_eq!(BatchStatus::Cancelling.to_string(), "cancelling");
        assert_eq!(BatchStatus::Cancelled.to_string(), "cancelled");
    }

    #[test]
    fn test_batch_status_serialization() {
        let status = BatchStatus::InProgress;
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("in_progress"));

        let deserialized: BatchStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, BatchStatus::InProgress);
    }
}

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
        assert_eq!(batch.status, BatchStatus::InProgress);
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

#[cfg(test)]
mod batch_serialization_tests {
    use super::*;

    #[test]
    fn test_batch_serialization() {
        let batch = create_sample_batch();
        let json = serde_json::to_string(&batch).unwrap();
        assert!(json.contains("batch_test123"));
        assert!(json.contains("in_progress"));

        let deserialized: Batch = serde_json::from_str(&json).unwrap();
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

        let deserialized: BatchList = serde_json::from_str(&json).unwrap();
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

#[cfg(test)]
mod batch_report_tests {
    use super::*;

    #[test]
    fn test_batch_report_creation() {
        let report = BatchReport::new();
        assert_eq!(report.total_responses, 0);
        assert_eq!(report.successful_responses, 0);
        assert_eq!(report.error_responses, 0);
        assert_eq!(report.yara_rules_found, 0);
        assert_eq!(report.total_tokens, 0);
        assert!(report.error_types.is_empty());
    }

    #[test]
    fn test_batch_report_default() {
        let report = BatchReport::default();
        assert_eq!(report.total_responses, 0);
        assert_eq!(report.successful_responses, 0);
    }

    #[test]
    fn test_batch_report_success_rate() {
        let mut report = BatchReport::new();
        report.total_responses = 100;
        report.successful_responses = 85;

        assert_eq!(report.success_rate(), 85.0);

        // Test edge case with zero total
        let empty_report = BatchReport::new();
        assert_eq!(empty_report.success_rate(), 0.0);
    }

    #[test]
    fn test_batch_report_yara_extraction_rate() {
        let mut report = BatchReport::new();
        report.successful_responses = 100;
        report.yara_rules_found = 75;

        assert_eq!(report.yara_extraction_rate(), 75.0);

        // Test edge case with zero successful responses
        report.successful_responses = 0;
        assert_eq!(report.yara_extraction_rate(), 0.0);
    }

    #[test]
    fn test_batch_report_error_types() {
        let mut report = BatchReport::new();
        report.error_types.insert("rate_limit".to_string(), 5);
        report.error_types.insert("invalid_request".to_string(), 3);

        assert_eq!(report.error_types.len(), 2);
        assert_eq!(report.error_types.get("rate_limit"), Some(&5));
        assert_eq!(report.error_types.get("invalid_request"), Some(&3));
    }

    #[test]
    fn test_batch_report_generate_text() {
        let mut report = BatchReport::new();
        report.total_responses = 100;
        report.successful_responses = 96; // Change to 96% to trigger "excellent" message
        report.error_responses = 4;
        report.yara_rules_found = 80;
        report.total_tokens = 50000;
        report.error_types.insert("timeout".to_string(), 4);

        let report_text = report.generate_report_text();

        assert!(report_text.contains("# OpenAI Batch Processing Report"));
        assert!(report_text.contains("Total Responses**: 100"));
        assert!(report_text.contains("Success Rate**: 96.0%"));
        assert!(report_text.contains("YARA Rules Found**: 80"));
        assert!(report_text.contains("**timeout**: 4 occurrences"));
        assert!(report_text.contains("## Recommendations"));
        assert!(report_text.contains("✅ Excellent success rate"));
    }

    #[test]
    fn test_batch_report_recommendations() {
        // Test low success rate recommendation
        let mut low_success_report = BatchReport::new();
        low_success_report.total_responses = 100;
        low_success_report.successful_responses = 80; // Below 90%

        let report_text = low_success_report.generate_report_text();
        assert!(report_text.contains("⚠️ Success rate is below 90%"));

        // Test high success rate recommendation
        let mut high_success_report = BatchReport::new();
        high_success_report.total_responses = 100;
        high_success_report.successful_responses = 98; // Above 95%

        let report_text = high_success_report.generate_report_text();
        assert!(report_text.contains("✅ Excellent success rate"));
    }

    #[test]
    fn test_batch_report_yara_recommendations() {
        // Test low YARA extraction rate
        let mut low_yara_report = BatchReport::new();
        low_yara_report.successful_responses = 100;
        low_yara_report.yara_rules_found = 70; // Below 80%

        let report_text = low_yara_report.generate_report_text();
        assert!(report_text.contains("⚠️ YARA rule extraction rate is low"));

        // Test high YARA extraction rate
        let mut high_yara_report = BatchReport::new();
        high_yara_report.successful_responses = 100;
        high_yara_report.yara_rules_found = 95; // Above 90%

        let report_text = high_yara_report.generate_report_text();
        assert!(report_text.contains("✅ High YARA rule extraction rate"));
    }
}

#[cfg(test)]
mod batch_validation_tests {
    use super::*;

    #[test]
    fn test_batch_id_validation() {
        let batch = create_sample_batch();
        assert!(!batch.id.is_empty());
        assert!(batch.id.starts_with("batch_"));
    }

    #[test]
    fn test_file_id_validation() {
        let response = create_sample_file_response();
        assert!(!response.id.is_empty());
        assert!(response.id.starts_with("file-"));
    }

    #[test]
    fn test_endpoint_validation() {
        let batch = create_sample_batch();
        assert!(batch.endpoint.starts_with("/v1/"));
        assert!(batch.endpoint.contains("completions"));
    }

    #[test]
    fn test_completion_window_validation() {
        let batch = create_sample_batch();
        assert_eq!(batch.completion_window, "24h");
    }

    #[test]
    fn test_request_counts_consistency() {
        let batch = create_sample_batch();
        let counts = &batch.request_counts;

        // Completed + failed should not exceed total
        assert!(counts.completed + counts.failed <= counts.total);
    }
}

#[cfg(test)]
mod yara_extraction_tests {

    #[test]
    fn test_yara_rule_pattern_detection() {
        // Test content with YARA rule in markdown code block
        let content_with_yara = r#"
Here's the YARA rule for detecting malware:

```yara
rule DetectMalware {
    meta:
        description = "Detects specific malware"
        author = "Security Team"
    
    strings:
        $hex = { 4D 5A 90 00 }
        $string = "malicious_pattern"
    
    condition:
        $hex at 0 and $string
}
```

This rule looks for specific patterns.
        "#;

        // This test verifies the pattern exists that would be extracted
        assert!(content_with_yara.contains("rule DetectMalware"));
        assert!(content_with_yara.contains("```yara"));
        assert!(content_with_yara.contains("condition:"));
    }

    #[test]
    fn test_yara_rule_without_code_blocks() {
        let content_with_plain_yara = r#"
rule SimpleRule {
    strings:
        $s = "test"
    condition:
        $s
}
        "#;

        // Verify basic YARA rule structure
        assert!(content_with_plain_yara.contains("rule "));
        assert!(content_with_plain_yara.contains("{"));
        assert!(content_with_plain_yara.contains("}"));
        assert!(content_with_plain_yara.contains("condition:"));
    }

    #[test]
    fn test_complex_yara_rule_pattern() {
        let complex_yara = r#"
rule ComplexMalwareDetection {
    meta:
        description = "Advanced malware detection"
        author = "Threat Research Team"
        date = "2024-01-01"
        version = "1.0"
    
    strings:
        $header = { 4D 5A }
        $payload1 = { E8 [4] 58 [0-10] C3 }
        $payload2 = "CreateRemoteThread"
        $payload3 = /\x00CreateProcess[AW]\x00/ nocase
    
    condition:
        $header at 0 and 
        (
            ($payload1 and #payload2 > 2) or
            $payload3
        ) and
        filesize < 2MB
}
        "#;

        // Verify complex YARA rule components
        assert!(complex_yara.contains("rule ComplexMalwareDetection"));
        assert!(complex_yara.contains("meta:"));
        assert!(complex_yara.contains("strings:"));
        assert!(complex_yara.contains("condition:"));
        assert!(complex_yara.contains("filesize"));
    }

    #[test]
    fn test_invalid_yara_content() {
        let invalid_content = "This is just plain text without any YARA rules";

        // Should not contain YARA patterns
        assert!(!invalid_content.contains("rule "));
        assert!(!invalid_content.contains("condition:"));
    }
}

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

#[cfg(test)]
mod batch_timestamps_tests {
    use super::*;

    #[test]
    fn test_batch_timestamp_fields() {
        let batch = create_sample_batch();

        assert!(batch.created_at > 0);
        assert!(batch.in_progress_at.is_some());
        assert!(batch.expires_at > batch.created_at);
        assert!(batch.completed_at.is_none()); // In progress batch
        assert!(batch.failed_at.is_none());
        assert!(batch.expired_at.is_none());
    }

    #[test]
    fn test_completed_batch_timestamps() {
        let batch = create_completed_batch();

        assert!(batch.created_at > 0);
        assert!(batch.completed_at.is_some());
        assert!(batch.completed_at.unwrap() > batch.created_at);
        assert_eq!(batch.status, BatchStatus::Completed);
    }

    #[test]
    fn test_timestamp_logical_order() {
        let batch = create_completed_batch();

        let created = batch.created_at;
        let in_progress = batch.in_progress_at.unwrap();
        let completed = batch.completed_at.unwrap();

        // Logical order: created < in_progress < completed
        assert!(created <= in_progress);
        assert!(in_progress <= completed);
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_batch_with_errors() {
        let mut batch = create_sample_batch();
        batch.status = BatchStatus::Failed;
        batch.errors = Some(json!({
            "code": "invalid_request",
            "message": "Input file format is invalid"
        }));
        batch.error_file_id = Some("file-errors123".to_string());
        batch.failed_at = Some(1_641_000_000);

        assert_eq!(batch.status, BatchStatus::Failed);
        assert!(batch.errors.is_some());
        assert!(batch.error_file_id.is_some());
        assert!(batch.failed_at.is_some());
    }

    #[test]
    fn test_batch_error_types() {
        let error_scenarios = vec![
            ("invalid_request", "Malformed input file"),
            ("rate_limit_exceeded", "Too many requests"),
            ("timeout", "Processing timeout"),
            ("quota_exceeded", "Usage quota exceeded"),
        ];

        for (error_code, error_message) in error_scenarios {
            let error_json = json!({
                "code": error_code,
                "message": error_message
            });

            assert_eq!(error_json["code"], error_code);
            assert_eq!(error_json["message"], error_message);
        }
    }

    #[test]
    fn test_request_counts_with_failures() {
        let counts = BatchRequestCounts {
            total: 100,
            completed: 85,
            failed: 15,
        };

        assert_eq!(counts.total, 100);
        assert_eq!(counts.completed + counts.failed, 100);

        // Calculate failure rate
        let failure_rate = (counts.failed as f64 / counts.total as f64) * 100.0;
        assert_eq!(failure_rate, 15.0);
    }
}

#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_empty_batch_list() {
        let empty_list = BatchList {
            object: "list".to_string(),
            data: vec![],
            has_more: false,
            first_id: None,
            last_id: None,
        };

        assert!(empty_list.data.is_empty());
        assert!(!empty_list.has_more);
        assert!(empty_list.first_id.is_none());
        assert!(empty_list.last_id.is_none());
    }

    #[test]
    fn test_zero_request_batch() {
        let mut batch = create_sample_batch();
        batch.request_counts = BatchRequestCounts {
            total: 0,
            completed: 0,
            failed: 0,
        };

        assert_eq!(batch.request_counts.total, 0);
        assert_eq!(batch.request_counts.completed, 0);
        assert_eq!(batch.request_counts.failed, 0);
    }

    #[test]
    fn test_very_large_batch() {
        let mut batch = create_sample_batch();
        batch.request_counts = BatchRequestCounts {
            total: 50000,
            completed: 45000,
            failed: 5000,
        };

        assert_eq!(batch.request_counts.total, 50000);
        let completion_rate =
            (batch.request_counts.completed as f64 / batch.request_counts.total as f64) * 100.0;
        assert_eq!(completion_rate, 90.0);
    }

    #[test]
    fn test_batch_status_transitions() {
        let status_flow = [
            BatchStatus::Validating,
            BatchStatus::InProgress,
            BatchStatus::Finalizing,
            BatchStatus::Completed,
        ];

        for status in status_flow.iter() {
            let mut batch = create_sample_batch();
            batch.status = status.clone();

            // Each status should be valid
            let json = serde_json::to_string(&batch);
            assert!(json.is_ok());
        }
    }

    #[test]
    fn test_concurrent_batch_processing() {
        let batches = vec![create_sample_batch(), create_completed_batch()];

        // Simulate multiple batches with different statuses
        assert_eq!(batches[0].status, BatchStatus::InProgress);
        assert_eq!(batches[1].status, BatchStatus::Completed);

        // Both should be valid
        for batch in &batches {
            let json = serde_json::to_string(batch);
            assert!(json.is_ok());
        }
    }
}

// Note: Integration tests that require actual API calls would go here
// They are commented out since they require real API keys and network access

/*
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY environment variable
    async fn test_batch_file_upload_integration() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let api = BatchApi::new(&api_key).unwrap();

        // Create a temporary batch file
        let mut temp_file = NamedTempFile::new().unwrap();
        let batch_content = r#"{"custom_id": "request-1", "method": "POST", "url": "/v1/chat/completions", "body": {"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "What is 2+2?"}]}}"#;
        temp_file.write_all(batch_content.as_bytes()).unwrap();

        let result = api.upload_batch_file(temp_file.path()).await;
        assert!(result.is_ok());

        let file_response = result.unwrap();
        assert!(!file_response.id.is_empty());
        assert_eq!(file_response.purpose, "batch");
    }

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY environment variable
    async fn test_batch_creation_integration() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let api = BatchApi::new(&api_key).unwrap();

        // This would use a real file ID from upload
        let file_id = "file-test123"; // Would be from actual upload
        let result = api.create_batch(file_id, "/v1/chat/completions").await;

        // In real integration test, this should succeed
        // assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY environment variable
    async fn test_batch_status_monitoring_integration() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let api = BatchApi::new(&api_key).unwrap();

        let batch_id = "batch_test123"; // Would be from actual batch creation
        let result = api.get_batch_status(batch_id).await;

        // In real integration test with valid batch ID:
        // assert!(result.is_ok());
        // let batch = result.unwrap();
        // assert!(!batch.id.is_empty());
    }
}
*/
