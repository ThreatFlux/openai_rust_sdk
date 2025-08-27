//! Tests for batch status handling

use openai_rust_sdk::api::batch::BatchStatus;

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
