//! Tests for batch timestamp handling

use crate::batch_tests::{create_completed_batch, create_sample_batch};
use openai_rust_sdk::api::batch::BatchStatus;

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
