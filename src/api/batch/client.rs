//! Main BatchApi client for OpenAI batch processing operations

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::Result;
use std::path::Path;

use super::file_ops::FileOperations;
use super::helpers::BatchHelpers;
use super::models::{Batch, BatchList, FileUploadResponse};
use super::operations::BatchOperations;
use super::reports::BatchReport;

/// `OpenAI` Batch API client for asynchronous batch processing
#[derive(Debug, Clone)]
pub struct BatchApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for BatchApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl BatchApi {
    // Core Batch Operations

    /// Creates a new batch processing job
    pub async fn create_batch(&self, input_file_id: &str, endpoint: &str) -> Result<Batch> {
        let ops = BatchOperations::new(&self.http_client);
        ops.create_batch(input_file_id, endpoint).await
    }

    /// Creates a new batch processing job with optional metadata
    pub async fn create_batch_with_metadata(
        &self,
        input_file_id: &str,
        endpoint: &str,
        metadata: Option<serde_json::Value>,
    ) -> Result<Batch> {
        let ops = BatchOperations::new(&self.http_client);
        ops.create_batch_with_metadata(input_file_id, endpoint, metadata)
            .await
    }

    /// Retrieves the current status of a batch
    pub async fn get_batch_status(&self, batch_id: &str) -> Result<Batch> {
        let ops = BatchOperations::new(&self.http_client);
        ops.get_batch_status(batch_id).await
    }

    /// Cancels a batch that is currently processing
    pub async fn cancel_batch(&self, batch_id: &str) -> Result<Batch> {
        let ops = BatchOperations::new(&self.http_client);
        ops.cancel_batch(batch_id).await
    }

    /// Lists all batches for the current user
    pub async fn list_batches(&self, limit: Option<u32>, after: Option<&str>) -> Result<BatchList> {
        let ops = BatchOperations::new(&self.http_client);
        ops.list_batches(limit, after).await
    }

    /// Waits for a batch to complete, polling the status at regular intervals
    pub async fn wait_for_completion(
        &self,
        batch_id: &str,
        poll_interval_secs: Option<u64>,
        max_wait_secs: Option<u64>,
    ) -> Result<Batch> {
        let ops = BatchOperations::new(&self.http_client);
        ops.wait_for_completion(batch_id, poll_interval_secs, max_wait_secs)
            .await
    }

    // File Operations

    /// Uploads a JSONL file for batch processing
    pub async fn upload_batch_file(&self, file_path: &Path) -> Result<FileUploadResponse> {
        let file_ops = FileOperations::new(&self.http_client);
        file_ops.upload_batch_file(file_path).await
    }

    /// Retrieves the results of a completed batch
    pub async fn get_batch_results(&self, batch_id: &str) -> Result<String> {
        let file_ops = FileOperations::new(&self.http_client);
        file_ops.get_batch_results(batch_id).await
    }

    /// Retrieves error information for a batch (if any errors occurred)
    pub async fn get_batch_errors(&self, batch_id: &str) -> Result<Option<String>> {
        let file_ops = FileOperations::new(&self.http_client);
        file_ops.get_batch_errors(batch_id).await
    }

    /// Downloads and saves batch results to a local file
    pub async fn download_batch_results(
        &self,
        batch_id: &str,
        output_path: &Path,
    ) -> Result<usize> {
        let file_ops = FileOperations::new(&self.http_client);
        file_ops.download_batch_results(batch_id, output_path).await
    }

    /// Downloads and saves batch errors to a local file (if any errors occurred)
    pub async fn download_batch_errors(&self, batch_id: &str, error_path: &Path) -> Result<usize> {
        let file_ops = FileOperations::new(&self.http_client);
        file_ops.download_batch_errors(batch_id, error_path).await
    }

    /// Downloads and saves all batch files (results and errors) to a directory
    pub async fn download_all_batch_files(
        &self,
        batch_id: &str,
        output_dir: &Path,
    ) -> Result<(usize, usize)> {
        let file_ops = FileOperations::new(&self.http_client);
        file_ops
            .download_all_batch_files(batch_id, output_dir)
            .await
    }

    // Helper Operations

    /// Processes batch results and extracts YARA rules from responses
    pub async fn process_yara_results(
        &self,
        results_file: &Path,
        output_dir: &Path,
    ) -> Result<usize> {
        let helpers = BatchHelpers::new(&self.http_client);
        helpers.process_yara_results(results_file, output_dir).await
    }

    /// Generates a comprehensive report from batch results
    pub async fn generate_batch_report(
        &self,
        results_file: &Path,
        errors_file: Option<&Path>,
        report_path: &Path,
    ) -> Result<BatchReport> {
        let helpers = BatchHelpers::new(&self.http_client);
        helpers
            .generate_batch_report(results_file, errors_file, report_path)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_api_creation() {
        let api = BatchApi::new("test-key").unwrap();
        assert_eq!(api.http_client.api_key(), "test-key");
        assert_eq!(api.http_client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_batch_api_custom_base_url() {
        let api = BatchApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.http_client.api_key(), "test-key");
        assert_eq!(api.http_client.base_url(), "https://custom.api.com");
    }
}
