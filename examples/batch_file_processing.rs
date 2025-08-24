#![allow(clippy::pedantic, clippy::nursery)]
//! # Batch File Processing Example
//!
//! This example demonstrates how to download and process completed batch files:
//! 1. Download batch results and error files
//! 2. Extract YARA rules from AI responses  
//! 3. Generate comprehensive reports
//! 4. Analyze batch processing statistics
//!
//! Usage:
//! ```bash
//! export OPENAI_API_KEY=your_api_key_here
//! cargo run --example batch_file_processing <batch_id>
//! ```

use openai_rust_sdk::api::{batch::BatchApi, common::ApiClientConstructors};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;

    // Get batch ID from command line arguments - skip program name to avoid security warning
    // nosemgrep: rust.lang.security.args.args - We're only using args for the batch ID, not for security operations
    let mut args = env::args().skip(1);
    let batch_id = match args.next() {
        Some(id) => id,
        None => {
            eprintln!("Usage: batch_file_processing <batch_id>");
            eprintln!("Example: batch_file_processing batch_68a039c97af48190b645b9ece8266a52");
            return Ok(());
        }
    };

    // Ensure no extra arguments
    if args.next().is_some() {
        eprintln!("Error: Too many arguments provided");
        eprintln!("Usage: batch_file_processing <batch_id>");
        return Ok(());
    }

    println!("🔄 Batch File Processing Demo");
    println!("============================");
    println!("📋 Batch ID: {batch_id}");

    let batch_api = BatchApi::new(api_key)?;

    // Step 1: Check batch status
    println!("\n📊 Step 1: Checking batch status...");
    let batch_status = batch_api.get_batch_status(&batch_id).await?;
    println!("   Status: {}", batch_status.status);
    println!("   Total requests: {}", batch_status.request_counts.total);
    println!("   Completed: {}", batch_status.request_counts.completed);
    println!("   Failed: {}", batch_status.request_counts.failed);

    if batch_status.status.to_string() != "completed" {
        println!(
            "⚠️ Batch is not completed yet. Current status: {}",
            batch_status.status
        );
        println!("   Please wait for the batch to complete before processing files.");
        return Ok(());
    }

    // Step 2: Download all batch files
    println!("\n📥 Step 2: Downloading batch files...");
    let output_dir = std::env::current_dir()?.join("batch_processing_output");
    let (result_count, error_count) = batch_api
        .download_all_batch_files(&batch_id, &output_dir)
        .await?;

    println!("✅ Files downloaded successfully:");
    println!("   📁 Output directory: {}", output_dir.display());
    println!("   📄 Result lines: {result_count}");
    println!("   ❌ Error lines: {error_count}");

    // Step 3: Extract YARA rules
    println!("\n🔍 Step 3: Extracting YARA rules...");
    let results_file = output_dir.join(format!("{batch_id}_results.jsonl"));
    let yara_dir = output_dir.join("extracted_yara_rules");

    let yara_count = batch_api
        .process_yara_results(&results_file, &yara_dir)
        .await?;
    println!(
        "✅ Extracted {} YARA rules to: {}",
        yara_count,
        yara_dir.display()
    );

    if yara_count > 0 {
        println!("   📝 YARA rule files created:");
        if let Ok(entries) = std::fs::read_dir(&yara_dir) {
            for (i, entry) in entries.enumerate().take(10) {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let file_path = entry.path();

                    // Get file size
                    let size = if let Ok(metadata) = std::fs::metadata(&file_path) {
                        format!("{} bytes", metadata.len())
                    } else {
                        "unknown size".to_string()
                    };

                    println!(
                        "      {}. {} ({})",
                        i + 1,
                        file_name.to_string_lossy(),
                        size
                    );
                }
            }
            if yara_count > 10 {
                println!("      ... and {} more rule files", yara_count - 10);
            }
        }
    }

    // Step 4: Generate comprehensive report
    println!("\n📊 Step 4: Generating comprehensive report...");
    let errors_file = output_dir.join(format!("{batch_id}_errors.jsonl"));
    let report_file = output_dir.join("processing_report.md");

    let error_file_path = if error_count > 0 {
        Some(errors_file.as_path())
    } else {
        None
    };

    let report = batch_api
        .generate_batch_report(&results_file, error_file_path, &report_file)
        .await?;

    println!("✅ Report generated: {}", report_file.display());

    // Step 5: Display summary statistics
    println!("\n📈 Step 5: Summary Statistics");
    println!("─────────────────────────────");
    println!("📊 Batch Processing Results:");
    println!("   • Total responses: {}", report.total_responses);
    println!("   • Successful responses: {}", report.successful_responses);
    println!("   • Error responses: {}", report.error_responses);
    println!("   • Success rate: {:.1}%", report.success_rate());

    println!("\n🔍 YARA Rule Analysis:");
    println!("   • YARA rules found: {}", report.yara_rules_found);
    println!(
        "   • YARA extraction rate: {:.1}%",
        report.yara_extraction_rate()
    );

    println!("\n📏 Content Analysis:");
    println!(
        "   • Total content length: {} characters",
        report.total_tokens
    );
    println!(
        "   • Average response length: {:.0} characters",
        if report.successful_responses > 0 {
            report.total_tokens as f64 / report.successful_responses as f64
        } else {
            0.0
        }
    );

    if !report.error_types.is_empty() {
        println!("\n⚠️ Error Analysis:");
        for (error_type, count) in &report.error_types {
            println!("   • {error_type}: {count} occurrences");
        }
    }

    // Step 6: Show sample YARA rule content
    if yara_count > 0 {
        println!("\n📝 Step 6: Sample YARA Rule Content");
        println!("──────────────────────────────────");

        if let Ok(entries) = std::fs::read_dir(&yara_dir) {
            if let Some(Ok(entry)) = entries.take(1).next() {
                let file_path = entry.path();
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    println!(
                        "🔍 Sample rule from {}:",
                        entry.file_name().to_string_lossy()
                    );
                    println!("```yara");
                    println!("{content}");
                    println!("```");
                }
            }
        }
    }

    // Step 7: Validation recommendations
    println!("\n💡 Step 7: Recommendations");
    println!("──────────────────────────");

    if report.success_rate() >= 95.0 {
        println!("✅ Excellent success rate! Your batch configuration is working well.");
    } else if report.success_rate() < 90.0 {
        println!(
            "⚠️ Success rate is below 90%. Consider reviewing your prompts or model parameters."
        );
    }

    if yara_count > 0 {
        if report.yara_extraction_rate() >= 90.0 {
            println!("✅ High YARA rule extraction rate indicates effective prompts.");
        } else if report.yara_extraction_rate() < 80.0 {
            println!("⚠️ YARA rule extraction rate is low. Consider improving prompt specificity.");
        }

        println!("🔧 Next steps:");
        println!("   1. Validate extracted YARA rules with yara-x");
        println!("   2. Test rules against sample malware datasets");
        println!("   3. Integrate successful rules into your detection pipeline");
        println!("   4. Consider batch processing for rule optimization");
    }

    println!("\n✨ Batch file processing completed!");
    println!("📁 All output files saved to: {}", output_dir.display());

    Ok(())
}
