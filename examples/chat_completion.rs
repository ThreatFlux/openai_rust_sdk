//! OpenAI Chat Completion Examples
//!
//! This example demonstrates various ways to use the OpenAI client for chat completions,
//! including simple text generation, streaming responses, multi-turn conversations,
//! and prompt template usage.

use futures::StreamExt;
use openai_rust_sdk::{
    from_env, ChatBuilder, OpenAIClient, PromptTemplate, PromptVariable, ResponseInput,
    ResponseRequest,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client from environment variable or use a test key
    let client = match from_env() {
        Ok(client) => client,
        Err(_) => {
            println!("OPENAI_API_KEY not set, using demo mode with test key");
            OpenAIClient::new("test-key-for-demo")?
        }
    };

    println!("=== OpenAI Chat Completion Examples ===\n");

    // Example 1: Simple text generation
    simple_text_generation(&client).await?;

    // Example 2: Streaming text generation
    streaming_text_generation(&client).await?;

    // Example 3: Multi-turn conversation
    multi_turn_conversation(&client).await?;

    // Example 4: Conversation with system instructions
    conversation_with_instructions(&client).await?;

    // Example 5: Custom parameters
    custom_parameters(&client).await?;

    // Example 6: Prompt template usage
    prompt_template_usage(&client).await?;

    // Example 7: Streaming conversation
    streaming_conversation(&client).await?;

    println!("\n=== All examples completed! ===");
    Ok(())
}

/// Example 1: Simple text generation
async fn simple_text_generation(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Example 1: Simple Text Generation");
    println!("-----------------------------------");

    let prompt = "Explain what Rust programming language is in one sentence.";

    match client.generate_text("gpt-3.5-turbo", prompt).await {
        Ok(response) => {
            println!("Prompt: {}", prompt);
            println!("Response: {}", response);
        }
        Err(e) => {
            println!("Error (expected in demo mode): {}", e);
        }
    }

    println!();
    Ok(())
}

/// Example 2: Streaming text generation
async fn streaming_text_generation(
    client: &OpenAIClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Example 2: Streaming Text Generation");
    println!("---------------------------------------");

    let prompt = "Write a short poem about artificial intelligence.";

    match client.generate_text_stream("gpt-3.5-turbo", prompt).await {
        Ok(mut stream) => {
            println!("Prompt: {}", prompt);
            print!("Streaming response: ");

            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        for choice in chunk.choices {
                            if let Some(content) = choice.delta.content {
                                print!("{}", content);
                            }
                        }
                    }
                    Err(e) => {
                        println!("\nStreaming error: {}", e);
                        break;
                    }
                }
            }
            println!();
        }
        Err(e) => {
            println!("Error (expected in demo mode): {}", e);
        }
    }

    println!();
    Ok(())
}

/// Example 3: Multi-turn conversation
async fn multi_turn_conversation(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("💬 Example 3: Multi-turn Conversation");
    println!("------------------------------------");

    let conversation = ChatBuilder::new()
        .user("Hello! Can you help me understand async programming?")
        .assistant("Of course! Async programming allows you to write non-blocking code that can handle multiple tasks concurrently. What specific aspect would you like to learn about?")
        .user("How does it work in Rust specifically?");

    match client.chat("gpt-3.5-turbo", conversation).await {
        Ok(response) => {
            println!("Final user message: How does it work in Rust specifically?");
            println!("Assistant response: {}", response);
        }
        Err(e) => {
            println!("Error (expected in demo mode): {}", e);
        }
    }

    println!();
    Ok(())
}

/// Example 4: Conversation with system instructions
async fn conversation_with_instructions(
    client: &OpenAIClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Example 4: Conversation with Instructions");
    println!("-------------------------------------------");

    let conversation = ChatBuilder::new()
        .developer("You are a helpful Rust programming tutor. Always provide code examples and explain concepts clearly.")
        .user("What is the difference between String and &str in Rust?");

    match client.chat("gpt-3.5-turbo", conversation).await {
        Ok(response) => {
            println!("System: You are a helpful Rust programming tutor...");
            println!("User: What is the difference between String and &str in Rust?");
            println!("Assistant: {}", response);
        }
        Err(e) => {
            println!("Error (expected in demo mode): {}", e);
        }
    }

    println!();
    Ok(())
}

/// Example 5: Custom parameters
async fn custom_parameters(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️  Example 5: Custom Parameters");
    println!("--------------------------------");

    let input =
        ResponseInput::Text("Write a creative story about a robot learning to paint.".to_string());

    match client
        .create_custom_response(
            "gpt-3.5-turbo",
            input,
            Some(0.9), // High temperature for creativity
            Some(150), // Limit tokens
            Some("Write in a whimsical, child-friendly style.".to_string()),
        )
        .await
    {
        Ok(response) => {
            println!("Custom parameters: temperature=0.9, max_tokens=150");
            println!("Instructions: Write in a whimsical, child-friendly style");
            if let Some(choice) = response.choices.first() {
                println!(
                    "Response: {}",
                    choice.message.content.as_deref().unwrap_or("No content")
                );
                if let Some(usage) = &response.usage {
                    println!("Usage: {} tokens", usage.total_tokens);
                } else {
                    println!("Usage: Not provided");
                }
            }
        }
        Err(e) => {
            println!("Error (expected in demo mode): {}", e);
        }
    }

    println!();
    Ok(())
}

/// Example 6: Prompt template usage
async fn prompt_template_usage(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Example 6: Prompt Template Usage");
    println!("----------------------------------");

    // Create a prompt template with variables
    let mut variables = HashMap::new();
    variables.insert(
        "language".to_string(),
        PromptVariable::String("Rust".to_string()),
    );
    variables.insert(
        "concept".to_string(),
        PromptVariable::String("ownership".to_string()),
    );
    variables.insert(
        "difficulty".to_string(),
        PromptVariable::String("beginner".to_string()),
    );

    let _template = PromptTemplate {
        id: "programming_tutor".to_string(),
        version: Some("1.0".to_string()),
        variables: Some(variables),
    };

    let request = ResponseRequest::new_text(
        "gpt-3.5-turbo",
        "Explain the concept of {{concept}} in {{language}} programming language for a {{difficulty}} level student."
    )
    .with_prompt_template("programming_tutor", "1.0")
    .with_instructions("Provide clear explanations with simple examples");

    match client.create_response(&request).await {
        Ok(response) => {
            println!("Template: Explain {{concept}} in {{language}} for {{difficulty}} level");
            println!("Variables: language=Rust, concept=ownership, difficulty=beginner");
            if let Some(choice) = response.choices.first() {
                println!(
                    "Response: {}",
                    choice.message.content.as_deref().unwrap_or("No content")
                );
            }
        }
        Err(e) => {
            println!("Error (expected in demo mode): {}", e);
        }
    }

    println!();
    Ok(())
}

/// Example 7: Streaming conversation
async fn streaming_conversation(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Example 7: Streaming Conversation");
    println!("-----------------------------------");

    let conversation = ChatBuilder::new()
        .developer("You are a helpful assistant that explains things step by step.")
        .user("Explain how to make a simple web server in Rust using warp crate");

    match client.chat_stream("gpt-3.5-turbo", conversation).await {
        Ok(mut stream) => {
            println!("User: Explain how to make a simple web server in Rust using warp crate");
            print!("Assistant (streaming): ");

            let mut full_response = String::new();
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        for choice in chunk.choices {
                            if let Some(content) = choice.delta.content {
                                print!("{}", content);
                                full_response.push_str(&content);
                            }

                            if choice.finish_reason.is_some() {
                                println!("\n");
                                println!(
                                    "Stream completed. Total response length: {} characters",
                                    full_response.len()
                                );
                                return Ok(());
                            }
                        }
                    }
                    Err(e) => {
                        println!("\nStreaming error: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Error (expected in demo mode): {}", e);
        }
    }

    println!();
    Ok(())
}

/// Example helper: Error handling patterns
#[allow(dead_code)]
async fn error_handling_example(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚨 Error Handling Example");
    println!("-------------------------");

    // This will likely fail with invalid API key in demo mode
    let result = client.generate_text("gpt-3.5-turbo", "Hello").await;

    match result {
        Ok(response) => {
            println!("Success: {}", response);
        }
        Err(openai_rust_sdk::OpenAIError::Authentication(msg)) => {
            println!("Authentication error: {}", msg);
        }
        Err(openai_rust_sdk::OpenAIError::Api {
            status_code,
            message,
        }) => {
            println!("API error {}: {}", status_code, message);
        }
        Err(openai_rust_sdk::OpenAIError::Request(e)) => {
            println!("Request error: {}", e);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }

    Ok(())
}
