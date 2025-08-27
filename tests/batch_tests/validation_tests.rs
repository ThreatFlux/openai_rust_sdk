//! Tests for batch validation

use crate::batch_tests::{create_sample_batch, create_sample_file_response};

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
