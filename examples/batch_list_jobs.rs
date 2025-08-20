#![allow(clippy::pedantic, clippy::nursery)]
//! Example: List and analyze all recent batch jobs
//!
//! This example demonstrates how to:
//! - Connect to OpenAI's Batch API
//! - List all batch jobs with pagination
//! - Display batch job details and statistics
//! - Filter by status and analyze completion rates

use chrono::{DateTime, Utc};
use openai_rust_sdk::api::batch::{BatchApi, BatchStatus};
use openai_rust_sdk::error::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment
    let api_key =
        env::var("OPENAI_API_KEY").expect("Please set OPENAI_API_KEY environment variable");

    println!("\n🔄 Fetching Recent Batch Jobs from OpenAI\n");
    println!("{}", "=".repeat(60));

    // Initialize the Batch API
    let batch_api = BatchApi::new(&api_key)?;

    // List all batch jobs (with pagination support)
    println!("📋 Retrieving batch jobs...\n");

    match batch_api.list_batches(None, None).await {
        Ok(batch_list) => {
            if batch_list.data.is_empty() {
                println!("No batch jobs found.");
                return Ok(());
            }

            println!("Found {} batch job(s)\n", batch_list.data.len());
            println!("{}", "-".repeat(60));

            // Statistics
            let mut stats = BatchStatistics::default();

            // Display each batch job
            for (idx, batch) in batch_list.data.iter().enumerate() {
                println!("\n📦 Batch Job #{}", idx + 1);
                println!("   ID: {}", batch.id);
                println!("   Status: {:?}", batch.status);
                println!("   Endpoint: {}", batch.endpoint);
                println!("   Input File: {}", batch.input_file_id);

                // Display output file if completed
                if let Some(output_file) = &batch.output_file_id {
                    println!("   Output File: {output_file}");
                }

                // Display error file if present
                if let Some(error_file) = &batch.error_file_id {
                    println!("   ⚠️  Error File: {error_file}");
                }

                // Parse and display timestamps
                println!("   Created: {}", format_timestamp(batch.created_at));

                if let Some(in_progress_at) = batch.in_progress_at {
                    println!("   Started: {}", format_timestamp(in_progress_at));
                }

                if let Some(completed_at) = batch.completed_at {
                    println!("   Completed: {}", format_timestamp(completed_at));

                    // Calculate duration if both timestamps exist
                    if let Some(started) = batch.in_progress_at {
                        let duration = completed_at - started;
                        println!("   Duration: {duration} seconds");
                    }
                }

                if let Some(failed_at) = batch.failed_at {
                    println!("   ❌ Failed: {}", format_timestamp(failed_at));
                }

                println!("   Expires: {}", format_timestamp(batch.expires_at));
                println!("   Completion Window: {}", batch.completion_window);

                // Display request counts
                let counts = &batch.request_counts;
                println!("\n   📊 Request Statistics:");
                println!("      Total: {}", counts.total);
                println!("      Completed: {}", counts.completed);
                println!("      Failed: {}", counts.failed);

                if counts.total > 0 {
                    let success_rate = (counts.completed as f64 / counts.total as f64) * 100.0;
                    println!("      Success Rate: {success_rate:.1}%");
                }

                // Display metadata if present
                if let Some(metadata) = &batch.metadata {
                    if metadata.is_object() {
                        if let Some(obj) = metadata.as_object() {
                            if !obj.is_empty() {
                                println!("\n   📎 Metadata:");
                                for (key, value) in obj {
                                    println!("      {key}: {value}");
                                }
                            }
                        }
                    }
                }

                // Update statistics
                stats.update(&batch.status);

                println!("\n{}", "-".repeat(60));
            }

            // Display summary statistics
            println!("\n📈 Summary Statistics");
            println!("{}", "=".repeat(60));
            println!("Total Batches: {}", stats.total);
            println!("Status Breakdown:");
            println!("  ✅ Completed: {}", stats.completed);
            println!("  ⏳ In Progress: {}", stats.in_progress);
            println!("  ⏸️  Validating: {}", stats.validating);
            println!("  🔄 Finalizing: {}", stats.finalizing);
            println!("  ❌ Failed: {}", stats.failed);
            println!("  ⏰ Expired: {}", stats.expired);
            println!("  🚫 Cancelled: {}", stats.cancelled);

            if stats.total > 0 {
                #[allow(clippy::cast_precision_loss)]
                let success_rate = (stats.completed as f64 / stats.total as f64) * 100.0;
                println!("\nOverall Success Rate: {success_rate:.1}%");
            }

            // Note about pagination
            if batch_list.data.len() >= 20 {
                println!(
                    "\n📌 Note: There may be more batch jobs. Use pagination to retrieve all."
                );
                if let Some(last_batch) = batch_list.data.last() {
                    println!("   Last batch ID: {}", last_batch.id);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error fetching batch jobs: {e}");
            eprintln!("\nPossible causes:");
            eprintln!("  - Invalid API key");
            eprintln!("  - Network connectivity issues");
            eprintln!("  - API rate limits");

            return Err(e);
        }
    }

    println!("\n✅ Batch job retrieval complete!\n");

    // Optional: Demonstrate getting details for a specific batch
    println!("\n💡 Tip: To get details for a specific batch, use:");
    println!("   batch_api.get_batch_status(\"batch_id\").await");

    // Optional: Demonstrate batch result retrieval
    println!("\n💡 To retrieve results from a completed batch:");
    println!("   batch_api.get_batch_results(\"batch_id\").await");

    Ok(())
}

/// Helper function to format Unix timestamps
fn format_timestamp(timestamp: u64) -> String {
    let datetime = DateTime::<Utc>::from_timestamp(timestamp as i64, 0).unwrap_or_else(Utc::now);
    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Statistics tracker for batch jobs
#[derive(Default)]
struct BatchStatistics {
    total: usize,
    completed: usize,
    in_progress: usize,
    validating: usize,
    finalizing: usize,
    failed: usize,
    expired: usize,
    cancelled: usize,
}

impl BatchStatistics {
    fn update(&mut self, status: &BatchStatus) {
        self.total += 1;
        match status {
            BatchStatus::Completed => self.completed += 1,
            BatchStatus::InProgress => self.in_progress += 1,
            BatchStatus::Validating => self.validating += 1,
            BatchStatus::Finalizing => self.finalizing += 1,
            BatchStatus::Failed => self.failed += 1,
            BatchStatus::Expired => self.expired += 1,
            BatchStatus::Cancelled => self.cancelled += 1,
            BatchStatus::Cancelling => self.cancelled += 1,
        }
    }
}
