//! # OpenAI Batch API Client
//!
//! This module provides a complete implementation of OpenAI's Batch API for processing
//! asynchronous groups of requests with 50% lower costs and higher rate limits.
//!
//! ## Features
//!
//! - **File Upload**: Upload JSONL batch files to OpenAI
//! - **Batch Creation**: Create and submit batch processing jobs  
//! - **Status Monitoring**: Check batch progress and completion status
//! - **Result Retrieval**: Download completed batch results
//! - **Batch Management**: List, cancel, and manage batch operations
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::batch::{BatchApi, BatchStatus};
//! use std::path::Path;
//!
//! # tokio_test::block_on(async {
//! let api = BatchApi::new("your-api-key")?;
//!
//! // Upload batch file
//! let file = api.upload_batch_file(Path::new("batch_input.jsonl")).await?;
//!
//! // Create batch
//! let batch = api.create_batch(&file.id, "/v1/chat/completions").await?;
//!
//! // Monitor status
//! let status = api.get_batch_status(&batch.id).await?;
//! println!("Batch status: {}", status.status);
//!
//! // Retrieve results when complete
//! if status.status == BatchStatus::Completed {
//!     let results = api.get_batch_results(&batch.id).await?;
//!     println!("Results: {}", results);
//! }
//! # Ok::<(), openai_rust_sdk::OpenAIError>(())
//! # });
//! ```

use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

/// `OpenAI` Batch API client for asynchronous batch processing
#[derive(Debug, Clone)]
pub struct BatchApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

/// Batch status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    /// The input file is being validated before the batch can begin
    Validating,
    /// The input file has failed the validation process
    Failed,
    /// The input file was successfully validated and the batch is currently being run
    InProgress,
    /// The batch has completed and the results are being prepared
    Finalizing,
    /// The batch has been completed and the results are ready
    Completed,
    /// The batch was not able to be completed within the 24-hour time window
    Expired,
    /// The batch is being cancelled (may take up to 10 minutes)
    Cancelling,
    /// The batch was cancelled
    Cancelled,
}

impl std::fmt::Display for BatchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            BatchStatus::Validating => "validating",
            BatchStatus::Failed => "failed",
            BatchStatus::InProgress => "in_progress",
            BatchStatus::Finalizing => "finalizing",
            BatchStatus::Completed => "completed",
            BatchStatus::Expired => "expired",
            BatchStatus::Cancelling => "cancelling",
            BatchStatus::Cancelled => "cancelled",
        };
        write!(f, "{status}")
    }
}

/// File upload response from `OpenAI` Files API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResponse {
    /// Unique identifier for the uploaded file
    pub id: String,
    /// Type of object (always "file")
    pub object: String,
    /// Size of the file in bytes
    pub bytes: u64,
    /// Unix timestamp of when the file was created
    pub created_at: u64,
    /// Name of the uploaded file
    pub filename: String,
    /// Purpose of the file upload (always "batch" for batch API)
    pub purpose: String,
}

/// Request counts for batch processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequestCounts {
    /// Total number of requests in the batch
    pub total: u32,
    /// Number of completed requests
    pub completed: u32,
    /// Number of failed requests
    pub failed: u32,
}

/// Complete batch object returned by `OpenAI`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    /// Unique identifier for the batch
    pub id: String,
    /// Type of object (always "batch")
    pub object: String,
    /// API endpoint used for the batch
    pub endpoint: String,
    /// Any errors that occurred during batch processing
    pub errors: Option<serde_json::Value>,
    /// ID of the input file
    pub input_file_id: String,
    /// Completion window (e.g., "24h")
    pub completion_window: String,
    /// Current status of the batch
    pub status: BatchStatus,
    /// ID of the output file (available when completed)
    pub output_file_id: Option<String>,
    /// ID of the error file (if any errors occurred)
    pub error_file_id: Option<String>,
    /// Unix timestamp of when the batch was created
    pub created_at: u64,
    /// Unix timestamp of when the batch started processing
    pub in_progress_at: Option<u64>,
    /// Unix timestamp of when the batch expires
    pub expires_at: u64,
    /// Unix timestamp of when the batch completed
    pub completed_at: Option<u64>,
    /// Unix timestamp of when the batch failed
    pub failed_at: Option<u64>,
    /// Unix timestamp of when the batch expired
    pub expired_at: Option<u64>,
    /// Request counts and statistics
    pub request_counts: BatchRequestCounts,
    /// Optional metadata for the batch
    pub metadata: Option<serde_json::Value>,
}

/// Request to create a new batch
#[derive(Debug, Clone, Serialize)]
pub struct CreateBatchRequest {
    /// ID of the uploaded input file
    pub input_file_id: String,
    /// API endpoint to use for the batch
    pub endpoint: String,
    /// Completion window (currently only "24h" is supported)
    pub completion_window: String,
    /// Optional metadata for the batch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// List of batches response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchList {
    /// Type of object (always "list")
    pub object: String,
    /// Array of batch objects
    pub data: Vec<Batch>,
    /// Whether there are more results available
    pub has_more: bool,
    /// ID of the first item in the list
    pub first_id: Option<String>,
    /// ID of the last item in the list
    pub last_id: Option<String>,
}

/// Comprehensive report generated from batch processing results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchReport {
    /// Total number of responses processed
    pub total_responses: usize,
    /// Number of successful responses
    pub successful_responses: usize,
    /// Number of error responses
    pub error_responses: usize,
    /// Number of YARA rules found in responses
    pub yara_rules_found: usize,
    /// Total content length (approximate token count)
    pub total_tokens: usize,
    /// Error types and their counts
    pub error_types: HashMap<String, usize>,
}

impl BatchReport {
    /// Creates a new empty batch report
    #[must_use]
    pub fn new() -> Self {
        Self {
            total_responses: 0,
            successful_responses: 0,
            error_responses: 0,
            yara_rules_found: 0,
            total_tokens: 0,
            error_types: HashMap::new(),
        }
    }

    /// Calculates the success rate as a percentage
    #[must_use]
    pub fn success_rate(&self) -> f64 {
        if self.total_responses == 0 {
            0.0
        } else {
            (self.successful_responses as f64 / self.total_responses as f64) * 100.0
        }
    }

    /// Calculates the YARA rule extraction rate as a percentage
    #[must_use]
    pub fn yara_extraction_rate(&self) -> f64 {
        if self.successful_responses == 0 {
            0.0
        } else {
            (self.yara_rules_found as f64 / self.successful_responses as f64) * 100.0
        }
    }

    /// Generates a formatted report text
    #[must_use]
    pub fn generate_report_text(&self) -> String {
        let mut report = String::new();

        report.push_str("# OpenAI Batch Processing Report\n\n");
        report.push_str(&format!(
            "Generated at: {}\n\n",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        report.push_str("## Summary Statistics\n\n");
        report.push_str(&format!(
            "- **Total Responses**: {}\n",
            self.total_responses
        ));
        report.push_str(&format!(
            "- **Successful Responses**: {}\n",
            self.successful_responses
        ));
        report.push_str(&format!(
            "- **Error Responses**: {}\n",
            self.error_responses
        ));
        report.push_str(&format!(
            "- **Success Rate**: {:.1}%\n",
            self.success_rate()
        ));
        report.push_str(&format!(
            "- **Total Content Length**: {} characters\n",
            self.total_tokens
        ));
        report.push_str(&format!(
            "- **Average Response Length**: {:.0} characters\n\n",
            if self.successful_responses > 0 {
                self.total_tokens as f64 / self.successful_responses as f64
            } else {
                0.0
            }
        ));

        report.push_str("## YARA Rule Analysis\n\n");
        report.push_str(&format!(
            "- **YARA Rules Found**: {}\n",
            self.yara_rules_found
        ));
        report.push_str(&format!(
            "- **YARA Extraction Rate**: {:.1}%\n\n",
            self.yara_extraction_rate()
        ));

        if !self.error_types.is_empty() {
            report.push_str("## Error Analysis\n\n");
            for (error_type, count) in &self.error_types {
                report.push_str(&format!("- **{error_type}**: {count} occurrences\n"));
            }
            report.push('\n');
        }

        report.push_str("## Recommendations\n\n");
        if self.success_rate() < 90.0 {
            report.push_str("- ⚠️ Success rate is below 90%. Consider reviewing your prompts or model parameters.\n");
        }
        if self.yara_extraction_rate() < 80.0 && self.yara_rules_found > 0 {
            report.push_str(
                "- ⚠️ YARA rule extraction rate is low. Consider improving prompt specificity.\n",
            );
        }
        if self.success_rate() >= 95.0 {
            report.push_str(
                "- ✅ Excellent success rate! Your batch configuration is working well.\n",
            );
        }
        if self.yara_extraction_rate() >= 90.0 && self.yara_rules_found > 0 {
            report.push_str("- ✅ High YARA rule extraction rate indicates effective prompts.\n");
        }

        report
    }
}

impl Default for BatchReport {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchApi {
    /// Creates a new Batch API client
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    ///
    /// # Example
    ///
    /// ```rust
    /// use openai_rust_sdk::api::batch::BatchApi;
    ///
    /// let api = BatchApi::new("your-api-key")?;
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// ```
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
        })
    }

    /// Creates a new Batch API client with custom base URL
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your `OpenAI` API key
    /// * `base_url` - Custom base URL for the API
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Uploads a JSONL file for batch processing
    ///
    /// The file must be in JSONL format where each line contains a valid batch request.
    /// Maximum file size is 200 MB and up to 50,000 requests per batch.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the JSONL file to upload
    ///
    /// # Returns
    ///
    /// Returns a `FileUploadResponse` containing the file ID needed to create a batch
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    /// use std::path::Path;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let file = api.upload_batch_file(Path::new("batch_input.jsonl")).await?;
    /// println!("Uploaded file ID: {}", file.id);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn upload_batch_file(&self, file_path: &Path) -> Result<FileUploadResponse> {
        // Read the file
        let file_contents = fs::read(file_path).await.map_err(|e| {
            OpenAIError::FileError(format!(
                "Failed to read file {}: {}",
                file_path.display(),
                e
            ))
        })?;

        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("batch_input.jsonl");

        // Create multipart form
        let part = multipart::Part::bytes(file_contents)
            .file_name(filename.to_string())
            .mime_str("application/jsonl")
            .map_err(|e| OpenAIError::RequestError(format!("Failed to create file part: {e}")))?;

        let form = multipart::Form::new()
            .part("file", part)
            .text("purpose", "batch");

        // Upload file
        self.http_client.post_multipart("/v1/files", form).await
    }

    /// Creates a new batch processing job
    ///
    /// # Arguments
    ///
    /// * `input_file_id` - ID of the uploaded JSONL file
    /// * `endpoint` - API endpoint to use (e.g., "/v1/chat/completions")
    /// * `metadata` - Optional metadata for the batch
    ///
    /// # Returns
    ///
    /// Returns a `Batch` object with the batch details and status
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let batch = api.create_batch("file-abc123", "/v1/chat/completions").await?;
    /// println!("Created batch: {}", batch.id);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn create_batch(&self, input_file_id: &str, endpoint: &str) -> Result<Batch> {
        self.create_batch_with_metadata(input_file_id, endpoint, None)
            .await
    }

    /// Creates a new batch processing job with optional metadata
    ///
    /// # Arguments
    ///
    /// * `input_file_id` - ID of the uploaded JSONL file
    /// * `endpoint` - API endpoint to use (e.g., "/v1/chat/completions")
    /// * `metadata` - Optional metadata for the batch
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
    ///
    /// # Arguments
    ///
    /// * `batch_id` - ID of the batch to check
    ///
    /// # Returns
    ///
    /// Returns the current `Batch` object with updated status information
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let status = api.get_batch_status("batch_abc123").await?;
    /// println!("Batch status: {}", status.status);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn get_batch_status(&self, batch_id: &str) -> Result<Batch> {
        self.http_client
            .get(&format!("/v1/batches/{batch_id}"))
            .await
    }

    /// Retrieves the results of a completed batch
    ///
    /// This method downloads the output file containing the results of all completed requests.
    /// The batch must be in "completed" status to retrieve results.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - ID of the completed batch
    ///
    /// # Returns
    ///
    /// Returns the content of the output file as a string (JSONL format)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let results = api.get_batch_results("batch_abc123").await?;
    /// println!("Batch results: {}", results);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn get_batch_results(&self, batch_id: &str) -> Result<String> {
        // First get the batch status to find the output file ID
        let batch = self.get_batch_status(batch_id).await?;

        let output_file_id = batch.output_file_id.ok_or_else(|| OpenAIError::ApiError {
            status: 400,
            message: format!(
                "Batch {} has no output file. Status: {}",
                batch_id, batch.status
            ),
        })?;

        // Download the output file
        self.download_file(&output_file_id).await
    }

    /// Downloads and saves batch results to a local file
    ///
    /// This method downloads the output file and saves it to the specified path.
    /// The batch must be in "completed" status to retrieve results.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - ID of the completed batch
    /// * `output_path` - Path where the results file will be saved
    ///
    /// # Returns
    ///
    /// Returns the number of result lines written to the file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    /// use std::path::Path;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let count = api.download_batch_results("batch_abc123", Path::new("results.jsonl")).await?;
    /// println!("Downloaded {} results to file", count);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn download_batch_results(
        &self,
        batch_id: &str,
        output_path: &Path,
    ) -> Result<usize> {
        let results = self.get_batch_results(batch_id).await?;

        // Write results to file
        fs::write(output_path, &results).await.map_err(|e| {
            OpenAIError::FileError(format!(
                "Failed to write results to {}: {}",
                output_path.display(),
                e
            ))
        })?;

        // Count the number of lines
        let line_count = results.lines().count();
        Ok(line_count)
    }

    /// Downloads and saves batch errors to a local file (if any errors occurred)
    ///
    /// # Arguments
    ///
    /// * `batch_id` - ID of the batch to check for errors
    /// * `error_path` - Path where the error file will be saved
    ///
    /// # Returns
    ///
    /// Returns the number of error lines written, or 0 if no errors occurred
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    /// use std::path::Path;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let error_count = api.download_batch_errors("batch_abc123", Path::new("errors.jsonl")).await?;
    /// if error_count > 0 {
    ///     println!("Downloaded {} errors to file", error_count);
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn download_batch_errors(&self, batch_id: &str, error_path: &Path) -> Result<usize> {
        if let Some(errors) = self.get_batch_errors(batch_id).await? {
            // Write errors to file
            fs::write(error_path, &errors).await.map_err(|e| {
                OpenAIError::FileError(format!(
                    "Failed to write errors to {}: {}",
                    error_path.display(),
                    e
                ))
            })?;

            // Count the number of error lines
            let line_count = errors.lines().count();
            Ok(line_count)
        } else {
            Ok(0)
        }
    }

    /// Downloads and saves all batch files (results and errors) to a directory
    ///
    /// This convenience method downloads both result and error files for a completed batch,
    /// saving them with standardized names in the specified directory.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - ID of the completed batch
    /// * `output_dir` - Directory where files will be saved
    ///
    /// # Returns
    ///
    /// Returns a tuple of (`result_count`, `error_count`) indicating the number of lines in each file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    /// use std::path::Path;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let (results, errors) = api.download_all_batch_files("batch_abc123", Path::new("./batch_output")).await?;
    /// println!("Downloaded {} results and {} errors", results, errors);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn download_all_batch_files(
        &self,
        batch_id: &str,
        output_dir: &Path,
    ) -> Result<(usize, usize)> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir).await.map_err(|e| {
            OpenAIError::FileError(format!(
                "Failed to create output directory {}: {}",
                output_dir.display(),
                e
            ))
        })?;

        // Download results
        let results_path = output_dir.join(format!("{batch_id}_results.jsonl"));
        let result_count = self.download_batch_results(batch_id, &results_path).await?;

        // Download errors (if any)
        let errors_path = output_dir.join(format!("{batch_id}_errors.jsonl"));
        let error_count = self.download_batch_errors(batch_id, &errors_path).await?;

        Ok((result_count, error_count))
    }

    /// Processes batch results and extracts YARA rules from responses
    ///
    /// This utility method reads a batch results file and extracts generated YARA rules
    /// from the AI responses, saving them as individual .yar files.
    ///
    /// # Arguments
    ///
    /// * `results_file` - Path to the batch results JSONL file
    /// * `output_dir` - Directory where YARA rule files will be saved
    ///
    /// # Returns
    ///
    /// Returns the number of YARA rules successfully extracted and saved
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    /// use std::path::Path;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let rule_count = api.process_yara_results(
    ///     Path::new("batch_results.jsonl"),
    ///     Path::new("./yara_rules")
    /// ).await?;
    /// println!("Extracted {} YARA rules", rule_count);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn process_yara_results(
        &self,
        results_file: &Path,
        output_dir: &Path,
    ) -> Result<usize> {
        // Create output directory
        fs::create_dir_all(output_dir).await.map_err(|e| {
            OpenAIError::FileError(format!(
                "Failed to create YARA rules directory {}: {}",
                output_dir.display(),
                e
            ))
        })?;

        // Read the results file
        let content = fs::read_to_string(results_file).await.map_err(|e| {
            OpenAIError::FileError(format!(
                "Failed to read results file {}: {}",
                results_file.display(),
                e
            ))
        })?;

        let mut rule_count = 0;

        for line in content.lines() {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(custom_id) = parsed.get("custom_id").and_then(|v| v.as_str()) {
                    if let Some(response) = parsed.get("response") {
                        if let Some(body) = response.get("body") {
                            if let Some(choices) = body.get("choices") {
                                if let Some(first_choice) = choices.get(0) {
                                    if let Some(message) = first_choice.get("message") {
                                        if let Some(content) =
                                            message.get("content").and_then(|v| v.as_str())
                                        {
                                            // Extract YARA rule from the response
                                            if let Some(yara_rule) = self.extract_yara_rule(content)
                                            {
                                                let rule_filename = format!("{custom_id}.yar");
                                                let rule_path = output_dir.join(rule_filename);

                                                fs::write(&rule_path, yara_rule).await.map_err(
                                                    |e| {
                                                        OpenAIError::FileError(format!(
                                                            "Failed to write YARA rule to {}: {}",
                                                            rule_path.display(),
                                                            e
                                                        ))
                                                    },
                                                )?;

                                                rule_count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(rule_count)
    }

    /// Extracts YARA rule content from AI response text
    ///
    /// This helper method searches for YARA rule patterns in the response content
    /// and extracts clean rule text.
    fn extract_yara_rule(&self, content: &str) -> Option<String> {
        // Look for content between ```yara and next ```
        if let Some(start) = content.find("```yara") {
            let rule_start = start + 7; // Skip "```yara"
            // Skip to the next line after ```yara
            let rule_start = if let Some(newline) = content[rule_start..].find('\n') {
                rule_start + newline + 1
            } else {
                rule_start
            };

            if let Some(end) = content[rule_start..].find("```") {
                let rule_end = rule_start + end;
                return Some(content[rule_start..rule_end].trim().to_string());
            }
        }

        // Look for content between ``` (generic code blocks)
        if let Some(start) = content.find("```") {
            let after_first = start + 3;
            // Skip any language identifier
            let rule_start = content[after_first..]
                .find('\n')
                .map_or(after_first, |i| after_first + i + 1);

            if let Some(end) = content[rule_start..].find("```") {
                let rule_end = rule_start + end;
                let potential_rule = content[rule_start..rule_end].trim();

                // Check if it looks like a YARA rule
                if potential_rule.contains("rule ")
                    && potential_rule.contains('{')
                    && potential_rule.contains('}')
                {
                    return Some(potential_rule.to_string());
                }
            }
        }

        // Look for rule patterns without code blocks
        if content.contains("rule ") && content.contains('{') && content.contains('}') {
            // Try to extract the rule part
            if let Some(rule_start) = content.find("rule ") {
                let remaining = &content[rule_start..];
                let mut brace_count = 0;
                let mut rule_end = remaining.len();

                for (i, ch) in remaining.char_indices() {
                    match ch {
                        '{' => brace_count += 1,
                        '}' => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                rule_end = i + 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                if brace_count == 0 {
                    let rule_text = &remaining[..rule_end];
                    return Some(rule_text.trim().to_string());
                }
            }
        }

        None
    }

    /// Generates a comprehensive report from batch results
    ///
    /// This method analyzes batch results and generates a detailed report including
    /// success rates, error analysis, and extracted content statistics.
    ///
    /// # Arguments
    ///
    /// * `results_file` - Path to the batch results JSONL file
    /// * `errors_file` - Optional path to the batch errors JSONL file
    /// * `report_path` - Path where the report will be saved
    ///
    /// # Returns
    ///
    /// Returns a `BatchReport` containing comprehensive analysis
    pub async fn generate_batch_report(
        &self,
        results_file: &Path,
        errors_file: Option<&Path>,
        report_path: &Path,
    ) -> Result<BatchReport> {
        let mut report = BatchReport::new();

        // Analyze results file
        if let Ok(content) = fs::read_to_string(results_file).await {
            for line in content.lines() {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(line) {
                    report.total_responses += 1;

                    if let Some(response) = parsed.get("response") {
                        if response.get("body").is_some() {
                            report.successful_responses += 1;

                            // Check for YARA rule content
                            if let Some(choices) =
                                response.get("body").and_then(|b| b.get("choices"))
                            {
                                if let Some(first_choice) = choices.get(0) {
                                    if let Some(message) = first_choice.get("message") {
                                        if let Some(content) =
                                            message.get("content").and_then(|v| v.as_str())
                                        {
                                            if self.extract_yara_rule(content).is_some() {
                                                report.yara_rules_found += 1;
                                            }
                                            report.total_tokens += content.len();
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if parsed.get("error").is_some() {
                        report.error_responses += 1;
                    }
                }
            }
        }

        // Analyze errors file if provided
        if let Some(errors_path) = errors_file {
            if let Ok(error_content) = fs::read_to_string(errors_path).await {
                for line in error_content.lines() {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(line) {
                        if let Some(error) = parsed.get("error") {
                            if let Some(code) = error.get("code").and_then(|v| v.as_str()) {
                                *report.error_types.entry(code.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        // Generate report text
        let report_text = report.generate_report_text();

        // Save report to file
        fs::write(report_path, report_text).await.map_err(|e| {
            OpenAIError::FileError(format!(
                "Failed to write report to {}: {}",
                report_path.display(),
                e
            ))
        })?;

        Ok(report)
    }

    /// Retrieves error information for a batch (if any errors occurred)
    ///
    /// # Arguments
    ///
    /// * `batch_id` - ID of the batch to check for errors
    ///
    /// # Returns
    ///
    /// Returns the content of the error file as a string if errors occurred, or None if no errors
    pub async fn get_batch_errors(&self, batch_id: &str) -> Result<Option<String>> {
        let batch = self.get_batch_status(batch_id).await?;

        if let Some(error_file_id) = batch.error_file_id {
            let error_content = self.download_file(&error_file_id).await?;
            Ok(Some(error_content))
        } else {
            Ok(None)
        }
    }

    /// Downloads a file by its ID
    ///
    /// # Arguments
    ///
    /// * `file_id` - ID of the file to download
    ///
    /// # Returns
    ///
    /// Returns the content of the file as a string
    async fn download_file(&self, file_id: &str) -> Result<String> {
        self.http_client
            .get_text(&format!("/v1/files/{file_id}/content"))
            .await
    }

    /// Cancels a batch that is currently processing
    ///
    /// The batch status will change to "cancelling" until in-flight requests complete,
    /// then change to "cancelled". This may take up to 10 minutes.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - ID of the batch to cancel
    ///
    /// # Returns
    ///
    /// Returns the updated `Batch` object with cancellation status
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let cancelled_batch = api.cancel_batch("batch_abc123").await?;
    /// println!("Batch status: {}", cancelled_batch.status);
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn cancel_batch(&self, batch_id: &str) -> Result<Batch> {
        // Use empty body for POST cancel request
        let empty_body = serde_json::Value::Null;
        self.http_client
            .post(&format!("/v1/batches/{batch_id}/cancel"), &empty_body)
            .await
    }

    /// Lists all batches for the current user
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of batches to return (default: 20, max: 100)
    /// * `after` - Cursor for pagination (batch ID to start after)
    ///
    /// # Returns
    ///
    /// Returns a `BatchList` containing an array of batch objects
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let batches = api.list_batches(Some(10), None).await?;
    /// for batch in batches.data {
    ///     println!("Batch {}: {}", batch.id, batch.status);
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
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
    ///
    /// This is a convenience method that automatically polls the batch status until
    /// it reaches a terminal state (completed, failed, expired, or cancelled).
    ///
    /// # Arguments
    ///
    /// * `batch_id` - ID of the batch to monitor
    /// * `poll_interval_secs` - How often to check the status (in seconds, default: 30)
    /// * `max_wait_secs` - Maximum time to wait before timing out (default: 24 hours)
    ///
    /// # Returns
    ///
    /// Returns the final `Batch` object when a terminal state is reached
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use openai_rust_sdk::api::batch::BatchApi;
    ///
    /// # tokio_test::block_on(async {
    /// let api = BatchApi::new("your-api-key")?;
    /// let final_batch = api.wait_for_completion("batch_abc123", Some(60), None).await?;
    ///
    /// match final_batch.status {
    ///     openai_rust_sdk::api::batch::BatchStatus::Completed => {
    ///         println!("Batch completed successfully!");
    ///         let results = api.get_batch_results(&final_batch.id).await?;
    ///         println!("Results: {}", results);
    ///     }
    ///     _ => println!("Batch finished with status: {}", final_batch.status),
    /// }
    /// # Ok::<(), openai_rust_sdk::OpenAIError>(())
    /// # });
    /// ```
    pub async fn wait_for_completion(
        &self,
        batch_id: &str,
        poll_interval_secs: Option<u64>,
        max_wait_secs: Option<u64>,
    ) -> Result<Batch> {
        let poll_interval = std::time::Duration::from_secs(poll_interval_secs.unwrap_or(30));
        let max_wait = std::time::Duration::from_secs(max_wait_secs.unwrap_or(24 * 60 * 60)); // 24 hours
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

            // Wait before next poll
            tokio::time::sleep(poll_interval).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_status_display() {
        assert_eq!(BatchStatus::Validating.to_string(), "validating");
        assert_eq!(BatchStatus::InProgress.to_string(), "in_progress");
        assert_eq!(BatchStatus::Completed.to_string(), "completed");
    }

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
