//! File operations for batch processing
//!
//! This module handles file upload and download operations for batch processing,
//! including batch file uploads, result downloads, and error file handling.

use crate::api::base::HttpClient;
use crate::api::shared_utilities::FormBuilder;
use crate::constants::endpoints;
use crate::error::{OpenAIError, Result};
use std::path::Path;
use tokio::fs;

use super::models::{Batch, FileUploadResponse};

/// File operations implementation for batch processing
pub struct FileOperations<'a> {
    /// HTTP client for making API requests
    pub http_client: &'a HttpClient,
}

impl<'a> FileOperations<'a> {
    /// Creates a new FileOperations instance
    pub fn new(http_client: &'a HttpClient) -> Self {
        Self { http_client }
    }

    /// Uploads a JSONL file for batch processing
    pub async fn upload_batch_file(&self, file_path: &Path) -> Result<FileUploadResponse> {
        let file_contents = crate::helpers::read_bytes(file_path).await?;

        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("batch_input.jsonl");

        let form = FormBuilder::create_jsonl_upload_form(
            file_contents,
            filename.to_string(),
            "batch".to_string(),
        )?;

        self.http_client.post_multipart("/v1/files", form).await
    }

    /// Downloads a file by its ID
    pub async fn download_file(&self, file_id: &str) -> Result<String> {
        self.http_client
            .get_text(&endpoints::files::content(file_id))
            .await
    }

    /// Retrieves the results of a completed batch
    pub async fn get_batch_results(&self, batch_id: &str) -> Result<String> {
        let batch = self.get_batch_status(batch_id).await?;

        let output_file_id = batch.output_file_id.ok_or_else(|| OpenAIError::ApiError {
            status: 400,
            message: format!(
                "Batch {} has no output file. Status: {}",
                batch_id, batch.status
            ),
        })?;

        self.download_file(&output_file_id).await
    }

    /// Retrieves error information for a batch (if any errors occurred)
    pub async fn get_batch_errors(&self, batch_id: &str) -> Result<Option<String>> {
        let batch = self.get_batch_status(batch_id).await?;

        if let Some(error_file_id) = batch.error_file_id {
            let error_content = self.download_file(&error_file_id).await?;
            Ok(Some(error_content))
        } else {
            Ok(None)
        }
    }

    /// Downloads and saves batch results to a local file
    pub async fn download_batch_results(
        &self,
        batch_id: &str,
        output_path: &Path,
    ) -> Result<usize> {
        let results = self.get_batch_results(batch_id).await?;

        crate::helpers::write_string(output_path, &results).await?;

        let line_count = results.lines().count();
        Ok(line_count)
    }

    /// Downloads and saves batch errors to a local file (if any errors occurred)
    pub async fn download_batch_errors(&self, batch_id: &str, error_path: &Path) -> Result<usize> {
        if let Some(errors) = self.get_batch_errors(batch_id).await? {
            crate::helpers::write_string(error_path, &errors).await?;

            let line_count = errors.lines().count();
            Ok(line_count)
        } else {
            Ok(0)
        }
    }

    /// Downloads and saves all batch files (results and errors) to a directory
    pub async fn download_all_batch_files(
        &self,
        batch_id: &str,
        output_dir: &Path,
    ) -> Result<(usize, usize)> {
        fs::create_dir_all(output_dir).await.map_err(|e| {
            OpenAIError::FileError(format!(
                "Failed to create output directory {}: {}",
                output_dir.display(),
                e
            ))
        })?;

        let results_path = output_dir.join(format!("{batch_id}_results.jsonl"));
        let result_count = self.download_batch_results(batch_id, &results_path).await?;

        let errors_path = output_dir.join(format!("{batch_id}_errors.jsonl"));
        let error_count = self.download_batch_errors(batch_id, &errors_path).await?;

        Ok((result_count, error_count))
    }

    /// Helper method to get batch status (used internally)
    async fn get_batch_status(&self, batch_id: &str) -> Result<Batch> {
        self.http_client
            .get(&endpoints::batches::by_id(batch_id))
            .await
    }
}
