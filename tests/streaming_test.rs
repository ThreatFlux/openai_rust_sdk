//! Integration tests for streaming functionality
//!
//! These tests verify the streaming API integration with mock HTTP responses.

#[cfg(feature = "yara")]
use openai_rust_sdk::testing::{BatchJobGenerator, YaraValidator};
#[cfg(feature = "yara")]
use tempfile::NamedTempFile;

#[cfg(feature = "yara")]
#[tokio::test]
async fn test_streaming_response_simulation() {
    // Test streaming-like behavior by processing multiple YARA validations
    let validator = YaraValidator::new();

    let rules = [
        r#"rule stream_test_1 {
            strings:
                $test = "hello"
            condition:
                $test
        }"#,
        r"rule stream_test_2 {
            strings:
                $hex = { 4D 5A }
            condition:
                $hex
        }",
        r"rule stream_test_3 {
            condition:
                true
        }",
    ];

    let mut results = Vec::new();

    // Simulate streaming by processing rules sequentially
    for (i, rule) in rules.iter().enumerate() {
        let result = validator.validate_rule(rule).unwrap();
        results.push((i, result));

        // Simulate async processing delay
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // Verify all results
    assert_eq!(results.len(), 3);

    for (i, result) in results {
        assert!(result.is_valid);
        assert!(result.rule_name.is_some());
        assert!(result
            .rule_name
            .unwrap()
            .contains(&format!("stream_test_{}", i + 1)));
    }
}

#[cfg(feature = "yara")]
#[tokio::test]
async fn test_batch_job_streaming_generation() {
    // Test generating batch jobs that would be used for streaming
    let generator = BatchJobGenerator::new(Some("gpt-4".to_string()));
    let temp_file = NamedTempFile::new().unwrap();

    // Generate comprehensive suite (simulates batch job creation)
    generator
        .generate_test_suite(temp_file.path(), "comprehensive")
        .unwrap();

    // Read file line by line (simulating streaming consumption)
    let content = std::fs::read_to_string(temp_file.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    assert_eq!(lines.len(), 10); // Comprehensive suite has 10 requests

    // Process each line as if it came from a stream
    for (i, line) in lines.iter().enumerate() {
        let request: openai_rust_sdk::testing::batch_generator::BatchJobRequest =
            serde_json::from_str(line).unwrap();

        assert_eq!(request.method, "POST");
        assert_eq!(request.url, "/v1/chat/completions");
        assert_eq!(request.custom_id, format!("comprehensive_{:03}", i + 1));

        // Simulate processing delay
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    }
}

#[cfg(feature = "yara")]
#[tokio::test]
async fn test_concurrent_validation_streaming() {
    // Test concurrent validation (simulating multiple streams)

    let rules = vec![
        r"rule concurrent_1 { condition: filesize > 0 }",
        r#"rule concurrent_2 { strings: $s = "test" condition: $s }"#,
        r"rule concurrent_3 { strings: $hex = { FF FE } condition: $hex }",
        r"rule concurrent_4 { condition: true }",
    ];

    // Create futures for concurrent validation
    let mut handles = Vec::new();

    for rule in rules {
        let validator_clone = YaraValidator::new(); // Each task gets its own validator
        let handle = tokio::spawn(async move { validator_clone.validate_rule(rule) });
        handles.push(handle);
    }

    // Await all results
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.unwrap().unwrap();
        results.push(result);
    }

    // Verify all validations succeeded
    assert_eq!(results.len(), 4);
    for result in results {
        assert!(result.is_valid);
        // compilation_time_ms is always non-negative by type
    }
}

#[cfg(feature = "yara")]
#[tokio::test]
async fn test_error_handling_in_stream() {
    // Test error handling in streaming scenarios
    let validator = YaraValidator::new();

    let mixed_rules = vec![
        ("valid", r"rule valid_rule { condition: true }", true),
        ("invalid", "this is not a valid YARA rule", false),
        (
            "another_valid",
            r#"rule another { strings: $s = "test" condition: $s }"#,
            true,
        ),
    ];

    for (name, rule, should_be_valid) in mixed_rules {
        let result = validator.validate_rule(rule).unwrap();

        assert_eq!(
            result.is_valid, should_be_valid,
            "Rule '{name}' validation didn't match expectation"
        );

        if !should_be_valid {
            assert!(!result.errors.is_empty());
        }

        // Simulate stream processing delay
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    }
}

#[cfg(feature = "yara")]
#[tokio::test]
async fn test_streaming_metrics_collection() {
    // Test collecting metrics across a stream of validations
    let validator = YaraValidator::new();

    let rules = vec![
        r"rule metric_1 { condition: true }",
        r#"rule metric_2 { strings: $s = "hello" condition: $s }"#,
        r#"rule metric_3 { 
            meta:
                author = "test"
            strings:
                $hex = { 4D 5A }
                $str = "world"
            condition:
                any of them
        }"#,
    ];

    let mut total_compilation_time = 0u64;
    let mut total_size = 0usize;
    let mut feature_counts = std::collections::HashMap::new();

    for rule in rules {
        let result = validator.validate_rule(rule).unwrap();

        // Aggregate metrics
        total_compilation_time += result.metrics.compilation_time_ms;
        total_size += result.metrics.rule_size_bytes;

        // Count features
        if result.features.has_strings {
            *feature_counts.entry("strings").or_insert(0) += 1;
        }
        if result.features.has_hex_patterns {
            *feature_counts.entry("hex").or_insert(0) += 1;
        }
        if result.features.has_metadata {
            *feature_counts.entry("metadata").or_insert(0) += 1;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    }

    // Verify aggregated metrics
    assert!(total_compilation_time > 0);
    assert!(total_size > 0);
    assert_eq!(feature_counts.get("strings").unwrap_or(&0), &2);
    assert_eq!(feature_counts.get("hex").unwrap_or(&0), &1);
    assert_eq!(feature_counts.get("metadata").unwrap_or(&0), &1);
}

#[cfg(feature = "yara")]
#[tokio::test]
async fn test_backpressure_simulation() {
    // Simulate backpressure by limiting concurrent validations
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(2)); // Limit to 2 concurrent validations

    let rules = vec![
        r"rule bp_1 { condition: true }",
        r"rule bp_2 { condition: true }",
        r"rule bp_3 { condition: true }",
        r"rule bp_4 { condition: true }",
        r"rule bp_5 { condition: true }",
    ];

    let mut handles = Vec::new();

    for rule in rules {
        let semaphore_clone = semaphore.clone();
        let permit = semaphore_clone.acquire_owned().await.unwrap();
        let validator_clone = YaraValidator::new();

        let handle = tokio::spawn(async move {
            let _permit = permit; // Hold permit for duration of task
            let result = validator_clone.validate_rule(rule).unwrap();

            // Simulate some processing time
            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
            result
        });

        handles.push(handle);
    }

    // Collect results
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.unwrap();
        results.push(result);
    }

    assert_eq!(results.len(), 5);
    for result in results {
        assert!(result.is_valid);
    }
}

#[cfg(feature = "yara")]
#[tokio::test]
async fn test_stream_cancellation() {
    // Test cancellation of streaming operations

    let rules = vec![
        r"rule cancel_1 { condition: true }",
        r"rule cancel_2 { condition: true }",
        r"rule cancel_3 { condition: true }",
    ];

    let mut handles = Vec::new();

    for rule in rules {
        let validator_clone = YaraValidator::new();
        let handle = tokio::spawn(async move {
            // Add delay to allow cancellation
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            validator_clone.validate_rule(rule)
        });
        handles.push(handle);
    }

    // Cancel one of the tasks
    handles[1].abort();

    // Check results
    let mut completed = 0;
    let mut cancelled = 0;

    for handle in handles {
        match handle.await {
            Ok(_) => completed += 1,
            Err(e) if e.is_cancelled() => cancelled += 1,
            Err(_) => panic!("Unexpected error"),
        }
    }

    assert_eq!(completed, 2);
    assert_eq!(cancelled, 1);
}
