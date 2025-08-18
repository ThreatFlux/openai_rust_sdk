//! Streaming validation demonstration
//!
//! This example shows how to process YARA rules in a streaming fashion,
//! useful for handling large batches of rules efficiently.

use anyhow::Result;
use openai_rust_sdk::testing::{BatchJobGenerator, YaraValidator};
use futures::stream::{self, StreamExt};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Streaming YARA Validation Demo");
    println!("==============================\n");

    // Demo 1: Sequential streaming validation
    await_sequential_streaming().await?;

    // Demo 2: Concurrent streaming validation
    await_concurrent_streaming().await?;

    // Demo 3: Batch job streaming simulation
    await_batch_job_streaming().await?;

    // Demo 4: Backpressure handling
    await_backpressure_demo().await?;

    println!("\n==============================");
    println!("Streaming demos completed!");

    Ok(())
}

async fn await_sequential_streaming() -> Result<()> {
    println!("1. Sequential Streaming Validation");
    println!("----------------------------------");

    let validator = YaraValidator::new();
    let rules = generate_sample_rules();

    println!("Processing {} rules sequentially...", rules.len());
    let start_time = Instant::now();

    for (i, (name, rule)) in rules.iter().enumerate() {
        println!("  Processing rule {}: {}", i + 1, name);

        let result = validator.validate_rule(rule)?;

        // Simulate processing time
        sleep(Duration::from_millis(100)).await;

        if result.is_valid {
            println!(
                "    ✓ Valid - Compilation: {}ms",
                result.metrics.compilation_time_ms
            );
        } else {
            println!("    ✗ Invalid - {} errors", result.errors.len());
        }
    }

    let total_time = start_time.elapsed();
    println!("Sequential processing completed in {total_time:?}\n");

    Ok(())
}

async fn await_concurrent_streaming() -> Result<()> {
    println!("2. Concurrent Streaming Validation");
    println!("----------------------------------");

    let rules = generate_sample_rules();
    println!("Processing {} rules concurrently...", rules.len());

    let start_time = Instant::now();

    // Create a stream of validation tasks
    let validation_stream = stream::iter(rules.into_iter().enumerate())
        .map(|(i, (name, rule))| async move {
            let validator = YaraValidator::new();
            println!("  Starting rule {}: {}", i + 1, name);

            // Simulate some async work
            sleep(Duration::from_millis(50)).await;

            let result = validator.validate_rule(&rule)?;

            Ok::<_, anyhow::Error>((i + 1, name, result))
        })
        .buffer_unordered(4); // Process up to 4 rules concurrently

    // Collect results
    let mut results = Vec::new();
    let mut stream = std::pin::pin!(validation_stream);

    while let Some(result) = stream.next().await {
        let (index, name, validation_result) = result?;

        if validation_result.is_valid {
            println!("    ✓ Rule {index}: {name} - Valid");
        } else {
            println!("    ✗ Rule {index}: {name} - Invalid");
        }

        results.push((name, validation_result));
    }

    let total_time = start_time.elapsed();
    println!("Concurrent processing completed in {total_time:?}");
    println!(
        "Valid rules: {}/{}\n",
        results.iter().filter(|(_, r)| r.is_valid).count(),
        results.len()
    );

    Ok(())
}

async fn await_batch_job_streaming() -> Result<()> {
    println!("3. Batch Job Streaming Simulation");
    println!("---------------------------------");

    let generator = BatchJobGenerator::new(Some("gpt-4".to_string()));
    let temp_file = tempfile::NamedTempFile::new()?;

    // Generate batch jobs
    generator.generate_test_suite(temp_file.path(), "comprehensive")?;

    let content = std::fs::read_to_string(temp_file.path())?;
    let lines: Vec<&str> = content.lines().collect();

    println!("Streaming {} batch job requests...", lines.len());

    // Process batch jobs as a stream
    let batch_stream = stream::iter(lines.into_iter().enumerate())
        .map(|(_i, line)| async move {
            // Parse the batch job request
            let request: openai_rust_sdk::testing::batch_generator::BatchJobRequest =
                serde_json::from_str(line)?;

            println!("  Processing batch job: {}", request.custom_id);

            // Simulate API call delay
            sleep(Duration::from_millis(200)).await;

            // Simulate response processing
            let simulated_response = format!(
                "rule Generated_{} {{ condition: true }}",
                request.custom_id.replace("comprehensive_", "")
            );

            // Validate the simulated response
            let validator = YaraValidator::new();
            let validation_result = validator.validate_rule(&simulated_response)?;

            Ok::<_, anyhow::Error>((request.custom_id, validation_result))
        })
        .buffer_unordered(3); // Limit concurrent API calls

    let mut processed = 0;
    let mut valid_responses = 0;
    let mut stream = std::pin::pin!(batch_stream);

    while let Some(result) = stream.next().await {
        let (custom_id, validation_result) = result?;
        processed += 1;

        if validation_result.is_valid {
            valid_responses += 1;
            println!("    ✓ {custom_id}: Generated valid rule");
        } else {
            println!("    ✗ {custom_id}: Generated invalid rule");
        }
    }

    println!("Batch job streaming completed: {valid_responses}/{processed} valid responses\n");

    Ok(())
}

async fn await_backpressure_demo() -> Result<()> {
    println!("4. Backpressure Handling Demo");
    println!("-----------------------------");

    let rules = generate_large_rule_set();
    println!(
        "Processing {} rules with backpressure control...",
        rules.len()
    );

    // Use a semaphore to limit concurrent validations
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(2));
    let start_time = Instant::now();

    let validation_stream = stream::iter(rules.into_iter().enumerate())
        .map(|(i, rule)| {
            let semaphore = semaphore.clone();
            async move {
                // Acquire permit (blocks if at limit)
                let _permit = semaphore.acquire().await.unwrap();

                println!("  Validating rule {} (permit acquired)", i + 1);

                let validator = YaraValidator::new();

                // Simulate heavy processing
                sleep(Duration::from_millis(300)).await;

                let result = validator.validate_rule(&rule)?;

                println!("    Rule {} completed", i + 1);

                Ok::<_, anyhow::Error>(result)
            }
        })
        .buffer_unordered(10); // Allow more tasks to be queued

    let mut completed = 0;
    let mut valid = 0;
    let mut stream = std::pin::pin!(validation_stream);

    while let Some(result) = stream.next().await {
        let validation_result = result?;
        completed += 1;

        if validation_result.is_valid {
            valid += 1;
        }
    }

    let total_time = start_time.elapsed();
    println!("Backpressure demo completed in {total_time:?}");
    println!("Results: {valid}/{completed} valid rules\n");

    Ok(())
}

fn generate_sample_rules() -> Vec<(String, String)> {
    vec![
        (
            "Hello World".to_string(),
            r#"rule HelloWorld { strings: $s = "Hello World" condition: $s }"#.to_string(),
        ),
        (
            "PE Header".to_string(),
            r"rule PEHeader { strings: $mz = { 4D 5A } condition: $mz at 0 }".to_string(),
        ),
        (
            "Email Pattern".to_string(),
            r"rule EmailPattern { 
                strings: 
                    $email = /[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/ 
                condition: 
                    $email 
            }"
            .to_string(),
        ),
        (
            "Complex Rule".to_string(),
            r#"rule ComplexRule {
                meta:
                    author = "demo"
                strings:
                    $str = "test"
                    $hex = { FF FE }
                condition:
                    any of them
            }"#
            .to_string(),
        ),
        (
            "Invalid Rule".to_string(),
            r"rule InvalidRule { condition: nonexistent_function() }".to_string(),
        ),
    ]
}

fn generate_large_rule_set() -> Vec<String> {
    (1..=8)
        .map(|i| {
            format!(
                r#"rule LargeSet_{i} {{
                    strings:
                        $pattern = "test_pattern_{i}"
                    condition:
                        $pattern
                }}"#
            )
        })
        .collect()
}
