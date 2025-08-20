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

use openai_rust_sdk::{
    api::models::{ModelUtils, ModelsApi},
    models::models::{CompletionType, Model, ModelFamily, ModelRequirements},
};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;

    println!("🤖 OpenAI Models API Demo");
    println!("=========================");

    let api = ModelsApi::new(api_key)?;

    // Example 1: List All Models
    println!("\n📋 Example 1: List All Available Models");
    println!("──────────────────────────────────────");

    let models_response = api.list_models().await?;
    println!("Total models available: {}", models_response.data.len());

    // Show first few models
    println!("\nFirst 5 models:");
    for (i, model) in models_response.data.iter().take(5).enumerate() {
        let family = model.family();
        let deprecated = if model.is_deprecated() {
            " (DEPRECATED)"
        } else {
            ""
        };
        println!("  {}. {} - {:?}{}", i + 1, model.id, family, deprecated);
        println!(
            "     Created: {}, Owner: {}",
            chrono::DateTime::from_timestamp(model.created as i64, 0)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| model.created.to_string()),
            model.owned_by
        );
    }

    // Example 2: Retrieve Specific Model Details
    println!("\n🔍 Example 2: Retrieve Specific Model Details");
    println!("─────────────────────────────────────────────");

    let model_id = "gpt-4";
    match api.retrieve_model(model_id).await {
        Ok(model) => {
            println!("Model: {}", model.id);
            println!("Owner: {}", model.owned_by);
            println!(
                "Created: {}",
                chrono::DateTime::from_timestamp(model.created as i64, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| model.created.to_string())
            );

            let capabilities = model.capabilities();
            println!("\nCapabilities:");
            println!("  Family: {:?}", capabilities.family);
            println!("  Tier: {:?}", capabilities.tier);
            if let Some(max_tokens) = capabilities.max_tokens {
                println!("  Max Tokens: {max_tokens}");
            }
            if let Some(cutoff) = &capabilities.training_cutoff {
                println!("  Training Cutoff: {cutoff}");
            }
            println!("  Completion Types: {:?}", capabilities.completion_types);
            println!(
                "  Function Calling: {}",
                capabilities.supports_function_calling
            );
            println!("  Vision: {}", capabilities.supports_vision);
            println!(
                "  Code Interpreter: {}",
                capabilities.supports_code_interpreter
            );

            if let (Some(input_cost), Some(output_cost)) = (
                capabilities.input_cost_per_1m_tokens,
                capabilities.output_cost_per_1m_tokens,
            ) {
                println!("  Input Cost: ${input_cost:.2}/1M tokens");
                println!("  Output Cost: ${output_cost:.2}/1M tokens");
            }
        }
        Err(e) => println!("Error retrieving model {model_id}: {e}"),
    }

    // Example 3: Group Models by Family
    println!("\n👨‍👩‍👧‍👦 Example 3: Group Models by Family");
    println!("──────────────────────────────────────────");

    let grouped_models = api.group_models_by_family().await?;

    for (family, models) in &grouped_models {
        if !models.is_empty() {
            println!("\n{:?} Family ({} models):", family, models.len());
            for model in models.iter().take(3) {
                // Show first 3 of each family
                let deprecated = if model.is_deprecated() {
                    " (DEPRECATED)"
                } else {
                    ""
                };
                println!("  • {}{}", model.id, deprecated);
            }
            if models.len() > 3 {
                println!("  ... and {} more", models.len() - 3);
            }
        }
    }

    // Example 4: Filter Models by Completion Type
    println!("\n🎯 Example 4: Filter Models by Completion Type");
    println!("──────────────────────────────────────────────");

    let completion_types = [
        CompletionType::Chat,
        CompletionType::Image,
        CompletionType::Audio,
        CompletionType::Embeddings,
    ];

    for completion_type in &completion_types {
        let models = api
            .list_models_by_completion_type(completion_type.clone())
            .await?;
        println!(
            "\n{:?} Models ({} available):",
            completion_type,
            models.len()
        );

        for model in models.iter().take(3) {
            let deprecated = if model.is_deprecated() {
                " (DEPRECATED)"
            } else {
                ""
            };
            println!("  • {}{}", model.id, deprecated);
        }
        if models.len() > 3 {
            println!("  ... and {} more", models.len() - 3);
        }
    }

    // Example 5: Find Available (Non-Deprecated) Models
    println!("\n✅ Example 5: Available (Non-Deprecated) Models");
    println!("───────────────────────────────────────────────");

    let available_models = api.list_available_models().await?;
    println!("Available models: {}", available_models.len());

    let mut by_family: HashMap<ModelFamily, Vec<&Model>> = HashMap::new();
    for model in &available_models {
        by_family.entry(model.family()).or_default().push(model);
    }

    for (family, models) in by_family {
        println!("  {:?}: {} models", family, models.len());
    }

    // Example 6: Get Latest Models from Each Family
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

    // Example 7: Find Models for Specific Use Cases
    println!("\n🔎 Example 7: Find Models for Specific Use Cases");
    println!("────────────────────────────────────────────────");

    let use_cases = [
        ("Basic Chat", ModelRequirements::chat()),
        ("Function Calling", ModelRequirements::function_calling()),
        ("Vision Tasks", ModelRequirements::vision()),
        ("Code Tasks", ModelRequirements::code()),
        (
            "High Context (100k+ tokens)",
            ModelRequirements::high_context(100_000),
        ),
    ];

    for (use_case, requirements) in &use_cases {
        let suitable_models = api.find_suitable_models(requirements).await?;
        println!("\n{} ({} models found):", use_case, suitable_models.len());

        for model in suitable_models.iter().take(3) {
            let caps = model.capabilities();
            let context = caps
                .max_tokens
                .map(|t| format!("{}k", t / 1000))
                .unwrap_or_else(|| "N/A".to_string());
            println!("  • {} (context: {})", model.id, context);
        }
        if suitable_models.len() > 3 {
            println!("  ... and {} more", suitable_models.len() - 3);
        }
    }

    // Example 8: Get Recommended Models
    println!("\n⭐ Example 8: Get Recommended Models for Use Cases");
    println!("─────────────────────────────────────────────────");

    for (use_case, requirements) in &use_cases {
        if let Some(recommended) = api.get_recommended_model(requirements).await? {
            let caps = recommended.capabilities();
            println!("{}: {} ({:?} tier)", use_case, recommended.id, caps.tier);
        } else {
            println!("{use_case}: No suitable model found");
        }
    }

    // Example 9: Cost Comparison
    println!("\n💰 Example 9: Model Cost Comparison");
    println!("───────────────────────────────────");

    let chat_models = ["gpt-4", "gpt-4-turbo", "gpt-4o", "gpt-3.5-turbo"];
    let monthly_input_tokens = 1_000_000u64; // 1M input tokens per month
    let monthly_output_tokens = 500_000u64; // 500k output tokens per month

    println!(
        "Estimated monthly costs for {}M input + {}k output tokens:",
        monthly_input_tokens / 1_000_000,
        monthly_output_tokens / 1000
    );

    let costs = api
        .compare_model_costs(&chat_models, monthly_input_tokens, monthly_output_tokens)
        .await?;

    for model_id in &chat_models {
        if let Some(cost_option) = costs.get(*model_id) {
            match cost_option {
                Some(cost) => println!("  {model_id}: ${cost:.2}/month"),
                None => println!("  {model_id}: Cost data not available"),
            }
        }
    }

    // Example 10: Get Models Sorted by Cost
    println!("\n📊 Example 10: Models Sorted by Cost (Cheapest First)");
    println!("─────────────────────────────────────────────────────");

    let chat_requirements = ModelRequirements::chat();
    let models_by_cost = api
        .get_models_by_cost(
            &chat_requirements,
            monthly_input_tokens,
            monthly_output_tokens,
        )
        .await?;

    println!("Chat models sorted by estimated monthly cost:");
    for (i, (model, cost)) in models_by_cost.iter().take(5).enumerate() {
        match cost {
            Some(cost) => println!("  {}. {}: ${:.2}/month", i + 1, model.id, cost),
            None => println!("  {}. {}: Cost data not available", i + 1, model.id),
        }
    }

    // Example 11: Model Statistics
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

    // Example 12: Model Utility Functions
    println!("\n🛠️ Example 12: Model Utility Functions");
    println!("──────────────────────────────────────");

    // Extract base model names
    let versioned_models = ["gpt-3.5-turbo-0613", "gpt-4-32k", "text-davinci-003"];
    println!("Base model name extraction:");
    for model_id in &versioned_models {
        let base_name = ModelUtils::extract_base_model_name(model_id);
        println!("  {model_id} → {base_name}");
    }

    // Check family relationships
    println!("\nFamily relationship checks:");
    let model_pairs = [
        ("gpt-4", "gpt-4-turbo"),
        ("gpt-3.5-turbo", "gpt-3.5-turbo-16k"),
        ("gpt-4", "dall-e-3"),
    ];

    for (model1, model2) in &model_pairs {
        let same_family = ModelUtils::are_same_family(model1, model2);
        println!(
            "  {} and {}: {}",
            model1,
            model2,
            if same_family {
                "Same family"
            } else {
                "Different families"
            }
        );
    }

    // Example 13: Model Availability Check
    println!("\n🔍 Example 13: Check Model Availability");
    println!("──────────────────────────────────────");

    let models_to_check = ["gpt-4", "gpt-3.5-turbo", "nonexistent-model"];

    for model_id in &models_to_check {
        match api.is_model_available(model_id).await {
            Ok(available) => {
                println!(
                    "  {}: {}",
                    model_id,
                    if available {
                        "✅ Available"
                    } else {
                        "❌ Not available"
                    }
                );
            }
            Err(e) => {
                println!("  {model_id}: ❓ Error checking availability: {e}");
            }
        }
    }

    // Example 14: Advanced Model Filtering
    println!("\n🎯 Example 14: Advanced Model Filtering");
    println!("─────────────────────────────────────────");

    // Custom requirements for a specific use case
    let advanced_requirements = ModelRequirements {
        completion_types: vec![CompletionType::Chat],
        min_max_tokens: Some(32_000),
        requires_function_calling: true,
        requires_vision: false,
        requires_code_interpreter: false,
        exclude_deprecated: true,
    };

    let advanced_models = api.find_suitable_models(&advanced_requirements).await?;
    println!("Models for advanced chat with 32k+ context and function calling:");

    for model in &advanced_models {
        let caps = model.capabilities();
        println!(
            "  • {} ({:?}, {}k context, functions: {})",
            model.id,
            caps.tier,
            caps.max_tokens.unwrap_or(0) / 1000,
            caps.supports_function_calling
        );
    }

    // Summary
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

    Ok(())
}
