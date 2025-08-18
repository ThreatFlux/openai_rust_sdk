//! # `OpenAI` Assistants API Demo
//!
//! This example demonstrates the complete usage of the `OpenAI` Assistants API,
//! including creating assistants with different tools, managing them, and
//! performing various operations.
//!
//! ## Features Demonstrated
//!
//! - Creating assistants with Code Interpreter
//! - Creating assistants with Retrieval capabilities
//! - Creating assistants with Function calling
//! - Listing and managing assistants
//! - Error handling and best practices
//!
//! ## Running the Example
//!
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! cargo run --example assistants_demo
//! ```

use openai_rust_sdk::api::assistants::AssistantsApi;
use openai_rust_sdk::error::{OpenAIError, Result};
use openai_rust_sdk::models::assistants::{
    AssistantRequest, AssistantTool, ListAssistantsParams, SortOrder,
};
use openai_rust_sdk::models::functions::FunctionTool;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the API client
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        OpenAIError::Authentication("OPENAI_API_KEY environment variable not set".to_string())
    })?;

    let api = AssistantsApi::new(api_key)?;

    println!("🤖 OpenAI Assistants API Demo");
    println!("==============================\n");

    // Demo 1: Create a Code Interpreter Assistant
    println!("📊 Demo 1: Creating a Data Analyst Assistant with Code Interpreter");
    let data_analyst = demo_code_interpreter_assistant(&api).await?;
    println!("✅ Created assistant: {}\n", data_analyst.id);

    // Demo 2: Create a Retrieval Assistant
    println!("📚 Demo 2: Creating a Knowledge Base Assistant with Retrieval");
    let knowledge_assistant = demo_retrieval_assistant(&api).await?;
    println!("✅ Created assistant: {}\n", knowledge_assistant.id);

    // Demo 3: Create a Function Calling Assistant
    println!("🔧 Demo 3: Creating a Weather Assistant with Function Calling");
    let weather_assistant = demo_function_calling_assistant(&api).await?;
    println!("✅ Created assistant: {}\n", weather_assistant.id);

    // Demo 4: List all assistants
    println!("📋 Demo 4: Listing All Assistants");
    demo_list_assistants(&api).await?;

    // Demo 5: Modify an assistant
    println!("✏️  Demo 5: Modifying an Assistant");
    demo_modify_assistant(&api, &data_analyst.id).await?;

    // Demo 6: Retrieve specific assistant
    println!("🔍 Demo 6: Retrieving Specific Assistant");
    demo_retrieve_assistant(&api, &weather_assistant.id).await?;

    // Demo 7: Demonstrate pagination
    println!("📄 Demo 7: Pagination Example");
    demo_pagination(&api).await?;

    // Demo 8: Error handling
    println!("⚠️  Demo 8: Error Handling Examples");
    demo_error_handling(&api).await?;

    // Cleanup: Delete the created assistants
    println!("🧹 Cleanup: Deleting Created Assistants");
    cleanup_assistants(
        &api,
        vec![
            &data_analyst.id,
            &knowledge_assistant.id,
            &weather_assistant.id,
        ],
    )
    .await?;

    println!("\n🎉 Demo completed successfully!");
    Ok(())
}

/// Demo 1: Create an assistant with Code Interpreter for data analysis
async fn demo_code_interpreter_assistant(
    api: &AssistantsApi,
) -> Result<openai_rust_sdk::models::assistants::Assistant> {
    let request = AssistantRequest::builder()
        .model("gpt-4")
        .name("Data Analyst")
        .description("A specialized assistant for data analysis and visualization")
        .instructions(
            "You are a professional data analyst. You help users analyze datasets, \
            create visualizations, and derive insights from data. You have access to \
            Python code execution through the Code Interpreter tool. Always explain \
            your analysis process and provide clear, actionable insights.",
        )
        .tool(AssistantTool::code_interpreter())
        .metadata_pair("category", "data_analysis")
        .metadata_pair("version", "1.0")
        .metadata_pair("use_case", "general_analytics")
        .build()?;

    println!("Creating Data Analyst Assistant with Code Interpreter...");
    println!("- Model: {}", request.model);
    println!("- Tools: Code Interpreter");
    println!("- Metadata: {} pairs", request.metadata.len());

    let assistant = api.create_assistant(request).await?;

    println!("Assistant created successfully!");
    println!("- ID: {}", assistant.id);
    println!("- Name: {:?}", assistant.name);
    println!("- Created at: {}", assistant.created_at);

    Ok(assistant)
}

/// Demo 2: Create an assistant with Retrieval capabilities
async fn demo_retrieval_assistant(
    api: &AssistantsApi,
) -> Result<openai_rust_sdk::models::assistants::Assistant> {
    let request = AssistantRequest::builder()
        .model("gpt-4")
        .name("Knowledge Base Assistant")
        .description("An assistant that can search and retrieve information from documents")
        .instructions(
            "You are a knowledge base assistant. You help users find information \
            from uploaded documents and files. Use the retrieval tool to search \
            through the available knowledge base and provide accurate, well-sourced \
            answers. Always cite your sources when possible.",
        )
        .tool(AssistantTool::retrieval())
        .metadata_pair("category", "knowledge_management")
        .metadata_pair("version", "1.0")
        .build()?;

    println!("Creating Knowledge Base Assistant with Retrieval...");
    println!("- Model: {}", request.model);
    println!("- Tools: Retrieval");

    let assistant = api.create_assistant(request).await?;

    println!("Assistant created successfully!");
    println!("- ID: {}", assistant.id);
    println!("- Name: {:?}", assistant.name);

    Ok(assistant)
}

/// Demo 3: Create an assistant with Function calling capabilities
async fn demo_function_calling_assistant(
    api: &AssistantsApi,
) -> Result<openai_rust_sdk::models::assistants::Assistant> {
    // Define a weather function
    let get_weather_function = FunctionTool {
        name: "get_weather".to_string(),
        description: "Get the current weather information for a specific location".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state/country, e.g. 'San Francisco, CA' or 'London, UK'"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "The temperature unit to use",
                    "default": "fahrenheit"
                }
            },
            "required": ["location"]
        }),
        strict: None,
    };

    // Define a forecast function
    let get_forecast_function = FunctionTool {
        name: "get_forecast".to_string(),
        description: "Get weather forecast for multiple days".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state/country"
                },
                "days": {
                    "type": "integer",
                    "description": "Number of days to forecast (1-7)",
                    "minimum": 1,
                    "maximum": 7,
                    "default": 3
                }
            },
            "required": ["location"]
        }),
        strict: None,
    };

    let request = AssistantRequest::builder()
        .model("gpt-4")
        .name("Weather Assistant")
        .description("An assistant that provides weather information and forecasts")
        .instructions(
            "You are a helpful weather assistant. You can provide current weather \
            information and forecasts for any location using the available weather \
            functions. Always be helpful and provide clear, accurate weather information. \
            If a user asks about weather, use the appropriate function to get real-time data.",
        )
        .tool(AssistantTool::function(get_weather_function))
        .tool(AssistantTool::function(get_forecast_function))
        .metadata_pair("category", "weather")
        .metadata_pair("version", "1.0")
        .metadata_pair("provider", "weather_api")
        .build()?;

    println!("Creating Weather Assistant with Function Calling...");
    println!("- Model: {}", request.model);
    println!("- Tools: 2 Functions (get_weather, get_forecast)");

    let assistant = api.create_assistant(request).await?;

    println!("Assistant created successfully!");
    println!("- ID: {}", assistant.id);
    println!("- Name: {:?}", assistant.name);
    println!("- Function tools: {}", assistant.tools.len());

    Ok(assistant)
}

/// Demo 4: List all assistants with various parameters
async fn demo_list_assistants(api: &AssistantsApi) -> Result<()> {
    println!("Listing all assistants...");

    // List with default parameters
    let assistants = api.list_assistants(None).await?;
    println!(
        "Found {} assistants (default listing)",
        assistants.data.len()
    );

    // List with custom parameters
    let params = ListAssistantsParams::new().limit(5).order(SortOrder::Desc);

    let limited_assistants = api.list_assistants(Some(params)).await?;
    println!(
        "Found {} assistants (limited to 5, newest first)",
        limited_assistants.data.len()
    );

    // Display details of each assistant
    for (i, assistant) in limited_assistants.data.iter().enumerate() {
        println!(
            "  {}. {} ({})",
            i + 1,
            assistant.name.as_deref().unwrap_or("Unnamed"),
            assistant.id
        );
        println!("     Model: {}", assistant.model);
        println!("     Tools: {}", assistant.tools.len());
        if !assistant.metadata.is_empty() {
            println!("     Metadata: {:?}", assistant.metadata);
        }
    }

    println!();
    Ok(())
}

/// Demo 5: Modify an existing assistant
async fn demo_modify_assistant(api: &AssistantsApi, assistant_id: &str) -> Result<()> {
    println!("Modifying assistant {assistant_id}...");

    let update_request = AssistantRequest::builder()
        .model("gpt-4")
        .name("Enhanced Data Analyst")
        .description("An enhanced data analyst with updated capabilities")
        .instructions(
            "You are an advanced data analyst with enhanced capabilities. \
            You can perform complex statistical analysis, create sophisticated \
            visualizations, and provide detailed insights. You have been updated \
            with the latest analytical techniques and best practices.",
        )
        .tool(AssistantTool::code_interpreter())
        .tool(AssistantTool::retrieval()) // Add retrieval capability
        .metadata_pair("category", "advanced_analytics")
        .metadata_pair("version", "2.0")
        .metadata_pair("updated", "true")
        .build()?;

    let updated_assistant = api.modify_assistant(assistant_id, update_request).await?;

    println!("Assistant modified successfully!");
    println!("- ID: {}", updated_assistant.id);
    println!("- New name: {:?}", updated_assistant.name);
    println!("- Tools count: {}", updated_assistant.tools.len());
    println!("- Metadata: {:?}", updated_assistant.metadata);
    println!();

    Ok(())
}

/// Demo 6: Retrieve a specific assistant
async fn demo_retrieve_assistant(api: &AssistantsApi, assistant_id: &str) -> Result<()> {
    println!("Retrieving assistant {assistant_id}...");

    let assistant = api.retrieve_assistant(assistant_id).await?;

    println!("Assistant retrieved successfully!");
    println!("- ID: {}", assistant.id);
    println!("- Name: {:?}", assistant.name);
    println!("- Description: {:?}", assistant.description);
    println!("- Model: {}", assistant.model);
    println!("- Created at: {}", assistant.created_at);
    println!("- Tools: {}", assistant.tools.len());

    for (i, tool) in assistant.tools.iter().enumerate() {
        println!("  {}. {}", i + 1, tool.tool_type());
        if let AssistantTool::Function { function } = tool {
            println!("     Function: {}", function.name);
            println!("     Description: {}", function.description);
        }
    }

    if !assistant.file_ids.is_empty() {
        println!("- File IDs: {:?}", assistant.file_ids);
    }

    if !assistant.metadata.is_empty() {
        println!("- Metadata:");
        for (key, value) in &assistant.metadata {
            println!("  {key}: {value}");
        }
    }

    println!();
    Ok(())
}

/// Demo 7: Demonstrate pagination with assistants
async fn demo_pagination(api: &AssistantsApi) -> Result<()> {
    println!("Demonstrating pagination...");

    // Get first page
    let first_page_params = ListAssistantsParams::new().limit(2).order(SortOrder::Desc);

    let first_page = api.list_assistants(Some(first_page_params)).await?;
    println!("First page: {} assistants", first_page.data.len());

    if let Some(last_id) = &first_page.last_id {
        println!("Last ID on first page: {last_id}");

        // Get next page using the last ID as cursor
        let next_page_params = ListAssistantsParams::new()
            .limit(2)
            .order(SortOrder::Desc)
            .after(last_id.clone());

        let next_page = api.list_assistants(Some(next_page_params)).await?;
        println!("Next page: {} assistants", next_page.data.len());
        println!("Has more: {}", next_page.has_more);
    }

    println!();
    Ok(())
}

/// Demo 8: Error handling examples
async fn demo_error_handling(api: &AssistantsApi) -> Result<()> {
    println!("Demonstrating error handling...");

    // Example 1: Try to retrieve a non-existent assistant
    println!("1. Trying to retrieve non-existent assistant...");
    match api.retrieve_assistant("asst_nonexistent").await {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => println!("   Expected error: {e}"),
    }

    // Example 2: Try to create an assistant with invalid parameters
    println!("2. Trying to create assistant with invalid name (too long)...");
    let invalid_request = AssistantRequest {
        model: "gpt-4".to_string(),
        name: Some("a".repeat(300)), // Too long
        description: None,
        instructions: None,
        tools: Vec::new(),
        file_ids: Vec::new(),
        metadata: HashMap::new(),
    };

    match api.create_assistant(invalid_request).await {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => println!("   Expected error: {e}"),
    }

    // Example 3: Try to delete a non-existent assistant
    println!("3. Trying to delete non-existent assistant...");
    match api.delete_assistant("asst_nonexistent").await {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => println!("   Expected error: {e}"),
    }

    println!();
    Ok(())
}

/// Cleanup: Delete created assistants
async fn cleanup_assistants(api: &AssistantsApi, assistant_ids: Vec<&str>) -> Result<()> {
    println!("Cleaning up created assistants...");

    for assistant_id in assistant_ids {
        match api.delete_assistant(assistant_id).await {
            Ok(deletion_status) => {
                println!(
                    "✅ Deleted assistant {}: {}",
                    assistant_id, deletion_status.deleted
                );
            }
            Err(e) => {
                println!("⚠️  Failed to delete assistant {assistant_id}: {e}");
            }
        }
    }

    Ok(())
}

/// Helper function to create a comprehensive assistant for testing
#[allow(dead_code)]
async fn create_comprehensive_assistant(
    api: &AssistantsApi,
) -> Result<openai_rust_sdk::models::assistants::Assistant> {
    // Create a function for getting user preferences
    let get_preferences_function = FunctionTool {
        name: "get_user_preferences".to_string(),
        description: "Retrieve user preferences and settings".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "user_id": {
                    "type": "string",
                    "description": "The unique user identifier"
                },
                "category": {
                    "type": "string",
                    "enum": ["display", "notification", "privacy", "general"],
                    "description": "The preference category to retrieve"
                }
            },
            "required": ["user_id"]
        }),
        strict: None,
    };

    let request = AssistantRequest::builder()
        .model("gpt-4")
        .name("Comprehensive Assistant")
        .description("A full-featured assistant with all tool types")
        .instructions(
            "You are a comprehensive AI assistant with access to multiple tools. \
            You can execute code, search documents, and call custom functions. \
            Adapt your responses based on the user's needs and available tools.",
        )
        .tool(AssistantTool::code_interpreter())
        .tool(AssistantTool::retrieval())
        .tool(AssistantTool::function(get_preferences_function))
        .file_id("file-example-123") // Example file ID
        .metadata_pair("type", "comprehensive")
        .metadata_pair("capabilities", "full")
        .metadata_pair("version", "1.0")
        .build()?;

    api.create_assistant(request).await
}
