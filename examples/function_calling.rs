//! Comprehensive examples of function calling with the OpenAI SDK
//!
//! This example demonstrates:
//! - Basic function calling (get_weather example)
//! - Multiple function calls in parallel
//! - Custom tools with grammar
//! - Streaming function calls
//! - Complete conversation flow with function results

use openai_rust_sdk::{
    ChatBuilder, Message, OpenAIClient,
    api::functions::FunctionConfig,
    builders::FunctionBuilder,
    models::functions::{CustomTool, FunctionCallOutput, Tool, ToolChoice},
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = OpenAIClient::new(
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set"),
    )?;

    println!("=== OpenAI Function Calling Examples ===\n");

    // Example 1: Basic function calling
    basic_function_calling(&client).await?;

    // Example 2: Multiple functions in parallel
    parallel_function_calling(&client).await?;

    // Example 3: Custom tools with grammar
    custom_tools_example(&client).await?;

    // Example 4: Streaming function calls
    streaming_function_calls(&client).await?;

    // Example 5: Complete conversation flow
    complete_conversation_flow(&client).await?;

    // Example 6: Weather assistant
    weather_assistant_example(&client).await?;

    Ok(())
}

/// Example 1: Basic function calling with get_weather
async fn basic_function_calling(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("1. Basic Function Calling - Weather Information");
    println!("================================================");

    // Create a weather function
    let weather_function = FunctionBuilder::weather_function(
        "get_weather",
        "Get current weather information for a location",
    )
    .build()?;

    let tools = vec![Tool::function(weather_function)];

    // Make a simple function call
    let response = client
        .call_function(
            "gpt-4",
            "What's the weather like in San Francisco?",
            tools,
            Some(ToolChoice::auto()),
        )
        .await?;

    println!("Response content: {:?}", response.content);
    println!("Function calls made: {}", response.function_calls.len());

    for call in &response.function_calls {
        println!("- Function: {} (ID: {})", call.name, call.call_id);
        println!("  Arguments: {}", call.arguments);

        // Execute the function call
        let result = client.execute_function_with_result(call).await?;
        println!("  Result: {}", result.output);
    }

    println!();
    Ok(())
}

/// Example 2: Multiple functions in parallel
async fn parallel_function_calling(
    client: &OpenAIClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("2. Parallel Function Calling");
    println!("============================");

    // Create multiple functions
    let weather_function =
        FunctionBuilder::weather_function("get_weather", "Get weather information for a location")
            .build()?;

    let time_function = FunctionBuilder::new()
        .name("get_time")
        .description("Get current time for a timezone")
        .optional_string("timezone", "Timezone (e.g., 'America/New_York')")
        .build()?;

    let search_function =
        FunctionBuilder::search_function("search_events", "Search for local events in a city")
            .build()?;

    let tools = vec![
        Tool::function(weather_function),
        Tool::function(time_function),
        Tool::function(search_function),
    ];

    // Make a complex request that might trigger multiple functions
    let config = FunctionConfig::new()
        .with_tools(tools)
        .with_parallel_calls(true);

    let conversation = ChatBuilder::new()
        .user("I'm visiting New York tomorrow. Can you tell me the weather, current time, and any interesting events happening?");

    let request = openai_rust_sdk::models::responses::ResponseRequest::new_messages(
        "gpt-4",
        conversation.build(),
    );

    let response = client.create_function_response(&request, &config).await?;

    println!("Response: {:?}", response.content);
    println!("Function calls: {}", response.function_calls.len());

    // Execute all function calls
    let mut results = Vec::new();
    for call in &response.function_calls {
        println!("Executing: {} with args: {}", call.name, call.arguments);
        let result = client.execute_function_with_result(call).await?;
        results.push(result);
    }

    // Submit results and get final response
    if !results.is_empty() {
        let final_response = client
            .submit_function_results(results, &request, &config)
            .await?;
        println!("Final response: {:?}", final_response.content);
    }

    println!();
    Ok(())
}

/// Example 3: Custom tools with grammar
async fn custom_tools_example(_client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("3. Custom Tools with Grammar");
    println!("============================");

    // Create a custom tool with Lark grammar
    let parser_tool = CustomTool::new(
        "parse_query",
        "Parse natural language queries into structured data",
    )
    .with_lark_grammar(
        r#"
        start: command
        command: action target*
        action: "find" | "search" | "get" | "show"
        target: location | time | person
        location: "in" WORD+
        time: "at" TIME | "on" DATE
        person: "for" WORD+
        
        TIME: /\d{1,2}:\d{2}/
        DATE: /\d{4}-\d{2}-\d{2}/
        WORD: /\w+/
        
        %ignore " "
    "#,
    );

    // Create a regex-based tool
    let email_extractor = CustomTool::new("extract_emails", "Extract email addresses from text")
        .with_regex_grammar(
            r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
            Some(vec!["global".to_string(), "case_insensitive".to_string()]),
        );

    println!("Created custom tools:");
    println!("- Parser tool: {}", parser_tool.name);
    println!("- Email extractor: {}", email_extractor.name);

    // In a real application, you would register these with the client
    // and use them in function calling workflows

    println!();
    Ok(())
}

/// Example 4: Streaming function calls
async fn streaming_function_calls(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("4. Streaming Function Calls");
    println!("===========================");

    let weather_function =
        FunctionBuilder::weather_function("get_weather", "Get weather information").build()?;

    let tools = vec![Tool::function(weather_function)];

    // Create a streaming function call
    let mut stream = client
        .streaming()
        .call_function_stream(
            "gpt-4",
            "What's the weather in Tokyo and London?",
            tools,
            Some(ToolChoice::auto()),
        )
        .await?;

    println!("Streaming function calls...");

    use futures::StreamExt;
    use openai_rust_sdk::api::streaming::FunctionStreamEvent;

    while let Some(event_result) = stream.next().await {
        match event_result? {
            FunctionStreamEvent::ContentDelta { content } => {
                print!("{}", content);
            }
            FunctionStreamEvent::FunctionCallStarted {
                call_id,
                function_name,
            } => {
                println!(
                    "\n🔧 Function call started: {} ({})",
                    function_name, call_id
                );
            }
            FunctionStreamEvent::FunctionCallArgumentsDelta {
                call_id,
                arguments_delta,
            } => {
                print!("Args[{}]: {}", call_id, arguments_delta);
            }
            FunctionStreamEvent::FunctionCallCompleted { call } => {
                println!("\n✅ Function call completed: {}", call.name);
                println!("   Final arguments: {}", call.arguments);

                // Execute the function
                let result = client.execute_function_with_result(&call).await?;
                println!("   Result: {}", result.output);
            }
            FunctionStreamEvent::Completed { .. } => {
                println!("\n🏁 Stream completed");
                break;
            }
            FunctionStreamEvent::Error { message } => {
                println!("\n❌ Error: {}", message);
                break;
            }
        }
    }

    println!();
    Ok(())
}

/// Example 5: Complete conversation flow with multiple rounds
async fn complete_conversation_flow(
    client: &OpenAIClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("5. Complete Conversation Flow");
    println!("=============================");

    // Define available functions
    let weather_function =
        FunctionBuilder::weather_function("get_weather", "Get weather information").build()?;

    let calendar_function = FunctionBuilder::new()
        .name("check_calendar")
        .description("Check calendar for availability")
        .required_string("date", "Date to check (YYYY-MM-DD)")
        .optional_string("time", "Specific time to check")
        .build()?;

    let booking_function = FunctionBuilder::new()
        .name("book_meeting")
        .description("Book a meeting")
        .required_string("date", "Meeting date")
        .required_string("time", "Meeting time")
        .required_string("duration", "Meeting duration")
        .required_string("title", "Meeting title")
        .build()?;

    let tools = vec![
        Tool::function(weather_function),
        Tool::function(calendar_function),
        Tool::function(booking_function),
    ];

    // Start conversation
    let initial_messages = vec![Message::user(
        "Hi! I need to schedule an outdoor meeting for tomorrow. Can you help me check the weather and find a good time?",
    )];

    println!(
        "User: Hi! I need to schedule an outdoor meeting for tomorrow. Can you help me check the weather and find a good time?"
    );

    // Execute the conversation with automatic function handling
    let conversation_results = client
        .function_conversation(
            "gpt-4",
            initial_messages,
            tools,
            Some(5), // Max 5 iterations
        )
        .await?;

    for (i, result) in conversation_results.iter().enumerate() {
        println!("\n--- Round {} ---", i + 1);

        if let Some(content) = &result.content {
            println!("Assistant: {}", content);
        }

        if !result.function_calls.is_empty() {
            println!("Function calls made: {}", result.function_calls.len());
            for call in &result.function_calls {
                println!("  - {}: {}", call.name, call.arguments);
            }
        }
    }

    println!();
    Ok(())
}

/// Example 6: Weather assistant with error handling
async fn weather_assistant_example(
    client: &OpenAIClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("6. Weather Assistant with Error Handling");
    println!("=======================================");

    // Create comprehensive weather functions
    let current_weather = FunctionBuilder::weather_function(
        "get_current_weather",
        "Get current weather for a location",
    )
    .build()?;

    let weather_forecast = FunctionBuilder::new()
        .name("get_weather_forecast")
        .description("Get weather forecast for multiple days")
        .required_string("location", "Location to get forecast for")
        .required_integer("days", "Number of days (1-7)")
        .build()?;

    let weather_alerts = FunctionBuilder::new()
        .name("get_weather_alerts")
        .description("Get weather alerts and warnings")
        .required_string("location", "Location to check for alerts")
        .build()?;

    let tools = vec![
        Tool::function(current_weather),
        Tool::function(weather_forecast),
        Tool::function(weather_alerts),
    ];

    // Create a conversation with the weather assistant
    let conversation = ChatBuilder::new()
        .developer("You are a helpful weather assistant. Use the available functions to provide accurate weather information. Always check for weather alerts when appropriate.")
        .user("I'm planning a camping trip to Yellowstone National Park next weekend. What should I expect weather-wise?");

    println!(
        "User: I'm planning a camping trip to Yellowstone National Park next weekend. What should I expect weather-wise?"
    );

    // Configure function calling with strict mode
    let config = FunctionConfig::new()
        .with_tools(tools)
        .with_tool_choice(ToolChoice::auto())
        .with_strict_mode(true)
        .with_parallel_calls(true);

    let request = openai_rust_sdk::models::responses::ResponseRequest::new_messages(
        "gpt-4",
        conversation.build(),
    );

    // Execute with error handling
    match client.create_function_response(&request, &config).await {
        Ok(response) => {
            if let Some(content) = &response.content {
                println!("Assistant: {}", content);
            }

            // Handle function calls
            let mut all_results = Vec::new();
            for call in &response.function_calls {
                match client.execute_function_with_result(call).await {
                    Ok(result) => {
                        println!("✅ {}: {}", call.name, result.output);
                        all_results.push(result);
                    }
                    Err(e) => {
                        println!("❌ Error executing {}: {}", call.name, e);
                        // Create error result
                        all_results.push(FunctionCallOutput::new(
                            &call.call_id,
                            format!("Error: {}", e),
                        ));
                    }
                }
            }

            // Get final response with all results
            if !all_results.is_empty() {
                match client
                    .submit_function_results(all_results, &request, &config)
                    .await
                {
                    Ok(final_response) => {
                        if let Some(final_content) = final_response.content {
                            println!("Assistant (final): {}", final_content);
                        }
                    }
                    Err(e) => {
                        println!("❌ Error getting final response: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Error in function calling: {}", e);
        }
    }

    println!();
    Ok(())
}

/// Helper function to demonstrate function result formats
fn _demo_function_results() {
    println!("Demo: Different function result formats");

    // JSON result
    let json_result = FunctionCallOutput::from_json(
        "call-123",
        json!({
            "temperature": 22.5,
            "humidity": 65,
            "description": "Partly cloudy",
            "wind": {
                "speed": 10,
                "direction": "NW"
            }
        }),
    )
    .unwrap();

    println!("JSON result: {}", json_result.output);

    // Simple string result
    let string_result = FunctionCallOutput::new(
        "call-456",
        "The meeting has been scheduled for 2:00 PM tomorrow.",
    );

    println!("String result: {}", string_result.output);

    // Error result
    let error_result = FunctionCallOutput::new(
        "call-789",
        "Error: Location not found. Please check the spelling and try again.",
    );

    println!("Error result: {}", error_result.output);
}
