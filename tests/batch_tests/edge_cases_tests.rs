//! Tests for batch edge cases

use crate::batch_tests::create_sample_batch;
use openai_rust_sdk::api::batch::{BatchList, BatchRequestCounts, BatchStatus};

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
    let batches = vec![
        create_sample_batch(),
        crate::batch_tests::create_completed_batch(),
    ];

    // Simulate multiple batches with different statuses
    assert_eq!(batches[0].status, BatchStatus::InProgress);
    assert_eq!(batches[1].status, BatchStatus::Completed);

    // Both should be valid
    for batch in &batches {
        let json = serde_json::to_string(batch);
        assert!(json.is_ok());
    }
}
