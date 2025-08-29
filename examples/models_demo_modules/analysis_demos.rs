//! Model analysis and filtering demonstrations
//!
//! This module contains advanced model analysis operations:
//! - Finding models for specific use cases
//! - Getting recommended models
//! - Model statistics and analytics

use crate::helpers::*;
use openai_rust_sdk::api::models::ModelsApi;
use std::collections::HashMap;

/// Helper function to print use case results
async fn print_use_case_results(
    api: &ModelsApi,
    use_case: &UseCaseConfig,
    limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let suitable_models = api.find_suitable_models(&use_case.requirements).await?;
    println!(
        "\n{} ({} models found):",
        use_case.name,
        suitable_models.len()
    );

    for model in suitable_models.iter().take(limit) {
        let caps = model.capabilities();
        let context = format_context_tokens(caps.max_tokens);
        println!("  • {} (context: {})", model.id, context);
    }

    if suitable_models.len() > limit {
        println!("  ... and {} more", suitable_models.len() - limit);
    }

    Ok(())
}

/// Demo: Find models for specific use cases
pub async fn demo_find_models_for_use_cases(
    api: &ModelsApi,
) -> Result<Vec<UseCaseConfig>, Box<dyn std::error::Error>> {
    println!("\n🔎 Example 7: Find Models for Specific Use Cases");
    println!("────────────────────────────────────────────");

    let use_cases = get_use_case_configs();

    for use_case in &use_cases {
        print_use_case_results(api, use_case, 3).await?;
    }

    Ok(use_cases)
}

/// Demo: Get recommended models for use cases
pub async fn demo_recommended_models(
    api: &ModelsApi,
    use_cases: &[UseCaseConfig],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⭐ Example 8: Get Recommended Models for Use Cases");
    println!("─────────────────────────────────────────────");

    for use_case in use_cases {
        if let Some(recommended) = api.get_recommended_model(&use_case.requirements).await? {
            let caps = recommended.capabilities();
            println!(
                "{}: {} ({:?} tier)",
                use_case.name, recommended.id, caps.tier
            );
        } else {
            println!("{}: No suitable model found", use_case.name);
        }
    }

    Ok(())
}

/// Demo: Model statistics and analytics
pub async fn demo_model_statistics(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📈 Example 11: Model Statistics and Analytics");
    println!("─────────────────────────────────────────────");

    let stats = api.get_model_statistics().await?;

    println!("Overall Statistics:");
    println!("  Total Models: {}", stats.total_models);
    println!("  Available Models: {}", stats.available_models);
    println!("  Deprecated Models: {}", stats.deprecated_models);
    println!("  Availability Rate: {:.1}%", stats.availability_rate());
    println!("  Deprecation Rate: {:.1}%", stats.deprecation_rate());

    if let Some((family, count)) = stats.most_common_family() {
        println!("  Most Common Family: {family} ({count} models)");
    }

    if let Some((tier, count)) = stats.most_common_tier() {
        println!("  Most Common Tier: {tier} ({count} models)");
    }

    println!("\nFamily Distribution:");
    for (family, count) in &stats.family_distribution {
        println!("  {family}: {count} models");
    }

    println!("\nCompletion Type Support:");
    for (completion_type, count) in &stats.completion_type_distribution {
        println!("  {completion_type}: {count} models");
    }

    Ok(())
}

/// Demo: Show available (non-deprecated) models
pub async fn demo_available_models(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n✅ Example 5: Available (Non-Deprecated) Models");
    println!("───────────────────────────────────────────");

    let available_models = api.list_available_models().await?;
    println!("Available models: {}", available_models.len());

    let mut by_family: HashMap<
        openai_rust_sdk::models::models::ModelFamily,
        Vec<&openai_rust_sdk::models::models::Model>,
    > = HashMap::new();
    for model in &available_models {
        by_family.entry(model.family()).or_default().push(model);
    }

    for (family, models) in by_family {
        println!("  {:?}: {} models", family, models.len());
    }

    Ok(())
}

/// Demo: Show latest models from each family
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

/// Run advanced analysis demonstrations
pub async fn run_advanced_demos(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    demo_available_models(api).await?;
    demo_latest_models(api).await?;
    let use_cases = demo_find_models_for_use_cases(api).await?;
    demo_recommended_models(api, &use_cases).await?;
    demo_model_statistics(api).await?;

    // Import and run utility demos
    super::utility_demos::demo_model_utility_functions();
    super::utility_demos::demo_model_availability_check(api).await?;
    super::utility_demos::demo_advanced_model_filtering(api).await?;

    Ok(())
}
