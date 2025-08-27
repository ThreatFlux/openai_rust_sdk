//! Tests for batch error handling

use crate::batch_tests::create_sample_batch;
use openai_rust_sdk::api::batch::{BatchRequestCounts, BatchStatus};
use serde_json::json;

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
