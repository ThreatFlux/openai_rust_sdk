#![allow(clippy::pedantic, clippy::nursery)]
//! # Images API Demo
//!
//! This example demonstrates the comprehensive usage of the Images API,
//! including image generation, editing, and variation creation using DALL-E models.

use openai_rust_sdk::{
    api::{common::ApiClientConstructors, images::ImagesApi},
    error::Result,
};

mod common;
mod images_demo_modules;

use images_demo_modules::{
    editing::demo_edit_builder,
    generation::{demo_convenience_method, demo_generation_builder, run_generation_demos},
    utilities::{demo_cost_estimation, run_utility_demos},
    variations::demo_variation_builder,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the Images API client
    let images_api = initialize_api()?;

    println!("🎨 OpenAI Images API Demo");
    println!("========================\n");

    // Run all image generation demos
    run_generation_demos(&images_api).await?;

    // Run image manipulation demos
    run_manipulation_demos(&images_api).await?;

    // Run utility and advanced demos
    run_utility_demos(&images_api).await?;

    // Run builder pattern demos
    run_builder_demos(&images_api).await?;

    println!("\n✅ All demos completed successfully!");

    Ok(())
}

/// Initialize the API client
fn initialize_api() -> Result<ImagesApi> {
    let api_key = common::get_api_key();
    ImagesApi::new(api_key)
}

/// Run image manipulation demos (editing and variations)
async fn run_manipulation_demos(images_api: &ImagesApi) -> Result<()> {
    use images_demo_modules::{editing::demo_image_editing, variations::demo_image_variations};

    // Demo 4: Image Editing (requires sample image)
    demo_image_editing(images_api).await?;

    // Demo 5: Image Variations (requires sample image)
    demo_image_variations(images_api).await?;

    Ok(())
}

/// Run builder pattern demos
async fn run_builder_demos(images_api: &ImagesApi) -> Result<()> {
    println!("🏗️ Demo 9: Builder Patterns");
    println!("---------------------------");

    demo_generation_builder();
    demo_edit_builder();
    demo_variation_builder();
    demo_convenience_method(images_api).await?;

    // Demo 8: Cost Estimation
    demo_cost_estimation();

    println!();
    Ok(())
}
