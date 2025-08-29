//! Utility functions and shared data structures for Models API demos

use openai_rust_sdk::models::models::{CompletionType, Model, ModelRequirements};
use std::collections::HashMap;

/// Demo configuration for each example
#[allow(dead_code)]
pub struct DemoConfig {
    pub title: &'static str,
    pub description: &'static str,
}

/// Configuration for use case demonstrations
pub struct UseCaseConfig {
    pub name: &'static str,
    pub requirements: ModelRequirements,
}

/// Configuration for model cost comparisons
#[allow(dead_code)]
pub struct CostConfig {
    pub models: Vec<&'static str>,
    pub monthly_input: u64,
    pub monthly_output: u64,
}

/// Helper function to format model creation date
pub fn format_creation_date(timestamp: u64) -> String {
    chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| timestamp.to_string())
}

/// Helper function to format detailed timestamp
pub fn format_detailed_timestamp(timestamp: u64) -> String {
    chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| timestamp.to_string())
}

/// Helper function to format context tokens
pub fn format_context_tokens(max_tokens: Option<u32>) -> String {
    max_tokens
        .map(|t| format!("{}k", t / 1000))
        .unwrap_or_else(|| "N/A".to_string())
}

/// Helper function to format deprecation status
pub fn deprecation_status(model: &Model) -> &'static str {
    if model.is_deprecated() {
        " (DEPRECATED)"
    } else {
        ""
    }
}

/// Helper function to print model summary
pub fn print_model_summary(model: &Model, index: usize) {
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
#[allow(dead_code)]
pub fn print_limited_models<'a>(
    models: impl Iterator<Item = &'a Model>,
    limit: usize,
    total_count: usize,
) {
    for model in models.take(limit) {
        let deprecated = deprecation_status(model);
        println!("  • {}{}", model.id, deprecated);
    }

    if total_count > limit {
        println!("  ... and {} more", total_count - limit);
    }
}

/// Helper function to print basic model info
pub fn print_basic_model_info(model: &Model) {
    println!("Model: {}", model.id);
    println!("Owner: {}", model.owned_by);
    println!("Created: {}", format_detailed_timestamp(model.created));
}

/// Helper function to print capabilities info
pub fn print_capabilities_info(capabilities: &openai_rust_sdk::models::models::ModelCapabilities) {
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
pub fn print_cost_info(capabilities: &openai_rust_sdk::models::models::ModelCapabilities) {
    if let (Some(input_cost), Some(output_cost)) = (
        capabilities.input_cost_per_1m_tokens,
        capabilities.output_cost_per_1m_tokens,
    ) {
        println!("  Input Cost: ${input_cost:.2}/1M tokens");
        println!("  Output Cost: ${output_cost:.2}/1M tokens");
    }
}

/// Helper function to get use case configurations
pub fn get_use_case_configs() -> Vec<UseCaseConfig> {
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

/// Helper function to get completion types to demo
pub fn get_completion_types_config() -> Vec<CompletionType> {
    vec![
        CompletionType::Chat,
        CompletionType::Image,
        CompletionType::Audio,
        CompletionType::Embeddings,
    ]
}

/// Helper function to get chat models for cost comparison
pub fn get_chat_models_for_comparison() -> Vec<&'static str> {
    vec!["gpt-4", "gpt-4-turbo", "gpt-4o", "gpt-3.5-turbo"]
}

/// Helper function to print cost results
pub fn print_cost_results(costs: &HashMap<String, Option<f64>>, models: &[&str]) {
    for model_id in models {
        if let Some(cost_option) = costs.get(*model_id) {
            match cost_option {
                Some(cost) => println!("  {model_id}: ${cost:.2}/month"),
                None => println!("  {model_id}: Cost data not available"),
            }
        }
    }
}

/// Helper function to print sorted cost models
pub fn print_sorted_cost_models(models_by_cost: &[(Model, Option<f64>)], limit: usize) {
    println!("Chat models sorted by estimated monthly cost:");
    for (i, (model, cost)) in models_by_cost.iter().take(limit).enumerate() {
        match cost {
            Some(cost) => println!("  {}. {}: ${:.2}/month", i + 1, model.id, cost),
            None => println!("  {}. {}: Cost data not available", i + 1, model.id),
        }
    }
}
