//! Basic model operations and demonstrations
//!
//! This module contains the fundamental model API operations:
//! - Listing all available models
//! - Retrieving specific model details
//! - Grouping models by family
//! - Filtering models by completion type
//! - Showing available (non-deprecated) models
//! - Finding latest models from each family

use crate::helpers::*;
use openai_rust_sdk::{
    api::models::ModelsApi,
    models::models::{Model, ModelFamily},
};
use std::collections::HashMap;

/// Demo: List all available models
pub async fn demo_list_all_models(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Example 1: List All Available Models");
    println!("──────────────────────────────────────");

    let models_response = api.list_models().await?;
    println!("Total models available: {}", models_response.data.len());

    println!("\nFirst 5 models:");
    for (i, model) in models_response.data.iter().take(5).enumerate() {
        print_model_summary(model, i);
    }

    Ok(())
}

/// Demo: Retrieve details for a specific model
pub async fn demo_retrieve_specific_model(
    api: &ModelsApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Example 2: Retrieve Specific Model Details");
    println!("─────────────────────────────────────────────");

    let model_id = "gpt-4";
    match api.retrieve_model(model_id).await {
        Ok(model) => {
            print_model_details(&model);
        }
        Err(e) => println!("Error retrieving model {model_id}: {e}"),
    }

    Ok(())
}

/// Print detailed model information
fn print_model_details(model: &Model) {
    print_basic_model_info(model);
    let capabilities = model.capabilities();
    print_capabilities_info(&capabilities);
    print_cost_info(&capabilities);
}

/// Helper function to print model families with limited output
fn print_model_families(grouped_models: &HashMap<ModelFamily, Vec<Model>>, limit: usize) {
    for (family, models) in grouped_models {
        if !models.is_empty() {
            println!("\n{:?} Family ({} models):", family, models.len());

            for model in models.iter().take(limit) {
                let deprecated = deprecation_status(model);
                println!("  • {}{}", model.id, deprecated);
            }

            if models.len() > limit {
                println!("  ... and {} more", models.len() - limit);
            }
        }
    }
}

/// Demo: Group models by family
pub async fn demo_group_models_by_family(
    api: &ModelsApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n👨‍👩‍👧‍👦 Example 3: Group Models by Family");
    println!("──────────────────────────────────────");

    let grouped_models = api.group_models_by_family().await?;
    print_model_families(&grouped_models, 3);

    Ok(())
}

/// Helper function to print models by completion type
async fn print_models_by_completion_type(
    api: &ModelsApi,
    completion_type: &openai_rust_sdk::models::models::CompletionType,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let models = api
        .list_models_by_completion_type(completion_type.clone())
        .await?;

    println!(
        "\n{:?} Models ({} available):",
        completion_type,
        models.len()
    );

    for model in models.iter().take(limit) {
        let deprecated = deprecation_status(model);
        println!("  • {}{}", model.id, deprecated);
    }

    if models.len() > limit {
        println!("  ... and {} more", models.len() - limit);
    }

    Ok(())
}

/// Demo: Filter models by completion type
pub async fn demo_filter_by_completion_type(
    api: &ModelsApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 Example 4: Filter Models by Completion Type");
    println!("──────────────────────────────────────────────");

    let completion_types = get_completion_types_config();

    for completion_type in &completion_types {
        print_models_by_completion_type(api, completion_type, 3).await?;
    }

    Ok(())
}

/// Demo: Show available (non-deprecated) models
#[allow(dead_code)]
pub async fn demo_available_models(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n✅ Example 5: Available (Non-Deprecated) Models");
    println!("───────────────────────────────────────────");

    let available_models = api.list_available_models().await?;
    println!("Available models: {}", available_models.len());

    let mut by_family: HashMap<ModelFamily, Vec<&Model>> = HashMap::new();
    for model in &available_models {
        by_family.entry(model.family()).or_default().push(model);
    }

    for (family, models) in by_family {
        println!("  {:?}: {} models", family, models.len());
    }

    Ok(())
}

/// Demo: Show latest models from each family
#[allow(dead_code)]
pub async fn demo_latest_models(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🆕 Example 6: Latest Models from Each Family");
    println!("─────────────────────────────────────────────");

    let latest_models = api.get_latest_models().await?;

    for (family, model) in &latest_models {
        println!(
            "{:?}: {} (created: {})",
            family,
            model.id,
            chrono::DateTime::from_timestamp(model.created as i64, 0)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| model.created.to_string())
        );
    }

    Ok(())
}

/// Run all basic demonstration functions
pub async fn run_basic_demos(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    demo_list_all_models(api).await?;
    demo_retrieve_specific_model(api).await?;
    demo_group_models_by_family(api).await?;
    demo_filter_by_completion_type(api).await?;
    Ok(())
}
