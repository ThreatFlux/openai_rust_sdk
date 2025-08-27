//! Model utility and advanced filtering demonstrations
//!
//! This module contains utility functions and advanced filtering operations:
//! - Model utility functions
//! - Model availability checking  
//! - Advanced model filtering

use openai_rust_sdk::{
    api::models::{ModelUtils, ModelsApi},
    models::models::ModelRequirements,
};

/// Demo: Model utility functions
pub fn demo_model_utility_functions() {
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

/// Demo: Check model availability
pub async fn demo_model_availability_check(
    api: &ModelsApi,
) -> Result<(), Box<dyn std::error::Error>> {
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

/// Demo: Advanced model filtering
pub async fn demo_advanced_model_filtering(
    api: &ModelsApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 Example 14: Advanced Model Filtering");
    println!("─────────────────────────────────────────");

    let advanced_requirements = ModelRequirements {
        completion_types: vec![openai_rust_sdk::models::models::CompletionType::Chat],
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
