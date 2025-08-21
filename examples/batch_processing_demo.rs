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
    api::batch::{BatchApi, BatchStatus},
    testing::BatchJobGenerator,
};
use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;

    println!("🚀 OpenAI Batch Processing Demo");
    println!("===============================");

    // Step 1: Generate batch file
    println!("\n📝 Step 1: Generating YARA validation batch file...");
    let batch_generator = BatchJobGenerator::new(Some("gpt-4o-mini".to_string()));
    let temp_dir = tempfile::tempdir()?;
    let batch_file = temp_dir.path().join("yara_batch_demo.jsonl");

    batch_generator.generate_test_suite(&batch_file, "malware")?;
    println!("✅ Generated batch file: {}", batch_file.display());

    // Step 2: Upload file to OpenAI
    println!("\n📤 Step 2: Uploading batch file to OpenAI...");
    let batch_api = BatchApi::new(api_key)?;
    let file_upload = batch_api.upload_batch_file(&batch_file).await?;
    println!("✅ File uploaded successfully:");
    println!("   File ID: {}", file_upload.id);
    println!("   File size: {} bytes", file_upload.bytes);

    // Step 3: Create batch job
    println!("\n🚀 Step 3: Creating batch job...");
    let batch = batch_api
        .create_batch(&file_upload.id, "/v1/chat/completions")
        .await?;
    println!("✅ Batch created successfully:");
    println!("   Batch ID: {}", batch.id);
    println!("   Status: {}", batch.status);
    println!("   Total requests: {}", batch.request_counts.total);

    let expires_at = format!("Unix timestamp: {}", batch.expires_at);
    println!("   Expires at: {expires_at}");

    // Step 4: Monitor batch progress
    println!("\n⏳ Step 4: Monitoring batch progress...");
    println!("   Polling every 10 seconds...");

    let mut poll_count = 0;
    let batch_id = batch.id.clone();

    loop {
        poll_count += 1;

        let status = batch_api.get_batch_status(&batch_id).await?;

        println!(
            "   Poll #{}: Status = {}, Completed = {}, Failed = {}",
            poll_count,
            status.status,
            status.request_counts.completed,
            status.request_counts.failed
        );

        match status.status {
            BatchStatus::Completed => {
                println!("🎉 Batch completed successfully!");

                // Step 5: Download and save results
                println!("\n📥 Step 5: Downloading and saving batch results...");

                // Create output directory
                let output_dir = std::env::current_dir()?.join("batch_output");
                let (result_count, error_count) = batch_api
                    .download_all_batch_files(&batch_id, &output_dir)
                    .await?;

                println!("✅ Files downloaded successfully:");
                println!("   📁 Output directory: {}", output_dir.display());
                println!("   📄 Result lines: {result_count}");
                println!("   ❌ Error lines: {error_count}");

                // Step 6: Process YARA rules and generate report
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

                println!(
                    "   📊 Generated comprehensive report: {}",
                    report_file.display()
                );
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

                if yara_count > 0 {
                    println!("\n🎯 YARA Rule Files Created:");
                    if let Ok(entries) = std::fs::read_dir(&yara_dir) {
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

                break;
            }
            BatchStatus::Failed => {
                println!("❌ Batch failed!");
                if let Ok(Some(errors)) = batch_api.get_batch_errors(&batch_id).await {
                    println!("Error details: {errors}");
                }
                break;
            }
            BatchStatus::Expired => {
                println!("⏰ Batch expired!");
                break;
            }
            BatchStatus::Cancelled => {
                println!("🚫 Batch was cancelled!");
                break;
            }
            _ => {
                // Continue polling for in-progress states
                if poll_count >= 30 {
                    println!("⏰ Stopping polling after 30 attempts (5 minutes)");
                    println!(
                        "   The batch is still processing. Current status: {}",
                        status.status
                    );
                    println!(
                        "   You can continue monitoring with: get_batch_status(\"{batch_id}\")"
                    );
                    break;
                }

                // Wait 10 seconds before next poll
                sleep(Duration::from_secs(10)).await;
            }
        }
    }

    // Clean up
    let _ = std::fs::remove_file(&batch_file);

    println!("\n✨ Batch processing demo completed!");
    println!("💡 Key takeaways:");
    println!("   • Batch API offers 50% cost savings vs real-time API");
    println!("   • Higher rate limits for bulk processing");
    println!("   • 24-hour completion guarantee");
    println!("   • Automatic file download and processing");
    println!("   • YARA rule extraction and validation");
    println!("   • Comprehensive reporting and analytics");
    println!("   • Perfect for large-scale cybersecurity workflows");

    Ok(())
}
