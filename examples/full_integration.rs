#![allow(clippy::pedantic, clippy::nursery)]
#[cfg(not(feature = "yara"))]
fn main() {
    println!("This example requires the 'yara' feature to be enabled.");
    println!("Run with: cargo run --example full_integration --features yara");
}

#[cfg(feature = "yara")]
use openai_rust_sdk::testing::{batch_generator::BatchJobGenerator, yara_validator::YaraValidator};
#[cfg(feature = "yara")]
use std::fs;

#[cfg(feature = "yara")]
fn main() {
    print_header();

    let batch_file = generate_batch_job();

    if has_api_key() {
        run_production_flow(&batch_file);
    } else {
        run_demo_flow();
    }

    print_completion_message();
}

#[cfg(feature = "yara")]
/// Print the demo header
fn print_header() {
    println!("OpenAI Batch API + Yara-X Integration Test");
    println!("===========================================\n");
}

#[cfg(feature = "yara")]
/// Generate batch job for yara-x questions
fn generate_batch_job() -> std::path::PathBuf {
    println!("Step 1: Generating batch job with yara-x questions...");
    let generator = BatchJobGenerator::new(Some("gpt-5-nano".to_string()));
    let batch_file = std::path::Path::new("test_batch.jsonl");
    generator.generate_test_suite(batch_file, "basic").unwrap();
    println!("✓ Generated batch file: {}", batch_file.display());
    batch_file.to_path_buf()
}

#[cfg(feature = "yara")]
/// Check if API key is available
fn has_api_key() -> bool {
    std::env::var("OPENAI_API_KEY").is_ok()
}

#[cfg(feature = "yara")]
/// Run production flow with API key
fn run_production_flow(batch_file: &std::path::Path) {
    println!("\nStep 2: Simulating OpenAI batch response...");
    println!("✓ OPENAI_API_KEY found. In production, you would:");
    println!("  1. Upload the batch file using OpenAI Files API");
    println!("  2. Create batch job with the file ID");
    println!("  3. Wait for batch completion (up to 24 hours)");
    println!("  4. Download results and validate with yara-x");

    let batch_content = fs::read_to_string(batch_file).unwrap();
    println!(
        "\n✓ Generated {} batch requests for yara-x testing",
        batch_content.lines().count()
    );
}

#[cfg(feature = "yara")]
/// Run demo flow without API key
fn run_demo_flow() {
    println!("\nStep 2: Simulating OpenAI batch response...");
    println!("⚠️  OPENAI_API_KEY not set. Using sample rule for validation...");

    let sample_rule = get_sample_yara_rule();
    validate_sample_rule(sample_rule);
}

#[cfg(feature = "yara")]
/// Get sample YARA rule for demonstration
fn get_sample_yara_rule() -> &'static str {
    r#"
rule DetectPE_UPX {
    meta:
        description = "Detects PE files packed with UPX"
        author = "AI Generated"
    
    strings:
        $mz = { 4D 5A }  // MZ header
        $upx0 = "UPX0"
        $upx1 = "UPX1"
        $upx_sig = { 55 50 58 21 }  // UPX!
        
    condition:
        $mz at 0 and 
        (all of ($upx*) or $upx_sig)
}
"#
}

#[cfg(feature = "yara")]
/// Validate sample YARA rule
fn validate_sample_rule(sample_rule: &str) {
    println!("\nStep 3: Validating generated YARA rule with yara-x...");
    let validator = YaraValidator::new();
    let result = validator.validate_rule(sample_rule).unwrap();

    print_validation_results(&result);
    print_feature_analysis(&result);
    print_performance_metrics(&result);
}

#[cfg(feature = "yara")]
/// Print validation results
fn print_validation_results(result: &openai_rust_sdk::testing::yara_validator::ValidationResult) {
    println!("\nValidation Results:");
    println!("==================");
    println!(
        "✓ Validity: {}",
        if result.is_valid { "VALID" } else { "INVALID" }
    );

    if let Some(name) = &result.rule_name {
        println!("✓ Rule Name: {name}");
    }

    if !result.errors.is_empty() {
        println!("\nErrors:");
        for error in &result.errors {
            println!("  ✗ {error}");
        }
    }

    if !result.warnings.is_empty() {
        println!("\nWarnings:");
        for warning in &result.warnings {
            println!("  ⚠ {warning}");
        }
    }
}

#[cfg(feature = "yara")]
/// Print feature analysis
fn print_feature_analysis(result: &openai_rust_sdk::testing::yara_validator::ValidationResult) {
    println!("\nFeatures Detected:");
    println!("  • Hex Patterns: {}", result.features.has_hex_patterns);
    println!("  • String Patterns: {}", result.features.has_strings);
    println!(
        "  • Regular Expressions: {}",
        result.features.has_regex_patterns
    );
    println!("  • Metadata: {}", result.features.has_metadata);
    println!("  • Imports: {}", result.features.has_imports);
}

#[cfg(feature = "yara")]
/// Print performance metrics
fn print_performance_metrics(result: &openai_rust_sdk::testing::yara_validator::ValidationResult) {
    println!("\nPerformance Metrics:");
    println!(
        "  • Compilation Time: {}ms",
        result.metrics.compilation_time_ms
    );
    println!("  • Rule Size: {} bytes", result.metrics.rule_size_bytes);
    println!("  • Pattern Count: {}", result.metrics.pattern_count);
    println!("  • Complexity Score: {}", result.features.complexity_score);
}

#[cfg(feature = "yara")]
/// Print completion message
fn print_completion_message() {
    println!("\n===========================================");
    println!("Integration test completed successfully!");
}
