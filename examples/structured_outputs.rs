#![allow(clippy::pedantic, clippy::nursery)]
//! Structured output examples for YARA validation
//!
//! This example demonstrates:
//! - Structured validation results
//! - JSON serialization of validation data
//! - Batch validation with structured output
//! - Feature analysis and reporting

#[cfg(not(feature = "yara"))]
#[tokio::main]
async fn main() {
    println!("This example requires the 'yara' feature to be enabled.");
    println!("Run with: cargo run --example structured_outputs --features yara");
}

#[cfg(feature = "yara")]
use openai_rust_sdk::testing::{YaraTestCases, YaraValidator};
#[cfg(feature = "yara")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "yara")]
use serde_json::json;

#[cfg(feature = "yara")]
#[derive(Debug, Serialize, Deserialize)]
struct ValidationReport {
    rule_name: String,
    is_valid: bool,
    compilation_time_ms: u64,
    feature_count: usize,
    errors: Vec<String>,
}

#[cfg(feature = "yara")]
#[derive(Debug, Serialize, Deserialize)]
struct BatchValidationSummary {
    total_rules: usize,
    valid_rules: usize,
    invalid_rules: usize,
    success_rate: f64,
    reports: Vec<ValidationReport>,
}

#[cfg(feature = "yara")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Structured YARA Validation Output Examples");
    println!("==========================================\n");

    // Example 1: Individual rule validation with structured output
    example_structured_validation().await?;

    // Example 2: Batch validation with summary report
    example_batch_structured_validation().await?;

    // Example 3: JSON export of validation results
    example_json_export().await?;

    println!("\n==========================================");
    println!("Structured output examples completed!");

    Ok(())
}

#[cfg(feature = "yara")]
async fn example_structured_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. Individual Rule Validation with Structured Output");
    println!("---------------------------------------------------");

    let validator = YaraValidator::new();

    let rules = vec![
        ("Simple Rule", r"rule simple { condition: true }"),
        (
            "String Rule",
            r#"rule with_string { strings: $s = "test" condition: $s }"#,
        ),
        (
            "Hex Rule",
            r"rule with_hex { strings: $h = { FF FE } condition: $h }",
        ),
        (
            "Invalid Rule ",
            r#"rule invalid { strings: $s = "test" condition: invalid_function() }"#,
        ),
    ];

    for (name, rule) in rules {
        let result = validator.validate_rule(rule)?;

        let report = ValidationReport {
            rule_name: result.rule_name.unwrap_or_else(|| "unnamed".to_string()),
            is_valid: result.is_valid,
            compilation_time_ms: result.metrics.compilation_time_ms,
            feature_count: result.features.string_count,
            errors: result.errors.iter().map(|e| e.to_string()).collect(),
        };

        // Output as pretty JSON
        let json_output = serde_json::to_string_pretty(&report)?;
        println!("Rule: {}", name);
        println!("{}", json_output);
        println!();
    }

    Ok(())
}

#[cfg(feature = "yara")]
async fn example_batch_structured_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. Batch Validation with Summary Report ");
    println!("---------------------------------------");

    let test_cases = YaraTestCases::new();
    let results = test_cases.run_all_tests()?;

    // Convert to structured format
    let reports: Vec<ValidationReport> = results
        .test_results
        .iter()
        .map(|test_result| ValidationReport {
            rule_name: test_result.test_name.clone(),
            is_valid: test_result.validation_result.is_valid,
            compilation_time_ms: test_result.validation_result.metrics.compilation_time_ms,
            feature_count: test_result.validation_result.features.string_count,
            errors: test_result
                .validation_result
                .errors
                .iter()
                .map(|e| e.to_string())
                .collect(),
        })
        .collect();

    let summary = BatchValidationSummary {
        total_rules: results.total_tests,
        valid_rules: results.passed_tests,
        invalid_rules: results.failed_tests,
        success_rate: results.success_rate,
        reports,
    };

    // Output summary as JSON
    let json_output = serde_json::to_string_pretty(&summary)?;
    println!("Batch Validation Summary:");
    println!("{}", json_output);

    Ok(())
}

#[cfg(feature = "yara")]
async fn example_json_export() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("3. JSON Export of Validation Results ");
    println!("------------------------------------");

    let validator = YaraValidator::new();
    let rule = r#"
        rule complex_example {
            meta:
                author = "example"
                description = "Complex rule for demonstration"
            strings:
                $text = "example"
                $hex = { 4D 5A }
                $regex = /test[0-9]+/
            condition:
                any of them
        }
    "#;

    let result = validator.validate_rule(rule)?;

    // Create comprehensive structured output
    let structured_result = json!({
        "validation": {
            "is_valid": result.is_valid,
            "rule_name": result.rule_name,
            "errors": result.errors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
            "warnings": result.warnings
        },
        "features": {
            "has_strings": result.features.has_strings,
            "has_hex_patterns": result.features.has_hex_patterns,
            "has_regex_patterns": result.features.has_regex_patterns,
            "has_metadata": result.features.has_metadata,
            "string_count": result.features.string_count,
            "complexity_score": result.features.complexity_score
        },
        "metrics": {
            "compilation_time_ms": result.metrics.compilation_time_ms,
            "rule_size_bytes": result.metrics.rule_size_bytes
        },
        "pattern_tests": result.pattern_tests.iter().map(|test| {
            json!({
                "pattern_id": test.pattern_id,
                "test_data": test.test_data,
                "matched": test.matched,
                "match_details": test.match_details
            })
        }).collect::<Vec<_>>()
    });

    println!("Comprehensive validation result as JSON:");
    println!("{}", serde_json::to_string_pretty(&structured_result)?);

    // Demonstrate filtering specific data
    let feature_summary = json!({
        "rule_complexity": result.features.complexity_score,
        "detected_features": {
            "strings": result.features.has_strings,
            "hex": result.features.has_hex_patterns,
            "regex": result.features.has_regex_patterns,
            "metadata": result.features.has_metadata
        },
        "performance": {
            "compilation_time_ms": result.metrics.compilation_time_ms
        }
    });
    println!();
    println!("Filtered feature summary:");
    let summary_json = serde_json::to_string_pretty(&feature_summary)?;
    println!("{}", summary_json);

    Ok(())
}
