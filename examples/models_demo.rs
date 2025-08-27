#![allow(clippy::pedantic, clippy::nursery)]
//! # Models API Demo
//!
//! This example demonstrates how to use OpenAI's Models API to:
//! - List all available models
//! - Get details about specific models
//! - Group models by family and capabilities
//! - Find suitable models for specific use cases
//! - Compare model costs and capabilities
//! - Get model statistics and recommendations
//!
//! Usage:
//! ```bash
//! export OPENAI_API_KEY=your_api_key_here
//! cargo run --example models_demo
//! ```

use openai_rust_sdk::api::models::ModelsApi;
use std::env;

// Import our organized modules
mod models_demo_modules;
use models_demo_modules::*;

/// Print demo summary
fn print_demo_summary() {
    println!("\n✨ Models API Summary");
    println!("────────────────────");
    println!("• List and retrieve detailed model information");
    println!("• Filter models by family, capabilities, and use cases");
    println!("• Compare costs and find cost-effective solutions");
    println!("• Get recommendations for specific requirements");
    println!("• Access comprehensive model statistics");
    println!("• Check model availability and deprecation status");

    println!("\n💡 Use Cases:");
    println!("• Model selection for applications");
    println!("• Cost optimization and budgeting");
    println!("• Capability assessment and planning");
    println!("• Migration planning for deprecated models");
    println!("• Performance benchmarking setup");
    println!("• Feature compatibility checking");
}

/// Run all demonstrations in sequence
async fn run_all_demos(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    // Run basic demos (Examples 1-4)
    run_basic_demos(api).await?;

    // Run advanced analysis demos (Examples 5-8, 11-14)
    run_advanced_demos(api).await?;

    // Run cost and analytics demos (Examples 9-10)
    run_cost_and_analytics_demos(api).await?;

    Ok(())
}

/// Setup the API client
fn setup_api() -> Result<ModelsApi, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;
    Ok(ModelsApi::new(api_key)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 OpenAI Models API Demo");
    println!("=========================");

    let api = setup_api()?;
    run_all_demos(&api).await?;
    print_demo_summary();

    Ok(())
}
