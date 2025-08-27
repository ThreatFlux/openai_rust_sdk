//! Model cost comparison and optimization demonstrations
//!
//! This module contains cost-related model analysis operations:
//! - Model cost comparisons
//! - Models sorted by cost
//! - Cost-effective model selection

use crate::helpers::*;
use openai_rust_sdk::{api::models::ModelsApi, models::models::ModelRequirements};

/// Demo: Model cost comparison
pub async fn demo_cost_comparison(
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

/// Demo: Models sorted by cost (cheapest first)
pub async fn demo_models_sorted_by_cost(
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

/// Run cost and analytics demonstrations
pub async fn run_cost_and_analytics_demos(
    api: &ModelsApi,
) -> Result<(), Box<dyn std::error::Error>> {
    const MONTHLY_INPUT_TOKENS: u64 = 1_000_000;
    const MONTHLY_OUTPUT_TOKENS: u64 = 500_000;

    demo_cost_comparison(api, MONTHLY_INPUT_TOKENS, MONTHLY_OUTPUT_TOKENS).await?;
    demo_models_sorted_by_cost(api, MONTHLY_INPUT_TOKENS, MONTHLY_OUTPUT_TOKENS).await?;

    Ok(())
}
