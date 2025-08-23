//! # OpenAI Fine-tuning API Client
//!
//! This module provides a complete implementation of OpenAI's Fine-tuning API, which allows you to
//! create custom models tailored to your specific use case by training on your own data.
//!
//! ## Features
//!
//! - **Job Management**: Create, retrieve, list, and cancel fine-tuning jobs
//! - **Event Monitoring**: Stream and list training events for progress tracking
//! - **Checkpoint Access**: List and retrieve training checkpoints
//! - **Hyperparameter Tuning**: Customize training parameters for optimal results
//! - **Error Handling**: Comprehensive error handling with detailed messages
//!
//! ## Fine-tuning Workflow
//!
//! 1. **Upload Training Data**: Use the Files API to upload JSONL training data
//! 2. **Create Fine-tuning Job**: Submit a job with your training file and hyperparameters
//! 3. **Monitor Progress**: Track job status and training events
//! 4. **Use Fine-tuned Model**: Use the resulting model for chat completions
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::fine_tuning::FineTuningApi;
//! use openai_rust_sdk::models::fine_tuning::{FineTuningJobRequest, Hyperparameters};
//!
//! # tokio_test::block_on(async {
//! let api = FineTuningApi::new("your-api-key")?;
//!
//! // Create a fine-tuning job
//! let job_request = FineTuningJobRequest::builder()
//!     .training_file("file-abc123")
//!     .model("gpt-3.5-turbo")
//!     .hyperparameters(Hyperparameters::builder()
//!         .n_epochs(3)
//!         .batch_size(16)
//!         .learning_rate_multiplier(0.1)
//!         .build())
//!     .suffix("my-custom-model")
//!     .build()?;
//!
//! let job = api.create_fine_tuning_job(job_request).await?;
//! println!("Created fine-tuning job: {}", job.id);
//!
//! // Monitor job progress
//! while !job.status.is_terminal() {
//!     let current_job = api.retrieve_fine_tuning_job(&job.id).await?;
//!     println!("Job status: {:?}", current_job.status);
//!     
//!     // Get recent events
//!     let events = api.list_fine_tuning_events(&job.id, None).await?;
//!     for event in &events.data {
//!         println!("{}: {}", event.level, event.message);
//!     }
//!     
//!     tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
//! }
//!
//! // Use the fine-tuned model
//! if let Some(model_name) = job.fine_tuned_model {
//!     println!("Fine-tuned model ready: {model_name}");
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```

use crate::api::base::HttpClient;
use crate::error::Result;
use crate::models::fine_tuning::{
    FineTuningJob, FineTuningJobEvent, FineTuningJobRequest, ListFineTuningJobCheckpointsParams,
    ListFineTuningJobCheckpointsResponse, ListFineTuningJobEventsParams,
    ListFineTuningJobEventsResponse, ListFineTuningJobsParams, ListFineTuningJobsResponse,
};
use std::time::Duration;
use tokio::time;

/// Type alias for event callback function
type EventCallback = Box<dyn Fn(&FineTuningJobEvent) + Send + Sync>;

/// `OpenAI` Fine-tuning API client for managing fine-tuning jobs
#[derive(Debug, Clone)]
pub struct FineTuningApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl FineTuningApi {
    /// Creates a new Fine-tuning API client
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    ///
    /// let api = FineTuningApi::new("your-api-key")?;
    /// # Ok::<(), openai_rust_sdk::error::OpenAIError>(())
    /// ```
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
        })
    }

    /// Creates a new Fine-tuning API client with custom base URL
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    /// * `base_url` - Custom base URL for the `OpenAI` API
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    ///
    /// let api = FineTuningApi::with_base_url("your-api-key", "https://api.openai.com")?;
    /// # Ok::<(), openai_rust_sdk::error::OpenAIError>(())
    /// ```
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Get the API key (for testing purposes)
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.http_client.api_key()
    }

    /// Creates a job that fine-tunes a specified model from a given dataset
    ///
    /// # Arguments
    ///
    /// * `request` - The fine-tuning job request
    ///
    /// # Returns
    ///
    /// A `FineTuningJob` object representing the created job
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    /// # use openai_rust_sdk::models::fine_tuning::{FineTuningJobRequest, Hyperparameters};
    /// # tokio_test::block_on(async {
    /// let api = FineTuningApi::new("your-api-key")?;
    ///
    /// let job_request = FineTuningJobRequest::builder()
    ///     .training_file("file-abc123")
    ///     .model("gpt-3.5-turbo")
    ///     .hyperparameters(Hyperparameters::builder()
    ///         .n_epochs(3)
    ///         .build())
    ///     .build()?;
    ///
    /// let job = api.create_fine_tuning_job(job_request).await?;
    /// println!("Created job: {}", job.id);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_fine_tuning_job(
        &self,
        request: FineTuningJobRequest,
    ) -> Result<FineTuningJob> {
        self.http_client
            .post("/v1/fine_tuning/jobs", &request)
            .await
    }

    /// List your organization's fine-tuning jobs
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for pagination
    ///
    /// # Returns
    ///
    /// A list of fine-tuning jobs with pagination information
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    /// # use openai_rust_sdk::models::fine_tuning::ListFineTuningJobsParams;
    /// # tokio_test::block_on(async {
    /// let api = FineTuningApi::new("your-api-key")?;
    ///
    /// let params = ListFineTuningJobsParams::new().limit(10);
    /// let jobs = api.list_fine_tuning_jobs(Some(params)).await?;
    ///
    /// for job in jobs.data {
    ///     println!("Job {}: {:?}", job.id, job.status);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn list_fine_tuning_jobs(
        &self,
        params: Option<ListFineTuningJobsParams>,
    ) -> Result<ListFineTuningJobsResponse> {
        let query_params = if let Some(params) = params {
            params.to_query_params()
        } else {
            Vec::new()
        };

        self.http_client
            .get_with_query("/v1/fine_tuning/jobs", &query_params)
            .await
    }

    /// Get info about a fine-tuning job
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job to retrieve
    ///
    /// # Returns
    ///
    /// The fine-tuning job object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    /// # tokio_test::block_on(async {
    /// let api = FineTuningApi::new("your-api-key")?;
    ///
    /// let job = api.retrieve_fine_tuning_job("ft-123").await?;
    /// println!("Job status: {:?}", job.status);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn retrieve_fine_tuning_job(
        &self,
        fine_tuning_job_id: impl Into<String>,
    ) -> Result<FineTuningJob> {
        let fine_tuning_job_id = fine_tuning_job_id.into();
        self.http_client
            .get(&format!("/v1/fine_tuning/jobs/{fine_tuning_job_id}"))
            .await
    }

    /// Immediately cancel a fine-tuning job
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job to cancel
    ///
    /// # Returns
    ///
    /// The updated fine-tuning job object with status "cancelled"
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    /// # tokio_test::block_on(async {
    /// let api = FineTuningApi::new("your-api-key")?;
    ///
    /// let cancelled_job = api.cancel_fine_tuning_job("ft-123").await?;
    /// println!("Job cancelled: {:?}", cancelled_job.status);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn cancel_fine_tuning_job(
        &self,
        fine_tuning_job_id: impl Into<String>,
    ) -> Result<FineTuningJob> {
        let fine_tuning_job_id = fine_tuning_job_id.into();
        self.http_client
            .post(
                &format!("/v1/fine_tuning/jobs/{fine_tuning_job_id}/cancel"),
                &(),
            )
            .await
    }

    /// Get status updates for a fine-tuning job
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job to get events for
    /// * `params` - Optional parameters for pagination
    ///
    /// # Returns
    ///
    /// A list of fine-tuning job events with pagination information
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    /// # use openai_rust_sdk::models::fine_tuning::ListFineTuningJobEventsParams;
    /// # tokio_test::block_on(async {
    /// let api = FineTuningApi::new("your-api-key")?;
    ///
    /// let params = ListFineTuningJobEventsParams::new().limit(50);
    /// let events = api.list_fine_tuning_events("ft-123", Some(params)).await?;
    ///
    /// for event in events.data {
    ///     println!("{}: {}", event.level, event.message);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn list_fine_tuning_events(
        &self,
        fine_tuning_job_id: impl Into<String>,
        params: Option<ListFineTuningJobEventsParams>,
    ) -> Result<ListFineTuningJobEventsResponse> {
        let fine_tuning_job_id = fine_tuning_job_id.into();
        let endpoint = format!("/v1/fine_tuning/jobs/{fine_tuning_job_id}/events");

        let query_params = if let Some(params) = params {
            params.to_query_params()
        } else {
            Vec::new()
        };

        self.http_client
            .get_with_query(&endpoint, &query_params)
            .await
    }

    /// Get training checkpoints for a fine-tuning job
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job to get checkpoints for
    /// * `params` - Optional parameters for pagination
    ///
    /// # Returns
    ///
    /// A list of fine-tuning job checkpoints with pagination information
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    /// # use openai_rust_sdk::models::fine_tuning::ListFineTuningJobCheckpointsParams;
    /// # tokio_test::block_on(async {
    /// let api = FineTuningApi::new("your-api-key")?;
    ///
    /// let params = ListFineTuningJobCheckpointsParams::new().limit(10);
    /// let checkpoints = api.list_fine_tuning_checkpoints("ft-123", Some(params)).await?;
    ///
    /// for checkpoint in checkpoints.data {
    ///     println!("Checkpoint {}: step {}", checkpoint.id, checkpoint.step_number);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn list_fine_tuning_checkpoints(
        &self,
        fine_tuning_job_id: impl Into<String>,
        params: Option<ListFineTuningJobCheckpointsParams>,
    ) -> Result<ListFineTuningJobCheckpointsResponse> {
        let fine_tuning_job_id = fine_tuning_job_id.into();
        let endpoint = format!("/v1/fine_tuning/jobs/{fine_tuning_job_id}/checkpoints");

        let query_params = if let Some(params) = params {
            params.to_query_params()
        } else {
            Vec::new()
        };

        self.http_client
            .get_with_query(&endpoint, &query_params)
            .await
    }

    /// Monitor a fine-tuning job until completion or failure
    ///
    /// This is a convenience method that polls the job status and events
    /// until the job reaches a terminal state.
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job to monitor
    /// * `poll_interval` - How often to check the job status (default: 30 seconds)
    /// * `event_callback` - Optional callback function called for each new event
    ///
    /// # Returns
    ///
    /// The final fine-tuning job object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    /// # use std::time::Duration;
    /// # tokio_test::block_on(async {
    /// let api = FineTuningApi::new("your-api-key")?;
    ///
    /// let final_job = api.monitor_fine_tuning_job(
    ///     "ft-123",
    ///     Some(Duration::from_secs(30)),
    ///     Some(Box::new(|event| {
    ///         println!("Event: {}", event.message);
    ///     }))
    /// ).await?;
    ///
    /// println!("Final status: {:?}", final_job.status);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn monitor_fine_tuning_job(
        &self,
        fine_tuning_job_id: impl Into<String>,
        poll_interval: Option<Duration>,
        event_callback: Option<EventCallback>,
    ) -> Result<FineTuningJob> {
        let fine_tuning_job_id = fine_tuning_job_id.into();
        let poll_interval = poll_interval.unwrap_or(Duration::from_secs(30));
        let mut last_event_id: Option<String> = None;

        loop {
            // Check job status
            let job = self.retrieve_fine_tuning_job(&fine_tuning_job_id).await?;

            // If job is in terminal state, return it
            if job.status.is_terminal() {
                return Ok(job);
            }

            // Get new events if callback is provided
            if let Some(callback) = &event_callback {
                let mut params = ListFineTuningJobEventsParams::new().limit(100);
                if let Some(after) = &last_event_id {
                    params = params.after(after.clone());
                }

                let events = self
                    .list_fine_tuning_events(&fine_tuning_job_id, Some(params))
                    .await?;

                // Process events in chronological order
                for event in &events.data {
                    callback(event);
                    last_event_id = Some(event.id.clone());
                }
            }

            // Wait before next poll
            time::sleep(poll_interval).await;
        }
    }

    /// Wait for a fine-tuning job to complete
    ///
    /// This is a simplified version of `monitor_fine_tuning_job` that just waits
    /// for completion without event monitoring.
    ///
    /// # Arguments
    ///
    /// * `fine_tuning_job_id` - The ID of the fine-tuning job to wait for
    /// * `poll_interval` - How often to check the job status (default: 30 seconds)
    ///
    /// # Returns
    ///
    /// The final fine-tuning job object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    /// # use std::time::Duration;
    /// # tokio_test::block_on(async {
    /// let api = FineTuningApi::new("your-api-key")?;
    ///
    /// let final_job = api.wait_for_completion(
    ///     "ft-123",
    ///     Some(Duration::from_secs(60))
    /// ).await?;
    ///
    /// if let Some(model_name) = final_job.fine_tuned_model {
    ///     println!("Fine-tuned model ready: {model_name}");
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn wait_for_completion(
        &self,
        fine_tuning_job_id: impl Into<String>,
        poll_interval: Option<Duration>,
    ) -> Result<FineTuningJob> {
        self.monitor_fine_tuning_job(fine_tuning_job_id, poll_interval, None)
            .await
    }

    /// Create a fine-tuning job and wait for it to complete
    ///
    /// This is a convenience method that combines job creation with monitoring.
    ///
    /// # Arguments
    ///
    /// * `request` - The fine-tuning job request
    /// * `poll_interval` - How often to check the job status (default: 30 seconds)
    /// * `event_callback` - Optional callback function called for each new event
    ///
    /// # Returns
    ///
    /// The final fine-tuning job object
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use openai_rust_sdk::api::fine_tuning::FineTuningApi;
    /// # use openai_rust_sdk::models::fine_tuning::FineTuningJobRequest;
    /// # use std::time::Duration;
    /// # tokio_test::block_on(async {
    /// let api = FineTuningApi::new("your-api-key")?;
    ///
    /// let job_request = FineTuningJobRequest::new("file-abc123", "gpt-3.5-turbo");
    ///
    /// let final_job = api.create_and_monitor_fine_tuning_job(
    ///     job_request,
    ///     Some(Duration::from_secs(30)),
    ///     Some(Box::new(|event| println!("Training event: {}", event.message)))
    /// ).await?;
    ///
    /// println!("Training completed with status: {:?}", final_job.status);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn create_and_monitor_fine_tuning_job(
        &self,
        request: FineTuningJobRequest,
        poll_interval: Option<Duration>,
        event_callback: Option<EventCallback>,
    ) -> Result<FineTuningJob> {
        // Create the job
        let job = self.create_fine_tuning_job(request).await?;

        // Monitor it to completion
        self.monitor_fine_tuning_job(&job.id, poll_interval, event_callback)
            .await
    }
}

#[cfg(test)]
mod tests {
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
    fn test_empty_api_key() {
        let result = FineTuningApi::new("");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("API key cannot be empty"));
    }

    #[test]
    fn test_whitespace_api_key() {
        let result = FineTuningApi::new("   ");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("API key cannot be empty"));
    }
}
