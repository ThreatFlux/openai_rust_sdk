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
    println!("OpenAI Batch API + Yara-X Integration Test");
    println!("===========================================\n");

    // Step 1: Generate batch job for yara-x questions
    println!("Step 1: Generating batch job with yara-x questions...");
    let generator = BatchJobGenerator::new(Some("gpt-5-nano".to_string()));
    let batch_file = std::path::Path::new("test_batch.jsonl");
    generator.generate_test_suite(batch_file, "basic").unwrap();
    println!("✓ Generated batch file: {}", batch_file.display());

    // Step 2: Simulate OpenAI response
    println!("\nStep 2: Simulating OpenAI batch response...");

    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("⚠️  OPENAI_API_KEY not set. Using sample rule for validation...");

        // Simulate a response with a sample YARA rule
        let sample_rule = r#"
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
"#;

        // Step 3: Validate the generated YARA rule
        println!("\nStep 3: Validating generated YARA rule with yara-x...");
        let validator = YaraValidator::new();
        let result = validator.validate_rule(sample_rule).unwrap();

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

        println!("\nFeatures Detected:");
        println!("  • Hex Patterns: {}", result.features.has_hex_patterns);
        println!("  • String Patterns: {}", result.features.has_strings);
        println!(
            "  • Regular Expressions: {}",
            result.features.has_regex_patterns
        );
        println!("  • Metadata: {}", result.features.has_metadata);
        println!("  • Imports: {}", result.features.has_imports);

        println!("\nPerformance Metrics:");
        println!(
            "  • Compilation Time: {}ms",
            result.metrics.compilation_time_ms
        );
        println!("  • Rule Size: {} bytes", result.metrics.rule_size_bytes);
        println!("  • Pattern Count: {}", result.metrics.pattern_count);
        println!("  • Complexity Score: {}", result.features.complexity_score);
    } else {
        println!("✓ OPENAI_API_KEY found. In production, you would:");
        println!("  1. Upload the batch file using OpenAI Files API");
        println!("  2. Create batch job with the file ID");
        println!("  3. Wait for batch completion (up to 24 hours)");
        println!("  4. Download results and validate with yara-x");

        // Read the generated batch file
        let batch_content = fs::read_to_string(batch_file).unwrap();
        println!(
            "\n✓ Generated {} batch requests for yara-x testing",
            batch_content.lines().count()
        );
    }

    println!("\n===========================================");
    println!("Integration test completed successfully!");
}
