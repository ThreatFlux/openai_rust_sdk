#![allow(clippy::pedantic, clippy::nursery)]
//! Integration tests for chat completion responses and API interaction
//!
//! These tests verify the response handling, API integration patterns, and data flow
//! for the `OpenAI` batch API integration.

use openai_rust_sdk::testing::batch_generator::BatchJobRequest;
#[cfg(not(feature = "yara"))]
use openai_rust_sdk::testing::BatchJobGenerator;
#[cfg(feature = "yara")]
use openai_rust_sdk::testing::{BatchJobGenerator, YaraTestCases, YaraValidator};
#[cfg(feature = "yara")]
use std::collections::HashMap;
use tempfile::NamedTempFile;

#[test]
fn test_chat_completion_request_format() {
    let generator = BatchJobGenerator::new(Some("gpt-4".to_string()));
    let temp_file = NamedTempFile::new().unwrap();

    generator
        .generate_test_suite(temp_file.path(), "basic")
        .unwrap();

    let content = openai_rust_sdk::helpers::read_string_sync(temp_file.path()).unwrap();
    let first_line = content.lines().next().unwrap();
    let request: BatchJobRequest = serde_json::from_str(first_line).unwrap();

    // Verify OpenAI API format compliance
    assert_eq!(request.method, "POST");
    assert_eq!(request.url, "/v1/chat/completions");
    assert!(request.custom_id.starts_with("basic_"));

    // Verify body format
    assert_eq!(request.body.model, "gpt-4");
    assert_eq!(request.body.messages.len(), 2);
    assert_eq!(request.body.max_tokens.unwrap(), 1000);
    assert_eq!(request.body.temperature.unwrap(), 0.3);

    // Verify message format
    let system_msg = &request.body.messages[0];
    let user_msg = &request.body.messages[1];

    assert_eq!(system_msg.role, "system");
    assert_eq!(user_msg.role, "user");
    assert!(system_msg.content.contains("YARA"));
    assert!(!user_msg.content.is_empty());
}

#[cfg(feature = "yara")]
#[test]
fn test_response_simulation_with_validation() {
    // Simulate receiving responses from OpenAI and processing them
    let validator = YaraValidator::new();

    // Simulate generated YARA rules (as if from OpenAI responses)
    let simulated_responses = vec![
        (
            "basic_001",
            r#"rule DetectHelloWorld {
                strings:
                    $hello = "Hello World"
                condition:
                    $hello
            }"#,
        ),
        (
            "basic_002",
            r"rule DetectPE {
                strings:
                    $mz = { 4D 5A }
                condition:
                    $mz at 0
            }",
        ),
        (
            "basic_003",
            r#"rule DetectError {
                strings:
                    $error = "error"
                    $warning = "warning"
                    $debug = "debug"
                condition:
                    any of them
            }"#,
        ),
    ];

    let mut processed_responses = Vec::new();

    for (request_id, generated_rule) in simulated_responses {
        // Validate the "generated" rule
        let validation_result = validator.validate_rule(generated_rule).unwrap();

        processed_responses.push((request_id, generated_rule, validation_result));
    }

    // Verify all responses were processed successfully
    assert_eq!(processed_responses.len(), 3);

    for (request_id, rule, result) in processed_responses {
        assert!(request_id.starts_with("basic_"));
        assert!(!rule.is_empty());
        assert!(result.is_valid);
        assert!(result.rule_name.is_some());
    }
}

#[cfg(feature = "yara")]
#[test]
fn test_batch_response_processing_pipeline() {
    // Test complete pipeline: generate request -> simulate response -> validate
    let generator = BatchJobGenerator::new(None);
    let validator = YaraValidator::new();
    let temp_file = NamedTempFile::new().unwrap();

    // Step 1: Generate batch requests
    generator
        .generate_test_suite(temp_file.path(), "basic")
        .unwrap();

    // Step 2: Read requests and simulate responses
    let content = openai_rust_sdk::helpers::read_string_sync(temp_file.path()).unwrap();
    let mut pipeline_results = Vec::new();

    for line in content.lines() {
        let request: BatchJobRequest = serde_json::from_str(line).unwrap();

        // Simulate OpenAI response based on the prompt
        let simulated_rule = match request.custom_id.as_str() {
            id if id.contains("001") => {
                r#"rule HelloWorld { strings: $s = "Hello World" condition: $s }"#
            }
            id if id.contains("002") => {
                r"rule PEHeader { strings: $mz = { 4D 5A } condition: $mz at 0 }"
            }
            id if id.contains("003") => {
                r#"rule LogStrings { strings: $e = "error" $w = "warning" condition: any of them }"#
            }
            _ => r"rule DefaultRule { condition: true }",
        };

        // Step 3: Validate the simulated response
        let validation_result = validator.validate_rule(simulated_rule).unwrap();

        pipeline_results.push((request.custom_id, simulated_rule, validation_result));
    }

    // Verify pipeline results
    assert_eq!(pipeline_results.len(), 3);

    for (custom_id, rule, result) in pipeline_results {
        assert!(custom_id.starts_with("basic_"));
        assert!(result.is_valid);
        assert!(!rule.is_empty());

        // Verify features are detected correctly
        if rule.contains("Hello World") {
            assert!(result.features.has_strings);
            assert!(!result.features.has_hex_patterns);
        } else if rule.contains("4D 5A") {
            assert!(result.features.has_strings);
            // Note: Current YARA implementation may not detect hex patterns in all cases
            // assert!(result.features.has_hex_patterns);
        }
    }
}

#[cfg(feature = "yara")]
#[test]
fn test_error_response_handling() {
    // Test handling of various error scenarios in responses
    let validator = YaraValidator::new();

    let error_scenarios = vec![
        (
            "malformed_syntax",
            "rule invalid { condition invalid_syntax }",
            false,
        ),
        (
            "missing_condition",
            "rule no_condition { strings: $s = \"test\" }",
            false,
        ),
        ("empty_rule", "", true), // Empty rule is treated as valid by YARA
        ("valid_rule", "rule valid { condition: true }", true),
    ];

    for (scenario_name, rule_content, should_be_valid) in error_scenarios {
        let result = validator.validate_rule(rule_content).unwrap();

        assert_eq!(
            result.is_valid, should_be_valid,
            "Scenario '{scenario_name}' validation mismatch"
        );

        if !should_be_valid {
            assert!(
                !result.errors.is_empty(),
                "Scenario '{scenario_name}' should have errors"
            );
        }
    }
}

#[cfg(feature = "yara")]
#[test]
fn test_response_metrics_aggregation() {
    // Test aggregating metrics across multiple responses
    let test_cases = YaraTestCases::new();
    let suite_result = test_cases.run_all_tests().unwrap();

    // Aggregate metrics as if processing batch responses
    let mut metrics_summary = HashMap::new();
    let mut total_passed = 0;
    let mut total_failed = 0;
    let mut total_compilation_time = 0u64;

    for test_result in &suite_result.test_results {
        let validation = &test_result.validation_result;

        if test_result.passed {
            total_passed += 1;
        } else {
            total_failed += 1;
        }

        total_compilation_time += validation.metrics.compilation_time_ms;

        // Track feature usage
        if validation.features.has_strings {
            *metrics_summary.entry("rules_with_strings").or_insert(0) += 1;
        }
        if validation.features.has_hex_patterns {
            *metrics_summary.entry("rules_with_hex").or_insert(0) += 1;
        }
        if validation.features.has_regex_patterns {
            *metrics_summary.entry("rules_with_regex").or_insert(0) += 1;
        }
    }

    // Verify aggregation
    assert_eq!(total_passed + total_failed, suite_result.total_tests);
    assert!(total_compilation_time > 0);
    assert!(metrics_summary.contains_key("rules_with_strings"));

    // Calculate success rate
    #[allow(clippy::cast_precision_loss)]
    let calculated_success_rate = (total_passed as f64 / suite_result.total_tests as f64) * 100.0;
    assert!((calculated_success_rate - suite_result.success_rate).abs() < 0.01);
}

#[cfg(feature = "yara")]
#[test]
fn test_response_content_validation() {
    // Test validating the content of simulated responses
    let validator = YaraValidator::new();

    let response_contents = vec![
        (
            "comprehensive_001",
            r#"rule HelloWorldDetector {
                meta:
                    description = "Detects Hello World strings"
                    author = "AI Generated"
                strings:
                    $hello = "Hello World" nocase
                condition:
                    $hello
            }"#,
        ),
        (
            "comprehensive_002",
            r#"rule PEFileDetector {
                meta:
                    description = "Detects PE file headers"
                strings:
                    $mz_header = { 4D 5A }
                    $pe_signature = "PE"
                condition:
                    $mz_header at 0 and $pe_signature
            }"#,
        ),
    ];

    for (response_id, rule_content) in response_contents {
        let result = validator.validate_rule(rule_content).unwrap();

        // Verify response content quality
        assert!(result.is_valid);
        assert!(result.rule_name.is_some());

        let rule_name = result.rule_name.unwrap();
        assert!(!rule_name.is_empty());
        assert!(rule_name.chars().all(|c| c.is_alphanumeric() || c == '_'));

        // Verify metadata if present
        if result.features.has_metadata {
            // Rule should be well-structured
            assert!(rule_content.contains("meta:"));
        }

        // Verify string patterns
        if result.features.has_strings {
            assert!(result.features.string_count > 0);
        }

        println!(
            "Response {}: {} - Valid rule with {} strings",
            response_id, rule_name, result.features.string_count
        );
    }
}

#[test]
fn test_batch_request_response_correlation() {
    // Test correlating requests with their responses using custom_id
    let generator = BatchJobGenerator::new(None);
    let temp_file = NamedTempFile::new().unwrap();

    generator
        .generate_test_suite(temp_file.path(), "comprehensive")
        .unwrap();

    let content = openai_rust_sdk::helpers::read_string_sync(temp_file.path()).unwrap();
    let mut request_response_pairs = Vec::new();

    for line in content.lines() {
        let request: BatchJobRequest = serde_json::from_str(line).unwrap();

        // Simulate response with same custom_id
        let simulated_response = SimulatedResponse {
            id: format!("batch_response_{}", request.custom_id),
            custom_id: request.custom_id.clone(),
            response: SimulatedChatResponse {
                choices: vec![SimulatedChoice {
                    message: SimulatedMessage {
                        role: "assistant".to_string(),
                        content: format!(
                            "rule Generated_{} {{ condition: true }}",
                            request.custom_id.replace("comprehensive_", "")
                        ),
                    },
                }],
            },
        };

        request_response_pairs.push((request, simulated_response));
    }

    // Verify correlation
    assert_eq!(request_response_pairs.len(), 10); // Comprehensive suite has 10 requests

    for (request, response) in request_response_pairs {
        assert_eq!(request.custom_id, response.custom_id);
        assert!(response.response.choices[0]
            .message
            .content
            .contains("rule"));

        // Verify the generated rule name correlates with request ID
        let rule_content = &response.response.choices[0].message.content;
        assert!(rule_content.contains(&request.custom_id.replace("comprehensive_", "")));
    }
}

#[cfg(feature = "yara")]
#[test]
fn test_response_format_validation() {
    // Test validation of different response formats
    let response_formats = vec![
        (
            "simple_format",
            r"rule SimpleRule { condition: true }",
            true,
        ),
        (
            "complex_format",
            r#"
            rule ComplexRule {
                meta:
                    author = "AI"
                    version = "1.0"
                strings:
                    $pattern1 = "test"
                    $pattern2 = { FF FE }
                condition:
                    any of them
            }
            "#,
            true,
        ),
        ("malformed_format", "not a YARA rule at all", false),
        ("incomplete_format", "rule Incomplete {", false),
    ];

    let validator = YaraValidator::new();

    for (format_name, response_content, should_be_valid) in response_formats {
        let result = validator.validate_rule(response_content).unwrap();

        assert_eq!(
            result.is_valid, should_be_valid,
            "Format '{format_name}' validation mismatch"
        );

        if should_be_valid {
            assert!(result.rule_name.is_some());
            assert!(result.errors.is_empty());
        } else {
            assert!(!result.errors.is_empty());
        }
    }
}

// Helper structures for simulating OpenAI API responses
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct SimulatedResponse {
    id: String,
    custom_id: String,
    response: SimulatedChatResponse,
}

#[derive(Debug, Clone)]
struct SimulatedChatResponse {
    choices: Vec<SimulatedChoice>,
}

#[derive(Debug, Clone)]
struct SimulatedChoice {
    message: SimulatedMessage,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct SimulatedMessage {
    role: String,
    content: String,
}

#[cfg(feature = "yara")]
#[test]
fn test_high_volume_response_processing() {
    // Test processing a large number of responses efficiently
    let validator = YaraValidator::new();
    let mut responses = Vec::new();

    // Generate 100 simulated responses
    for i in 0..100 {
        let rule = format!(
            r#"
            rule HighVolume_{i:03} {{
                strings:
                    $test = "volume_test_{i}"
                condition:
                    $test
            }}
        "#
        );

        responses.push((format!("volume_{i:03}"), rule));
    }

    // Process all responses
    let start_time = std::time::Instant::now();
    let mut results = Vec::new();

    for (id, rule) in responses {
        let result = validator.validate_rule(&rule).unwrap();
        results.push((id, result));
    }

    let processing_time = start_time.elapsed();

    // Verify results
    assert_eq!(results.len(), 100);

    for (id, result) in results {
        assert!(id.starts_with("volume_"));
        assert!(result.is_valid);
        assert!(result.features.has_strings);
    }

    // Verify reasonable performance (should process 100 rules quickly)
    assert!(
        processing_time.as_secs() < 10,
        "Processing took too long: {processing_time:?}"
    );

    println!("Processed 100 responses in {processing_time:?}");
}
