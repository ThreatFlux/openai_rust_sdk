//! # YARA Test Cases and Test Suite Management
//!
//! This module provides pre-defined test cases and test suite management
//! for comprehensive YARA rule validation testing.
//!
//! ## Features
//!
//! - **Pre-defined Test Cases**: Collection of known-good and known-bad rules
//! - **Test Suite Execution**: Automated execution of all test cases
//! - **Result Aggregation**: Comprehensive reporting of test results
//! - **Success Rate Analysis**: Statistical analysis of validation performance
//!
//! ## Example
//!
//! ```
//! use openai_rust_sdk::testing::YaraTestCases;
//!
//! let test_cases = YaraTestCases::new();
//! let results = test_cases.run_all_tests()?;
//!
//! println!("Passed: {}/{}", results.passed_tests, results.total_tests);
//! println!("Success rate: {:.1}%", results.success_rate);
//! # Ok::<(), anyhow::Error>(())
//! ```

use crate::testing::yara_validator::{ValidationResult, YaraValidator};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Results from running a complete test suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteResult {
    /// Total number of tests executed
    pub total_tests: usize,
    /// Number of tests that passed
    pub passed_tests: usize,
    /// Number of tests that failed
    pub failed_tests: usize,
    /// Success rate as a percentage (0.0-100.0)
    pub success_rate: f64,
    /// Individual test case results
    pub test_results: Vec<TestCaseResult>,
}

/// Result from a single test case execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseResult {
    /// Unique identifier for the test case
    pub test_id: String,
    /// Human-readable name for the test case
    pub test_name: String,
    /// Whether the test case passed
    pub passed: bool,
    /// Detailed validation result
    pub validation_result: ValidationResult,
    /// Error message if the test failed
    pub error_message: Option<String>,
}

/// Test case manager for YARA rule validation
///
/// Provides a collection of predefined test cases and methods
/// to execute them as a complete test suite.
pub struct YaraTestCases {
    /// The YARA validator instance used for testing
    validator: YaraValidator,
}

impl YaraTestCases {
    /// Creates a new test case manager with default validator
    ///
    /// # Example
    ///
    /// ```
    /// use openai_rust_sdk::testing::YaraTestCases;
    ///
    /// let test_cases = YaraTestCases::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            validator: YaraValidator::new(),
        }
    }

    /// Executes all predefined test cases and returns aggregated results
    ///
    /// Runs a comprehensive test suite including:
    /// - Valid rules that should compile successfully
    /// - Invalid rules that should fail compilation
    /// - Rules with various features and complexity levels
    ///
    /// # Returns
    ///
    /// Returns a `TestSuiteResult` with detailed results and statistics
    ///
    /// # Errors
    ///
    /// Returns an error if the validation system encounters a critical failure
    ///
    /// # Example
    ///
    /// ```
    /// use openai_rust_sdk::testing::YaraTestCases;
    ///
    /// let test_cases = YaraTestCases::new();
    /// let results = test_cases.run_all_tests()?;
    ///
    /// if results.success_rate >= 90.0 {
    ///     println!("Test suite passed with {:.1}% success rate", results.success_rate);
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn run_all_tests(&self) -> Result<TestSuiteResult> {
        let test_cases = vec![
            (
                "basic_001",
                "Simple String",
                r#"
                rule simple_string {
                    strings:
                        $str = "hello"
                    condition:
                        $str
                }
            "#,
                true,
            ),
            (
                "hex_001",
                "Hex Pattern",
                r"
                rule hex_pattern {
                    strings:
                        $hex = { 4D 5A }
                    condition:
                        $hex
                }
            ",
                true,
            ),
            (
                "invalid_001",
                "Invalid Rule",
                r#"
                rule invalid_rule {
                    strings:
                        $str = "test"
                    condition:
                        invalid_syntax
                }
            "#,
                false,
            ),
        ];

        let mut test_results = Vec::new();
        let mut passed_tests = 0;

        for (id, name, rule, expected_valid) in test_cases {
            let validation_result = self.validator.validate_rule(rule)?;
            let passed = validation_result.is_valid == expected_valid;

            if passed {
                passed_tests += 1;
            }

            let error_message = if passed {
                None
            } else {
                Some(format!(
                    "Expected valid: {}, got: {}",
                    expected_valid, validation_result.is_valid
                ))
            };

            test_results.push(TestCaseResult {
                test_id: id.to_string(),
                test_name: name.to_string(),
                passed,
                validation_result,
                error_message,
            });
        }

        let total_tests = test_results.len();
        let failed_tests = total_tests - passed_tests;
        let success_rate = if total_tests > 0 {
            #[allow(clippy::cast_precision_loss)]
            {
                (passed_tests as f64 / total_tests as f64) * 100.0
            }
        } else {
            0.0
        };

        Ok(TestSuiteResult {
            total_tests,
            passed_tests,
            failed_tests,
            success_rate,
            test_results,
        })
    }
}

impl Default for YaraTestCases {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_test_cases_creation() {
        let test_cases = YaraTestCases::new();
        // Validator should be initialized
        assert_eq!(test_cases.validator.test_samples().len(), 2);
    }

    #[test]
    fn test_run_all_tests() {
        let test_cases = YaraTestCases::new();
        let result = test_cases.run_all_tests().unwrap();

        // Should have run some tests
        assert!(result.total_tests > 0);
        assert_eq!(
            result.total_tests,
            result.passed_tests + result.failed_tests
        );

        // Should have test results
        assert_eq!(result.test_results.len(), result.total_tests);

        // Success rate should be calculated correctly
        let expected_rate = if result.total_tests > 0 {
            #[allow(clippy::cast_precision_loss)]
            {
                (result.passed_tests as f64 / result.total_tests as f64) * 100.0
            }
        } else {
            0.0
        };
        assert!((result.success_rate - expected_rate).abs() < 0.01);
    }

    #[test]
    fn test_individual_test_results() {
        let test_cases = YaraTestCases::new();
        let result = test_cases.run_all_tests().unwrap();

        // Check that each test result has required fields
        for test_result in &result.test_results {
            assert!(!test_result.test_id.is_empty());
            assert!(!test_result.test_name.is_empty());

            // If test failed, should have error message
            if !test_result.passed {
                assert!(test_result.error_message.is_some());
            }
        }
    }

    #[test]
    fn test_basic_valid_rule_passes() {
        let test_cases = YaraTestCases::new();
        let result = test_cases.run_all_tests().unwrap();

        // Find the basic string test
        let basic_test = result
            .test_results
            .iter()
            .find(|test| test.test_id == "basic_001")
            .expect("basic_001 test should exist");

        assert!(basic_test.passed);
        assert!(basic_test.validation_result.is_valid);
        assert!(basic_test.error_message.is_none());
    }

    #[test]
    fn test_hex_pattern_rule_passes() {
        let test_cases = YaraTestCases::new();
        let result = test_cases.run_all_tests().unwrap();

        // Find the hex pattern test
        let hex_test = result
            .test_results
            .iter()
            .find(|test| test.test_id == "hex_001")
            .expect("hex_001 test should exist");

        assert!(hex_test.passed);
        assert!(hex_test.validation_result.is_valid);
        assert!(hex_test.validation_result.features.has_hex_patterns);
    }

    #[test]
    fn test_invalid_rule_fails() {
        let test_cases = YaraTestCases::new();
        let result = test_cases.run_all_tests().unwrap();

        // Find the invalid rule test
        let invalid_test = result
            .test_results
            .iter()
            .find(|test| test.test_id == "invalid_001")
            .expect("invalid_001 test should exist");

        assert!(invalid_test.passed); // Test passes because it correctly identified invalid rule
        assert!(!invalid_test.validation_result.is_valid); // But the rule itself is invalid
    }

    #[test]
    fn test_test_suite_result_serialization() {
        let test_cases = YaraTestCases::new();
        let result = test_cases.run_all_tests().unwrap();

        // Test serialization to JSON
        let json = serde_json::to_string(&result).unwrap();
        assert!(!json.is_empty());

        // Test deserialization
        let deserialized: TestSuiteResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result.total_tests, deserialized.total_tests);
        assert_eq!(result.passed_tests, deserialized.passed_tests);
        assert_eq!(result.failed_tests, deserialized.failed_tests);
        assert!((result.success_rate - deserialized.success_rate).abs() < 0.01);
    }

    #[test]
    fn test_test_case_result_fields() {
        let test_cases = YaraTestCases::new();
        let result = test_cases.run_all_tests().unwrap();

        for test_result in &result.test_results {
            // Test that all required fields are populated
            assert!(!test_result.test_id.is_empty());
            assert!(!test_result.test_name.is_empty());

            // Validation result should have basic fields
            assert!(test_result.validation_result.metrics.rule_size_bytes > 0);
            // Compilation time should be non-negative (no need to check >= 0 for unsigned type)
        }
    }

    #[test]
    fn test_success_rate_calculation() {
        let test_cases = YaraTestCases::new();
        let result = test_cases.run_all_tests().unwrap();

        // Manually verify success rate calculation
        let manual_passed = result
            .test_results
            .iter()
            .filter(|test| test.passed)
            .count();

        assert_eq!(result.passed_tests, manual_passed);

        let manual_rate = if result.total_tests > 0 {
            #[allow(clippy::cast_precision_loss)]
            {
                (manual_passed as f64 / result.total_tests as f64) * 100.0
            }
        } else {
            0.0
        };

        assert!((result.success_rate - manual_rate).abs() < 0.01);
    }

    #[test]
    fn test_default_implementation() {
        let test_cases1 = YaraTestCases::new();
        let test_cases2 = YaraTestCases::default();

        // Both should produce similar results
        let result1 = test_cases1.run_all_tests().unwrap();
        let result2 = test_cases2.run_all_tests().unwrap();

        assert_eq!(result1.total_tests, result2.total_tests);
        assert_eq!(result1.test_results.len(), result2.test_results.len());
    }

    #[test]
    fn test_feature_detection_in_tests() {
        let test_cases = YaraTestCases::new();
        let result = test_cases.run_all_tests().unwrap();

        // Find specific tests and verify feature detection
        for test_result in &result.test_results {
            match test_result.test_id.as_str() {
                "basic_001" => {
                    assert!(test_result.validation_result.features.has_strings);
                    assert!(!test_result.validation_result.features.has_hex_patterns);
                }
                "hex_001" => {
                    assert!(test_result.validation_result.features.has_strings);
                    assert!(test_result.validation_result.features.has_hex_patterns);
                }
                _ => {} // Other tests
            }
        }
    }
}
