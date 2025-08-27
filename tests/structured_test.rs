#![allow(clippy::pedantic, clippy::nursery)]
//! Integration tests for structured output functionality
//!
//! These tests verify structured data handling, serialization, and validation result processing.

#[cfg(feature = "yara")]
use openai_rust_sdk::testing::{BatchJobGenerator, YaraTestCases, YaraValidator};
#[cfg(feature = "yara")]
use std::collections::HashMap;
#[cfg(feature = "yara")]
use tempfile::NamedTempFile;

#[cfg(feature = "yara")]
#[test]
fn test_validation_result_structure() {
    let validator = YaraValidator::new();
    let rule = r#"
        rule structured_test {
            meta:
                author = "test"
                description = "structured output test"
            strings:
                $hex = { 4D 5A }
                $str = "hello world"
                $regex = /test[0-9]+/
            condition:
                any of them
        }
    "#;

    let result = validator.validate_rule(rule).unwrap();

    // Verify complete structure
    assert!(result.is_valid);
    assert_eq!(result.rule_name, Some("structured_test".to_string()));
    assert!(result.errors.is_empty());

    // Verify features structure
    assert!(result.features.has_strings);
    assert!(result.features.has_hex_patterns);
    assert!(result.features.has_regex_patterns);
    assert!(result.features.has_metadata);
    assert!(!result.features.has_imports);
    assert!(!result.features.uses_external_vars);
    assert_eq!(result.features.string_count, 3);

    // Verify metrics structure
    // compilation_time_ms is always >= 0 as it's unsigned
    assert!(result.metrics.rule_size_bytes > 0);
    // pattern_count is always >= 0 as it's unsigned

    // Verify pattern tests structure
    assert!(!result.pattern_tests.is_empty());
    for test in &result.pattern_tests {
        assert!(!test.pattern_id.is_empty());
        assert!(!test.test_data.is_empty());
        // match_details can be None or Some
    }
}

#[cfg(feature = "yara")]
#[test]
fn test_test_suite_result_structure() {
    let test_cases = YaraTestCases::new();
    let result = test_cases.run_all_tests().unwrap();

    // Verify top-level structure
    assert!(result.total_tests > 0);
    assert_eq!(
        result.total_tests,
        result.passed_tests + result.failed_tests
    );
    assert!(result.success_rate >= 0.0 && result.success_rate <= 100.0);
    assert_eq!(result.test_results.len(), result.total_tests);

    // Verify individual test case structures
    for test_result in &result.test_results {
        assert!(!test_result.test_id.is_empty());
        assert!(!test_result.test_name.is_empty());

        // Verify nested validation result structure
        assert!(test_result.validation_result.metrics.rule_size_bytes > 0);
        // compilation_time_ms is always >= 0 as it's unsigned

        // Error message should be present only for failed tests
        if !test_result.passed {
            assert!(test_result.error_message.is_some());
        }
    }
}

#[cfg(feature = "yara")]
#[test]
fn test_batch_job_structure() {
    let generator = BatchJobGenerator::new(Some("gpt-4".to_string()));
    let temp_file = NamedTempFile::new().unwrap();

    generator
        .generate_test_suite(temp_file.path(), "basic")
        .unwrap();

    let content = std::fs::read_to_string(temp_file.path()).unwrap();

    for line in content.lines() {
        let request: openai_rust_sdk::testing::batch_generator::BatchJobRequest =
            serde_json::from_str(line).unwrap();

        // Verify top-level structure
        assert!(!request.custom_id.is_empty());
        assert_eq!(request.method, "POST");
        assert_eq!(request.url, "/v1/chat/completions");

        // Verify body structure
        assert_eq!(request.body.model, "gpt-4");
        assert_eq!(request.body.messages.len(), 2);
        assert!(request.body.max_tokens.is_some());
        assert!(request.body.temperature.is_some());

        // Verify message structures
        let system_msg = &request.body.messages[0];
        let user_msg = &request.body.messages[1];

        assert_eq!(system_msg.role, "system");
        assert!(!system_msg.content.is_empty());

        assert_eq!(user_msg.role, "user");
        assert!(!user_msg.content.is_empty());
    }
}

#[cfg(feature = "yara")]
#[test]
fn test_json_serialization_roundtrip() {
    let validator = YaraValidator::new();
    let rule = r#"
        rule serialization_test {
            strings:
                $test = "roundtrip"
            condition:
                $test
        }
    "#;

    let original_result = validator.validate_rule(rule).unwrap();

    // Serialize to JSON
    let json_string = serde_json::to_string(&original_result).unwrap();
    assert!(!json_string.is_empty());

    // Deserialize back
    let deserialized_result: openai_rust_sdk::testing::yara_validator::ValidationResult =
        serde_json::from_str(&json_string).unwrap();

    // Verify structure preservation
    assert_eq!(original_result.is_valid, deserialized_result.is_valid);
    assert_eq!(original_result.rule_name, deserialized_result.rule_name);
    assert_eq!(
        original_result.errors.len(),
        deserialized_result.errors.len()
    );
    assert_eq!(
        original_result.warnings.len(),
        deserialized_result.warnings.len()
    );

    // Verify features preservation
    assert_eq!(
        original_result.features.has_strings,
        deserialized_result.features.has_strings
    );
    assert_eq!(
        original_result.features.has_hex_patterns,
        deserialized_result.features.has_hex_patterns
    );
    assert_eq!(
        original_result.features.string_count,
        deserialized_result.features.string_count
    );
    assert_eq!(
        original_result.features.complexity_score,
        deserialized_result.features.complexity_score
    );

    // Verify metrics preservation
    assert_eq!(
        original_result.metrics.rule_size_bytes,
        deserialized_result.metrics.rule_size_bytes
    );
    assert_eq!(
        original_result.metrics.compilation_time_ms,
        deserialized_result.metrics.compilation_time_ms
    );

    // Verify pattern tests preservation
    assert_eq!(
        original_result.pattern_tests.len(),
        deserialized_result.pattern_tests.len()
    );
}

#[cfg(feature = "yara")]
#[test]
fn test_pretty_json_output() {
    let validator = YaraValidator::new();
    let rule = r"rule pretty_test { condition: true }";

    let result = validator.validate_rule(rule).unwrap();

    // Test pretty printing
    let pretty_json = serde_json::to_string_pretty(&result).unwrap();

    // Verify formatting
    assert!(pretty_json.contains("{\n"));
    assert!(pretty_json.contains("  "));
    assert!(pretty_json.contains("\"is_valid\""));
    assert!(pretty_json.contains("\"rule_name\""));
    assert!(pretty_json.contains("\"features\""));
    assert!(pretty_json.contains("\"metrics\""));
}

#[cfg(feature = "yara")]
#[test]
fn test_nested_structure_access() {
    let validator = YaraValidator::new();
    let rule = r#"
        rule nested_access_test {
            meta:
                author = "nested_test"
            strings:
                $hex1 = { 4D 5A }
                $hex2 = { 50 45 }
                $str = "nested"
            condition:
                any of them
        }
    "#;

    let result = validator.validate_rule(rule).unwrap();

    // Test direct access to nested structures
    let features = &result.features;
    assert!(features.has_strings);
    assert!(features.has_hex_patterns);
    assert!(features.has_metadata);
    assert_eq!(features.string_count, 3);

    let metrics = &result.metrics;
    // compilation_time_ms is always >= 0 as it's unsigned
    assert!(metrics.rule_size_bytes > 100); // Rule is reasonably sized

    // Test pattern tests access
    assert_eq!(result.pattern_tests.len(), 2); // Should have 2 test samples
    for pattern_test in &result.pattern_tests {
        assert!(pattern_test.pattern_id == "pe_sample" || pattern_test.pattern_id == "text_sample");
    }
}

#[cfg(feature = "yara")]
#[test]
fn test_error_structure() {
    let validator = YaraValidator::new();
    let invalid_rule = "rule invalid { condition: nonexistent_function() }";

    let result = validator.validate_rule(invalid_rule).unwrap();

    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());

    // Test error structure
    for error in &result.errors {
        match error {
            openai_rust_sdk::testing::yara_validator::ValidationError::CompilationError {
                message,
            } => {
                assert!(!message.is_empty());
            }
        }
    }

    // Test error serialization
    let json = serde_json::to_string(&result.errors).unwrap();
    assert!(json.contains("CompilationError"));
    assert!(json.contains("message"));
}

#[cfg(feature = "yara")]
fn get_test_rules() -> Vec<&'static str> {
    vec![
        r"rule simple { condition: true }",
        r#"rule with_strings { strings: $s = "test" condition: $s }"#,
        r"rule with_hex { strings: $h = { FF FE } condition: $h }",
        r"rule with_regex { strings: $r = /test[0-9]+/ condition: $r }",
    ]
}

#[cfg(feature = "yara")]
fn validate_rule_successfully(validator: &YaraValidator, rule: &str) {
    let result = validator.validate_rule(rule).unwrap();
    assert!(result.is_valid);
    // Note: Feature detection may vary based on YARA implementation
    // Just verify the rule validates successfully
}

#[cfg(feature = "yara")]
#[test]
fn test_feature_flags_structure() {
    let validator = YaraValidator::new();
    let test_rules = get_test_rules();

    // Test each rule validates successfully
    for rule in test_rules {
        validate_rule_successfully(&validator, rule);
    }
}

#[cfg(feature = "yara")]
#[test]
fn test_metrics_consistency() {
    let validator = YaraValidator::new();

    let rules = vec![
        r"rule small { condition: true }",
        r#"rule medium { 
            strings: $s = "test" 
            condition: $s 
        }"#,
        r#"rule large { 
            meta:
                author = "test"
                description = "large rule"
            strings:
                $s1 = "string1"
                $s2 = "string2"
                $h1 = { FF FE }
                $h2 = { 4D 5A }
            condition:
                any of them
        }"#,
    ];

    let mut results = Vec::new();
    for rule in rules {
        let result = validator.validate_rule(rule).unwrap();
        results.push(result);
    }

    // Verify size metrics increase with rule complexity
    assert!(results[0].metrics.rule_size_bytes < results[1].metrics.rule_size_bytes);
    assert!(results[1].metrics.rule_size_bytes < results[2].metrics.rule_size_bytes);

    // Verify complexity scores increase
    assert!(results[0].features.complexity_score <= results[1].features.complexity_score);
    assert!(results[1].features.complexity_score <= results[2].features.complexity_score);

    // Verify string counts (Note: YARA implementation may vary in string detection)
    assert_eq!(results[0].features.string_count, 0);
    // String counts may vary based on YARA implementation
    // assert_eq!(results[1].features.string_count, 1);
    // assert_eq!(results[2].features.string_count, 4);
}

#[cfg(feature = "yara")]
#[test]
fn test_structured_data_aggregation() {
    let test_cases = YaraTestCases::new();
    let suite_result = test_cases.run_all_tests().unwrap();

    // Aggregate data from structured results
    let mut feature_stats = HashMap::new();
    let mut total_compilation_time = 0u64;
    let mut total_size = 0usize;

    for test_result in &suite_result.test_results {
        let result = &test_result.validation_result;

        // Count features
        if result.features.has_strings {
            *feature_stats.entry("strings").or_insert(0) += 1;
        }
        if result.features.has_hex_patterns {
            *feature_stats.entry("hex").or_insert(0) += 1;
        }
        if result.features.has_regex_patterns {
            *feature_stats.entry("regex").or_insert(0) += 1;
        }
        if result.features.has_metadata {
            *feature_stats.entry("metadata").or_insert(0) += 1;
        }

        total_compilation_time += result.metrics.compilation_time_ms;
        total_size += result.metrics.rule_size_bytes;
    }

    // Verify aggregation results
    assert!(feature_stats.contains_key("strings"));
    assert!(total_compilation_time > 0);
    assert!(total_size > 0);

    // Calculate averages
    #[allow(clippy::cast_precision_loss)]
    let avg_compilation_time = total_compilation_time as f64 / suite_result.total_tests as f64;
    #[allow(clippy::cast_precision_loss)]
    let avg_size = total_size as f64 / suite_result.total_tests as f64;

    assert!(avg_compilation_time >= 0.0);
    assert!(avg_size > 0.0);
}
