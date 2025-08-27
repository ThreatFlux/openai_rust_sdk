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

use openai_rust_sdk::{
    api::models::{ModelUtils, ModelsApi},
    models::models::{CompletionType, Model, ModelFamily, ModelRequirements},
};
use std::collections::HashMap;
use std::env;

/// Demo configuration for each example
struct DemoConfig {
    title: &'static str,
    description: &'static str,
}

/// Configuration for use case demonstrations
struct UseCaseConfig {
    name: &'static str,
    requirements: ModelRequirements,
}

/// Configuration for model cost comparisons
struct CostConfig {
    models: Vec<&'static str>,
    monthly_input: u64,
    monthly_output: u64,
}

/// Helper function to format model creation date
fn format_creation_date(timestamp: u64) -> String {
    chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| timestamp.to_string())
}

/// Helper function to format deprecation status
fn deprecation_status(model: &Model) -> &'static str {
    if model.is_deprecated() {
        " (DEPRECATED)"
    } else {
        ""
    }
}

/// Helper function to print model summary
fn print_model_summary(model: &Model, index: usize) {
    let family = model.family();
    let deprecated = deprecation_status(model);
    println!("  {}. {} - {:?}{}", index + 1, model.id, family, deprecated);
    println!(
        "     Created: {}, Owner: {}",
        format_creation_date(model.created),
        model.owned_by
    );
}

/// Helper function to print limited model list with optional count
fn print_limited_models<'a>(
    models: impl Iterator<Item = &'a Model>,
    limit: usize,
    total_count: usize,
) {
    for (_i, model) in models.take(limit).enumerate() {
        let deprecated = deprecation_status(model);
        println!("  • {}{}", model.id, deprecated);
    }

    if total_count > limit {
        println!("  ... and {} more", total_count - limit);
    }
}

async fn demo_list_all_models(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
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

async fn demo_retrieve_specific_model(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
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

/// Helper function to format detailed timestamp
fn format_detailed_timestamp(timestamp: u64) -> String {
    chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| timestamp.to_string())
}

/// Helper function to print basic model info
fn print_basic_model_info(model: &Model) {
    println!("Model: {}", model.id);
    println!("Owner: {}", model.owned_by);
    println!("Created: {}", format_detailed_timestamp(model.created));
}

/// Helper function to print capabilities info
fn print_capabilities_info(capabilities: &openai_rust_sdk::models::models::ModelCapabilities) {
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
}

/// Helper function to print cost info
fn print_cost_info(capabilities: &openai_rust_sdk::models::models::ModelCapabilities) {
    if let (Some(input_cost), Some(output_cost)) = (
        capabilities.input_cost_per_1m_tokens,
        capabilities.output_cost_per_1m_tokens,
    ) {
        println!("  Input Cost: ${input_cost:.2}/1M tokens");
        println!("  Output Cost: ${output_cost:.2}/1M tokens");
    }
}

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

async fn demo_group_models_by_family(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n👨‍👩‍👧‍👦 Example 3: Group Models by Family");
    println!("──────────────────────────────────────");

    let grouped_models = api.group_models_by_family().await?;
    print_model_families(&grouped_models, 3);

    Ok(())
}

/// Helper function to get completion types to demo
fn get_completion_types_config() -> Vec<CompletionType> {
    vec![
        CompletionType::Chat,
        CompletionType::Image,
        CompletionType::Audio,
        CompletionType::Embeddings,
    ]
}

/// Helper function to print models by completion type
async fn print_models_by_completion_type(
    api: &ModelsApi,
    completion_type: &CompletionType,
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

async fn demo_filter_by_completion_type(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 Example 4: Filter Models by Completion Type");
    println!("──────────────────────────────────────────────");

    let completion_types = get_completion_types_config();

    for completion_type in &completion_types {
        print_models_by_completion_type(api, completion_type, 3).await?;
    }

    Ok(())
}

async fn demo_available_models(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
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

async fn demo_latest_models(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
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

/// Helper function to get use case configurations
fn get_use_case_configs() -> Vec<UseCaseConfig> {
    vec![
        UseCaseConfig {
            name: "Basic Chat",
            requirements: ModelRequirements::chat(),
        },
        UseCaseConfig {
            name: "Function Calling",
            requirements: ModelRequirements::function_calling(),
        },
        UseCaseConfig {
            name: "Vision Tasks",
            requirements: ModelRequirements::vision(),
        },
        UseCaseConfig {
            name: "Code Tasks",
            requirements: ModelRequirements::code_interpreter(),
        },
        UseCaseConfig {
            name: "High Context (100k+ tokens)",
            requirements: ModelRequirements::high_context(100_000),
        },
    ]
}

/// Helper function to format context tokens
fn format_context_tokens(max_tokens: Option<u32>) -> String {
    max_tokens
        .map(|t| format!("{}k", t / 1000))
        .unwrap_or_else(|| "N/A".to_string())
}

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

async fn demo_find_models_for_use_cases(
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

async fn demo_remaining_examples(
    api: &ModelsApi,
    use_cases: &[UseCaseConfig],
) -> Result<(), Box<dyn std::error::Error>> {
    demo_recommended_models(api, use_cases).await?;
    run_cost_and_analytics_demos(api).await?;
    Ok(())
}

async fn run_cost_and_analytics_demos(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    const MONTHLY_INPUT_TOKENS: u64 = 1_000_000;
    const MONTHLY_OUTPUT_TOKENS: u64 = 500_000;

    demo_additional_features(api, MONTHLY_INPUT_TOKENS, MONTHLY_OUTPUT_TOKENS).await
}

async fn demo_recommended_models(
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

async fn demo_additional_features(
    api: &ModelsApi,
    monthly_input_tokens: u64,
    monthly_output_tokens: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    demo_cost_comparison(api, monthly_input_tokens, monthly_output_tokens).await?;
    demo_models_sorted_by_cost(api, monthly_input_tokens, monthly_output_tokens).await?;
    demo_model_statistics(api).await?;
    demo_model_utility_functions();
    demo_model_availability_check(api).await?;
    demo_advanced_model_filtering(api).await?;

    Ok(())
}

/// Helper function to get chat models for cost comparison
fn get_chat_models_for_comparison() -> Vec<&'static str> {
    vec!["gpt-4", "gpt-4-turbo", "gpt-4o", "gpt-3.5-turbo"]
}

/// Helper function to print cost results
fn print_cost_results(costs: &std::collections::HashMap<String, Option<f64>>, models: &[&str]) {
    for model_id in models {
        if let Some(cost_option) = costs.get(*model_id) {
            match cost_option {
                Some(cost) => println!("  {model_id}: ${cost:.2}/month"),
                None => println!("  {model_id}: Cost data not available"),
            }
        }
    }
}

async fn demo_cost_comparison(
    api: &ModelsApi,
    monthly_input_tokens: u64,
    monthly_output_tokens: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💰 Example 9: Model Cost Comparison");
    println!("───────────────────────────────────");

    let chat_models = get_chat_models_for_comparison();

    println!(
        "Estimated monthly costs for {}M input + {}k output tokens:",
        monthly_input_tokens / 1_000_000,
        monthly_output_tokens / 1000
    );

    let costs = api
        .compare_model_costs(&chat_models, monthly_input_tokens, monthly_output_tokens)
        .await?;

    print_cost_results(&costs, &chat_models);

    Ok(())
}

/// Helper function to print sorted cost models
fn print_sorted_cost_models(models_by_cost: &[(Model, Option<f64>)], limit: usize) {
    println!("Chat models sorted by estimated monthly cost:");
    for (i, (model, cost)) in models_by_cost.iter().take(limit).enumerate() {
        match cost {
            Some(cost) => println!("  {}. {}: ${:.2}/month", i + 1, model.id, cost),
            None => println!("  {}. {}: Cost data not available", i + 1, model.id),
        }
    }
}

async fn demo_models_sorted_by_cost(
    api: &ModelsApi,
    monthly_input_tokens: u64,
    monthly_output_tokens: u64,
) -> Result<(), Box<dyn std::error::Error>> {
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

    print_sorted_cost_models(&models_by_cost, 5);

    Ok(())
}

async fn demo_model_statistics(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
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

fn demo_model_utility_functions() {
    println!("\n🛠️ Example 12: Model Utility Functions");
    println!("──────────────────────────────────────");

    let versioned_models = ["gpt-3.5-turbo-0613", "gpt-4-32k", "text-davinci-003"];
    println!("Base model name extraction:");
    for model_id in &versioned_models {
        let base_name = ModelUtils::extract_base_model_name(model_id);
        println!("  {model_id} → {base_name}");
    }

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
}

async fn demo_model_availability_check(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

async fn demo_advanced_model_filtering(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 Example 14: Advanced Model Filtering");
    println!("─────────────────────────────────────────");

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

    Ok(())
}

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

async fn run_basic_demos(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    demo_list_all_models(api).await?;
    demo_retrieve_specific_model(api).await?;
    demo_group_models_by_family(api).await?;
    demo_filter_by_completion_type(api).await?;
    Ok(())
}

async fn run_advanced_demos(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    demo_available_models(api).await?;
    demo_latest_models(api).await?;
    let use_cases = demo_find_models_for_use_cases(api).await?;
    demo_remaining_examples(api, &use_cases).await?;
    Ok(())
}

async fn run_all_demos(api: &ModelsApi) -> Result<(), Box<dyn std::error::Error>> {
    run_basic_demos(api).await?;
    run_advanced_demos(api).await?;
    Ok(())
}

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
