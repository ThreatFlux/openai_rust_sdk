#![allow(clippy::pedantic, clippy::nursery)]
//! # OpenAI Batch API Processing Demo
//!
//! This example demonstrates the complete Batch API workflow:
//! 1. Generate a batch file with YARA validation prompts
//! 2. Upload the file to OpenAI
//! 3. Create and submit a batch job
//! 4. Monitor the batch status until completion
//! 5. Retrieve and display the results
//!
//! Usage:
//! ```bash
//! export OPENAI_API_KEY=your_api_key_here
//! cargo run --example batch_processing_demo
//! ```

use openai_rust_sdk::{
    api::{
        batch::{BatchApi, BatchStatus},
        common::ApiClientConstructors,
    },
    testing::BatchJobGenerator,
};
use std::env;
use tokio::time::{sleep, Duration};

/// Initialize the batch processing environment and API
fn initialize_batch_environment(
) -> Result<(String, BatchJobGenerator, std::path::PathBuf), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;

    println!("🚀 OpenAI Batch Processing Demo");
    println!("===============================");

    let batch_generator = BatchJobGenerator::new(Some("gpt-4o-mini".to_string()));
    let temp_dir = tempfile::tempdir()?;
    let batch_file = temp_dir.path().join("yara_batch_demo.jsonl");

    Ok((api_key, batch_generator, batch_file))
}

/// Generate and upload the batch file to OpenAI
async fn generate_and_upload_batch_file(
    batch_generator: &BatchJobGenerator,
    batch_file: &std::path::Path,
    batch_api: &BatchApi,
) -> Result<openai_rust_sdk::api::batch::FileUploadResponse, Box<dyn std::error::Error>> {
    println!("\n📝 Step 1: Generating YARA validation batch file...");
    batch_generator.generate_test_suite(batch_file, "malware")?;
    println!("✅ Generated batch file: {}", batch_file.display());

    println!("\n📤 Step 2: Uploading batch file to OpenAI...");
    let file_upload = batch_api.upload_batch_file(batch_file).await?;
    println!("✅ File uploaded successfully:");
    println!("   File ID: {}", file_upload.id);
    println!("   File size: {} bytes", file_upload.bytes);

    Ok(file_upload)
}

/// Create and submit the batch job
async fn create_batch_job(
    batch_api: &BatchApi,
    file_id: &str,
) -> Result<openai_rust_sdk::api::batch::Batch, Box<dyn std::error::Error>> {
    println!("\n🚀 Step 3: Creating batch job...");
    let batch = batch_api
        .create_batch(file_id, "/v1/chat/completions")
        .await?;
    println!("✅ Batch created successfully:");
    println!("   Batch ID: {}", batch.id);
    println!("   Status: {}", batch.status);
    println!("   Total requests: {}", batch.request_counts.total);

    let expires_at = format!("Unix timestamp: {}", batch.expires_at);
    println!("   Expires at: {expires_at}");

    Ok(batch)
}

/// Process completed batch results
async fn process_completed_batch(
    batch_api: &BatchApi,
    batch_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎉 Batch completed successfully!");
    println!("\n📥 Step 5: Downloading and saving batch results...");

    let output_dir = std::env::current_dir()?.join("batch_output");
    let (result_count, error_count) = batch_api
        .download_all_batch_files(batch_id, &output_dir)
        .await?;

    println!("✅ Files downloaded successfully:");
    println!("   📁 Output directory: {}", output_dir.display());
    println!("   📄 Result lines: {result_count}");
    println!("   ❌ Error lines: {error_count}");

    extract_yara_rules_and_generate_report(batch_api, batch_id, &output_dir, error_count).await?;
    Ok(())
}

/// Extract YARA rules and generate comprehensive report
async fn extract_yara_rules_and_generate_report(
    batch_api: &BatchApi,
    batch_id: &str,
    output_dir: &std::path::Path,
    error_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 Step 6: Processing YARA rules and generating report...");

    let results_file = output_dir.join(format!("{batch_id}_results.jsonl"));
    let errors_file = output_dir.join(format!("{batch_id}_errors.jsonl"));
    let yara_dir = output_dir.join("yara_rules");
    let report_file = output_dir.join("batch_report.md");

    // Extract YARA rules
    let yara_count = batch_api
        .process_yara_results(&results_file, &yara_dir)
        .await?;
    println!(
        "   🔍 Extracted {} YARA rules to: {}",
        yara_count,
        yara_dir.display()
    );

    // Generate comprehensive report
    let error_file_path = if error_count > 0 {
        Some(errors_file.as_path())
    } else {
        None
    };
    let report = batch_api
        .generate_batch_report(&results_file, error_file_path, &report_file)
        .await?;

    display_batch_statistics(&report, yara_count);
    display_yara_files(&yara_dir, yara_count)?;

    Ok(())
}

/// Display batch processing statistics
fn display_batch_statistics(report: &openai_rust_sdk::api::batch::BatchReport, _yara_count: usize) {
    println!("   📊 Generated comprehensive report: batch_report.md");
    println!("\n📈 Quick Statistics:");
    println!("   ✅ Total responses: {}", report.total_responses);
    println!(
        "   ✅ Successful responses: {}",
        report.successful_responses
    );
    println!("   ❌ Error responses: {}", report.error_responses);
    println!("   🎯 Success rate: {:.1}%", report.success_rate());
    println!("   🔍 YARA rules found: {}", report.yara_rules_found);
    println!(
        "   📏 Average response length: {:.0} characters",
        if report.successful_responses > 0 {
            report.total_tokens as f64 / report.successful_responses as f64
        } else {
            0.0
        }
    );
}

/// Display information about created YARA files
fn display_yara_files(
    yara_dir: &std::path::Path,
    yara_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if yara_count > 0 {
        println!("\n🎯 YARA Rule Files Created:");
        if let Ok(entries) = std::fs::read_dir(yara_dir) {
            for (i, entry) in entries.enumerate().take(5) {
                if let Ok(entry) = entry {
                    println!("   {}. {}", i + 1, entry.file_name().to_string_lossy());
                }
            }
            if yara_count > 5 {
                println!("   ... and {} more rule files", yara_count - 5);
            }
        }
    }
    Ok(())
}

/// Handle different batch status outcomes
async fn handle_batch_status(
    status: &openai_rust_sdk::api::batch::Batch,
    batch_api: &BatchApi,
    batch_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    match status.status {
        BatchStatus::Completed => {
            process_completed_batch(batch_api, batch_id).await?;
            Ok(true) // Signal completion
        }
        BatchStatus::Failed => {
            println!("❌ Batch failed!");
            if let Ok(Some(errors)) = batch_api.get_batch_errors(batch_id).await {
                println!("Error details: {errors}");
            }
            Ok(true) // Signal completion
        }
        BatchStatus::Expired => {
            println!("⏰ Batch expired!");
            Ok(true) // Signal completion
        }
        BatchStatus::Cancelled => {
            println!("🚫 Batch was cancelled!");
            Ok(true) // Signal completion
        }
        _ => Ok(false), // Continue polling
    }
}

/// Monitor batch progress with polling
async fn monitor_batch_progress(
    batch_api: &BatchApi,
    batch_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⏳ Step 4: Monitoring batch progress...");
    println!("   Polling every 10 seconds...");

    let mut poll_count = 0;
    const MAX_POLLS: u32 = 30;

    loop {
        poll_count += 1;
        let status = batch_api.get_batch_status(batch_id).await?;

        println!(
            "   Poll #{}: Status = {}, Completed = {}, Failed = {}",
            poll_count,
            status.status,
            status.request_counts.completed,
            status.request_counts.failed
        );

        if handle_batch_status(&status, batch_api, batch_id).await? {
            break; // Processing completed or terminated
        }

        // Check polling timeout
        if poll_count >= MAX_POLLS {
            println!("⏰ Stopping polling after {MAX_POLLS} attempts (5 minutes)");
            println!(
                "   The batch is still processing. Current status: {}",
                status.status
            );
            println!("   You can continue monitoring with: get_batch_status(\"{batch_id}\")");
            break;
        }

        // Wait before next poll
        sleep(Duration::from_secs(10)).await;
    }

    Ok(())
}

/// Display final completion message and key takeaways
fn display_completion_summary() {
    println!("\n✨ Batch processing demo completed!");
    println!("💡 Key takeaways:");
    println!("   • Batch API offers 50% cost savings vs real-time API");
    println!("   • Higher rate limits for bulk processing");
    println!("   • 24-hour completion guarantee");
    println!("   • Automatic file download and processing");
    println!("   • YARA rule extraction and validation");
    println!("   • Comprehensive reporting and analytics");
    println!("   • Perfect for large-scale cybersecurity workflows");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (api_key, batch_generator, batch_file) = initialize_batch_environment()?;
    let batch_api = BatchApi::new(api_key)?;

    let file_upload =
        generate_and_upload_batch_file(&batch_generator, &batch_file, &batch_api).await?;
    let batch = create_batch_job(&batch_api, &file_upload.id).await?;

    monitor_batch_progress(&batch_api, &batch.id).await?;

    // Clean up
    let _ = std::fs::remove_file(&batch_file);

    display_completion_summary();
    Ok(())
}
