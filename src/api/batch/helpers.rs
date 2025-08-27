//! Helper functions and utilities for batch processing
//!
//! This module contains utility functions for YARA processing, report generation,
//! and other helper operations used in batch processing workflows.

use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
use std::path::Path;
use tokio::fs;

use super::reports::BatchReport;
use super::types::YaraRuleInfo;
use super::yara::YaraProcessor;

/// Helper operations for batch processing
pub struct BatchHelpers<'a> {
    /// HTTP client for making API requests
    pub http_client: &'a HttpClient,
}

impl<'a> BatchHelpers<'a> {
    /// Creates a new BatchHelpers instance
    pub fn new(http_client: &'a HttpClient) -> Self {
        Self { http_client }
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
        crate::helpers::write_string(report_path, &report_text).await?;

        Ok(report)
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
                return Ok(Some(YaraRuleInfo::new(id.to_string(), yara_rule)));
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
}
