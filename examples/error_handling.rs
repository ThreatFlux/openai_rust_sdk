#![allow(clippy::pedantic, clippy::nursery)]
//! Error handling patterns demonstration
//!
//! This example showcases comprehensive error handling patterns and best practices
//! when working with YARA rule validation and batch processing.

#[cfg(not(feature = "yara"))]
fn main() {
    println!("This example requires the 'yara' feature to be enabled.");
    println!("Run with: cargo run --example error_handling --features yara");
}

#[cfg(feature = "yara")]
use anyhow::{Context, Result};
#[cfg(feature = "yara")]
use openai_rust_sdk::testing::{BatchJobGenerator, YaraTestCases, YaraValidator};
#[cfg(feature = "yara")]
use std::collections::HashMap;

#[cfg(feature = "yara")]
fn main() -> Result<()> {
    println!("Error Handling Patterns Demo");
    println!("============================\n");

    // Demo 1: Basic validation error handling
    demo_validation_errors()?;

    // Demo 2: Batch processing error handling
    demo_batch_processing_errors()?;

    // Demo 3: File operation error handling
    demo_file_operation_errors()?;

    // Demo 4: Recovery strategies
    demo_recovery_strategies()?;

    // Demo 5: Error aggregation and reporting
    demo_error_aggregation()?;

    println!("\n============================");
    println!("Error handling demos completed!");

    Ok(())
}

#[cfg(feature = "yara")]
fn demo_validation_errors() -> Result<()> {
    println!("1. Validation Error Handling");
    println!("----------------------------");

    let validator = YaraValidator::new();
    let error_cases = vec![
        ("Syntax Error", "rule invalid { condition invalid_syntax }"),
        (
            "Missing Condition",
            r#"rule incomplete {
                strings:
                    $str = "test"
            }"#,
        ),
        (
            "Invalid Function",
            "rule bad_function { condition: nonexistent_function() }",
        ),
        ("Empty Rule", ""),
        ("Malformed Structure", "this is not a YARA rule at all"),
    ];

    for (error_type, rule) in error_cases {
        println!("Testing: {}", error_type);

        match validator.validate_rule(rule) {
            Ok(result) => {
                if result.is_valid {
                    println!("  ⚠️  Unexpected: Rule was marked as valid");
                } else {
                    println!("  ✓ Expected: Rule validation failed");
                    println!("    Errors ({}): ", result.errors.len());
                    for (i, error) in result.errors.iter().enumerate() {
                        println!("      {}: {}", i + 1, error);
                    }
                }
            }
            Err(e) => {
                println!("  ❌ Validation system error: {}", e);
                // In production, you might want to log this and continue
                // or implement retry logic
            }
        }
        println!();
    }

    Ok(())
}

#[cfg(feature = "yara")]
fn demo_batch_processing_errors() -> Result<()> {
    println!("2. Batch Processing Error Handling");
    println!("----------------------------------");

    let test_cases = YaraTestCases::new();

    // Simulate batch processing with error handling
    match test_cases.run_all_tests() {
        Ok(results) => {
            println!("✓ Test suite completed successfully");
            println!("  Total tests: {}", results.total_tests);
            println!("  Passed: {}", results.passed_tests);
            println!("  Failed: {}", results.failed_tests);
            println!("  Success rate: {:.1}%", results.success_rate);

            // Handle individual test failures
            if results.failed_tests > 0 {
                println!("\n  Failed test details:");
                for test_result in &results.test_results {
                    if !test_result.passed {
                        println!(
                            "    ❌ {}: {}",
                            test_result.test_id,
                            test_result
                                .error_message
                                .as_ref()
                                .unwrap_or(&"Unknown error".to_string())
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Test suite failed to execute: {}", e);

            // Implement fallback strategy
            println!("  Attempting individual test validation...");
            attempt_individual_validation()?;
        }
    }

    Ok(())
}

#[cfg(feature = "yara")]
fn demo_file_operation_errors() -> Result<()> {
    println!("\n3. File Operation Error Handling");
    println!("--------------------------------");

    let generator = BatchJobGenerator::new(None);
    execute_file_operation_tests(&generator)
}

#[cfg(feature = "yara")]
fn execute_file_operation_tests(generator: &BatchJobGenerator) -> Result<()> {
    test_valid_file_operations(generator)?;
    test_invalid_file_operations(generator)?;
    Ok(())
}

#[cfg(feature = "yara")]
fn test_valid_file_operations(generator: &BatchJobGenerator) -> Result<()> {
    println!("Testing valid file operations...");
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path().join("test_batch_valid.jsonl");

    match generator.generate_test_suite(&temp_path, "basic") {
        Ok(()) => {
            println!("  ✓ Successfully generated batch file");
            cleanup_temp_file(&temp_path);
        }
        Err(e) => {
            println!("  ❌ Failed to generate batch file: {}", e);
        }
    }

    Ok(())
}

#[cfg(feature = "yara")]
fn cleanup_temp_file(temp_path: &std::path::Path) {
    if let Err(e) = std::fs::remove_file(temp_path) {
        println!("  ⚠️  Warning: Could not clean up file: {}", e);
    }
}

#[cfg(feature = "yara")]
fn test_invalid_file_operations(generator: &BatchJobGenerator) -> Result<()> {
    println!("\nTesting invalid file operations...");
    let invalid_scenarios = create_invalid_file_scenarios();

    for (scenario, path) in invalid_scenarios {
        test_invalid_scenario(generator, scenario, path);
    }

    Ok(())
}

#[cfg(feature = "yara")]
fn create_invalid_file_scenarios() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Non-existent directory",
            "/non/existent/directory/batch.jsonl",
        ),
        (
            "Permission denied (simulated)",
            "/root/batch.jsonl", // Assuming no root permissions
        ),
    ]
}

#[cfg(feature = "yara")]
fn test_invalid_scenario(generator: &BatchJobGenerator, scenario: &str, path: &str) {
    println!("  Scenario: {}", scenario);

    match generator.generate_test_suite(std::path::Path::new(path), "basic") {
        Ok(()) => {
            println!("    ⚠️  Unexpected success");
        }
        Err(e) => {
            println!("    ✓ Expected error: {}", e);
            attempt_fallback_recovery(generator);
        }
    }
}

#[cfg(feature = "yara")]
fn attempt_fallback_recovery(generator: &BatchJobGenerator) {
    println!("    Attempting fallback to temp directory...");

    let fallback_dir = tempfile::tempdir().expect("Failed to create fallback temp directory");
    let fallback_path = fallback_dir.path().join("fallback_batch.jsonl");

    match generator.generate_test_suite(&fallback_path, "basic") {
        Ok(()) => {
            println!("    ✓ Fallback successful");
            let _ = std::fs::remove_file(fallback_path);
        }
        Err(fallback_err) => {
            println!("    ❌ Fallback also failed: {}", fallback_err);
        }
    }
}

#[cfg(feature = "yara")]
fn demo_recovery_strategies() -> Result<()> {
    println!("\n4. Recovery Strategies");
    println!("---------------------");

    let validator = YaraValidator::new();
    let mixed_rules = create_mixed_test_rules();

    let (results, error_count) = process_rules_with_recovery(&validator, mixed_rules)?;
    print_recovery_summary(&results, error_count);

    Ok(())
}

#[cfg(feature = "yara")]
fn create_mixed_test_rules() -> Vec<(&'static str, &'static str, bool)> {
    vec![
        ("Valid Rule 1", "rule valid1 { condition: true }", true),
        (
            "Invalid Rule",
            "rule invalid { condition: bad_function() }",
            false,
        ),
        (
            "Valid Rule 2",
            r#"rule valid2 { strings: $s = "test" condition: $s }"#,
            true,
        ),
        ("Another Invalid", "malformed rule syntax", false),
        (
            "Valid Rule 3",
            "rule valid3 { condition: filesize > 0 }",
            true,
        ),
    ]
}

#[cfg(feature = "yara")]
fn process_rules_with_recovery<'a>(
    validator: &'a YaraValidator,
    mixed_rules: Vec<(&'a str, &'a str, bool)>,
) -> Result<(
    Vec<(
        &'a str,
        openai_rust_sdk::testing::yara_validator::ValidationResult,
    )>,
    usize,
)> {
    let mut results = Vec::new();
    let mut error_count = 0;
    let max_errors = 3;

    println!("Processing rules with error recovery...");

    for (name, rule, _expected) in mixed_rules {
        println!("  Processing: {}", name);

        match validator.validate_rule(rule) {
            Ok(result) => {
                if result.is_valid {
                    println!("    ✓ Valid");
                    results.push((name, result));
                } else {
                    error_count += 1;
                    handle_validation_error(&result, error_count);

                    if error_count >= max_errors {
                        implement_error_recovery();
                        break;
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                handle_system_error(&e, error_count, max_errors)?;
            }
        }
    }

    Ok((results, error_count))
}

#[cfg(feature = "yara")]
fn handle_validation_error(
    result: &openai_rust_sdk::testing::yara_validator::ValidationResult,
    error_count: usize,
) {
    println!("    ❌ Invalid (error count: {})", error_count);
    for error in &result.errors {
        println!("      Error: {}", error);
    }
}

#[cfg(feature = "yara")]
fn implement_error_recovery() {
    println!("    🛑 Error threshold reached, implementing recovery...");
    println!("    Switching to basic validation mode...");
    // In a real scenario, you might switch validators or modes
}

#[cfg(feature = "yara")]
fn handle_system_error(e: &anyhow::Error, error_count: usize, max_errors: usize) -> Result<()> {
    println!("    ❌ System error: {}", e);
    if error_count >= max_errors {
        return Err(anyhow::anyhow!(e.to_string())).context("Too many system errors, aborting");
    }
    Ok(())
}

#[cfg(feature = "yara")]
fn print_recovery_summary(
    results: &[(
        &str,
        openai_rust_sdk::testing::yara_validator::ValidationResult,
    )],
    error_count: usize,
) {
    println!(
        "Recovery demo completed. Processed {} valid rules with {} errors",
        results.len(),
        error_count
    );
}

#[cfg(feature = "yara")]
fn demo_error_aggregation() -> Result<()> {
    println!("\n5. Error Aggregation and Reporting");
    println!("----------------------------------");

    let validator = YaraValidator::new();
    let mut error_stats = ErrorStatistics::new();

    // Process a batch of rules and aggregate errors
    let test_rules = vec![
        ("syntax_error_1", "rule bad1 { condition invalid }"),
        ("syntax_error_2", "rule bad2 { condition also_invalid }"),
        (
            "missing_condition",
            "rule incomplete { strings: $s = \"test\" }",
        ),
        ("valid_rule", "rule good { condition: true }"),
        ("empty_rule", ""),
        ("function_error", "rule func_err { condition: bad_func() }"),
    ];

    for (rule_id, rule) in test_rules {
        match validator.validate_rule(rule) {
            Ok(result) => {
                if result.is_valid {
                    error_stats.record_success(rule_id);
                } else {
                    for error in &result.errors {
                        error_stats.record_error(rule_id, error);
                    }
                }
            }
            Err(e) => {
                error_stats.record_system_error(rule_id, &e);
            }
        }
    }

    // Generate error report
    error_stats.print_report();

    Ok(())
}

#[cfg(feature = "yara")]
fn attempt_individual_validation() -> Result<()> {
    println!("  Attempting basic individual validations...");
    let validator = YaraValidator::new();

    let basic_rules = [
        "rule test1 { condition: true }",
        "rule test2 { strings: $s = \"test\" condition: $s }",
    ];

    for (i, rule) in basic_rules.iter().enumerate() {
        match validator.validate_rule(rule) {
            Ok(result) => {
                if result.is_valid {
                    println!("    ✓ Individual test {} passed", i + 1);
                } else {
                    println!("    ❌ Individual test {} failed", i + 1);
                }
            }
            Err(e) => {
                println!("    ❌ Individual test {} error: {}", i + 1, e);
            }
        }
    }

    Ok(())
}

// Helper struct for error aggregation
#[cfg(feature = "yara")]
struct ErrorStatistics {
    success_count: usize,
    validation_errors: HashMap<String, usize>,
    system_errors: HashMap<String, usize>,
    rule_errors: Vec<(String, String)>,
}

#[cfg(feature = "yara")]
impl ErrorStatistics {
    fn new() -> Self {
        Self {
            success_count: 0,
            validation_errors: HashMap::new(),
            system_errors: HashMap::new(),
            rule_errors: Vec::new(),
        }
    }

    fn record_success(&mut self, _rule_id: &str) {
        self.success_count += 1;
    }

    fn record_error(
        &mut self,
        rule_id: &str,
        error: &openai_rust_sdk::testing::yara_validator::ValidationError,
    ) {
        let error_type = match error {
            openai_rust_sdk::testing::yara_validator::ValidationError::CompilationError {
                ..
            } => "CompilationError",
        };

        *self
            .validation_errors
            .entry(error_type.to_string())
            .or_insert(0) += 1;
        self.rule_errors
            .push((rule_id.to_string(), error.to_string()));
    }

    fn record_system_error(&mut self, rule_id: &str, error: &anyhow::Error) {
        let error_type = "SystemError";
        *self
            .system_errors
            .entry(error_type.to_string())
            .or_insert(0) += 1;
        self.rule_errors
            .push((rule_id.to_string(), error.to_string()));
    }

    fn print_report(&self) {
        println!("\nError Report Summary:");
        println!("  Successful validations: {}", self.success_count);
        println!("  Total errors: {}", self.rule_errors.len());

        if !self.validation_errors.is_empty() {
            println!("\n  Validation error types:");
            for (error_type, count) in &self.validation_errors {
                println!("    {}: {} occurrences", error_type, count);
            }
        }

        if !self.system_errors.is_empty() {
            println!("\n  System error types:");
            for (error_type, count) in &self.system_errors {
                println!("    {}: {} occurrences", error_type, count);
            }
        }

        if !self.rule_errors.is_empty() {
            println!("\n  Detailed errors:");
            for (rule_id, error) in &self.rule_errors {
                println!("    {}: {}", rule_id, error);
            }
        }

        let total_processed = self.success_count + self.rule_errors.len();
        if total_processed > 0 {
            let success_rate = (self.success_count as f64 / total_processed as f64) * 100.0;
            println!("\n  Overall success rate: {:.1}%", success_rate);
        }
    }
}
