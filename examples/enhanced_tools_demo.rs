#![allow(clippy::pedantic, clippy::nursery)]
//! # Enhanced Tools Demo
//!
//! This example demonstrates how to use the comprehensive tools support including:
//! - Web Search: Include internet data in responses
//! - File Search: Search uploaded files for context
//! - Remote MCP: Access Model Context Protocol servers
//! - Function Calling: Call custom functions
//! - Image Generation: Generate or edit images
//! - Code Interpreter: Execute code in secure containers
//!
//! Usage:
//! ```bash
//! export OPENAI_API_KEY=your_api_key_here
//! cargo run --example enhanced_tools_demo
//! ```

use openai_rust_sdk::{
    api::responses::ResponsesApi,
    models::{
        responses::ResponseRequest,
        tools::{McpApproval, ToolBuilder},
    },
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with: export OPENAI_API_KEY=your_key_here"
    })?;

    println!("🛠️ Enhanced Tools Demo");
    println!("======================");

    let api = ResponsesApi::new(api_key)?;

    // Example 1: Web Search
    println!("\n📡 Example 1: Web Search Tool");
    println!("─────────────────────────────");

    let web_search_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "What are the latest developments in quantum computing from 2025?",
    )
    .with_web_search()
    .with_max_tokens(200);

    println!("🔍 Searching the web for quantum computing updates...");
    let response = api.create_response(&web_search_request).await?;
    println!("✅ Web Search Response:");
    println!("{}", response.output_text());

    // Example 2: Advanced Web Search with filters
    println!("\n🌐 Example 2: Advanced Web Search with Filters");
    println!("───────────────────────────────────────────────");

    let advanced_web_tool = ToolBuilder::web_search_advanced()
        .max_results(5)
        .include_domains(vec!["arxiv.org".to_string(), "nature.com".to_string()])
        .time_range("2025")
        .build();

    let filtered_search_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "Find recent research papers on transformer architectures",
    )
    .with_enhanced_tool(advanced_web_tool)
    .with_max_tokens(300);

    println!("🔬 Searching academic sources for transformer research...");
    let response = api.create_response(&filtered_search_request).await?;
    println!("✅ Filtered Search Response:");
    println!("{}", response.output_text());

    // Example 3: File Search with Vector Stores
    println!("\n📂 Example 3: File Search with Vector Stores");
    println!("─────────────────────────────────────────────");

    // Note: You need to have a vector store ID from uploaded files
    let vector_store_ids = vec!["vs_example123".to_string()]; // Replace with actual ID

    let file_search_tool = ToolBuilder::file_search(vector_store_ids.clone())
        .max_chunks(10)
        .file_types(vec!["pdf".to_string(), "txt".to_string()])
        .build();

    let file_search_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "What does the documentation say about batch processing?",
    )
    .with_enhanced_tool(file_search_tool)
    .with_max_tokens(200);

    println!("📚 Searching uploaded files for batch processing information...");
    // Note: This will only work if you have actual vector stores
    match api.create_response(&file_search_request).await {
        Ok(response) => {
            println!("✅ File Search Response:");
            println!("{}", response.output_text());
        }
        Err(e) => {
            println!("⚠️ File search requires valid vector store IDs: {e}");
        }
    }

    // Example 4: Remote MCP Server Integration
    println!("\n🔗 Example 4: Remote MCP Server Integration");
    println!("────────────────────────────────────────────");

    let mcp_tool = ToolBuilder::mcp("deepwiki", "https://mcp.deepwiki.com/mcp")
        .require_approval(McpApproval::Never)
        .timeout_ms(10000)
        .build();

    let mcp_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "What transport protocols are supported in the latest MCP specification?",
    )
    .with_enhanced_tool(mcp_tool)
    .with_max_tokens(250);

    println!("🌐 Querying DeepWiki MCP server...");
    match api.create_response(&mcp_request).await {
        Ok(response) => {
            println!("✅ MCP Server Response:");
            println!("{}", response.output_text());
        }
        Err(e) => {
            println!("⚠️ MCP server query failed: {e}");
        }
    }

    // Example 5: Multiple Tools in One Request
    println!("\n🎯 Example 5: Multiple Tools Combined");
    println!("──────────────────────────────────────");

    let multi_tool_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "Search the web for OpenAI's latest announcements and compare with any documentation we have"
    )
    .with_web_search()
    .with_file_search(vector_store_ids)
    .with_max_tokens(300);

    println!("🔄 Using multiple tools simultaneously...");
    let response = api.create_response(&multi_tool_request).await?;
    println!("✅ Multi-Tool Response:");
    println!("{}", response.output_text());

    // Example 6: Function Calling with Enhanced Tools
    println!("\n⚡ Example 6: Function Calling with Enhanced Tools");
    println!("───────────────────────────────────────────────────");

    let function_tool = ToolBuilder::function("get_weather", "Get current weather for a location")
        .parameters(serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City and country"
                }
            },
            "required": ["location"]
        }))
        .strict(true)
        .build();

    let function_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "What's the weather in Tokyo and search for recent weather patterns there?",
    )
    .with_enhanced_tool(function_tool)
    .with_web_search()
    .with_max_tokens(200);

    println!("☁️ Checking weather and searching for patterns...");
    let response = api.create_response(&function_request).await?;
    println!("✅ Function + Web Search Response:");
    println!("{}", response.output_text());

    // Example 7: Image Generation Tool
    println!("\n🎨 Example 7: Image Generation Tool");
    println!("────────────────────────────────────");

    let image_tool = ToolBuilder::image_generation()
        .size("1024x1024")
        .quality("hd")
        .style("vivid")
        .count(1)
        .build();

    let image_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "Generate an image of a futuristic cybersecurity operations center",
    )
    .with_enhanced_tool(image_tool)
    .with_max_tokens(100);

    println!("🖼️ Generating image...");
    match api.create_response(&image_request).await {
        Ok(response) => {
            println!("✅ Image Generation Response:");
            println!("{}", response.output_text());
        }
        Err(e) => {
            println!("⚠️ Image generation not available in this context: {e}");
        }
    }

    // Example 8: Code Interpreter Tool
    println!("\n💻 Example 8: Code Interpreter Tool");
    println!("────────────────────────────────────");

    let code_tool = ToolBuilder::code_interpreter()
        .language("python")
        .max_execution_time_ms(5000)
        .libraries(vec!["numpy".to_string(), "pandas".to_string()])
        .build();

    let code_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "Calculate the first 20 Fibonacci numbers and analyze their growth rate",
    )
    .with_enhanced_tool(code_tool)
    .with_max_tokens(300);

    println!("🧮 Running code interpreter...");
    match api.create_response(&code_request).await {
        Ok(response) => {
            println!("✅ Code Interpreter Response:");
            println!("{}", response.output_text());
        }
        Err(e) => {
            println!("⚠️ Code interpreter not available: {e}");
        }
    }

    // Example 9: Tool Choice Control
    println!("\n🎛️ Example 9: Tool Choice Control");
    println!("──────────────────────────────────");

    use openai_rust_sdk::models::tools::EnhancedToolChoice;

    let controlled_request = ResponseRequest::new_text(
        "gpt-4o-mini",
        "Tell me about Paris (use web search if needed)",
    )
    .with_web_search()
    .with_enhanced_tool_choice(EnhancedToolChoice::Required) // Force tool use
    .with_max_tokens(200);

    println!("🎯 Forcing tool usage...");
    let response = api.create_response(&controlled_request).await?;
    println!("✅ Controlled Tool Response:");
    println!("{}", response.output_text());

    println!("\n✨ Enhanced Tools Demo Complete!");
    println!("💡 Key Takeaways:");
    println!("   • Web Search enables real-time internet data");
    println!("   • File Search queries your uploaded documents");
    println!("   • MCP servers provide external service integration");
    println!("   • Multiple tools can be combined in one request");
    println!("   • Tool choice can be controlled explicitly");
    println!("   • Advanced tools like image generation and code interpreter extend capabilities");

    Ok(())
}
