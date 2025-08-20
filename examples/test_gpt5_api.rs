#![allow(clippy::pedantic, clippy::nursery)]
//! Test GPT-5 API capabilities
//!
//! This example demonstrates all GPT-5 features including:
//! - Reasoning with different effort levels
//! - Text configuration (verbosity, length)
//! - Multi-turn conversations
//! - Enhanced tools and function calling

use openai_rust_sdk::{
    api::gpt5::GPT5Api,
    error::Result,
    models::{
        gpt5::{models, ReasoningEffort, Verbosity},
        responses::{Message, MessageContentInput, MessageRole, ResponseInput},
    },
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| "test-key".to_string());

    println!("\n🚀 Testing GPT-5 API Capabilities\n");
    println!("{}", "=".repeat(60));

    // Initialize GPT-5 API
    let gpt5_api = GPT5Api::new(api_key)?;
    println!("✅ GPT-5 API initialized successfully");

    // Test 1: Basic GPT-5 Request with Minimal Reasoning
    println!("\n📝 Test 1: Basic GPT-5 Request (Minimal Reasoning)");
    println!("{}", "-".repeat(60));

    let basic_messages = vec![Message {
        role: MessageRole::User,
        content: MessageContentInput::Text("What is 2+2? Answer in one word.".to_string()),
    }];

    match gpt5_api
        .create_minimal_response(models::GPT_5, ResponseInput::Messages(basic_messages))
        .await
    {
        Ok(response) => {
            println!("✅ Basic request successful!");
            if let Some(choice) = response.choices.first() {
                if let Some(content) = &choice.message.content {
                    println!("   Response: {content}");
                }
            }
            println!("   Model: {}", response.model);
        }
        Err(e) => {
            println!("❌ Basic request failed: {e}");
            if e.to_string().contains("401") {
                println!("   Note: Invalid API key");
            } else if e.to_string().contains("model") {
                println!("   Note: GPT-5 model may not be available yet");
            }
        }
    }

    // Test 2: Fast Response with Low Verbosity
    println!("\n⚡ Test 2: Fast Response (Low Reasoning, Low Verbosity)");
    println!("{}", "-".repeat(60));

    let fast_messages = vec![Message {
        role: MessageRole::User,
        content: MessageContentInput::Text(
            "Explain quantum computing in one sentence.".to_string(),
        ),
    }];

    match gpt5_api
        .create_fast_response(
            models::GPT_5,
            ResponseInput::Messages(fast_messages),
            Verbosity::Low,
        )
        .await
    {
        Ok(response) => {
            println!("✅ Fast response successful!");
            if let Some(choice) = response.choices.first() {
                if let Some(content) = &choice.message.content {
                    println!("   Response: {content}");
                }
            }
        }
        Err(e) => {
            println!("❌ Fast response failed: {e}");
        }
    }

    // Test 3: Different Reasoning Levels
    println!("\n🧠 Test 3: Different Reasoning Levels");
    println!("{}", "-".repeat(60));

    let reasoning_messages = vec![Message {
        role: MessageRole::User,
        content: MessageContentInput::Text(
            "Solve this step by step: If a train travels 120 miles in 2 hours, what is its speed?"
                .to_string(),
        ),
    }];

    // Test different reasoning levels
    let reasoning_levels = vec![
        ("Low", ReasoningEffort::Low),
        ("Medium", ReasoningEffort::Medium),
        ("High", ReasoningEffort::High),
    ];

    for (level, effort) in reasoning_levels {
        println!("\n   Testing {level} reasoning effort:");

        match gpt5_api
            .create_reasoned_response(
                models::GPT_5,
                ResponseInput::Messages(reasoning_messages.clone()),
                effort,
                Verbosity::Medium,
            )
            .await
        {
            Ok(response) => {
                println!("   ✅ {level} reasoning successful");
                if let Some(usage) = &response.usage {
                    println!("      Tokens used: {}", usage.total_tokens);
                }
                if let Some(choice) = response.choices.first() {
                    if let Some(content) = &choice.message.content {
                        let preview = if content.len() > 100 {
                            format!("{}...", &content[..100])
                        } else {
                            content.clone()
                        };
                        println!("      Response preview: {preview}");
                    }
                }
            }
            Err(e) => {
                println!("   ❌ {level} reasoning failed: {e}");
            }
        }
    }

    // Test 4: Complex Reasoning
    println!("\n🔬 Test 4: Complex Reasoning Task");
    println!("{}", "-".repeat(60));

    let complex_messages = vec![Message {
        role: MessageRole::User,
        content: MessageContentInput::Text(
            "Design a distributed system for real-time collaborative document editing. \
             Consider consistency, latency, and conflict resolution."
                .to_string(),
        ),
    }];

    match gpt5_api
        .create_complex_response(
            ResponseInput::Messages(complex_messages),
            Some("Provide a detailed technical design with trade-offs analysis.".to_string()),
        )
        .await
    {
        Ok(response) => {
            println!("✅ Complex reasoning successful!");
            if let Some(usage) = &response.usage {
                println!("   Total tokens: {}", usage.total_tokens);
                // Note: reasoning_tokens field may be added in future versions
                println!("   Prompt tokens: {}", usage.prompt_tokens);
                println!("   Completion tokens: {}", usage.completion_tokens);
            }
            if let Some(choice) = response.choices.first() {
                if let Some(content) = &choice.message.content {
                    let preview = if content.len() > 200 {
                        format!("{}...", &content[..200])
                    } else {
                        content.clone()
                    };
                    println!("   Response preview:\n   {preview}");
                }
            }
        }
        Err(e) => {
            println!("❌ Complex reasoning failed: {e}");
        }
    }

    // Test 5: Coding Task
    println!("\n💻 Test 5: Coding Task");
    println!("{}", "-".repeat(60));

    let coding_messages = vec![Message {
        role: MessageRole::User,
        content: MessageContentInput::Text(
            "Write a Rust function to find the nth Fibonacci number using dynamic programming."
                .to_string(),
        ),
    }];

    match gpt5_api
        .create_coding_response(ResponseInput::Messages(coding_messages), Verbosity::High)
        .await
    {
        Ok(response) => {
            println!("✅ Coding response successful!");
            if let Some(choice) = response.choices.first() {
                if let Some(content) = &choice.message.content {
                    println!("   Generated code:\n{content}");
                }
            }
        }
        Err(e) => {
            println!("❌ Coding response failed: {e}");
        }
    }

    // Test 6: Frontend Development
    println!("\n🎨 Test 6: Frontend Development Task");
    println!("{}", "-".repeat(60));

    let frontend_messages = vec![Message {
        role: MessageRole::User,
        content: MessageContentInput::Text(
            "Create a React component for a responsive navigation bar with dark mode toggle."
                .to_string(),
        ),
    }];

    match gpt5_api
        .create_frontend_response(ResponseInput::Messages(frontend_messages))
        .await
    {
        Ok(response) => {
            println!("✅ Frontend response successful!");
            if let Some(choice) = response.choices.first() {
                if let Some(content) = &choice.message.content {
                    let preview = if content.len() > 300 {
                        format!("{}...", &content[..300])
                    } else {
                        content.clone()
                    };
                    println!("   Generated component preview:\n{preview}");
                }
            }
        }
        Err(e) => {
            println!("❌ Frontend response failed: {e}");
        }
    }

    // Test 7: Conversation Continuation (CoT)
    println!("\n🔗 Test 7: Conversation Continuation");
    println!("{}", "-".repeat(60));

    // First message
    let first_messages = vec![Message {
        role: MessageRole::User,
        content: MessageContentInput::Text(
            "Let's solve a logic puzzle. There are 3 boxes: A, B, and C. \
             One contains gold, one contains silver, and one is empty. \
             Box A says 'The gold is not here'. Box B says 'The gold is not in C'. \
             Box C says 'The gold is in B'. Only one statement is true. Where is the gold?"
                .to_string(),
        ),
    }];

    match gpt5_api
        .create_reasoned_response(
            models::GPT_5,
            ResponseInput::Messages(first_messages),
            ReasoningEffort::High,
            Verbosity::Medium,
        )
        .await
    {
        Ok(first_response) => {
            println!("✅ First conversation turn successful!");
            let response_id = first_response
                .id
                .clone()
                .unwrap_or_else(|| "test-id".to_string());

            if let Some(choice) = first_response.choices.first() {
                if let Some(content) = &choice.message.content {
                    println!("   First response: {content}");
                }
            }

            // Continue the conversation
            let followup_messages = vec![Message {
                role: MessageRole::User,
                content: MessageContentInput::Text(
                    "Great! Now, can you verify your answer by checking each possibility?"
                        .to_string(),
                ),
            }];

            match gpt5_api
                .continue_conversation(
                    models::GPT_5,
                    ResponseInput::Messages(followup_messages),
                    response_id,
                    ReasoningEffort::Medium,
                )
                .await
            {
                Ok(followup_response) => {
                    println!("✅ Conversation continuation successful!");
                    if let Some(choice) = followup_response.choices.first() {
                        if let Some(content) = &choice.message.content {
                            let preview = if content.len() > 200 {
                                format!("{}...", &content[..200])
                            } else {
                                content.clone()
                            };
                            println!("   Followup response: {preview}");
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Conversation continuation failed: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ First conversation turn failed: {e}");
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("✅ GPT-5 API testing complete!");
    println!("\nNote: Some tests may fail if GPT-5 models are not yet available.");
    println!("Currently testing with model identifiers that may change.");

    Ok(())
}
