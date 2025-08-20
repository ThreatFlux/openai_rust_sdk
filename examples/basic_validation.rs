#![allow(clippy::pedantic, clippy::nursery)]
//! Basic YARA rule validation example
//!
//! This example demonstrates the fundamental usage of the YARA validator
//! for validating individual rules and understanding the validation results.

#[cfg(not(feature = "yara"))]
fn main() {
    println!("This example requires the 'yara' feature to be enabled.");
    println!("Run with: cargo run --example basic_validation --features yara");
}

#[cfg(feature = "yara")]
use anyhow::Result;
#[cfg(feature = "yara")]
use openai_rust_sdk::testing::YaraValidator;

#[cfg(feature = "yara")]
fn main() -> Result<()> {
    println!("Basic YARA Rule Validation Example");
    println!("==================================\n");

    // Create a new validator instance
    let validator = YaraValidator::new();
    println!(
        "✓ Created YARA validator with {} test samples",
        validator.test_samples().len()
    );

    // Example 1: Simple valid rule
    println!("\n1. Validating a simple rule:");
    let simple_rule = r#"
        rule HelloWorld {
            strings:
                $hello = "Hello World"
            condition:
                $hello
        }
    "#;

    let result = validator.validate_rule(simple_rule)?;
    print_validation_result("Simple Rule", &result);

    // Example 2: Complex rule with multiple features
    println!("\n2. Validating a complex rule:");
    let complex_rule = r#"
        rule ComplexExample {
            meta:
                author = "Example Author"
                description = "Complex rule demonstrating multiple features"
                version = "1.0"
            strings:
                $text_pattern = "example text"
                $hex_pattern = { 4D 5A 90 00 }
                $regex_pattern = /[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/
            condition:
                any of them
        }
    "#;

    let result = validator.validate_rule(complex_rule)?;
    print_validation_result("Complex Rule", &result);

    // Example 3: Invalid rule (demonstrates error handling)
    println!("\n3. Validating an invalid rule:");
    let invalid_rule = r#"
        rule InvalidExample {
            strings:
                $test = "test"
            condition:
                nonexistent_function()
        }
    "#;

    let result = validator.validate_rule(invalid_rule)?;
    print_validation_result("Invalid Rule", &result);

    // Example 4: Minimal rule
    println!("\n4. Validating a minimal rule:");
    let minimal_rule = "rule Minimal { condition: true }";

    let result = validator.validate_rule(minimal_rule)?;
    print_validation_result("Minimal Rule", &result);

    println!("\n==================================");
    println!("Basic validation examples completed!");

    Ok(())
}

#[cfg(feature = "yara")]
fn print_validation_result(
    name: &str,
    result: &openai_rust_sdk::testing::yara_validator::ValidationResult,
) {
    println!("{name}:");

    // Basic validation status
    if result.is_valid {
        println!("  ✓ Status: VALID");
    } else {
        println!("  ✗ Status: INVALID");
    }

    // Rule name
    if let Some(rule_name) = &result.rule_name {
        println!("  📝 Rule Name: {rule_name}");
    }

    // Errors
    if !result.errors.is_empty() {
        println!("  ❌ Errors:");
        for error in &result.errors {
            println!("     - {error}");
        }
    }

    // Warnings
    if !result.warnings.is_empty() {
        println!("  ⚠️  Warnings:");
        for warning in &result.warnings {
            println!("     - {warning}");
        }
    }

    // Features
    println!("  🔍 Features:");
    println!("     - Has strings: {}", result.features.has_strings);
    println!(
        "     - Has hex patterns: {}",
        result.features.has_hex_patterns
    );
    println!(
        "     - Has regex patterns: {}",
        result.features.has_regex_patterns
    );
    println!("     - Has metadata: {}", result.features.has_metadata);
    println!("     - String count: {}", result.features.string_count);
    println!(
        "     - Complexity score: {}/10",
        result.features.complexity_score
    );

    // Metrics
    println!("  📊 Metrics:");
    println!(
        "     - Compilation time: {}ms",
        result.metrics.compilation_time_ms
    );
    println!("     - Rule size: {} bytes", result.metrics.rule_size_bytes);

    // Pattern tests
    if !result.pattern_tests.is_empty() {
        println!("  🧪 Pattern Tests:");
        for test in &result.pattern_tests {
            let status = if test.matched { "✓" } else { "✗" };
            println!("     {} {}: {}", status, test.pattern_id, test.test_data);
            if let Some(details) = &test.match_details {
                println!("       Details: {details}");
            }
        }
    }
}
