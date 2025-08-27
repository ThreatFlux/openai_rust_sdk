#![allow(clippy::pedantic, clippy::nursery)]
//! # Vector Stores API Demo
//!
//! This example demonstrates the complete OpenAI Vector Stores API functionality,
//! including creating vector stores, managing files, batch operations, and
//! advanced features like expiration policies and chunking strategies.
//!
//! ## Features Demonstrated
//!
//! - Creating vector stores with various configurations
//! - Managing individual files and batch operations
//! - Monitoring processing status and file counts
//! - Using different chunking strategies for optimal embeddings
//! - Setting up expiration policies for automatic cleanup
//! - Error handling and best practices
//!
//! ## Prerequisites
//!
//! Set your OpenAI API key:
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! ```
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example vector_stores_demo
//! ```

mod vector_stores_modules;

use openai_rust_sdk::error::{OpenAIError, Result};
use std::env;
use vector_stores_modules::{
    file_operations::demo_file_upload,
    utilities::{cleanup_resources, create_demo_stores, initialize_apis, run_demo_workflow},
};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        OpenAIError::Authentication("Please set OPENAI_API_KEY environment variable".to_string())
    })?;

    let (vector_stores_api, files_api) = initialize_apis(&api_key).await?;
    let (basic_store, expiring_store) = create_demo_stores(&vector_stores_api).await?;
    let uploaded_file_ids = demo_file_upload(&files_api).await?;

    let advanced_store = run_demo_workflow(
        &vector_stores_api,
        &files_api,
        &basic_store,
        &uploaded_file_ids,
    )
    .await?;

    cleanup_resources(
        &files_api,
        &vector_stores_api,
        &uploaded_file_ids,
        &basic_store,
        &expiring_store,
        &advanced_store,
    )
    .await?;

    Ok(())
}
