//! Core batch operations for OpenAI batch processing
//!
//! This module contains the main batch operations including creation, status checking,
//! cancellation, listing, and completion waiting.

use crate::api::base::HttpClient;
use crate::constants::endpoints;
use crate::error::{OpenAIError, Result};
use tokio::time;

use super::models::{Batch, BatchList, CreateBatchRequest};
use super::types::BatchStatus;

/// Core batch operations implementation
pub struct BatchOperations<'a> {
    /// HTTP client for making API requests
    pub http_client: &'a HttpClient,
}

impl<'a> BatchOperations<'a> {
    /// Creates a new BatchOperations instance
    pub fn new(http_client: &'a HttpClient) -> Self {
        Self { http_client }
    }

    /// Creates a new batch processing job
    pub async fn create_batch(&self, input_file_id: &str, endpoint: &str) -> Result<Batch> {
        self.create_batch_with_metadata(input_file_id, endpoint, None)
            .await
    }

    /// Creates a new batch processing job with optional metadata
    pub async fn create_batch_with_metadata(
        &self,
        input_file_id: &str,
        endpoint: &str,
        metadata: Option<serde_json::Value>,
    ) -> Result<Batch> {
        let request = CreateBatchRequest {
            input_file_id: input_file_id.to_string(),
            endpoint: endpoint.to_string(),
            completion_window: "24h".to_string(),
            metadata,
        };

        self.http_client.post("/v1/batches", &request).await
    }

    /// Retrieves the current status of a batch
    pub async fn get_batch_status(&self, batch_id: &str) -> Result<Batch> {
        self.http_client
            .get(&endpoints::batches::by_id(batch_id))
            .await
    }

    /// Cancels a batch that is currently processing
    pub async fn cancel_batch(&self, batch_id: &str) -> Result<Batch> {
        let empty_body = serde_json::Value::Null;
        self.http_client
            .post(&endpoints::batches::cancel(batch_id), &empty_body)
            .await
    }

    /// Lists all batches for the current user
    pub async fn list_batches(&self, limit: Option<u32>, after: Option<&str>) -> Result<BatchList> {
        let mut params = Vec::new();

        if let Some(limit) = limit {
            params.push(("limit".to_string(), limit.to_string()));
        }

        if let Some(after) = after {
            params.push(("after".to_string(), after.to_string()));
        }

        self.http_client
            .get_with_query("/v1/batches", &params)
            .await
    }

    /// Waits for a batch to complete, polling the status at regular intervals
    pub async fn wait_for_completion(
        &self,
        batch_id: &str,
        poll_interval_secs: Option<u64>,
        max_wait_secs: Option<u64>,
    ) -> Result<Batch> {
        let poll_interval = time::Duration::from_secs(poll_interval_secs.unwrap_or(30));
        let max_wait = time::Duration::from_secs(max_wait_secs.unwrap_or(24 * 60 * 60)); // 24 hours
        let start_time = std::time::Instant::now();

        loop {
            let batch = self.get_batch_status(batch_id).await?;

            // Check if batch has reached a terminal state
            match batch.status {
                BatchStatus::Completed
                | BatchStatus::Failed
                | BatchStatus::Expired
                | BatchStatus::Cancelled => {
                    return Ok(batch);
                }
                _ => {
                    // Continue polling
                }
            }

            // Check if we've exceeded the maximum wait time
            if start_time.elapsed() > max_wait {
                return Err(OpenAIError::RequestError(format!(
                    "Timeout waiting for batch {} to complete after {} seconds",
                    batch_id,
                    max_wait.as_secs()
                )));
            }

            time::sleep(poll_interval).await;
        }
    }
}
