#![allow(clippy::pedantic, clippy::nursery)]
//! # OpenAI Runs API Demo
//!
//! This example demonstrates the complete OpenAI Runs API functionality including:
//! - Creating and executing runs
//! - Handling tool calls and submitting outputs
//! - Monitoring run progress through steps
//! - Handling different run statuses
//! - Demonstrating error recovery
//! - Showing streaming updates
//! - Tracking token usage
//!
//! To run this demo:
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! cargo run --example runs_demo
//! ```

mod runs_demo {
    pub mod core_demos;
    pub mod error_handling;
    pub mod setup;
    pub mod streaming;
    pub mod utilities;

    pub use core_demos::*;
    pub use error_handling::*;
    pub use setup::*;
    pub use streaming::*;
}

use runs_demo::*;
use std::env;

/// Run specialized demonstrations (error handling and streaming)
async fn run_specialized_demos(
    runs_api: &openai_rust_sdk::api::runs::RunsApi,
    thread_id: &str,
    assistant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Demo 9: Error handling
    println!("\n🚨 Demo 9: Demonstrating error handling...");
    error_handling_demo(runs_api).await?;

    // Demo 10: Streaming runs
    println!("\n📡 Demo 10: Creating streaming runs...");
    streaming_demo(runs_api, thread_id, assistant_id).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is required");

    println!("🚀 OpenAI Runs API Demo Starting...\n");

    // Initialize API clients
    let (runs_api, assistants_api, threads_api) = initialize_api_clients(&api_key)?;

    // Setup initial resources
    let (assistant_id, thread_id) = setup_demo_resources(&assistants_api, &threads_api).await?;

    // Run demo groups
    run_basic_demos(&runs_api, &thread_id, &assistant_id).await?;
    run_advanced_demos(&runs_api, &thread_id, &assistant_id).await?;
    run_specialized_demos(&runs_api, &thread_id, &assistant_id).await?;

    // Clean up
    println!("\n🧹 Cleaning up...");
    cleanup(&assistants_api, &threads_api, &assistant_id, &thread_id).await?;

    println!("\n🎉 All demos completed successfully!");
    Ok(())
}
