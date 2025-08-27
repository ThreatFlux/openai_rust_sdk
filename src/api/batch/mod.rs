//! # OpenAI Batch API Client
//!
//! This module provides a complete implementation of OpenAI's Batch API for processing
//! asynchronous groups of requests with 50% lower costs and higher rate limits.
//!
//! ## Features
//!
//! - **File Upload**: Upload JSONL batch files to OpenAI
//! - **Batch Creation**: Create and submit batch processing jobs  
//! - **Status Monitoring**: Check batch progress and completion status
//! - **Result Retrieval**: Download completed batch results
//! - **Batch Management**: List, cancel, and manage batch operations
//! - **YARA Processing**: Extract and process YARA rules from batch results
//! - **Comprehensive Reporting**: Generate detailed analysis reports
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::batch::{BatchApi, BatchStatus};
//! use openai_rust_sdk::api::common::ApiClientConstructors;
//! use std::path::Path;
//!
//! # tokio_test::block_on(async {
//! let api = BatchApi::new("your-api-key")?;
//!
//! // Upload batch file
//! let file = api.upload_batch_file(Path::new("batch_input.jsonl")).await?;
//!
//! // Create batch
//! let batch = api.create_batch(&file.id, "/v1/chat/completions").await?;
//!
//! // Monitor status
//! let status = api.get_batch_status(&batch.id).await?;
//! println!("Batch status: {}", status.status);
//!
//! // Retrieve results when complete
//! if status.status == BatchStatus::Completed {
//!     let results = api.get_batch_results(&batch.id).await?;
//!     println!("Results: {results}");
//! }
//! # Ok::<(), openai_rust_sdk::OpenAIError>(())
//! # });
//! ```

pub mod client;
pub mod file_ops;
pub mod helpers;
pub mod models;
pub mod operations;
pub mod reports;
pub mod types;
pub mod yara;

// Re-export main types and functions for convenience
pub use client::BatchApi;
pub use models::{Batch, BatchList, CreateBatchRequest, FileUploadResponse};
pub use reports::BatchReport;
pub use types::{BatchRequestCounts, BatchStatus, YaraRuleInfo};
pub use yara::YaraProcessor;

// Keep legacy re-exports for API compatibility
pub use BatchApi as BatchClient; // Alternative name
