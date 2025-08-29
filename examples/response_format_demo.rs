#![allow(clippy::pedantic, clippy::nursery)]
//! Response Format Enforcement Demo
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
    schema_examples::create_example_schemas,
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

/// Demonstrate the example schemas functionality
fn demonstrate_example_schemas() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📚 Example Schema Library");
    println!("--------------------------");

    let schemas = create_example_schemas();
    println!("Available example schemas:");
    for name in schemas.keys() {
        println!("  - {name}");
    }

    println!("\n✅ Schema library contains {} schemas", schemas.len());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Response Format Enforcement Demo");
    println!("====================================\n");

    let client = initialize_responses_api()?;

    // Run all demonstration modules
    run_basic_format_demos(&client).await?;
    run_advanced_schema_demos(&client).await?;
    run_validation_demos(&client).await?;
    run_error_handling_demos(&client).await?;

    // Demonstrate the schema examples library
    demonstrate_example_schemas()?;

    println!("\n✅ All examples completed successfully!");
    println!("\nℹ️  This demo uses modular structure:");
    println!("   - Basic demos: JSON mode & simple validation");
    println!("   - Schema builders: Complex schema construction");
    println!("   - Validation: Type-safe parsing & strict mode");
    println!("   - Error handling: Validation failure cases");
    println!("   - Schema examples: Reusable schema library");

    Ok(())
}

// Tests are now moved to individual modules
