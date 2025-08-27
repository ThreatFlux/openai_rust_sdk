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
//! - **YARA Processing**: Extract and process YARA rules from batch results
//! - **Comprehensive Reporting**: Generate detailed analysis reports
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::{batch::{BatchApi, BatchStatus}, common::ApiClientConstructors};
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
//!     println!("Results: {results}");
//! }
//! # Ok::<(), openai_rust_sdk::OpenAIError>(())
//! # });
//! ```

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::api::shared_utilities::FormBuilder;
use crate::constants::endpoints;
use crate::error::{OpenAIError, Result};
use crate::{De, Ser};
use reqwest::multipart;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

// =============================================================================
// TYPES MODULE
// =============================================================================

/// Batch status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
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
            Self::Validating => "validating",
            Self::Failed => "failed",
            Self::InProgress => "in_progress",
            Self::Finalizing => "finalizing",
            Self::Completed => "completed",
            Self::Expired => "expired",
            Self::Cancelling => "cancelling",
            Self::Cancelled => "cancelled",
        };
        write!(f, "{status}")
    }
}

/// Information about a YARA rule extracted from batch results
#[derive(Debug, Clone)]
struct YaraRuleInfo {
    /// Custom ID from the batch request
    custom_id: String,
    /// The extracted YARA rule content
    rule_content: String,
}

/// Request counts for batch processing
#[derive(Debug, Clone, Ser, De)]
pub struct BatchRequestCounts {
    /// Total number of requests in the batch
    pub total: u32,
    /// Number of completed requests
    pub completed: u32,
    /// Number of failed requests
    pub failed: u32,
}

// =============================================================================
// MODELS MODULE
// =============================================================================

/// File upload response from `OpenAI` Files API
#[derive(Debug, Clone, Ser, De)]
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

/// Complete batch object returned by `OpenAI`
#[derive(Debug, Clone, Ser, De)]
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
#[derive(Debug, Clone, Ser)]
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
#[derive(Debug, Clone, Ser, De)]
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

// =============================================================================
// REPORT MODULE
// =============================================================================

/// Comprehensive report generated from batch processing results
#[derive(Debug, Clone, Ser, De)]
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
            #[allow(clippy::cast_precision_loss)]
            {
                (self.successful_responses as f64 / self.total_responses as f64) * 100.0
            }
        }
    }

    /// Calculates the YARA rule extraction rate as a percentage
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn yara_extraction_rate(&self) -> f64 {
        if self.successful_responses == 0 {
            0.0
        } else {
            (self.yara_rules_found as f64 / self.successful_responses as f64) * 100.0
        }
    }

    /// Adds metrics for a successful response
    pub fn add_successful_response(&mut self, content_length: usize, has_yara_rule: bool) {
        self.total_responses += 1;
        self.successful_responses += 1;
        self.total_tokens += content_length;

        if has_yara_rule {
            self.yara_rules_found += 1;
        }
    }

    /// Adds metrics for an error response
    pub fn add_error_response(&mut self, error_type: Option<String>) {
        self.total_responses += 1;
        self.error_responses += 1;

        if let Some(error_type) = error_type {
            *self.error_types.entry(error_type).or_insert(0) += 1;
        }
    }

    /// Generates a formatted report text
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::format_push_string)]
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

// =============================================================================
// YARA UTILITIES MODULE
// =============================================================================

/// YARA processing utilities
pub struct YaraProcessor;

impl YaraProcessor {
    /// Extracts YARA rule content from AI response text
    pub fn extract_yara_rule(content: &str) -> Option<String> {
        // Look for content between ```yara and next ```
        if let Some(start) = content.find("```yara") {
            let rule_start = start + 7; // Skip "```yara"
            let rule_start = content[rule_start..]
                .find('\n')
                .map_or(rule_start, |newline| rule_start + newline + 1);

            if let Some(end) = content[rule_start..].find("```") {
                let rule_end = rule_start + end;
                return Some(content[rule_start..rule_end].trim().to_string());
            }
        }

        // Look for content between ``` (generic code blocks)
        if let Some(start) = content.find("```") {
            let after_first = start + 3;
            let rule_start = content[after_first..]
                .find('\n')
                .map_or(after_first, |i| after_first + i + 1);

            if let Some(end) = content[rule_start..].find("```") {
                let rule_end = rule_start + end;
                let potential_rule = content[rule_start..rule_end].trim();

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
}

// =============================================================================
// MAIN API CLIENT
// =============================================================================

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

    /// Downloads a file by its ID
    async fn download_file(&self, file_id: &str) -> Result<String> {
        self.http_client
            .get_text(&endpoints::files::content(file_id))
            .await
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

    /// Processes batch results and extracts YARA rules from responses
    pub async fn process_yara_results(
        &self,
        results_file: &Path,
        output_dir: &Path,
    ) -> Result<usize> {
        self.ensure_output_directory(output_dir).await?;
        let content = crate::helpers::read_string(results_file).await?;

        let mut rule_count = 0;
        for line in content.lines() {
            if let Some(rule_info) = self.parse_yara_response_line(line)? {
                self.save_yara_rule(&rule_info, output_dir).await?;
                rule_count += 1;
            }
        }

        Ok(rule_count)
    }

    /// Ensures the output directory exists
    async fn ensure_output_directory(&self, output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir).await.map_err(|e| {
            OpenAIError::FileError(format!(
                "Failed to create YARA rules directory {}: {}",
                output_dir.display(),
                e
            ))
        })
    }

    /// Parses a single response line and extracts YARA rule information
    fn parse_yara_response_line(&self, line: &str) -> Result<Option<YaraRuleInfo>> {
        let parsed: serde_json::Value = match serde_json::from_str(line) {
            Ok(val) => val,
            Err(_) => return Ok(None),
        };

        let custom_id = parsed.get("custom_id").and_then(|v| v.as_str());
        if custom_id.is_none() {
            return Ok(None);
        }

        let yara_content = self.extract_yara_content_from_response(&parsed);
        if let (Some(id), Some(content)) = (custom_id, yara_content) {
            if let Some(yara_rule) = YaraProcessor::extract_yara_rule(&content) {
                return Ok(Some(YaraRuleInfo {
                    custom_id: id.to_string(),
                    rule_content: yara_rule.to_string(),
                }));
            }
        }

        Ok(None)
    }

    /// Extracts YARA content from a parsed response
    fn extract_yara_content_from_response(&self, parsed: &serde_json::Value) -> Option<String> {
        let content = parsed
            .get("response")?
            .get("body")?
            .get("choices")?
            .get(0)?
            .get("message")?
            .get("content")
            .and_then(|v| v.as_str());

        content.map(|s| s.to_string())
    }

    /// Saves a YARA rule to the output directory
    async fn save_yara_rule(&self, rule_info: &YaraRuleInfo, output_dir: &Path) -> Result<()> {
        let rule_filename = format!("{}.yar", rule_info.custom_id);
        let rule_path = output_dir.join(rule_filename);
        crate::helpers::write_string(&rule_path, &rule_info.rule_content).await
    }

    /// Generates a comprehensive report from batch results
    pub async fn generate_batch_report(
        &self,
        results_file: &Path,
        errors_file: Option<&Path>,
        report_path: &Path,
    ) -> Result<BatchReport> {
        let mut report = BatchReport::new();

        self.analyze_results_file(&mut report, results_file).await?;
        self.analyze_errors_file(&mut report, errors_file).await?;

        let report_text = report.generate_report_text();
        crate::helpers::write_string(report_path, report_text).await?;

        Ok(report)
    }

    /// Analyzes the results file and updates the report
    async fn analyze_results_file(
        &self,
        report: &mut BatchReport,
        results_file: &Path,
    ) -> Result<()> {
        let Ok(content) = crate::helpers::read_string(results_file).await else {
            return Ok(()); // Skip if file can't be read
        };

        for line in content.lines() {
            self.process_result_line(report, line);
        }

        Ok(())
    }

    /// Processes a single result line and updates the report
    fn process_result_line(&self, report: &mut BatchReport, line: &str) {
        let Ok(parsed) = serde_json::from_str::<serde_json::Value>(line) else {
            return;
        };

        // Handle successful responses
        if let Some(response_content) = self.extract_response_content(&parsed) {
            let has_yara = YaraProcessor::extract_yara_rule(&response_content).is_some();
            report.add_successful_response(response_content.len(), has_yara);
        }

        // Handle error responses
        if parsed.get("error").is_some() {
            report.add_error_response(None);
        }
    }

    /// Extracts response content from a parsed JSON value
    fn extract_response_content(&self, parsed: &serde_json::Value) -> Option<String> {
        parsed
            .get("response")?
            .get("body")?
            .get("choices")?
            .get(0)?
            .get("message")?
            .get("content")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Analyzes the errors file and updates the report
    async fn analyze_errors_file(
        &self,
        report: &mut BatchReport,
        errors_file: Option<&Path>,
    ) -> Result<()> {
        let Some(errors_path) = errors_file else {
            return Ok(());
        };

        let Ok(error_content) = crate::helpers::read_string(errors_path).await else {
            return Ok(()); // Skip if file can't be read
        };

        for line in error_content.lines() {
            self.process_error_line(report, line);
        }

        Ok(())
    }

    /// Processes a single error line and updates the report
    fn process_error_line(&self, report: &mut BatchReport, line: &str) {
        let Ok(parsed) = serde_json::from_str::<serde_json::Value>(line) else {
            return;
        };

        if let Some(error) = parsed.get("error") {
            let error_code = error
                .get("code")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            report.add_error_response(error_code);
        }
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

    #[test]
    fn test_batch_report_new() {
        let report = BatchReport::new();
        assert_eq!(report.total_responses, 0);
        assert_eq!(report.successful_responses, 0);
        assert_eq!(report.error_responses, 0);
        assert_eq!(report.yara_rules_found, 0);
        assert_eq!(report.total_tokens, 0);
        assert!(report.error_types.is_empty());
    }

    #[test]
    fn test_yara_processor_extract_rule() {
        let content = r#"Here's a YARA rule:

```yara
rule TestRule {
    strings:
        $a = "test"
    condition:
        $a
}
```

Hope this helps!"#;

        let result = YaraProcessor::extract_yara_rule(content);
        assert!(result.is_some());
        let rule = result.unwrap();
        assert!(rule.contains("rule TestRule"));
        assert!(rule.contains("strings:"));
        assert!(rule.contains("condition:"));
    }

    #[test]
    fn test_yara_processor_no_rule() {
        let content = "This is just regular text with no YARA rules.";
        let result = YaraProcessor::extract_yara_rule(content);
        assert!(result.is_none());
    }

    #[test]
    fn test_batch_report_success_rate() {
        let mut report = BatchReport::new();
        assert_eq!(report.success_rate(), 0.0);

        report.add_successful_response(100, false);
        report.add_successful_response(200, false);
        report.add_error_response(Some("test_error".to_string()));

        assert_eq!(report.total_responses, 3);
        assert_eq!(report.successful_responses, 2);
        assert_eq!(report.error_responses, 1);

        let expected_rate = (2.0 / 3.0) * 100.0;
        assert!((report.success_rate() - expected_rate).abs() < 0.01);
    }
}
