//! Report generation functionality for batch processing results

use crate::{De, Ser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    /// Calculates the average response length
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn average_response_length(&self) -> f64 {
        if self.successful_responses == 0 {
            0.0
        } else {
            self.total_tokens as f64 / self.successful_responses as f64
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

        self.add_summary_section(&mut report);
        self.add_yara_section(&mut report);
        self.add_error_section(&mut report);
        self.add_recommendations_section(&mut report);

        report
    }

    /// Add summary statistics section to the report
    fn add_summary_section(&self, report: &mut String) {
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
            self.average_response_length()
        ));
    }

    /// Add YARA analysis section to the report
    fn add_yara_section(&self, report: &mut String) {
        report.push_str("## YARA Rule Analysis\n\n");
        report.push_str(&format!(
            "- **YARA Rules Found**: {}\n",
            self.yara_rules_found
        ));
        report.push_str(&format!(
            "- **YARA Extraction Rate**: {:.1}%\n\n",
            self.yara_extraction_rate()
        ));
    }

    /// Add error analysis section to the report
    fn add_error_section(&self, report: &mut String) {
        if !self.error_types.is_empty() {
            report.push_str("## Error Analysis\n\n");
            for (error_type, count) in &self.error_types {
                report.push_str(&format!("- **{error_type}**: {count} occurrences\n"));
            }
            report.push('\n');
        }
    }

    /// Add recommendations section to the report
    fn add_recommendations_section(&self, report: &mut String) {
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
    }

    /// Reset all metrics to zero
    pub fn reset(&mut self) {
        self.total_responses = 0;
        self.successful_responses = 0;
        self.error_responses = 0;
        self.yara_rules_found = 0;
        self.total_tokens = 0;
        self.error_types.clear();
    }
}

impl Default for BatchReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_yara_extraction_rate() {
        let mut report = BatchReport::new();

        report.add_successful_response(100, true);
        report.add_successful_response(200, false);

        assert_eq!(report.yara_rules_found, 1);
        assert_eq!(report.successful_responses, 2);
        assert_eq!(report.yara_extraction_rate(), 50.0);
    }

    #[test]
    fn test_average_response_length() {
        let mut report = BatchReport::new();

        report.add_successful_response(100, false);
        report.add_successful_response(200, false);

        assert_eq!(report.average_response_length(), 150.0);
    }

    #[test]
    fn test_error_types_counting() {
        let mut report = BatchReport::new();

        report.add_error_response(Some("rate_limit".to_string()));
        report.add_error_response(Some("rate_limit".to_string()));
        report.add_error_response(Some("timeout".to_string()));

        assert_eq!(report.error_types.get("rate_limit"), Some(&2));
        assert_eq!(report.error_types.get("timeout"), Some(&1));
    }

    #[test]
    fn test_report_generation() {
        let mut report = BatchReport::new();
        report.add_successful_response(100, true);
        report.add_error_response(Some("test_error".to_string()));

        let report_text = report.generate_report_text();
        assert!(report_text.contains("OpenAI Batch Processing Report"));
        assert!(report_text.contains("Summary Statistics"));
        assert!(report_text.contains("YARA Rule Analysis"));
        assert!(report_text.contains("Error Analysis"));
        assert!(report_text.contains("Recommendations"));
    }
}
