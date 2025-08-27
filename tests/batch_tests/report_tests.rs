//! Tests for batch report generation

use openai_rust_sdk::api::batch::BatchReport;

#[cfg(test)]
mod batch_report_tests {
    use super::*;

    #[test]
    fn test_batch_report_creation() {
        let report = BatchReport::new();
        assert_eq!(report.total_responses, 0);
        assert_eq!(report.successful_responses, 0);
        assert_eq!(report.error_responses, 0);
        assert_eq!(report.yara_rules_found, 0);
        assert_eq!(report.total_tokens, 0);
        assert!(report.error_types.is_empty());
    }

    #[test]
    fn test_batch_report_default() {
        let report = BatchReport::default();
        assert_eq!(report.total_responses, 0);
        assert_eq!(report.successful_responses, 0);
    }

    #[test]
    fn test_batch_report_success_rate() {
        let mut report = BatchReport::new();
        report.total_responses = 100;
        report.successful_responses = 85;

        assert_eq!(report.success_rate(), 85.0);

        // Test edge case with zero total
        let empty_report = BatchReport::new();
        assert_eq!(empty_report.success_rate(), 0.0);
    }

    #[test]
    fn test_batch_report_yara_extraction_rate() {
        let mut report = BatchReport::new();
        report.successful_responses = 100;
        report.yara_rules_found = 75;

        assert_eq!(report.yara_extraction_rate(), 75.0);

        // Test edge case with zero successful responses
        report.successful_responses = 0;
        assert_eq!(report.yara_extraction_rate(), 0.0);
    }

    #[test]
    fn test_batch_report_error_types() {
        let mut report = BatchReport::new();
        report.error_types.insert("rate_limit".to_string(), 5);
        report.error_types.insert("invalid_request".to_string(), 3);

        assert_eq!(report.error_types.len(), 2);
        assert_eq!(report.error_types.get("rate_limit"), Some(&5));
        assert_eq!(report.error_types.get("invalid_request"), Some(&3));
    }

    #[test]
    fn test_batch_report_generate_text() {
        let mut report = BatchReport::new();
        report.total_responses = 100;
        report.successful_responses = 96; // Change to 96% to trigger "excellent" message
        report.error_responses = 4;
        report.yara_rules_found = 80;
        report.total_tokens = 50000;
        report.error_types.insert("timeout".to_string(), 4);

        let report_text = report.generate_report_text();

        assert!(report_text.contains("# OpenAI Batch Processing Report"));
        assert!(report_text.contains("Total Responses**: 100"));
        assert!(report_text.contains("Success Rate**: 96.0%"));
        assert!(report_text.contains("YARA Rules Found**: 80"));
        assert!(report_text.contains("**timeout**: 4 occurrences"));
        assert!(report_text.contains("## Recommendations"));
        assert!(report_text.contains("✅ Excellent success rate"));
    }

    #[test]
    fn test_batch_report_recommendations() {
        // Test low success rate recommendation
        let mut low_success_report = BatchReport::new();
        low_success_report.total_responses = 100;
        low_success_report.successful_responses = 80; // Below 90%

        let report_text = low_success_report.generate_report_text();
        assert!(report_text.contains("⚠️ Success rate is below 90%"));

        // Test high success rate recommendation
        let mut high_success_report = BatchReport::new();
        high_success_report.total_responses = 100;
        high_success_report.successful_responses = 98; // Above 95%

        let report_text = high_success_report.generate_report_text();
        assert!(report_text.contains("✅ Excellent success rate"));
    }

    #[test]
    fn test_batch_report_yara_recommendations() {
        // Test low YARA extraction rate
        let mut low_yara_report = BatchReport::new();
        low_yara_report.successful_responses = 100;
        low_yara_report.yara_rules_found = 70; // Below 80%

        let report_text = low_yara_report.generate_report_text();
        assert!(report_text.contains("⚠️ YARA rule extraction rate is low"));

        // Test high YARA extraction rate
        let mut high_yara_report = BatchReport::new();
        high_yara_report.successful_responses = 100;
        high_yara_report.yara_rules_found = 95; // Above 90%

        let report_text = high_yara_report.generate_report_text();
        assert!(report_text.contains("✅ High YARA rule extraction rate"));
    }
}
