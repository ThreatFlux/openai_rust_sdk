#![allow(clippy::pedantic, clippy::nursery)]
//! GPT-5 feature demonstration
//!
//! This example showcases the advanced features of GPT-5 models including:
//! - Reasoning effort control
//! - Verbosity settings
//! - Model selection based on use case
//! - Chain of thought continuation
//! - Optimal settings for different tasks

use openai_rust_sdk::{
    api::{GPT5Api, GPT5RequestBuilder},
    models::gpt5::{models, GPT5ModelSelector, ReasoningEffort, Verbosity},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GPT-5 Advanced Features Demo");
    println!("=============================\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| "demo_key".to_string());

    if api_key == "demo_key" {
        println!("⚠️  OPENAI_API_KEY not set. Running in demo mode.\n");
        demo_mode();
    } else {
        // Run actual API calls
        run_examples(api_key).await?;
    }

    Ok(())
}

async fn run_examples(api_key: String) -> Result<(), Box<dyn std::error::Error>> {
    let api = GPT5Api::new(api_key)?;

    // Example 1: Minimal reasoning for fastest response
    println!("1. Minimal Reasoning (Fastest Response)");
    println!("----------------------------------------");
    let response = api
        .create_minimal_response(models::GPT_5, "Write a haiku about code.")
        .await?;
    println!("Response: {}\n", response.output_text());

    // Example 2: Low latency with custom verbosity
    println!("2. Fast Response with Low Verbosity");
    println!("------------------------------------");
    let response = api
        .create_fast_response(models::GPT_5, "What is 42?", Verbosity::Low)
        .await?;
    println!("Response: {}\n", response.output_text());

    // Example 3: Complex reasoning task
    println!("3. Complex Reasoning Task");
    println!("-------------------------");
    let response = api
        .create_complex_response(
            "How much gold would it take to coat the Statue of Liberty in a 1mm layer?",
            Some("Think step by step and show your calculations.".to_string()),
        )
        .await?;
    println!("Response: {}\n", response.output_text());

    // Example 4: Coding task with optimal settings
    println!("4. Coding Task");
    println!("--------------");
    let response = api
        .create_coding_response(
            "Write a Python function to find the nth Fibonacci number using memoization.",
            Verbosity::Medium,
        )
        .await?;
    println!("Response:\n{}\n", response.output_text());

    // Example 5: Cost-optimized response with GPT-5-mini
    println!("5. Cost-Optimized Response (GPT-5-mini)");
    println!("----------------------------------------");
    let response = api
        .create_cost_optimized_response("Explain photosynthesis in simple terms.")
        .await?;
    println!("Response: {}\n", response.output_text());

    // Example 6: High-throughput response with GPT-5-nano
    println!("6. High-Throughput Response (GPT-5-nano)");
    println!("-----------------------------------------");
    let response = api
        .create_high_throughput_response("Classify this sentiment: 'I love this product!'")
        .await?;
    println!("Response: {}\n", response.output_text());

    // Example 7: Multi-turn conversation with CoT
    println!("7. Multi-turn Conversation with Chain of Thought");
    println!("-------------------------------------------------");
    let first_response = api
        .create_reasoned_response(
            models::GPT_5,
            "What are the key principles of functional programming?",
            ReasoningEffort::Medium,
            Verbosity::Medium,
        )
        .await?;
    println!("First response: {}", first_response.output_text());

    // Continue conversation using previous response ID
    if let Some(response_id) = first_response.id {
        let followup = api
            .continue_conversation(
                models::GPT_5,
                "How do these principles apply to Rust?",
                response_id,
                ReasoningEffort::Low, // Less reasoning needed due to context
            )
            .await?;
        println!("Follow-up: {}\n", followup.output_text());
    }

    // Example 8: Using the request builder
    println!("8. Request Builder Pattern");
    println!("--------------------------");
    let request = GPT5RequestBuilder::new()
        .gpt5()
        .input("Explain quantum computing to a 10-year-old.")
        .instructions("Use simple analogies and avoid technical jargon.")
        .low_reasoning() // Simple explanation doesn't need complex reasoning
        .high_verbosity() // Detailed explanation needed
        .temperature(0.7)
        .max_tokens(500)
        .build()?;

    println!("Built request for model: {}", request.model);
    println!("Reasoning: {:?}", request.reasoning);
    println!("Text config: {:?}\n", request.text);

    // Example 9: Model selection based on use case
    println!("9. Model Selection Helper");
    println!("-------------------------");
    println!(
        "For complex reasoning: {}",
        GPT5ModelSelector::for_complex_reasoning()
    );
    println!(
        "For cost optimization: {}",
        GPT5ModelSelector::for_cost_optimized()
    );
    println!(
        "For high throughput: {}",
        GPT5ModelSelector::for_high_throughput()
    );
    println!("For coding: {}", GPT5ModelSelector::for_coding());
    println!("For chat: {}", GPT5ModelSelector::for_chat());

    println!("\n10. Migration Recommendations");
    println!("------------------------------");
    println!("From o3 → {}", GPT5ModelSelector::migration_from("o3"));
    println!(
        "From gpt-4.1 → {}",
        GPT5ModelSelector::migration_from("gpt-4.1")
    );
    println!(
        "From gpt-4.1-mini → {}",
        GPT5ModelSelector::migration_from("gpt-4.1-mini")
    );
    println!(
        "From gpt-3.5-turbo → {}",
        GPT5ModelSelector::migration_from("gpt-3.5-turbo")
    );

    Ok(())
}

fn demo_mode() {
    println!("Demo Mode - Showing GPT-5 Configuration Examples");
    println!("=================================================\n");

    // Example 1: Different reasoning levels
    println!("1. Reasoning Effort Levels:");
    println!("   - Minimal: Fastest time-to-first-token, best for simple tasks");
    println!("   - Low: Balance of speed and quality");
    println!("   - Medium: Default, good for most tasks");
    println!("   - High: Most thorough reasoning for complex problems\n");

    // Example 2: Verbosity control
    println!("2. Verbosity Control:");
    println!("   - Low: Concise answers (e.g., '42' or short SQL queries)");
    println!("   - Medium: Balanced responses");
    println!("   - High: Detailed explanations with examples\n");

    // Example 3: Model variants
    println!("3. GPT-5 Model Family:");
    println!(
        "   - {}: Best for complex reasoning and code",
        models::GPT_5
    );
    println!(
        "   - {}: Cost-optimized, balanced capabilities",
        models::GPT_5_MINI
    );
    println!(
        "   - {}: High-throughput, simple tasks\n",
        models::GPT_5_NANO
    );

    // Example 4: Optimal settings for different tasks
    println!("4. Optimal Settings by Task Type:");
    println!("   Coding:");
    println!("     - Model: {}", models::GPT_5);
    println!("     - Reasoning: Medium");
    println!("     - Verbosity: Medium-High\n");

    println!("   Quick Answers:");
    println!(
        "     - Model: {} or {}",
        models::GPT_5_MINI,
        models::GPT_5_NANO
    );
    println!("     - Reasoning: Minimal");
    println!("     - Verbosity: Low\n");

    println!("   Complex Analysis:");
    println!("     - Model: {}", models::GPT_5);
    println!("     - Reasoning: High");
    println!("     - Verbosity: High\n");

    println!("   Chat Applications:");
    println!("     - Model: {}", models::GPT_5_CHAT_LATEST);
    println!("     - Reasoning: Low-Medium");
    println!("     - Verbosity: Medium\n");

    // Example 5: Request builder usage
    println!("5. Building a GPT-5 Request:");
    let request = GPT5RequestBuilder::new()
        .gpt5_mini()
        .input("Example input")
        .minimal_reasoning()
        .low_verbosity()
        .temperature(0.3)
        .build()
        .unwrap();

    println!("   Model: {}", request.model);
    println!("   Reasoning: {:?}", request.reasoning);
    println!("   Text: {:?}", request.text);
    println!("   Temperature: {:?}\n", request.temperature);

    // Example 6: Multi-turn conversation
    println!("6. Multi-turn Conversations:");
    println!("   - Use previous_response_id to maintain chain of thought");
    println!("   - Reduces re-reasoning and improves context awareness");
    println!("   - Lower latency due to cached reasoning\n");

    // Example 7: Custom tools with minimal reasoning
    println!("7. Function Calling with GPT-5:");
    println!("   - Use minimal reasoning for tool selection");
    println!("   - Faster tool calls with low verbosity");
    println!("   - Preambles for transparency\n");

    println!("To run actual API calls, set OPENAI_API_KEY environment variable:");
    println!("  export OPENAI_API_KEY=your_api_key_here");
    println!("  cargo run --example gpt5_features");
}
