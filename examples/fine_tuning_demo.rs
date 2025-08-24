#![allow(
    clippy::too_many_lines,
    clippy::uninlined_format_args,
    clippy::cast_precision_loss,
    clippy::ignored_unit_patterns
)]
//! # `OpenAI` Fine-tuning API Demo
//!
//! This example demonstrates how to use the `OpenAI` Fine-tuning API to create,
//! monitor, and manage fine-tuning jobs for custom model training.
//!
//! ## Features Demonstrated
//!
//! - Creating fine-tuning jobs with custom hyperparameters
//! - Monitoring job progress through events and status updates
//! - Listing and managing multiple fine-tuning jobs
//! - Handling job completion and model deployment
//! - Error handling for failed jobs and API issues
//! - Checkpoint management and training metrics
//!
//! ## Prerequisites
//!
//! 1. Set the `OPENAI_API_KEY` environment variable
//! 2. Upload training data as a JSONL file using the Files API
//! 3. Optionally upload validation data for better training monitoring
//!
//! ## Usage
//!
//! ```bash
//! # Set your OpenAI API key
//! export OPENAI_API_KEY="your-api-key-here"
//!
//! # Run the demo
//! cargo run --example fine_tuning_demo
//! ```

use openai_rust_sdk::api::{
    common::ApiClientConstructors, files::FilesApi, fine_tuning::FineTuningApi,
};
use openai_rust_sdk::error::{OpenAIError, Result};
use openai_rust_sdk::models::files::{FilePurpose, FileUploadRequest};
use openai_rust_sdk::models::fine_tuning::{
    FineTuningJob, FineTuningJobRequest, FineTuningJobStatus, Hyperparameters,
    ListFineTuningJobCheckpointsParams, ListFineTuningJobsParams,
};
use std::env;
use std::time::Duration;

/// Training data example for fine-tuning
const TRAINING_DATA: &str = r#"{"messages": [{"role": "user", "content": "What is the capital of France?"}, {"role": "assistant", "content": "The capital of France is Paris."}]}
{"messages": [{"role": "user", "content": "What is 2 + 2?"}, {"role": "assistant", "content": "2 + 2 equals 4."}]}
{"messages": [{"role": "user", "content": "Who wrote Romeo and Juliet?"}, {"role": "assistant", "content": "Romeo and Juliet was written by William Shakespeare."}]}
{"messages": [{"role": "user", "content": "What is the largest planet in our solar system?"}, {"role": "assistant", "content": "Jupiter is the largest planet in our solar system."}]}
{"messages": [{"role": "user", "content": "What is the chemical symbol for gold?"}, {"role": "assistant", "content": "The chemical symbol for gold is Au."}]}"#;

/// Validation data example for fine-tuning
const VALIDATION_DATA: &str = r#"{"messages": [{"role": "user", "content": "What is the capital of Spain?"}, {"role": "assistant", "content": "The capital of Spain is Madrid."}]}
{"messages": [{"role": "user", "content": "What is 3 + 3?"}, {"role": "assistant", "content": "3 + 3 equals 6."}]}"#;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 OpenAI Fine-tuning API Demo");
    println!("==============================\n");

    // Initialize the API client
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| OpenAIError::authentication("OPENAI_API_KEY environment variable not set"))?;

    let fine_tuning_api = FineTuningApi::new(&api_key)?;
    let files_api = FilesApi::new(&api_key)?;

    println!("✅ Initialized Fine-tuning API client\n");

    // Demo 1: Upload training and validation files
    println!("📁 Demo 1: Uploading training and validation files");
    println!("================================================");

    let (training_file_id, validation_file_id) = upload_training_files(&files_api).await?;
    println!("✅ Training file uploaded: {training_file_id}");
    println!("✅ Validation file uploaded: {validation_file_id}");
    println!();

    // Demo 2: Create fine-tuning jobs with different configurations
    println!("🎯 Demo 2: Creating fine-tuning jobs");
    println!("===================================");

    let job_ids = create_fine_tuning_jobs(
        &fine_tuning_api,
        &training_file_id,
        Some(&validation_file_id),
    )
    .await?;
    println!("✅ Created {} fine-tuning jobs\n", job_ids.len());

    // Demo 3: List and monitor jobs
    println!("📊 Demo 3: Listing and monitoring jobs");
    println!("=====================================");

    list_and_monitor_jobs(&fine_tuning_api, &job_ids).await?;
    println!();

    // Demo 4: Monitor a specific job with event streaming
    println!("🔄 Demo 4: Monitoring job with event streaming");
    println!("============================================");

    if let Some(first_job_id) = job_ids.first() {
        monitor_job_with_events(&fine_tuning_api, first_job_id).await?;
    }
    println!();

    // Demo 5: Handle job cancellation
    println!("❌ Demo 5: Demonstrating job cancellation");
    println!("========================================");

    if job_ids.len() > 1 {
        demonstrate_job_cancellation(&fine_tuning_api, &job_ids[1]).await?;
    }
    println!();

    // Demo 6: List checkpoints for completed jobs
    println!("📈 Demo 6: Listing training checkpoints");
    println!("=====================================");

    for job_id in &job_ids {
        list_job_checkpoints(&fine_tuning_api, job_id).await?;
    }
    println!();

    // Demo 7: Error handling examples
    println!("⚠️ Demo 7: Error handling examples");
    println!("=================================");

    demonstrate_error_handling(&fine_tuning_api).await?;
    println!();

    // Demo 8: Using fine-tuned models (simulated)
    println!("🎉 Demo 8: Using fine-tuned models");
    println!("=================================");

    demonstrate_model_usage(&fine_tuning_api, &job_ids[0]).await?;

    println!("\n🎊 Fine-tuning demo completed successfully!");
    println!("Check the OpenAI dashboard for detailed job progress and metrics.");

    Ok(())
}

/// Upload training and validation files
async fn upload_training_files(files_api: &FilesApi) -> Result<(String, String)> {
    // Create temporary files
    let training_path = "/tmp/training_data.jsonl";
    let validation_path = "/tmp/validation_data.jsonl";

    std::fs::write(training_path, TRAINING_DATA)
        .map_err(|e| OpenAIError::FileError(format!("Failed to write training file: {e}")))?;

    std::fs::write(validation_path, VALIDATION_DATA)
        .map_err(|e| OpenAIError::FileError(format!("Failed to write validation file: {e}")))?;

    // Upload training file
    let training_data = std::fs::read(training_path)
        .map_err(|e| OpenAIError::FileError(format!("Failed to read training file: {e}")))?;
    let training_request = FileUploadRequest::new(
        training_data,
        "training_data.jsonl".to_string(),
        FilePurpose::FineTune,
    );

    let training_file = files_api.upload_file(training_request).await?;

    // Upload validation file
    let validation_data = std::fs::read(validation_path)
        .map_err(|e| OpenAIError::FileError(format!("Failed to read validation file: {e}")))?;
    let validation_request = FileUploadRequest::new(
        validation_data,
        "validation_data.jsonl".to_string(),
        FilePurpose::FineTune,
    );

    let validation_file = files_api.upload_file(validation_request).await?;

    // Clean up temporary files
    let _ = std::fs::remove_file(training_path);
    let _ = std::fs::remove_file(validation_path);

    Ok((training_file.id, validation_file.id))
}

/// Create fine-tuning jobs with different configurations
async fn create_fine_tuning_jobs(
    api: &FineTuningApi,
    training_file_id: &str,
    validation_file_id: Option<&str>,
) -> Result<Vec<String>> {
    let mut job_ids = Vec::new();

    // Job 1: Basic configuration with auto hyperparameters
    println!("Creating job 1: Basic configuration with auto hyperparameters");
    let basic_request = FineTuningJobRequest::builder()
        .training_file(training_file_id)
        .model("gpt-3.5-turbo")
        .suffix("basic-demo")
        .metadata_entry("demo", "basic")
        .metadata_entry("created_by", "fine_tuning_demo")
        .build()?;

    let basic_job = api.create_fine_tuning_job(basic_request).await?;
    job_ids.push(basic_job.id.clone());
    println!(
        "  ✅ Job created: {} (status: {:?})",
        basic_job.id, basic_job.status
    );

    // Job 2: Custom hyperparameters with validation file
    if let Some(val_file_id) = validation_file_id {
        println!("Creating job 2: Custom hyperparameters with validation");
        let custom_request = FineTuningJobRequest::builder()
            .training_file(training_file_id)
            .validation_file(val_file_id)
            .model("gpt-3.5-turbo")
            .hyperparameters(
                Hyperparameters::builder()
                    .n_epochs(3)
                    .batch_size(16)
                    .learning_rate_multiplier(0.1)
                    .build(),
            )
            .suffix("custom-demo")
            .metadata_entry("demo", "custom")
            .metadata_entry("hyperparameters", "optimized")
            .build()?;

        let custom_job = api.create_fine_tuning_job(custom_request).await?;
        job_ids.push(custom_job.id.clone());
        println!(
            "  ✅ Job created: {} (status: {:?})",
            custom_job.id, custom_job.status
        );
    }

    // Job 3: Conservative hyperparameters
    println!("Creating job 3: Conservative hyperparameters");
    let conservative_request = FineTuningJobRequest::builder()
        .training_file(training_file_id)
        .model("gpt-3.5-turbo")
        .hyperparameters(
            Hyperparameters::builder()
                .n_epochs(1)
                .batch_size(8)
                .learning_rate_multiplier(0.05)
                .build(),
        )
        .suffix("conservative")
        .metadata_entry("demo", "conservative")
        .metadata_entry("training_approach", "cautious")
        .build()?;

    let conservative_job = api.create_fine_tuning_job(conservative_request).await?;
    job_ids.push(conservative_job.id.clone());
    println!(
        "  ✅ Job created: {} (status: {:?})",
        conservative_job.id, conservative_job.status
    );

    Ok(job_ids)
}

/// List and monitor jobs
async fn list_and_monitor_jobs(api: &FineTuningApi, job_ids: &[String]) -> Result<()> {
    // List all fine-tuning jobs
    println!("Listing all fine-tuning jobs:");
    let list_params = ListFineTuningJobsParams::new().limit(10);
    let jobs_response = api.list_fine_tuning_jobs(Some(list_params)).await?;

    println!(
        "  📋 Found {} jobs (showing up to 10)",
        jobs_response.data.len()
    );
    for job in &jobs_response.data {
        println!("    - {}: {:?} (model: {})", job.id, job.status, job.model);
        if let Some(fine_tuned_model) = &job.fine_tuned_model {
            println!("      Fine-tuned model: {fine_tuned_model}");
        }
    }

    // Monitor specific jobs
    println!("\nMonitoring created jobs:");
    for job_id in job_ids {
        let job = api.retrieve_fine_tuning_job(job_id).await?;
        println!("  📊 Job {}: {:?}", job_id, job.status);

        if let Some(trained_tokens) = job.trained_tokens {
            println!("    Trained tokens: {trained_tokens}");
        }

        if let Some(error) = &job.error {
            println!("    ❌ Error: {} - {}", error.code, error.message);
        }
    }

    Ok(())
}

/// Monitor a job with event streaming
async fn monitor_job_with_events(api: &FineTuningApi, job_id: &str) -> Result<()> {
    println!("Monitoring job {job_id} with event streaming:");

    // Set up event callback
    let event_callback = Box::new(
        |event: &openai_rust_sdk::models::fine_tuning::FineTuningJobEvent| {
            let timestamp = chrono::DateTime::from_timestamp(event.created_at, 0).map_or_else(
                || "Unknown".to_string(),
                |dt| dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            );

            println!(
                "  🔔 [{}] {}: {}",
                timestamp,
                event.level.to_uppercase(),
                event.message
            );
        },
    );

    // Monitor for a short time (in production, you'd wait for completion)
    println!("  Monitoring for 30 seconds...");

    // Create a timeout future
    let timeout_future = tokio::time::sleep(Duration::from_secs(30));
    let monitor_future = api.monitor_fine_tuning_job(
        job_id,
        Some(Duration::from_secs(10)), // Poll every 10 seconds
        Some(event_callback),
    );

    // Race between timeout and completion
    tokio::select! {
        result = monitor_future => {
            match result {
                Ok(final_job) => {
                    println!("  ✅ Job completed with status: {:?}", final_job.status);
                    if let Some(model) = final_job.fine_tuned_model {
                        println!("  🎉 Fine-tuned model available: {model}");
                    }
                }
                Err(e) => {
                    println!("  ❌ Error monitoring job: {e}");
                }
            }
        }
        () = timeout_future => {
            println!("  ⏰ Monitoring timeout reached (job may still be running)");

            // Get final status
            match api.retrieve_fine_tuning_job(job_id).await {
                Ok(job) => println!("  📊 Current status: {:?}", job.status),
                Err(e) => println!("  ❌ Failed to get final status: {e}"),
            }
        }
    }

    Ok(())
}

/// Demonstrate job cancellation
async fn demonstrate_job_cancellation(api: &FineTuningApi, job_id: &str) -> Result<()> {
    println!("Demonstrating job cancellation for job: {job_id}");

    // Check current status
    let job = api.retrieve_fine_tuning_job(job_id).await?;
    println!("  📊 Current status: {:?}", job.status);

    // Only cancel if job is still active
    if job.status.is_active() {
        println!("  🛑 Cancelling job...");
        match api.cancel_fine_tuning_job(job_id).await {
            Ok(cancelled_job) => {
                println!("  ✅ Job cancelled successfully");
                println!("  📊 New status: {:?}", cancelled_job.status);
            }
            Err(e) => {
                println!("  ❌ Failed to cancel job: {e}");
            }
        }
    } else {
        println!("  ℹ️ Job is not in an active state, cannot cancel");
    }

    Ok(())
}

/// List checkpoints for a job
async fn list_job_checkpoints(api: &FineTuningApi, job_id: &str) -> Result<()> {
    println!("Listing checkpoints for job: {job_id}");

    let params = ListFineTuningJobCheckpointsParams::new().limit(5);
    match api.list_fine_tuning_checkpoints(job_id, Some(params)).await {
        Ok(checkpoints_response) => {
            if checkpoints_response.data.is_empty() {
                println!("  📈 No checkpoints available yet");
            } else {
                println!(
                    "  📈 Found {} checkpoints:",
                    checkpoints_response.data.len()
                );
                for checkpoint in &checkpoints_response.data {
                    println!(
                        "    - Step {}: {}",
                        checkpoint.step_number, checkpoint.fine_tuned_model_checkpoint
                    );

                    // Show training metrics if available
                    if let Some(train_loss) = checkpoint.metrics.train_loss {
                        println!("      Training loss: {train_loss:.4}");
                    }
                    if let Some(valid_loss) = checkpoint.metrics.valid_loss {
                        println!("      Validation loss: {valid_loss:.4}");
                    }
                    if let Some(train_acc) = checkpoint.metrics.train_mean_token_accuracy {
                        println!("      Training accuracy: {train_acc:.4}");
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ Failed to list checkpoints: {e}");
        }
    }

    Ok(())
}

/// Demonstrate error handling
async fn demonstrate_error_handling(api: &FineTuningApi) -> Result<()> {
    println!("Demonstrating error handling:");

    // 1. Invalid job ID
    println!("  🔍 Testing invalid job ID...");
    match api.retrieve_fine_tuning_job("invalid-job-id").await {
        Ok(_) => println!("    Unexpected success"),
        Err(e) => println!("    ✅ Expected error: {e}"),
    }

    // 2. Invalid file ID in job creation
    println!("  🔍 Testing invalid file ID...");
    let invalid_request = FineTuningJobRequest::new("invalid-file-id", "gpt-3.5-turbo");
    match api.create_fine_tuning_job(invalid_request).await {
        Ok(_) => println!("    Unexpected success"),
        Err(e) => println!("    ✅ Expected error: {e}"),
    }

    // 3. Invalid model name
    println!("  🔍 Testing invalid model...");
    let invalid_model_request = FineTuningJobRequest::new("file-abc123", "invalid-model");
    match api.create_fine_tuning_job(invalid_model_request).await {
        Ok(_) => println!("    Unexpected success"),
        Err(e) => println!("    ✅ Expected error: {e}"),
    }

    Ok(())
}

/// Demonstrate using fine-tuned models
async fn demonstrate_model_usage(api: &FineTuningApi, job_id: &str) -> Result<()> {
    println!("Demonstrating fine-tuned model usage:");

    // Get job details
    let job = api.retrieve_fine_tuning_job(job_id).await?;

    if let Some(model_name) = job.fine_tuned_model {
        println!("  🎯 Fine-tuned model available: {model_name}");
        println!("  💡 You can now use this model for chat completions:");
        println!("     model: '{model_name}'");
        println!("     Example usage:");
        println!("     ```rust");
        println!("     let request = ChatCompletionRequest::builder()");
        println!("         .model('{model_name}');");
        println!("         .messages(messages)");
        println!("         .build()?;");
        println!("     let response = client.create_chat_completion(request).await?;");
        println!("     ```");
    } else {
        println!("  ⏳ Fine-tuned model not yet available");
        println!("  📊 Job status: {:?}", job.status);

        if job.status == FineTuningJobStatus::Failed {
            if let Some(error) = &job.error {
                println!("  ❌ Training failed: {} - {}", error.code, error.message);
            }
        } else if job.status.is_active() {
            println!("  🔄 Training still in progress, check back later");
        }
    }

    // Show job statistics
    println!("  📊 Job Statistics:");
    println!("    - Model: {}", job.model);
    println!("    - Training file: {}", job.training_file);
    if let Some(validation_file) = &job.validation_file {
        println!("    - Validation file: {validation_file}");
    }
    if let Some(trained_tokens) = job.trained_tokens {
        println!("    - Trained tokens: {trained_tokens}");
    }

    // Show hyperparameters
    println!("  ⚙️ Hyperparameters:");
    if let Some(epochs) = job.hyperparameters.n_epochs {
        println!("    - Epochs: {epochs}");
    } else {
        println!("    - Epochs: auto");
    }
    if let Some(batch_size) = job.hyperparameters.batch_size {
        println!("    - Batch size: {batch_size}");
    } else {
        println!("    - Batch size: auto");
    }
    if let Some(lr_mult) = job.hyperparameters.learning_rate_multiplier {
        println!("    - Learning rate multiplier: {lr_mult}");
    } else {
        println!("    - Learning rate multiplier: auto");
    }

    Ok(())
}

/// Helper function to format duration
#[allow(dead_code)]
fn format_duration(seconds: i64) -> String {
    if seconds < 60 {
        format!("{seconds}s")
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    }
}

/// Helper function to calculate training duration
#[allow(dead_code)]
fn calculate_training_duration(job: &FineTuningJob) -> Option<String> {
    if let (Some(finished), created) = (job.finished_at, job.created_at) {
        Some(format_duration(finished - created))
    } else {
        None
    }
}
