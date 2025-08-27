#![allow(clippy::pedantic, clippy::nursery)]
//! Example: List and analyze all recent batch jobs
//!
//! This example demonstrates how to:
//! - Connect to OpenAI's Batch API
//! - List all batch jobs with pagination
//! - Display batch job details and statistics
//! - Filter by status and analyze completion rates

use chrono::{DateTime, Utc};
use openai_rust_sdk::{
    api::{
        batch::{BatchApi, BatchStatus},
        common::ApiClientConstructors,
    },
    error::Result,
};

mod common;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = common::get_api_key();
    let batch_api = BatchApi::new(&api_key)?;

    print_header();
    let batch_list = fetch_batch_jobs(&batch_api).await?;

    if batch_list.data.is_empty() {
        println!("No batch jobs found.");
        return Ok(());
    }

    let stats = display_batch_jobs(&batch_list.data);
    display_summary_statistics(&stats);
    display_pagination_note(&batch_list.data);
    display_tips();

    Ok(())
}

/// Print the application header
fn print_header() {
    println!("\n🔄 Fetching Recent Batch Jobs from OpenAI\n");
    println!("{}", "=".repeat(60));
    println!("📋 Retrieving batch jobs...\n");
}

/// Fetch batch jobs from the API with error handling
async fn fetch_batch_jobs(batch_api: &BatchApi) -> Result<openai_rust_sdk::api::batch::BatchList> {
    match batch_api.list_batches(None, None).await {
        Ok(batch_list) => {
            println!("Found {} batch job(s)\n", batch_list.data.len());
            println!("{}", "-".repeat(60));
            Ok(batch_list)
        }
        Err(e) => {
            handle_fetch_error(&e);
            Err(e)
        }
    }
}

/// Handle and display fetch errors
fn handle_fetch_error(error: &openai_rust_sdk::error::OpenAIError) {
    eprintln!("❌ Error fetching batch jobs: {error}");
    eprintln!("\nPossible causes:");
    eprintln!("  - Invalid API key");
    eprintln!("  - Network connectivity issues");
    eprintln!("  - API rate limits");
}

/// Display all batch jobs and return statistics
fn display_batch_jobs(batches: &[openai_rust_sdk::api::batch::Batch]) -> BatchStatistics {
    let mut stats = BatchStatistics::default();

    for (idx, batch) in batches.iter().enumerate() {
        display_single_batch_job(batch, idx + 1);
        stats.update(&batch.status);
        println!("\n{}", "-".repeat(60));
    }

    stats
}

/// Display details for a single batch job
fn display_single_batch_job(batch: &openai_rust_sdk::api::batch::Batch, job_number: usize) {
    println!("\n📦 Batch Job #{job_number}");
    println!("   ID: {}", batch.id);
    println!("   Status: {:?}", batch.status);
    println!("   Endpoint: {}", batch.endpoint);
    println!("   Input File: {}", batch.input_file_id);

    display_optional_files(batch);
    display_timestamps(batch);
    display_request_statistics(&batch.request_counts);
    display_metadata(&batch.metadata);
}

/// Display optional output and error files
fn display_optional_files(batch: &openai_rust_sdk::api::batch::Batch) {
    if let Some(output_file) = &batch.output_file_id {
        println!("   Output File: {output_file}");
    }

    if let Some(error_file) = &batch.error_file_id {
        println!("   ⚠️  Error File: {error_file}");
    }
}

/// Display batch job timestamps and duration
fn display_timestamps(batch: &openai_rust_sdk::api::batch::Batch) {
    println!("   Created: {}", format_timestamp(batch.created_at));

    if let Some(in_progress_at) = batch.in_progress_at {
        println!("   Started: {}", format_timestamp(in_progress_at));
    }

    if let Some(completed_at) = batch.completed_at {
        println!("   Completed: {}", format_timestamp(completed_at));
        display_duration(batch.in_progress_at, completed_at);
    }

    if let Some(failed_at) = batch.failed_at {
        println!("   ❌ Failed: {}", format_timestamp(failed_at));
    }

    println!("   Expires: {}", format_timestamp(batch.expires_at));
    println!("   Completion Window: {}", batch.completion_window);
}

/// Display duration between start and completion
fn display_duration(started: Option<u64>, completed: u64) {
    if let Some(started_time) = started {
        let duration = completed - started_time;
        println!("   Duration: {duration} seconds");
    }
}

/// Display request statistics for a batch job
fn display_request_statistics(counts: &openai_rust_sdk::api::batch::BatchRequestCounts) {
    println!("\n   📊 Request Statistics:");
    println!("      Total: {}", counts.total);
    println!("      Completed: {}", counts.completed);
    println!("      Failed: {}", counts.failed);

    if counts.total > 0 {
        let success_rate = (counts.completed as f64 / counts.total as f64) * 100.0;
        println!("      Success Rate: {success_rate:.1}%");
    }
}

/// Display metadata if present
fn display_metadata(metadata: &Option<serde_json::Value>) {
    if let Some(metadata) = metadata {
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
}

/// Display summary statistics
fn display_summary_statistics(stats: &BatchStatistics) {
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

    println!("\n✅ Batch job retrieval complete!\n");
}

/// Display pagination note if applicable
fn display_pagination_note(batches: &[openai_rust_sdk::api::batch::Batch]) {
    if batches.len() >= 20 {
        println!("\n📌 Note: There may be more batch jobs. Use pagination to retrieve all.");
        if let Some(last_batch) = batches.last() {
            println!("   Last batch ID: {}", last_batch.id);
        }
    }
}

/// Display helpful tips for users
fn display_tips() {
    println!("\n💡 Tip: To get details for a specific batch, use:");
    println!("   batch_api.get_batch_status(\"batch_id\").await");
    println!("\n💡 To retrieve results from a completed batch:");
    println!("   batch_api.get_batch_results(\"batch_id\").await");
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
