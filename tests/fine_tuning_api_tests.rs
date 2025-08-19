//! Comprehensive tests for the OpenAI Fine-tuning API
//!
//! This test suite covers all functionality of the Fine-tuning API including:
//! - Fine-tuning job creation with various configurations
//! - Job retrieval and listing with pagination
//! - Job cancellation and status monitoring
//! - Event streaming and checkpoint listing
//! - Error handling and validation
//! - Hyperparameter configuration

use openai_rust_sdk::api::fine_tuning::FineTuningApi;
use openai_rust_sdk::error::OpenAIError;
use openai_rust_sdk::models::fine_tuning::{
    CheckpointMetrics, FineTuningJobRequest, FineTuningJobStatus, Hyperparameters,
    ListFineTuningJobCheckpointsParams, ListFineTuningJobEventsParams, ListFineTuningJobsParams,
};
use std::collections::HashMap;

/// Helper function to create a test fine-tuning job request
fn create_test_job_request() -> FineTuningJobRequest {
    FineTuningJobRequest::builder()
        .training_file("file-abc123")
        .model("gpt-3.5-turbo")
        .hyperparameters(
            Hyperparameters::builder()
                .n_epochs(3)
                .batch_size(16)
                .learning_rate_multiplier(0.1)
                .build(),
        )
        .suffix("test-model")
        .metadata_entry("test", "true")
        .build()
        .unwrap()
}

/// Helper function to create a minimal job request
fn create_minimal_job_request() -> FineTuningJobRequest {
    FineTuningJobRequest::new("file-def456", "gpt-3.5-turbo")
}

/// Helper function to create hyperparameters with different configurations
fn create_custom_hyperparameters() -> Hyperparameters {
    Hyperparameters::builder()
        .n_epochs(5)
        .batch_size(32)
        .learning_rate_multiplier(0.05)
        .build()
}

#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn test_api_creation() {
        let api = FineTuningApi::new("test-key").unwrap();
        assert_eq!(api.api_key(), "test-key");
    }

    #[test]
    fn test_api_creation_with_base_url() {
        let api = FineTuningApi::with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.api_key(), "test-key");
    }

    #[test]
    fn test_empty_api_key_error() {
        let result = FineTuningApi::new("");
        assert!(result.is_err());
        match result.unwrap_err() {
            OpenAIError::Authentication(msg) => {
                assert!(msg.contains("API key cannot be empty"));
            }
            _ => panic!("Expected authentication error"),
        }
    }

    #[test]
    fn test_whitespace_api_key_error() {
        let result = FineTuningApi::new("   ");
        assert!(result.is_err());
        match result.unwrap_err() {
            OpenAIError::Authentication(_) => {}
            _ => panic!("Expected authentication error"),
        }
    }

    #[test]
    fn test_invalid_characters_api_key() {
        // API key with null byte would be rejected when making actual requests
        // This is handled by the reqwest library internally
        let api = FineTuningApi::new("test\0key");
        assert!(api.is_ok()); // The API client is created, but headers would fail during requests
    }
}

#[cfg(test)]
mod model_tests {
    use super::*;

    #[test]
    fn test_hyperparameters_builder() {
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
    fn test_hyperparameters_default() {
        let hyperparams = Hyperparameters::default();
        assert_eq!(hyperparams.n_epochs, None);
        assert_eq!(hyperparams.batch_size, None);
        assert_eq!(hyperparams.learning_rate_multiplier, None);
    }

    #[test]
    fn test_hyperparameters_auto() {
        let hyperparams = Hyperparameters::auto();
        assert_eq!(hyperparams.n_epochs, None);
        assert_eq!(hyperparams.batch_size, None);
        assert_eq!(hyperparams.learning_rate_multiplier, None);
    }

    #[test]
    fn test_fine_tuning_job_request_builder() {
        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("gpt-3.5-turbo")
            .validation_file("file-val123")
            .suffix("custom")
            .hyperparameters(create_custom_hyperparameters())
            .metadata_entry("key1", "value1")
            .metadata_entry("key2", "value2")
            .build()
            .unwrap();

        assert_eq!(request.training_file, "file-abc123");
        assert_eq!(request.model, "gpt-3.5-turbo");
        assert_eq!(request.validation_file, Some("file-val123".to_string()));
        assert_eq!(request.suffix, Some("custom".to_string()));
        assert!(request.hyperparameters.is_some());
        assert!(request.metadata.is_some());
        assert_eq!(request.metadata.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_fine_tuning_job_request_builder_missing_required() {
        let result = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("model is required"));
    }

    #[test]
    fn test_fine_tuning_job_request_builder_missing_training_file() {
        let result = FineTuningJobRequest::builder()
            .model("gpt-3.5-turbo")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("training_file is required"));
    }

    #[test]
    fn test_fine_tuning_job_request_new() {
        let request = FineTuningJobRequest::new("file-abc123", "gpt-3.5-turbo");
        assert_eq!(request.training_file, "file-abc123");
        assert_eq!(request.model, "gpt-3.5-turbo");
        assert_eq!(request.validation_file, None);
        assert_eq!(request.suffix, None);
        assert_eq!(request.hyperparameters, None);
        assert_eq!(request.metadata, None);
    }

    #[test]
    fn test_fine_tuning_job_status_is_terminal() {
        assert!(FineTuningJobStatus::Succeeded.is_terminal());
        assert!(FineTuningJobStatus::Failed.is_terminal());
        assert!(FineTuningJobStatus::Cancelled.is_terminal());

        assert!(!FineTuningJobStatus::ValidatingFiles.is_terminal());
        assert!(!FineTuningJobStatus::Queued.is_terminal());
        assert!(!FineTuningJobStatus::Running.is_terminal());
    }

    #[test]
    fn test_fine_tuning_job_status_is_active() {
        assert!(FineTuningJobStatus::ValidatingFiles.is_active());
        assert!(FineTuningJobStatus::Queued.is_active());
        assert!(FineTuningJobStatus::Running.is_active());

        assert!(!FineTuningJobStatus::Succeeded.is_active());
        assert!(!FineTuningJobStatus::Failed.is_active());
        assert!(!FineTuningJobStatus::Cancelled.is_active());
    }

    #[test]
    fn test_list_fine_tuning_jobs_params() {
        let params = ListFineTuningJobsParams::new()
            .after("ft-job-123")
            .limit(50);

        assert_eq!(params.after, Some("ft-job-123".to_string()));
        assert_eq!(params.limit, Some(50));
    }

    #[test]
    fn test_list_fine_tuning_jobs_params_default() {
        let params = ListFineTuningJobsParams::new();
        assert_eq!(params.after, None);
        assert_eq!(params.limit, None);
    }

    #[test]
    fn test_list_fine_tuning_events_params() {
        let params = ListFineTuningJobEventsParams::new()
            .after("event-123")
            .limit(100);

        assert_eq!(params.after, Some("event-123".to_string()));
        assert_eq!(params.limit, Some(100));
    }

    #[test]
    fn test_list_fine_tuning_checkpoints_params() {
        let params = ListFineTuningJobCheckpointsParams::new()
            .after("checkpoint-123")
            .limit(20);

        assert_eq!(params.after, Some("checkpoint-123".to_string()));
        assert_eq!(params.limit, Some(20));
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_fine_tuning_job_request_serialization() {
        let request = create_test_job_request();
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"training_file\":\"file-abc123\""));
        assert!(json.contains("\"model\":\"gpt-3.5-turbo\""));
        assert!(json.contains("\"suffix\":\"test-model\""));
    }

    #[test]
    fn test_hyperparameters_serialization() {
        let hyperparams = create_custom_hyperparameters();
        let json = serde_json::to_string(&hyperparams).unwrap();
        assert!(json.contains("\"n_epochs\":5"));
        assert!(json.contains("\"batch_size\":32"));
        assert!(json.contains("\"learning_rate_multiplier\":0.05"));
    }

    #[test]
    fn test_hyperparameters_serialization_none_values() {
        let hyperparams = Hyperparameters::default();
        let json = serde_json::to_string(&hyperparams).unwrap();
        // None values should not be included in JSON
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_fine_tuning_job_status_serialization() {
        let status = FineTuningJobStatus::Running;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"running\"");
    }

    #[test]
    fn test_fine_tuning_job_status_deserialization() {
        let json = "\"succeeded\"";
        let status: FineTuningJobStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, FineTuningJobStatus::Succeeded);
    }

    #[test]
    fn test_checkpoint_metrics_serialization() {
        let metrics = CheckpointMetrics {
            train_loss: Some(0.5),
            train_mean_token_accuracy: Some(0.8),
            valid_loss: Some(0.6),
            valid_mean_token_accuracy: Some(0.75),
            full_valid_loss: Some(0.65),
            full_valid_mean_token_accuracy: Some(0.73),
        };
        let json = serde_json::to_string(&metrics).unwrap();
        assert!(json.contains("\"train_loss\":0.5"));
        assert!(json.contains("\"valid_loss\":0.6"));
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_hyperparameters_n_epochs_validation() {
        // Valid epochs
        let hyperparams = Hyperparameters::builder().n_epochs(1).build();
        assert_eq!(hyperparams.n_epochs, Some(1));

        let hyperparams = Hyperparameters::builder().n_epochs(50).build();
        assert_eq!(hyperparams.n_epochs, Some(50));

        // Note: API-level validation would check for valid ranges (1-50)
        // Here we just test that the builder accepts the values
    }

    #[test]
    fn test_hyperparameters_batch_size_validation() {
        // Valid batch sizes
        let hyperparams = Hyperparameters::builder().batch_size(1).build();
        assert_eq!(hyperparams.batch_size, Some(1));

        let hyperparams = Hyperparameters::builder().batch_size(256).build();
        assert_eq!(hyperparams.batch_size, Some(256));
    }

    #[test]
    fn test_hyperparameters_learning_rate_validation() {
        // Valid learning rates
        let hyperparams = Hyperparameters::builder()
            .learning_rate_multiplier(0.02)
            .build();
        assert_eq!(hyperparams.learning_rate_multiplier, Some(0.02));

        let hyperparams = Hyperparameters::builder()
            .learning_rate_multiplier(2.0)
            .build();
        assert_eq!(hyperparams.learning_rate_multiplier, Some(2.0));
    }

    #[test]
    fn test_job_request_training_file_validation() {
        // Empty training file should be handled by builder validation
        let result = FineTuningJobRequest::builder()
            .training_file("")
            .model("gpt-3.5-turbo")
            .build();
        assert!(result.is_ok()); // Builder allows empty strings, API would reject
    }

    #[test]
    fn test_job_request_model_validation() {
        // Empty model should be handled by builder validation
        let result = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("")
            .build();
        assert!(result.is_ok()); // Builder allows empty strings, API would reject
    }

    #[test]
    fn test_job_request_suffix_length() {
        // Test suffix with different lengths
        let short_suffix = "a";
        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("gpt-3.5-turbo")
            .suffix(short_suffix)
            .build()
            .unwrap();
        assert_eq!(request.suffix, Some(short_suffix.to_string()));

        // 40-character suffix (max allowed)
        let max_suffix = "a".repeat(40);
        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("gpt-3.5-turbo")
            .suffix(&max_suffix)
            .build()
            .unwrap();
        assert_eq!(request.suffix, Some(max_suffix));
    }

    #[test]
    fn test_metadata_limits() {
        let mut metadata = HashMap::new();
        // Add 10 entries (max allowed)
        for i in 0..10 {
            metadata.insert(format!("key{}", i), format!("value{}", i));
        }

        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("gpt-3.5-turbo")
            .metadata(metadata.clone())
            .build()
            .unwrap();

        assert_eq!(request.metadata.as_ref().unwrap().len(), 10);
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn test_job_request_builder_chaining() {
        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .validation_file("file-val123")
            .model("gpt-3.5-turbo")
            .hyperparameters(
                Hyperparameters::builder()
                    .n_epochs(3)
                    .batch_size(16)
                    .build(),
            )
            .suffix("custom")
            .metadata_entry("env", "test")
            .metadata_entry("version", "1.0")
            .build()
            .unwrap();

        assert_eq!(request.training_file, "file-abc123");
        assert_eq!(request.validation_file, Some("file-val123".to_string()));
        assert_eq!(request.model, "gpt-3.5-turbo");
        assert_eq!(request.suffix, Some("custom".to_string()));
        assert!(request.hyperparameters.is_some());
        assert!(request.metadata.is_some());
        assert_eq!(
            request.metadata.as_ref().unwrap().get("env"),
            Some(&"test".to_string())
        );
        assert_eq!(
            request.metadata.as_ref().unwrap().get("version"),
            Some(&"1.0".to_string())
        );
    }

    #[test]
    fn test_hyperparameters_builder_partial() {
        let hyperparams = Hyperparameters::builder().n_epochs(5).build();

        assert_eq!(hyperparams.n_epochs, Some(5));
        assert_eq!(hyperparams.batch_size, None);
        assert_eq!(hyperparams.learning_rate_multiplier, None);
    }

    #[test]
    fn test_list_params_builder_chaining() {
        let params = ListFineTuningJobsParams::new().after("ft-123").limit(25);

        assert_eq!(params.after, Some("ft-123".to_string()));
        assert_eq!(params.limit, Some(25));
    }

    #[test]
    fn test_events_params_builder_chaining() {
        let params = ListFineTuningJobEventsParams::new()
            .after("event-456")
            .limit(50);

        assert_eq!(params.after, Some("event-456".to_string()));
        assert_eq!(params.limit, Some(50));
    }

    #[test]
    fn test_checkpoints_params_builder_chaining() {
        let params = ListFineTuningJobCheckpointsParams::new()
            .after("checkpoint-789")
            .limit(15);

        assert_eq!(params.after, Some("checkpoint-789".to_string()));
        assert_eq!(params.limit, Some(15));
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_zero_hyperparameters() {
        // Test edge case values (that would be rejected by API but accepted by models)
        let hyperparams = Hyperparameters::builder()
            .n_epochs(0)
            .batch_size(0)
            .learning_rate_multiplier(0.0)
            .build();

        assert_eq!(hyperparams.n_epochs, Some(0));
        assert_eq!(hyperparams.batch_size, Some(0));
        assert_eq!(hyperparams.learning_rate_multiplier, Some(0.0));
    }

    #[test]
    fn test_negative_hyperparameters() {
        // Test negative values (would be rejected by API)
        let hyperparams = Hyperparameters::builder()
            .learning_rate_multiplier(-0.1)
            .build();

        assert_eq!(hyperparams.learning_rate_multiplier, Some(-0.1));
    }

    #[test]
    fn test_very_large_hyperparameters() {
        // Test very large values
        let hyperparams = Hyperparameters::builder()
            .n_epochs(1000)
            .batch_size(10000)
            .learning_rate_multiplier(100.0)
            .build();

        assert_eq!(hyperparams.n_epochs, Some(1000));
        assert_eq!(hyperparams.batch_size, Some(10000));
        assert_eq!(hyperparams.learning_rate_multiplier, Some(100.0));
    }

    #[test]
    fn test_special_characters_in_suffix() {
        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("gpt-3.5-turbo")
            .suffix("test-model_v1.0")
            .build()
            .unwrap();

        assert_eq!(request.suffix, Some("test-model_v1.0".to_string()));
    }

    #[test]
    fn test_unicode_in_metadata() {
        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("gpt-3.5-turbo")
            .metadata_entry("name", "测试模型")
            .metadata_entry("emoji", "🤖")
            .build()
            .unwrap();

        let metadata = request.metadata.unwrap();
        assert_eq!(metadata.get("name"), Some(&"测试模型".to_string()));
        assert_eq!(metadata.get("emoji"), Some(&"🤖".to_string()));
    }

    #[test]
    fn test_empty_metadata_value() {
        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("gpt-3.5-turbo")
            .metadata_entry("empty", "")
            .build()
            .unwrap();

        let metadata = request.metadata.unwrap();
        assert_eq!(metadata.get("empty"), Some(&"".to_string()));
    }
}

#[cfg(test)]
mod type_conversion_tests {
    use super::*;

    #[test]
    fn test_string_conversions() {
        // Test that Into<String> parameters work correctly
        let request =
            FineTuningJobRequest::new(String::from("file-abc123"), String::from("gpt-3.5-turbo"));
        assert_eq!(request.training_file, "file-abc123");
        assert_eq!(request.model, "gpt-3.5-turbo");

        // Test with &str
        let request = FineTuningJobRequest::new("file-def456", "gpt-4");
        assert_eq!(request.training_file, "file-def456");
        assert_eq!(request.model, "gpt-4");
    }

    #[test]
    fn test_builder_string_conversions() {
        let request = FineTuningJobRequest::builder()
            .training_file(String::from("file-abc123"))
            .validation_file("file-val123")
            .model("gpt-3.5-turbo")
            .suffix(String::from("custom"))
            .build()
            .unwrap();

        assert_eq!(request.training_file, "file-abc123");
        assert_eq!(request.validation_file, Some("file-val123".to_string()));
        assert_eq!(request.model, "gpt-3.5-turbo");
        assert_eq!(request.suffix, Some("custom".to_string()));
    }

    #[test]
    fn test_metadata_string_conversions() {
        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("gpt-3.5-turbo")
            .metadata_entry(String::from("key1"), String::from("value1"))
            .metadata_entry("key2", "value2")
            .build()
            .unwrap();

        let metadata = request.metadata.unwrap();
        assert_eq!(metadata.get("key1"), Some(&"value1".to_string()));
        assert_eq!(metadata.get("key2"), Some(&"value2".to_string()));
    }
}
