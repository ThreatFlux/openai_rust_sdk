//! # OpenAI Fine-tuning API Models
//!
//! This module provides data structures for OpenAI's Fine-tuning API, which allows you to
//! create custom models tailored to your specific use case by training on your own data.
//!
//! ## Overview
//!
//! The Fine-tuning API supports:
//! - **Job Management**: Create, monitor, and cancel fine-tuning jobs
//! - **Hyperparameter Tuning**: Customize training parameters for optimal results
//! - **Event Streaming**: Monitor training progress through real-time events
//! - **Checkpoint Management**: Access intermediate training checkpoints
//! - **Model Deployment**: Use fine-tuned models for inference
//!
//! ## Fine-tuning Process
//!
//! 1. Upload training data as a JSONL file
//! 2. Create a fine-tuning job with desired hyperparameters
//! 3. Monitor job progress through events and status updates
//! 4. Use the resulting fine-tuned model for chat completions
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::fine_tuning::{FineTuningJobRequest, Hyperparameters};
//!
//! // Create a fine-tuning job request
//! let job_request = FineTuningJobRequest::builder()
//!     .training_file("file-abc123")
//!     .model("gpt-3.5-turbo")
//!     .hyperparameters(Hyperparameters::builder()
//!         .n_epochs(3)
//!         .batch_size(16)
//!         .learning_rate_multiplier(0.1)
//!         .build())
//!     .suffix("custom-model")
//!     .build();
//! ```

use crate::{impl_fine_tuning_params, impl_status_enum, impl_status_methods, De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// Default object type for fine-tuning jobs
fn default_object_type() -> String {
    "fine_tuning.job".to_string()
}

/// Default status for fine-tuning jobs
fn default_status() -> FineTuningJobStatus {
    FineTuningJobStatus::ValidatingFiles
}

/// Status of a fine-tuning job
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum FineTuningJobStatus {
    /// Job is validating the uploaded files
    ValidatingFiles,
    /// Job is queued for processing
    Queued,
    /// Job is currently running
    Running,
    /// Job completed successfully
    Succeeded,
    /// Job failed with an error
    Failed,
    /// Job was cancelled by the user
    Cancelled,
}

// Generate status enum methods using macro
impl_status_enum!(FineTuningJobStatus, {
    terminal: [Succeeded, Failed, Cancelled],
    active: [ValidatingFiles, Queued, Running],
    failed: [Failed],
    completed: [Succeeded],
});

impl FineTuningJobStatus {
    /// Check if the job is in a terminal state (completed, failed, or cancelled)
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed | Self::Cancelled)
    }

    /// Check if the job is currently active (validating, queued, or running)
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(self, Self::ValidatingFiles | Self::Queued | Self::Running)
    }
}

/// Hyperparameters for fine-tuning
#[derive(Debug, Clone, PartialEq, Ser, De, Default)]
pub struct Hyperparameters {
    /// Number of epochs to train the model for (1-50, default: auto)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_epochs: Option<u32>,
    /// Batch size for training (1-256, default: auto)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,
    /// Learning rate multiplier (0.02-2.0, default: auto)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<f64>,
}

impl Hyperparameters {
    /// Create a new hyperparameters builder
    #[must_use]
    pub fn builder() -> HyperparametersBuilder {
        HyperparametersBuilder::default()
    }

    /// Create hyperparameters with auto settings
    #[must_use]
    pub fn auto() -> Self {
        Self::default()
    }
}

/// Builder for hyperparameters
#[derive(Debug, Default)]
pub struct HyperparametersBuilder {
    /// Number of training epochs
    n_epochs: Option<u32>,
    /// Batch size for training
    batch_size: Option<u32>,
    /// Learning rate multiplier
    learning_rate_multiplier: Option<f64>,
}

impl HyperparametersBuilder {
    /// Set the number of training epochs
    #[must_use]
    pub fn n_epochs(mut self, n_epochs: u32) -> Self {
        self.n_epochs = Some(n_epochs);
        self
    }

    /// Set the batch size for training
    #[must_use]
    pub fn batch_size(mut self, batch_size: u32) -> Self {
        self.batch_size = Some(batch_size);
        self
    }

    /// Set the learning rate multiplier
    #[must_use]
    pub fn learning_rate_multiplier(mut self, learning_rate_multiplier: f64) -> Self {
        self.learning_rate_multiplier = Some(learning_rate_multiplier);
        self
    }

    /// Build the hyperparameters
    #[must_use]
    pub fn build(self) -> Hyperparameters {
        Hyperparameters {
            n_epochs: self.n_epochs,
            batch_size: self.batch_size,
            learning_rate_multiplier: self.learning_rate_multiplier,
        }
    }
}

/// Error details for a failed fine-tuning job
#[derive(Debug, Clone, Ser, De)]
pub struct FineTuningError {
    /// Error code for programmatic handling
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Additional error parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

/// A fine-tuning job represents the entity that tracks the progress of a fine-tuning operation
#[derive(Debug, Clone, Ser, De)]
pub struct FineTuningJob {
    /// The identifier of the fine-tuning job
    pub id: String,
    /// The object type, which is always "`fine_tuning.job`"
    #[serde(default = "default_object_type")]
    pub object: String,
    /// The Unix timestamp (in seconds) when the fine-tuning job was created
    pub created_at: i64,
    /// The Unix timestamp (in seconds) when the fine-tuning job finished
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,
    /// The name of the fine-tuned model that is being created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuned_model: Option<String>,
    /// The organization that owns the fine-tuning job
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// The current status of the fine-tuning job
    #[serde(default = "default_status")]
    pub status: FineTuningJobStatus,
    /// The hyperparameters used for the fine-tuning job
    pub hyperparameters: Hyperparameters,
    /// The base model that is being fine-tuned
    pub model: String,
    /// The file ID used for training
    pub training_file: String,
    /// The file ID used for validation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
    /// The compiled results files from the fine-tuning job
    pub result_files: Vec<String>,
    /// The total number of billable tokens processed during this fine-tuning job
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trained_tokens: Option<u64>,
    /// For failed jobs, this field contains error information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<FineTuningError>,
    /// Up to 10 sets of key-value pairs that can be attached to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Request to create a fine-tuning job
#[derive(Debug, Clone, Ser, De)]
pub struct FineTuningJobRequest {
    /// The ID of an uploaded file that contains training data
    pub training_file: String,
    /// The ID of an uploaded file that contains validation data (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
    /// The name of the model to fine-tune
    pub model: String,
    /// The hyperparameters used for the fine-tuning job
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparameters: Option<Hyperparameters>,
    /// A string of up to 40 characters that will be added to your fine-tuned model name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Up to 10 sets of key-value pairs that can be attached to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl FineTuningJobRequest {
    /// Create a new fine-tuning job request builder
    #[must_use]
    pub fn builder() -> FineTuningJobRequestBuilder {
        FineTuningJobRequestBuilder::default()
    }

    /// Create a basic fine-tuning job request
    pub fn new(training_file: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            training_file: training_file.into(),
            validation_file: None,
            model: model.into(),
            hyperparameters: None,
            suffix: None,
            metadata: None,
        }
    }
}

/// Builder for fine-tuning job requests
#[derive(Debug, Default)]
pub struct FineTuningJobRequestBuilder {
    /// The training file ID
    training_file: Option<String>,
    /// The validation file ID
    validation_file: Option<String>,
    /// The model to fine-tune
    model: Option<String>,
    /// Hyperparameters for fine-tuning
    hyperparameters: Option<Hyperparameters>,
    /// Custom suffix for the fine-tuned model name
    suffix: Option<String>,
    /// Metadata for the fine-tuning job
    metadata: Option<HashMap<String, String>>,
}

impl FineTuningJobRequestBuilder {
    /// Set the training file ID
    pub fn training_file(mut self, training_file: impl Into<String>) -> Self {
        self.training_file = Some(training_file.into());
        self
    }

    /// Set the validation file ID
    pub fn validation_file(mut self, validation_file: impl Into<String>) -> Self {
        self.validation_file = Some(validation_file.into());
        self
    }

    /// Set the model to fine-tune
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set the hyperparameters for training
    #[must_use]
    pub fn hyperparameters(mut self, hyperparameters: Hyperparameters) -> Self {
        self.hyperparameters = Some(hyperparameters);
        self
    }

    /// Set the model name suffix
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Add metadata key-value pair
    pub fn metadata_entry(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        self.metadata
            .as_mut()
            .unwrap()
            .insert(key.into(), value.into());
        self
    }

    /// Set the metadata map
    #[must_use]
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

// Generate the build method for FineTuningJobRequestBuilder
crate::impl_builder_build! {
    FineTuningJobRequestBuilder => FineTuningJobRequest {
        required: [training_file: "training_file is required", model: "model is required"],
        optional: [validation_file, hyperparameters, suffix, metadata]
    }
}

/// Fine-tuning job event for tracking training progress
#[derive(Debug, Clone, Ser, De)]
pub struct FineTuningJobEvent {
    /// The identifier of the event
    pub id: String,
    /// The object type, which is always "`fine_tuning.job.event`"
    #[serde(default = "default_event_object_type")]
    pub object: String,
    /// The Unix timestamp (in seconds) when the event was created
    pub created_at: i64,
    /// The level of the event (info, warn, error)
    pub level: String,
    /// The message describing the event
    pub message: String,
    /// Additional data associated with the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Default object type for fine-tuning job events
fn default_event_object_type() -> String {
    "fine_tuning.job.event".to_string()
}

/// Training checkpoint from a fine-tuning job
#[derive(Debug, Clone, Ser, De)]
pub struct FineTuningJobCheckpoint {
    /// The identifier of the checkpoint
    pub id: String,
    /// The object type, which is always "`fine_tuning.job.checkpoint`"
    #[serde(default = "default_checkpoint_object_type")]
    pub object: String,
    /// The Unix timestamp (in seconds) when the checkpoint was created
    pub created_at: i64,
    /// The name of the fine-tuned model checkpoint
    pub fine_tuned_model_checkpoint: String,
    /// The step number that the checkpoint was created at
    pub step_number: u32,
    /// Metrics recorded at this checkpoint
    pub metrics: CheckpointMetrics,
    /// The ID of the fine-tuning job that this checkpoint belongs to
    pub fine_tuning_job_id: String,
}

/// Default object type for fine-tuning job checkpoints
fn default_checkpoint_object_type() -> String {
    "fine_tuning.job.checkpoint".to_string()
}

/// Metrics recorded at a training checkpoint
#[derive(Debug, Clone, Ser, De)]
pub struct CheckpointMetrics {
    /// Training loss at this checkpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub train_loss: Option<f64>,
    /// Training accuracy at this checkpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub train_mean_token_accuracy: Option<f64>,
    /// Validation loss at this checkpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_loss: Option<f64>,
    /// Validation accuracy at this checkpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_mean_token_accuracy: Option<f64>,
    /// Full validation loss at this checkpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_valid_loss: Option<f64>,
    /// Full validation accuracy at this checkpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_valid_mean_token_accuracy: Option<f64>,
}

/// Response for listing fine-tuning jobs
#[derive(Debug, Clone, Ser, De)]
pub struct ListFineTuningJobsResponse {
    /// The object type, which is always "list"
    #[serde(default = "default_list_object_type")]
    pub object: String,
    /// List of fine-tuning jobs
    pub data: Vec<FineTuningJob>,
    /// Whether there are more items available
    pub has_more: bool,
}

/// Response for listing fine-tuning job events
#[derive(Debug, Clone, Ser, De)]
pub struct ListFineTuningJobEventsResponse {
    /// The object type, which is always "list"
    #[serde(default = "default_list_object_type")]
    pub object: String,
    /// List of fine-tuning job events
    pub data: Vec<FineTuningJobEvent>,
    /// Whether there are more items available
    pub has_more: bool,
}

/// Response for listing fine-tuning job checkpoints
#[derive(Debug, Clone, Ser, De)]
pub struct ListFineTuningJobCheckpointsResponse {
    /// The object type, which is always "list"
    #[serde(default = "default_list_object_type")]
    pub object: String,
    /// List of fine-tuning job checkpoints
    pub data: Vec<FineTuningJobCheckpoint>,
    /// Whether there are more items available
    pub has_more: bool,
    /// First ID in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// Last ID in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
}

/// Default object type for list responses
fn default_list_object_type() -> String {
    "list".to_string()
}

// Generate fine-tuning job list params using macro
impl_fine_tuning_params!(ListFineTuningJobsParams, "fine-tuning jobs");

// Generate fine-tuning job events list params using macro
impl_fine_tuning_params!(ListFineTuningJobEventsParams, "fine-tuning job events");

// Generate fine-tuning job checkpoints list params using macro
impl_fine_tuning_params!(
    ListFineTuningJobCheckpointsParams,
    "fine-tuning job checkpoints"
);

/// Response when cancelling a fine-tuning job
#[derive(Debug, Clone, Ser, De)]
pub struct CancelFineTuningJobResponse {
    /// The identifier of the cancelled job
    pub id: String,
    /// The object type, which is always "`fine_tuning.job`"
    #[serde(default = "default_object_type")]
    pub object: String,
    /// The current status of the job (should be "cancelled")
    pub status: FineTuningJobStatus,
}

#[cfg(test)]
mod tests {
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
    fn test_fine_tuning_job_request_builder() {
        let request = FineTuningJobRequest::builder()
            .training_file("file-abc123")
            .model("gpt-3.5-turbo")
            .suffix("custom")
            .build()
            .unwrap();

        assert_eq!(request.training_file, "file-abc123");
        assert_eq!(request.model, "gpt-3.5-turbo");
        assert_eq!(request.suffix, Some("custom".to_string()));
    }

    #[test]
    fn test_fine_tuning_job_status() {
        assert!(FineTuningJobStatus::Succeeded.is_terminal());
        assert!(FineTuningJobStatus::Failed.is_terminal());
        assert!(FineTuningJobStatus::Cancelled.is_terminal());

        assert!(!FineTuningJobStatus::Running.is_terminal());
        assert!(FineTuningJobStatus::Running.is_active());
    }

    #[test]
    fn test_list_params() {
        let params = ListFineTuningJobsParams::new()
            .after("ft-job-123")
            .limit(50);

        assert_eq!(params.after, Some("ft-job-123".to_string()));
        assert_eq!(params.limit, Some(50));
    }
}
