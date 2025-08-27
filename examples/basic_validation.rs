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
    print_header();
    let validator = create_validator();

    run_validation_examples(&validator)?;

    print_footer();
    Ok(())
}

#[cfg(feature = "yara")]
fn print_header() {
    println!("Basic YARA Rule Validation Example");
    println!("==================================\n");
}

#[cfg(feature = "yara")]
fn create_validator() -> YaraValidator {
    let validator = YaraValidator::new();
    println!(
        "✓ Created YARA validator with {} test samples",
        validator.test_samples().len()
    );
    validator
}

#[cfg(feature = "yara")]
fn run_validation_examples(validator: &YaraValidator) -> Result<()> {
    validate_simple_rule(validator)?;
    validate_complex_rule(validator)?;
    validate_invalid_rule(validator)?;
    validate_minimal_rule(validator)?;
    Ok(())
}

#[cfg(feature = "yara")]
fn validate_simple_rule(validator: &YaraValidator) -> Result<()> {
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
    Ok(())
}

#[cfg(feature = "yara")]
fn validate_complex_rule(validator: &YaraValidator) -> Result<()> {
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
    Ok(())
}

#[cfg(feature = "yara")]
fn validate_invalid_rule(validator: &YaraValidator) -> Result<()> {
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
    Ok(())
}

#[cfg(feature = "yara")]
fn validate_minimal_rule(validator: &YaraValidator) -> Result<()> {
    println!("\n4. Validating a minimal rule:");
    let minimal_rule = "rule Minimal { condition: true }";
    let result = validator.validate_rule(minimal_rule)?;
    print_validation_result("Minimal Rule", &result);
    Ok(())
}

#[cfg(feature = "yara")]
fn print_footer() {
    println!("\n==================================");
    println!("Basic validation examples completed!");
}

#[cfg(feature = "yara")]
fn print_validation_result(
    name: &str,
    result: &openai_rust_sdk::testing::yara_validator::ValidationResult,
) {
    println!("{name}:");

    // Basic validation status
    let status_icon = if result.is_valid { "✓" } else { "✗" };
    let status_text = if result.is_valid { "VALID" } else { "INVALID" };
    println!("  {status_icon} Status: {status_text}");

    // Rule name
    if let Some(rule_name) = &result.rule_name {
        println!("  📝 Rule Name: {rule_name}");
    }

    // Errors and warnings
    print_message_list("❌ Errors", &result.errors);
    print_warnings(&result.warnings);

    // Features
    println!("  🔍 Features:");
    print_feature_flags(&result.features);
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
    print_pattern_tests(&result.pattern_tests);
}

#[cfg(feature = "yara")]
fn print_message_list(
    title: &str,
    messages: &[openai_rust_sdk::testing::yara_validator::ValidationError],
) {
    if !messages.is_empty() {
        println!("  {title}:");
        for message in messages {
            println!("     - {message}");
        }
    }
}

#[cfg(feature = "yara")]
fn print_warnings(warnings: &[String]) {
    if !warnings.is_empty() {
        println!("  ⚠️  Warnings:");
        for warning in warnings {
            println!("     - {warning}");
        }
    }
}

#[cfg(feature = "yara")]
fn print_feature_flags(features: &openai_rust_sdk::testing::yara_validator::RuleFeatures) {
    let flags = [
        ("Has strings", features.has_strings),
        ("Has hex patterns", features.has_hex_patterns),
        ("Has regex patterns", features.has_regex_patterns),
        ("Has metadata", features.has_metadata),
    ];

    for (label, value) in flags {
        println!("     - {}: {}", label, value);
    }
}

#[cfg(feature = "yara")]
fn print_pattern_tests(tests: &[openai_rust_sdk::testing::yara_validator::PatternTestResult]) {
    if !tests.is_empty() {
        println!("  🧪 Pattern Tests:");
        for test in tests {
            let status = if test.matched { "✓" } else { "✗" };
            println!("     {} {}: {}", status, test.pattern_id, test.test_data);
            if let Some(details) = &test.match_details {
                println!("       Details: {details}");
            }
        }
    }
}
