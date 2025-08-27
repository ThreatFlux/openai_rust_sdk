#![allow(clippy::pedantic, clippy::nursery)]
//! Response Format Enforcement Demo - Modular Version
//!
//! This example demonstrates the new response format enforcement features:
//! - JSON Object mode for unstructured JSON responses
//! - JSON Schema mode for strictly validated structured outputs
//! - Schema builders for creating complex validation schemas
//! - Type-safe parsing of structured responses

use openai_rust_sdk::api::{common::ApiClientConstructors, ResponsesApi};
use std::env;

mod response_format_modules;

use response_format_modules::{
    basic_demos::run_basic_format_demos,
    builders::demo_complex_schema_builder,
    error_handling::run_error_handling_demos,
    validation::{demo_strict_mode_enforcement, run_validation_demos},
};

/// Initialize the responses API client
fn initialize_responses_api() -> Result<ResponsesApi, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    Ok(ResponsesApi::new(api_key)?)
}

/// Run advanced schema demonstrations
async fn run_advanced_schema_demos(
    client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    // Example 3: Complex Schema with Schema Builder
    println!("\n🏗️  Example 3: Complex Schema with Builder");
    println!("------------------------------------------");
    demo_complex_schema_builder(client).await?;

    // Example 4: Strict Mode Schema Enforcement
    println!("\n🔒 Example 4: Strict Mode Enforcement");
    println!("-------------------------------------");
    demo_strict_mode_enforcement(client).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Response Format Enforcement Demo - Modular Version");
    println!("======================================================\n");

    let client = initialize_responses_api()?;

    run_basic_format_demos(&client).await?;
    run_advanced_schema_demos(&client).await?;
    run_validation_demos(&client).await?;
    run_error_handling_demos(&client).await?;

    println!("\n✅ All examples completed successfully!");

    Ok(())
}
