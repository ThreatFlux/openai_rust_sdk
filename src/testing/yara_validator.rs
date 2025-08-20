//! # YARA Rule Validation
//!
//! This module provides comprehensive YARA rule validation using the yara-x engine.
//! It includes compilation testing, feature analysis, and pattern matching validation.
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::missing_const_for_fn)]
//!
//! ## Features
//!
//! - **Rule Compilation**: Validates YARA rule syntax and compilation
//! - **Feature Detection**: Analyzes rule features like hex patterns, regex, metadata
//! - **Performance Metrics**: Measures compilation time and resource usage
//! - **Pattern Testing**: Tests patterns against sample data
//! - **Error Reporting**: Detailed error messages and validation results
//!
//! ## Example
//!
//! ```rust
//! use openai_rust_sdk::testing::YaraValidator;
//!
//! let validator = YaraValidator::new();
//! let rule = r#"
//! rule detect_pe {
//!     strings:
//!         $mz = { 4D 5A }
//!     condition:
//!         $mz at 0
//! }
//! "#;
//!
//! let result = validator.validate_rule(rule)?;
//! assert!(result.is_valid);
//! assert!(result.features.has_hex_patterns);
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use yara_x::{Compiler, Rules, Scanner};

/// Errors that can occur during YARA rule validation
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    /// YARA rule compilation failed
    #[error("Compilation failed: {message}")]
    CompilationError {
        /// Error message from the compiler
        message: String,
    },
}

/// Result of YARA rule validation including metrics and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the rule compiled successfully
    pub is_valid: bool,
    /// Name of the rule extracted from the rule definition
    pub rule_name: Option<String>,
    /// Compilation or validation errors
    pub errors: Vec<ValidationError>,
    /// Non-fatal warnings about the rule
    pub warnings: Vec<String>,
    /// Detected features and characteristics of the rule
    pub features: RuleFeatures,
    /// Performance and size metrics
    pub metrics: ValidationMetrics,
    /// Results of testing patterns against sample data
    pub pattern_tests: Vec<PatternTestResult>,
}

/// Analysis of YARA rule features and complexity
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleFeatures {
    /// Whether the rule has a strings section
    pub has_strings: bool,
    /// Whether the rule contains hex patterns
    pub has_hex_patterns: bool,
    /// Whether the rule contains regular expressions
    pub has_regex_patterns: bool,
    /// Whether the rule has metadata
    pub has_metadata: bool,
    /// Whether the rule imports external modules
    pub has_imports: bool,
    /// Whether the rule uses external variables
    pub uses_external_vars: bool,
    /// Whether the rule uses iterators (any of, all of)
    pub uses_iterators: bool,
    /// Number of string patterns in the rule
    pub string_count: usize,
    /// Complexity score from 1-10 based on rule features
    pub complexity_score: u8,
}

/// Performance metrics for rule validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    /// Time taken to compile the rule in milliseconds
    pub compilation_time_ms: u64,
    /// Size of the rule source code in bytes
    pub rule_size_bytes: usize,
    /// Number of patterns in the compiled rule
    pub pattern_count: usize,
}

/// Result of testing a pattern against sample data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTestResult {
    /// Identifier for the test pattern
    pub pattern_id: String,
    /// Description of the test data used
    pub test_data: String,
    /// Whether the pattern matched the test data
    pub matched: bool,
    /// Additional details about the match result
    pub match_details: Option<String>,
}

/// YARA rule validator with built-in test samples
///
/// The validator provides comprehensive rule validation including:
/// - Syntax and compilation checking
/// - Feature analysis and classification
/// - Pattern testing against sample data
/// - Performance metrics collection
pub struct YaraValidator {
    /// Sample data for testing patterns
    test_samples: HashMap<String, Vec<u8>>,
}

impl Default for YaraValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl YaraValidator {
    /// Creates a new YARA validator with default test samples
    ///
    /// The validator comes pre-loaded with test samples including:
    /// - PE header sample for testing executable detection
    /// - Text sample for testing string patterns
    ///
    /// # Example
    ///
    /// ```
    /// use openai_rust_sdk::testing::YaraValidator;
    ///
    /// let validator = YaraValidator::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let mut test_samples = HashMap::new();
        test_samples.insert("pe_sample".to_string(), b"MZ\x90\x00PE\x00\x00".to_vec());
        test_samples.insert("text_sample".to_string(), b"email@example.com".to_vec());
        Self { test_samples }
    }

    /// Get the test samples (for testing)
    #[allow(dead_code)]
    #[must_use]
    pub fn test_samples(&self) -> &HashMap<String, Vec<u8>> {
        &self.test_samples
    }

    /// Validates a YARA rule and returns comprehensive analysis
    ///
    /// This method performs complete validation including:
    /// - Syntax checking and compilation
    /// - Feature analysis
    /// - Pattern testing against sample data
    /// - Performance metrics collection
    ///
    /// # Arguments
    ///
    /// * `rule_source` - The YARA rule source code as a string
    ///
    /// # Returns
    ///
    /// Returns a `ValidationResult` containing validation status, metrics,
    /// detected features, and pattern test results.
    ///
    /// # Example
    ///
    /// ```
    /// use openai_rust_sdk::testing::YaraValidator;
    ///
    /// let validator = YaraValidator::new();
    /// let rule = r#"
    /// rule test {
    ///     condition: true
    /// }
    /// "#;
    ///
    /// let result = validator.validate_rule(rule)?;
    /// assert!(result.is_valid);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn validate_rule(&self, rule_source: &str) -> Result<ValidationResult> {
        let start_time = std::time::Instant::now();

        let mut result = ValidationResult {
            is_valid: false,
            rule_name: self.extract_rule_name(rule_source),
            errors: Vec::new(),
            warnings: Vec::new(),
            features: self.analyze_features(rule_source),
            metrics: ValidationMetrics {
                compilation_time_ms: 0,
                rule_size_bytes: rule_source.len(),
                pattern_count: 0,
            },
            pattern_tests: Vec::new(),
        };

        match self.compile_rule(rule_source) {
            Ok(rules) => {
                result.is_valid = true;
                result.metrics.compilation_time_ms = start_time.elapsed().as_millis() as u64;
                result.pattern_tests = self.test_patterns(&rules)?;
            }
            Err(e) => {
                result.errors.push(ValidationError::CompilationError {
                    message: e.to_string(),
                });
                result.metrics.compilation_time_ms = start_time.elapsed().as_millis() as u64;
            }
        }

        Ok(result)
    }

    /// Compiles a YARA rule using the yara-x compiler
    ///
    /// # Arguments
    ///
    /// * `rule_source` - The YARA rule source code
    ///
    /// # Returns
    ///
    /// Returns compiled `Rules` on success or an error if compilation fails
    #[allow(clippy::unused_self)]
    fn compile_rule(&self, rule_source: &str) -> Result<Rules> {
        let mut compiler = Compiler::new();
        compiler
            .add_source(rule_source)
            .context("Failed to add rule source")?;
        let rules = compiler.build();
        Ok(rules)
    }

    /// Extracts the rule name from YARA rule source code
    ///
    /// Parses the rule source to find and extract the rule name from
    /// the rule definition line.
    ///
    /// # Arguments
    ///
    /// * `rule_source` - The YARA rule source code
    ///
    /// # Returns
    ///
    /// Returns the rule name if found, None otherwise
    #[allow(clippy::unused_self)]
    fn extract_rule_name(&self, rule_source: &str) -> Option<String> {
        for line in rule_source.lines() {
            let line = line.trim();
            if line.starts_with("rule ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Some(parts[1].trim_end_matches('{').to_string());
                }
            }
        }
        None
    }

    /// Analyzes YARA rule features and characteristics
    ///
    /// Performs static analysis of the rule source to detect:
    /// - String patterns and types
    /// - Metadata presence
    /// - External variable usage
    /// - Iterator usage
    /// - Complexity scoring
    ///
    /// # Arguments
    ///
    /// * `rule_source` - The YARA rule source code
    ///
    /// # Returns
    ///
    /// Returns a `RuleFeatures` struct with detailed feature analysis
    #[allow(clippy::unused_self)]
    fn analyze_features(&self, rule_source: &str) -> RuleFeatures {
        let mut features = RuleFeatures::default();
        let source_lower = rule_source.to_lowercase();

        features.has_strings = source_lower.contains("strings:");
        features.has_metadata = source_lower.contains("meta:");
        features.has_imports = source_lower.contains("import ");
        features.uses_external_vars = source_lower.contains("filesize");
        features.uses_iterators =
            source_lower.contains("any of") || source_lower.contains("all of");

        let mut string_count = 0;
        let mut in_strings_section = false;

        for line in rule_source.lines() {
            let line = line.trim();

            if line.starts_with("strings:") {
                in_strings_section = true;
                continue;
            }

            if line.starts_with("condition:") {
                in_strings_section = false;
                continue;
            }

            if in_strings_section && line.starts_with('$') {
                string_count += 1;

                if line.contains('{') && line.contains('}') {
                    features.has_hex_patterns = true;
                }

                if line.contains('/') && line.matches('/').count() >= 2 {
                    features.has_regex_patterns = true;
                }
            }
        }

        features.string_count = string_count;
        features.complexity_score = (string_count + 1).min(10) as u8;

        features
    }

    /// Tests compiled patterns against sample data
    ///
    /// Runs the compiled rule against all available test samples
    /// to verify pattern matching behavior.
    ///
    /// # Arguments
    ///
    /// * `rules` - Compiled YARA rules to test
    ///
    /// # Returns
    ///
    /// Returns a vector of `PatternTestResult` with match results
    /// for each test sample
    #[allow(clippy::unnecessary_wraps)]
    fn test_patterns(&self, rules: &Rules) -> Result<Vec<PatternTestResult>> {
        let mut results = Vec::new();
        let mut scanner = Scanner::new(rules);

        for (sample_name, sample_data) in &self.test_samples {
            match scanner.scan(sample_data) {
                Ok(scan_results) => {
                    let matched = scan_results.matching_rules().len() > 0;

                    results.push(PatternTestResult {
                        pattern_id: sample_name.clone(),
                        test_data: format!("{} ({} bytes)", sample_name, sample_data.len()),
                        matched,
                        match_details: if matched {
                            Some(format!(
                                "{} rules matched",
                                scan_results.matching_rules().len()
                            ))
                        } else {
                            None
                        },
                    });
                }
                Err(e) => {
                    results.push(PatternTestResult {
                        pattern_id: sample_name.clone(),
                        test_data: format!("{} ({} bytes)", sample_name, sample_data.len()),
                        matched: false,
                        match_details: Some(format!("Scan error: {e}")),
                    });
                }
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_validator_creation() {
        let validator = YaraValidator::new();
        assert!(!validator.test_samples.is_empty());
        assert!(validator.test_samples.contains_key("pe_sample"));
        assert!(validator.test_samples.contains_key("text_sample"));
    }

    #[test]
    fn test_valid_simple_rule() {
        let validator = YaraValidator::new();
        let rule = r#"
            rule test_rule {
                strings:
                    $hello = "hello"
                condition:
                    $hello
            }
        "#;

        let result = validator.validate_rule(rule).unwrap();

        assert!(result.is_valid);
        assert_eq!(result.rule_name, Some("test_rule".to_string()));
        assert!(result.errors.is_empty());
        assert!(result.features.has_strings);
        assert!(!result.features.has_hex_patterns);
        assert!(!result.features.has_regex_patterns);
        assert_eq!(result.features.string_count, 1);
        // Compilation time is always non-negative by type
        assert!(result.metrics.rule_size_bytes > 0);
    }

    #[test]
    fn test_valid_hex_pattern_rule() {
        let validator = YaraValidator::new();
        let rule = r"
            rule hex_rule {
                strings:
                    $hex = { 4D 5A }
                condition:
                    $hex
            }
        ";

        let result = validator.validate_rule(rule).unwrap();

        assert!(result.is_valid);
        assert_eq!(result.rule_name, Some("hex_rule".to_string()));
        assert!(result.features.has_strings);
        assert!(result.features.has_hex_patterns);
        assert!(!result.features.has_regex_patterns);
        assert_eq!(result.features.string_count, 1);
    }

    #[test]
    fn test_invalid_rule() {
        let validator = YaraValidator::new();
        let rule = r#"
            rule invalid_rule {
                strings:
                    $str = "test"
                condition:
                    invalid_function()
            }
        "#;

        let result = validator.validate_rule(rule).unwrap();

        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
        assert_eq!(result.rule_name, Some("invalid_rule".to_string()));
    }

    #[test]
    fn test_rule_with_metadata() {
        let validator = YaraValidator::new();
        let rule = r#"
            rule metadata_rule {
                meta:
                    author = "test"
                    description = "test rule"
                strings:
                    $str = "test"
                condition:
                    $str
            }
        "#;

        let result = validator.validate_rule(rule).unwrap();

        assert!(result.is_valid);
        assert!(result.features.has_metadata);
        assert!(result.features.has_strings);
    }

    #[test]
    fn test_rule_with_regex() {
        let validator = YaraValidator::new();
        let rule = r"
            rule regex_rule {
                strings:
                    $regex = /test[0-9]+/
                condition:
                    $regex
            }
        ";

        let result = validator.validate_rule(rule).unwrap();

        assert!(result.is_valid);
        assert!(result.features.has_regex_patterns);
        assert!(result.features.has_strings);
    }

    #[test]
    fn test_rule_with_iterators() {
        let validator = YaraValidator::new();
        let rule = r#"
            rule iterator_rule {
                strings:
                    $str1 = "test1"
                    $str2 = "test2"
                condition:
                    any of them
            }
        "#;

        let result = validator.validate_rule(rule).unwrap();

        assert!(result.is_valid);
        assert!(result.features.uses_iterators);
        assert_eq!(result.features.string_count, 2);
    }

    #[test]
    fn test_extract_rule_name() {
        let validator = YaraValidator::new();

        let rule1 = "rule simple_rule { condition: true }";
        assert_eq!(
            validator.extract_rule_name(rule1),
            Some("simple_rule".to_string())
        );

        let rule2 = "rule complex_rule_name { condition: true }";
        assert_eq!(
            validator.extract_rule_name(rule2),
            Some("complex_rule_name".to_string())
        );

        let rule3 = "invalid rule";
        assert_eq!(validator.extract_rule_name(rule3), None);
    }

    #[test]
    fn test_feature_analysis() {
        let validator = YaraValidator::new();

        let rule = r#"
            import "pe"
            rule complex_rule {
                meta:
                    author = "test"
                strings:
                    $str = "hello"
                    $hex = { 4D 5A }
                    $regex = /test[0-9]+/
                condition:
                    filesize > 1000 and any of them
            }
        "#;

        let features = validator.analyze_features(rule);

        assert!(features.has_strings);
        assert!(features.has_hex_patterns);
        assert!(features.has_regex_patterns);
        assert!(features.has_metadata);
        assert!(features.has_imports);
        assert!(features.uses_external_vars);
        assert!(features.uses_iterators);
        assert_eq!(features.string_count, 3);
        assert!(features.complexity_score > 1);
    }

    #[test]
    fn test_complexity_scoring() {
        let validator = YaraValidator::new();

        let simple_rule = "rule simple { condition: true }";
        let simple_features = validator.analyze_features(simple_rule);
        assert_eq!(simple_features.complexity_score, 1);

        let complex_rule = r#"
            rule complex {
                strings:
                    $s1 = "test1"
                    $s2 = "test2"
                    $s3 = "test3"
                    $s4 = "test4"
                    $s5 = "test5"
                condition:
                    any of them
            }
        "#;
        let complex_features = validator.analyze_features(complex_rule);
        assert!(complex_features.complexity_score > simple_features.complexity_score);
    }

    #[test]
    fn test_pattern_testing() {
        let validator = YaraValidator::new();
        let rule = r"
            rule pe_detector {
                strings:
                    $mz = { 4D 5A }
                condition:
                    $mz at 0
            }
        ";

        let result = validator.validate_rule(rule).unwrap();

        assert!(result.is_valid);
        assert!(!result.pattern_tests.is_empty());

        // Should have tested against both samples
        assert_eq!(result.pattern_tests.len(), 2);

        // PE sample should match
        let pe_test = result
            .pattern_tests
            .iter()
            .find(|test| test.pattern_id == "pe_sample")
            .expect("PE sample test should exist");
        assert!(pe_test.matched);
    }

    #[test]
    fn test_validation_metrics() {
        let validator = YaraValidator::new();
        let rule = r"
            rule metric_test {
                condition: true
            }
        ";

        let result = validator.validate_rule(rule).unwrap();

        // Compilation time and pattern count are always non-negative by type
        assert_eq!(result.metrics.rule_size_bytes, rule.len());
    }

    #[test]
    fn test_error_handling() {
        let validator = YaraValidator::new();
        let invalid_rule = "this is not a valid YARA rule";

        let result = validator.validate_rule(invalid_rule).unwrap();

        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());

        // Check that error is properly categorized
        let ValidationError::CompilationError { message } = &result.errors[0];
        assert!(!message.is_empty());
    }

    #[test]
    fn test_serialization() {
        let validator = YaraValidator::new();
        let rule = "
            rule serialization_test {
                strings:
                    $test = \"hello\"
                condition:
                    $test
            }
        ";

        let result = validator.validate_rule(rule).unwrap();

        // Test that result can be serialized to JSON
        let json = serde_json::to_string(&result).unwrap();
        assert!(!json.is_empty());

        // Test that it can be deserialized back
        let deserialized: ValidationResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result.is_valid, deserialized.is_valid);
        assert_eq!(result.rule_name, deserialized.rule_name);
    }
}
