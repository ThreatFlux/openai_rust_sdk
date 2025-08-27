//! Basic response format demonstrations

use openai_rust_sdk::{
    api::ResponsesApi,
    models::responses::{JsonSchemaSpec, ResponseRequest, SchemaUtils},
};
use serde_json::json;

/// Demonstrate JSON object mode without strict schema validation
pub async fn demo_json_object_mode(
    _client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Requesting JSON object response without strict schema...");

    let request = ResponseRequest::new_text(
        "gpt-4",
        "Generate a JSON object representing a book with title, author, year, and genres (array)",
    )
    .with_json_mode();

    // Note: In a real scenario, this would make an API call
    // For demo purposes, we'll show the request structure
    println!("Request format: {:?}", request.response_format);

    // Simulated response processing
    let example_response = json!({
        "title": "The Rust Programming Language",
        "author": "Steve Klabnik",
        "year": 2018,
        "genres": ["Programming", "Computer Science", "Technology"]
    });

    println!(
        "✅ Received JSON object: {}",
        serde_json::to_string_pretty(&example_response)?
    );

    Ok(())
}

/// Demonstrate simple schema validation using SchemaUtils
pub async fn demo_simple_schema_validation(
    _client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a simple person schema...");

    // Create a simple schema using SchemaUtils
    let person_schema = SchemaUtils::object_schema(&[
        ("name", "string"),
        ("age", "integer"),
        ("occupation", "string"),
    ]);

    let request =
        ResponseRequest::new_text("gpt-4", "Generate information about a software engineer")
            .with_json_schema("person", person_schema.clone());

    println!("Schema: {}", serde_json::to_string_pretty(&person_schema)?);
    println!("Request format: {:?}", request.response_format);

    // Validate example data against the schema
    let spec = JsonSchemaSpec::new("person", person_schema);
    let example_data = json!({
        "name": "Alice Johnson",
        "age": 29,
        "occupation": "Software Engineer"
    });

    let validation = spec.validate(&example_data);
    println!("✅ Validation result: {validation:?}");

    Ok(())
}

/// Run all basic format demonstrations
pub async fn run_basic_format_demos(
    client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Basic JSON Object Mode
    println!("📝 Example 1: JSON Object Mode");
    println!("-----------------------------");
    demo_json_object_mode(client).await?;

    // Example 2: JSON Schema with Simple Validation
    println!("\n🔍 Example 2: Simple Schema Validation");
    println!("--------------------------------------");
    demo_simple_schema_validation(client).await?;

    Ok(())
}
