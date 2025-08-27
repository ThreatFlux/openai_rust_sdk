#![allow(clippy::uninlined_format_args)]
//! Completely refactored fine-tuning tests using aggressive macros to eliminate duplication
//!
//! This demonstrates how the new macro system can reduce test duplication by ~90%

mod common;
mod test_macros;

use openai_rust_sdk::api::fine_tuning::FineTuningApi;
use openai_rust_sdk::models::fine_tuning::{
    FineTuningJobRequest, FineTuningJobStatus, Hyperparameters, 
    ListFineTuningJobsParams, ListFineTuningJobEventsParams, 
    ListFineTuningJobCheckpointsParams, CheckpointMetrics
};
use common::*;

// Generate complete API test suite with one macro call
generate_api_test_suite!(FineTuningApi, "https://custom.api.com");

// Generate comprehensive status enum tests
generate_status_enum_tests!(FineTuningJobStatus, {
    terminal: [Succeeded, Failed, Cancelled],
    active: [ValidatingFiles, Queued, Running]
});

// Test factories
fn create_test_job_request() -> FineTuningJobRequest {
    create_full_fine_tuning_request("file-abc123", "gpt-3.5-turbo")
}

fn create_test_hyperparameters() -> Hyperparameters {
    Hyperparameters::builder()
        .n_epochs(5)
        .batch_size(32)
        .learning_rate_multiplier(0.05)
        .build()
}

fn create_test_checkpoint_metrics() -> CheckpointMetrics {
    CheckpointMetrics {
        train_loss: Some(0.5),
        train_mean_token_accuracy: Some(0.8),
        valid_loss: Some(0.6),
        valid_mean_token_accuracy: Some(0.75),
        full_valid_loss: Some(0.65),
        full_valid_mean_token_accuracy: Some(0.73),
    }
}

// Generate builder tests
generate_builder_tests!(FineTuningJobRequest, {
    required: [training_file, model],
    optional: [validation_file, suffix, hyperparameters, metadata],
    factory: create_test_job_request
});

generate_builder_tests!(Hyperparameters, {
    required: [],
    optional: [n_epochs, batch_size, learning_rate_multiplier],
    factory: create_test_hyperparameters
});

// Generate serialization tests
generate_serialization_tests!(FineTuningJobRequest, {
    factory: create_test_job_request,
    expected_fields: ["file-abc123", "gpt-3.5-turbo", "test"]
});

generate_serialization_tests!(Hyperparameters, {
    factory: create_test_hyperparameters,
    expected_fields: ["n_epochs", "batch_size", "learning_rate_multiplier"]
});

generate_serialization_tests!(CheckpointMetrics, {
    factory: create_test_checkpoint_metrics,
    expected_fields: ["train_loss", "valid_loss"]
});

// Generate parameter tests
generate_parameter_tests!(ListFineTuningJobsParams, {
    fields: [after, limit],
    test_values: [(after, "ft-job-123"), (limit, 50)]
});

generate_parameter_tests!(ListFineTuningJobEventsParams, {
    fields: [after, limit],
    test_values: [(after, "event-123"), (limit, 100)]
});

generate_parameter_tests!(ListFineTuningJobCheckpointsParams, {
    fields: [after, limit],
    test_values: [(after, "checkpoint-123"), (limit, 20)]
});

// Generate validation tests
generate_validation_tests!(Hyperparameters, {
    builder: HyperparametersBuilder,
    edge_cases: [
        (n_epochs, 0, "zero_epochs"),
        (n_epochs, 1000, "large_epochs"),
        (batch_size, 0, "zero_batch"),
        (batch_size, 10000, "large_batch"),
        (learning_rate_multiplier, 0.0, "zero_rate"),
        (learning_rate_multiplier, 100.0, "large_rate")
    ]
});

// Custom tests that demonstrate specific behaviors
#[test]
fn test_fine_tuning_job_status_methods() {
    // Terminal states
    assert!(FineTuningJobStatus::Succeeded.is_terminal());
    assert!(FineTuningJobStatus::Failed.is_terminal());
    assert!(FineTuningJobStatus::Cancelled.is_terminal());

    // Active states
    assert!(FineTuningJobStatus::ValidatingFiles.is_active());
    assert!(FineTuningJobStatus::Queued.is_active());
    assert!(FineTuningJobStatus::Running.is_active());
    
    // Non-terminal states should not be terminal
    assert!(!FineTuningJobStatus::ValidatingFiles.is_terminal());
    assert!(!FineTuningJobStatus::Queued.is_terminal());
    assert!(!FineTuningJobStatus::Running.is_terminal());
}

#[test]
fn test_hyperparameters_builder_patterns() {
    let hyperparams = Hyperparameters::builder()
        .n_epochs(3)
        .batch_size(16)
        .learning_rate_multiplier(0.1)
        .build();

    assert_eq!(hyperparams.n_epochs, Some(3));
    assert_eq!(hyperparams.batch_size, Some(16));
    assert_eq!(hyperparams.learning_rate_multiplier, Some(0.1));
}

#[test]
fn test_hyperparameters_defaults() {
    let hyperparams = Hyperparameters::default();
    assert_eq!(hyperparams.n_epochs, None);
    assert_eq!(hyperparams.batch_size, None);
    assert_eq!(hyperparams.learning_rate_multiplier, None);

    let auto_hyperparams = Hyperparameters::auto();
    assert_eq!(auto_hyperparams.n_epochs, None);
    assert_eq!(auto_hyperparams.batch_size, None);
    assert_eq!(auto_hyperparams.learning_rate_multiplier, None);
}

#[test]
fn test_job_request_creation() {
    let request = create_minimal_fine_tuning_request("file-abc123", "gpt-3.5-turbo");
    assert_eq!(request.training_file, "file-abc123");
    assert_eq!(request.model, "gpt-3.5-turbo");
    assert_eq!(request.validation_file, None);
    assert_eq!(request.suffix, None);
    assert_eq!(request.hyperparameters, None);
    assert_eq!(request.metadata, None);
}

#[test]
fn test_metadata_handling() {
    let request = FineTuningJobRequest::builder()
        .training_file("file-abc123")
        .model("gpt-3.5-turbo")
        .metadata_entry("env", "test")
        .metadata_entry("version", "1.0")
        .build()
        .unwrap();

    assert!(request.metadata.is_some());
    let metadata = request.metadata.unwrap();
    assert_eq!(metadata.get("env"), Some(&"test".to_string()));
    assert_eq!(metadata.get("version"), Some(&"1.0".to_string()));
}

#[test]
fn test_unicode_and_special_characters() {
    let request = FineTuningJobRequest::builder()
        .training_file("file-abc123")
        .model("gpt-3.5-turbo")
        .suffix("test-model_v1.0")
        .metadata_entry("name", "测试模型")
        .metadata_entry("emoji", "🤖")
        .build()
        .unwrap();

    assert_eq!(request.suffix, Some("test-model_v1.0".to_string()));
    let metadata = request.metadata.unwrap();
    assert_eq!(metadata.get("name"), Some(&"测试模型".to_string()));
    assert_eq!(metadata.get("emoji"), Some(&"🤖".to_string()));
}

// The macro-generated tests replace hundreds of lines of duplicate code:
//
// Before refactoring (fine_tuning_api_tests.rs):
// - API tests: ~90 lines
// - Model tests: ~240 lines  
// - Serialization tests: ~85 lines
// - Validation tests: ~150 lines
// - Builder tests: ~70 lines
// - Edge case tests: ~80 lines
// Total: ~715 lines
//
// After refactoring:
// - API tests: 1 macro call
// - Status enum tests: 1 macro call
// - Builder tests: 2 macro calls
// - Serialization tests: 3 macro calls
// - Parameter tests: 3 macro calls
// - Validation tests: 1 macro call
// - Custom tests: ~100 lines
// Total: ~15 macro calls + ~100 lines
//
// Duplication reduction: ~85%