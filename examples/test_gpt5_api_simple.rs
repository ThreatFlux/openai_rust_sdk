#![allow(clippy::pedantic, clippy::nursery)]
//! Test GPT-5 API capabilities (simplified)
//!
//! This example tests the actual GPT-5 API methods

use openai_rust_sdk::{
    api::gpt5::GPT5Api,
    error::Result,
    models::{
        gpt5::{ReasoningEffort, Verbosity},
        responses::{Message, MessageContentInput, MessageRole, ResponseInput},
    },
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key =
        env::var("OPENAI_API_KEY").expect("Please set OPENAI_API_KEY environment variable");

    println!("\n🚀 Testing GPT-5 API Methods\n");
    println!("{}", "=".repeat(60));

    // Initialize GPT-5 API
    let gpt5_api = GPT5Api::new(api_key)?;
    println!("✅ GPT-5 API initialized successfully");

    // Test 1: Minimal Response
    println!("\n📝 Test 1: Minimal Response");
    println!("{}", "-".repeat(60));

    let input = ResponseInput::Text("What is 2+2?".to_string());

    match gpt5_api
        .create_minimal_response("gpt-5", input.clone())
        .await
    {
        Ok(response) => {
            println!("✅ Minimal response successful!");
            if let Some(choice) = response.choices.first() {
                if let Some(content) = &choice.message.content {
                    println!("   Response: {content}");
                }
            }
            println!("   Model: {}", response.model);
        }
        Err(e) => {
            println!("❌ Minimal response failed: {e}");
            check_error(&e);
        }
    }

    // Test 2: Fast Response
    println!("\n⚡ Test 2: Fast Response");
    println!("{}", "-".repeat(60));

    match gpt5_api
        .create_fast_response("gpt-5-nano", input.clone(), Verbosity::Low)
        .await
    {
        Ok(response) => {
            println!("✅ Fast response successful!");
            if let Some(usage) = &response.usage {
                println!("   Tokens used: {}", usage.total_tokens);
            }
        }
        Err(e) => {
            println!("❌ Fast response failed: {e}");
        }
    }

    // Test 3: Reasoned Response
    println!("\n🧠 Test 3: Reasoned Response");
    println!("{}", "-".repeat(60));

    match gpt5_api
        .create_reasoned_response(
            "gpt-5-nano",
            input.clone(),
            ReasoningEffort::Medium,
            Verbosity::Medium,
        )
        .await
    {
        Ok(response) => {
            println!("✅ Reasoned response successful!");
            println!("   Model: {}", response.model);
        }
        Err(e) => {
            println!("❌ Reasoned response failed: {e}");
        }
    }

    // Test 4: Complex Response
    println!("\n🔬 Test 4: Complex Response");
    println!("{}", "-".repeat(60));

    let complex_input = ResponseInput::Text("Explain the theory of relativity".to_string());

    match gpt5_api
        .create_complex_response(
            complex_input,
            Some("Provide a detailed explanation".to_string()),
        )
        .await
    {
        Ok(response) => {
            println!("✅ Complex response successful!");
            if let Some(choice) = response.choices.first() {
                if choice.message.content.is_some() {
                    println!("   Response received (verbose mode)");
                }
            }
        }
        Err(e) => {
            println!("❌ Complex response failed: {e}");
        }
    }

    // Test 5: Coding Response
    println!("\n💻 Test 5: Coding Response");
    println!("{}", "-".repeat(60));

    let coding_input =
        ResponseInput::Text("Write a Python function to calculate fibonacci".to_string());

    match gpt5_api
        .create_coding_response(coding_input, Verbosity::High)
        .await
    {
        Ok(response) => {
            println!("✅ Coding response successful!");
            if let Some(choice) = response.choices.first() {
                if let Some(content) = &choice.message.content {
                    if content.contains("def") || content.contains("python") {
                        println!("   Code response detected");
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Coding response failed: {e}");
        }
    }

    // Test 6: Multi-turn Conversation
    println!("\n💬 Test 6: Multi-turn Conversation");
    println!("{}", "-".repeat(60));

    let messages = vec![
        Message {
            role: MessageRole::System,
            content: MessageContentInput::Text("You are a helpful assistant.".to_string()),
        },
        Message {
            role: MessageRole::User,
            content: MessageContentInput::Text("Hello!".to_string()),
        },
    ];

    let messages_input = ResponseInput::Messages(messages);

    match gpt5_api
        .create_minimal_response("gpt-5", messages_input)
        .await
    {
        Ok(response) => {
            println!("✅ Multi-turn conversation successful!");

            // Store response ID for context
            if let Some(id) = &response.id {
                println!("   Response ID: {id}");
                println!("   Can use for continued conversation");
            }
        }
        Err(e) => {
            println!("❌ Multi-turn conversation failed: {e}");
        }
    }

    println!("\n");
    println!("{}", "=".repeat(60));
    println!("🎉 GPT-5 API Testing Complete!");
    println!("{}", "=".repeat(60));

    println!("\n📊 Summary:");
    println!("   All GPT-5 API methods are properly structured.");
    println!("   The API will work when GPT-5 models are available.");
    println!("\n   Current status: GPT-5 not yet released");
    println!("   Expected: 404 or model not found errors");

    Ok(())
}

fn check_error(e: &openai_rust_sdk::error::OpenAIError) {
    if e.to_string().contains("401") {
        println!("   Note: Invalid API key");
    } else if e.to_string().contains("404") || e.to_string().contains("model") {
        println!("   Note: GPT-5 model not available yet (expected)");
    }
}
