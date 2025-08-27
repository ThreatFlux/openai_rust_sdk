#![allow(clippy::pedantic, clippy::nursery)]
//! Response Format Enforcement Demo
//!
//! This example demonstrates the new response format enforcement features:
//! - JSON Object mode for unstructured JSON responses
//! - JSON Schema mode for strictly validated structured outputs
//! - Schema builders for creating complex validation schemas
//! - Type-safe parsing of structured responses

use openai_rust_sdk::{
    api::{common::ApiClientConstructors, ResponsesApi},
    models::responses::{JsonSchemaSpec, ResponseRequest, SchemaUtils},
    schema::SchemaBuilder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    occupation: String,
    skills: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct TaskList {
    title: String,
    tasks: Vec<Task>,
    priority: Priority,
    estimated_hours: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Task {
    id: String,
    description: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Initialize the responses API client
fn initialize_responses_api() -> Result<ResponsesApi, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    Ok(ResponsesApi::new(api_key)?)
}

/// Run basic format demonstrations
async fn run_basic_format_demos(client: &ResponsesApi) -> Result<(), Box<dyn std::error::Error>> {
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

/// Run advanced schema demonstrations
async fn run_advanced_schema_demos(
    client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    // Example 3: Complex Schema with Schema Builder
    println!("\n🏗️  Example 3: Complex Schema with Builder");
    println!("------------------------------------------");
    demo_complex_schema_builder(client).await?;

    // Example 4: Strict Mode Schema Enforcement
    println!("\n🔒 Example 4: Strict Mode Enforcement");
    println!("-------------------------------------");
    demo_strict_mode_enforcement(client).await?;

    Ok(())
}

/// Run validation and error handling demonstrations
async fn run_validation_demos(client: &ResponsesApi) -> Result<(), Box<dyn std::error::Error>> {
    // Example 5: Type-Safe Structured Responses
    println!("\n🎯 Example 5: Type-Safe Responses");
    println!("---------------------------------");
    demo_type_safe_responses(client).await?;

    // Example 6: Error Handling and Validation
    println!("\n❌ Example 6: Error Handling");
    println!("----------------------------");
    demo_error_handling(client).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Response Format Enforcement Demo");
    println!("====================================\n");

    let client = initialize_responses_api()?;

    run_basic_format_demos(&client).await?;
    run_advanced_schema_demos(&client).await?;
    run_validation_demos(&client).await?;

    println!("\n✅ All examples completed successfully!");

    Ok(())
}

async fn demo_json_object_mode(_client: &ResponsesApi) -> Result<(), Box<dyn std::error::Error>> {
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

async fn demo_simple_schema_validation(
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

async fn demo_complex_schema_builder(
    _client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Building complex task list schema...");

    let schemas = build_task_list_schemas();
    let request = create_task_list_request(schemas.task_list_schema.clone());
    demonstrate_schema_output(&request, &schemas.task_list_schema)?;
    validate_example_data(&schemas.task_list_schema)?;

    Ok(())
}

struct TaskListSchemas {
    task_schema: SchemaBuilder,
    priority_schema: SchemaBuilder,
    task_list_schema: SchemaBuilder,
}

fn build_task_list_schemas() -> TaskListSchemas {
    let task_schema = build_task_schema();
    let priority_schema = build_priority_schema();
    let task_list_schema = build_task_list_schema(&task_schema, &priority_schema);

    TaskListSchemas {
        task_schema,
        priority_schema,
        task_list_schema,
    }
}

fn build_task_schema() -> SchemaBuilder {
    SchemaBuilder::object()
        .property(
            "id",
            SchemaBuilder::string().description("Unique task identifier"),
        )
        .property(
            "description",
            SchemaBuilder::string().min_length(1).max_length(200),
        )
        .property("completed", SchemaBuilder::boolean())
        .required(&["id", "description", "completed"])
        .additional_properties(false)
}

fn build_priority_schema() -> SchemaBuilder {
    SchemaBuilder::string()
        .enum_values(&[
            json!("low"),
            json!("medium"),
            json!("high"),
            json!("critical"),
        ])
        .description("Task priority level")
}

fn build_task_list_schema(
    task_schema: &SchemaBuilder,
    priority_schema: &SchemaBuilder,
) -> SchemaBuilder {
    SchemaBuilder::object()
        .property("title", SchemaBuilder::string().min_length(1))
        .property(
            "tasks",
            SchemaBuilder::array()
                .items(task_schema.clone())
                .min_items(1),
        )
        .property("priority", priority_schema.clone())
        .property("estimated_hours", SchemaBuilder::number().minimum(0.0))
        .required(&["title", "tasks", "priority", "estimated_hours"])
        .description("A list of tasks with metadata")
        .additional_properties(false)
}

fn create_task_list_request(task_list_schema: SchemaBuilder) -> ResponseRequest {
    ResponseRequest::new_text(
        "gpt-4",
        "Create a task list for developing a web application with 3 tasks",
    )
    .with_schema_builder("task_list", task_list_schema)
}

fn demonstrate_schema_output(
    request: &ResponseRequest,
    task_list_schema: &SchemaBuilder,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Request format: {:?}", request.response_format);

    let schema_json = task_list_schema.clone().build().to_value();
    println!(
        "Generated schema: {}",
        serde_json::to_string_pretty(&schema_json)?
    );

    Ok(())
}

fn validate_example_data(
    task_list_schema: &SchemaBuilder,
) -> Result<(), Box<dyn std::error::Error>> {
    let schema_json = task_list_schema.clone().build().to_value();
    let spec = JsonSchemaSpec::new("task_list", schema_json);
    let example_task_list = create_example_task_list();

    let validation = spec.validate(&example_task_list);
    println!("✅ Complex schema validation: {:?}", validation.is_valid);

    Ok(())
}

fn create_example_task_list() -> serde_json::Value {
    json!({
        "title": "Web App Development",
        "tasks": [
            {
                "id": "task-1",
                "description": "Set up project structure",
                "completed": true
            },
            {
                "id": "task-2",
                "description": "Implement user authentication",
                "completed": false
            },
            {
                "id": "task-3",
                "description": "Create responsive UI",
                "completed": false
            }
        ],
        "priority": "high",
        "estimated_hours": 40.5
    })
}

async fn demo_strict_mode_enforcement(
    _client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating strict mode schema enforcement...");

    let strict_schema = json!({
        "type": "object",
        "properties": {
            "status": {
                "type": "string",
                "enum": ["success", "error", "pending"]
            },
            "data": {
                "type": "object",
                "properties": {
                    "id": {"type": "string"},
                    "value": {"type": "number"}
                },
                "required": ["id", "value"],
                "additionalProperties": false
            },
            "timestamp": {
                "type": "string",
                "format": "date-time"
            }
        },
        "required": ["status", "data"],
        "additionalProperties": false
    });

    let _request = ResponseRequest::new_text(
        "gpt-4",
        "Generate an API response with status, data, and timestamp",
    )
    .with_strict_json_schema("api_response", strict_schema.clone());

    println!(
        "Strict schema: {}",
        serde_json::to_string_pretty(&strict_schema)?
    );

    // Test strict validation
    let spec = JsonSchemaSpec::strict("api_response", strict_schema);

    // Valid data (follows schema exactly)
    let valid_data = json!({
        "status": "success",
        "data": {
            "id": "item-123",
            "value": 42.5
        },
        "timestamp": "2024-01-01T12:00:00Z"
    });

    let validation = spec.validate(&valid_data);
    println!("✅ Valid strict data: {}", validation.is_valid);

    // Invalid data (extra properties)
    let invalid_data = json!({
        "status": "success",
        "data": {
            "id": "item-123",
            "value": 42.5,
            "extra_field": "not allowed"  // This should fail in strict mode
        },
        "timestamp": "2024-01-01T12:00:00Z"
    });

    let validation = spec.validate(&invalid_data);
    println!(
        "❌ Invalid strict data: {} (errors: {:?})",
        validation.is_valid, validation.errors
    );

    Ok(())
}

async fn demo_type_safe_responses(
    _client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating type-safe response parsing...");

    // Create schema for Person struct
    let person_schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "integer", "minimum": 0},
            "occupation": {"type": "string"},
            "skills": {
                "type": "array",
                "items": {"type": "string"},
                "minItems": 1
            }
        },
        "required": ["name", "age", "occupation", "skills"],
        "additionalProperties": false
    });

    let request =
        ResponseRequest::new_text("gpt-4", "Generate a person profile for a data scientist")
            .with_json_schema("person", person_schema.clone());

    println!("Type-safe request format: {:?}", request.response_format);

    // Simulate parsing structured response
    let example_person_data = json!({
        "name": "Dr. Sarah Chen",
        "age": 34,
        "occupation": "Data Scientist",
        "skills": ["Python", "Machine Learning", "Statistics", "SQL", "Deep Learning"]
    });

    // Parse into strongly-typed struct
    let person: Person = serde_json::from_value(example_person_data.clone())?;
    println!("✅ Parsed Person: {person:#?}");

    // Validate before parsing
    let spec = JsonSchemaSpec::new("person", person_schema);
    let validation = spec.validate(&example_person_data);

    if validation.is_valid {
        println!("✅ Schema validation passed, safe to parse");
    } else {
        println!("❌ Schema validation failed: {:?}", validation.errors);
    }

    Ok(())
}

async fn demo_error_handling(_client: &ResponsesApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating error handling and validation failures...");

    let schema = json!({
        "type": "object",
        "properties": {
            "email": {"type": "string", "format": "email"},
            "age": {"type": "integer", "minimum": 18, "maximum": 100},
            "category": {"type": "string", "enum": ["student", "teacher", "admin"]}
        },
        "required": ["email", "age", "category"],
        "additionalProperties": false
    });

    let spec = JsonSchemaSpec::new("user", schema);

    // Test various validation failures
    let test_cases = vec![
        (
            "Invalid email format",
            json!({"email": "not-an-email", "age": 25, "category": "student"}),
        ),
        (
            "Age out of range",
            json!({"email": "user@example.com", "age": 200, "category": "student"}),
        ),
        (
            "Invalid enum value",
            json!({"email": "user@example.com", "age": 25, "category": "invalid"}),
        ),
        (
            "Missing required field",
            json!({"email": "user@example.com", "age": 25}),
        ),
        (
            "Extra properties",
            json!({"email": "user@example.com", "age": 25, "category": "student", "extra": "not allowed"}),
        ),
    ];

    for (description, test_data) in test_cases {
        let validation = spec.validate(&test_data);
        println!("Test: {} -> Valid: {}", description, validation.is_valid);
        if !validation.is_valid {
            println!("  Errors: {:?}", validation.errors);
        }
    }

    // Valid case
    let valid_data = json!({
        "email": "student@university.edu",
        "age": 20,
        "category": "student"
    });

    let validation = spec.validate(&valid_data);
    println!("✅ Valid case: {} (no errors)", validation.is_valid);

    Ok(())
}

/// Helper function to create example schemas for different use cases
#[must_use]
pub fn create_example_schemas() -> std::collections::HashMap<String, serde_json::Value> {
    let mut schemas = std::collections::HashMap::new();

    add_product_schema(&mut schemas);
    add_article_schema(&mut schemas);
    add_analytics_report_schema(&mut schemas);

    schemas
}

fn add_product_schema(schemas: &mut std::collections::HashMap<String, serde_json::Value>) {
    let product_schema = create_product_schema();
    schemas.insert("product".to_string(), product_schema);
}

fn create_product_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "id": {"type": "string"},
            "name": {"type": "string"},
            "price": {"type": "number", "minimum": 0},
            "category": {"type": "string", "enum": ["electronics", "books", "clothing", "home"]},
            "in_stock": {"type": "boolean"},
            "tags": {"type": "array", "items": {"type": "string"}}
        },
        "required": ["id", "name", "price", "category"],
        "additionalProperties": false
    })
}

fn add_article_schema(schemas: &mut std::collections::HashMap<String, serde_json::Value>) {
    let article_schema = create_article_schema();
    schemas.insert("article".to_string(), article_schema);
}

fn create_article_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "title": {"type": "string", "maxLength": 200},
            "content": {"type": "string"},
            "author": create_author_schema(),
            "published_at": {"type": "string", "format": "date-time"},
            "tags": {"type": "array", "items": {"type": "string"}},
            "word_count": {"type": "integer", "minimum": 1}
        },
        "required": ["title", "content", "author"],
        "additionalProperties": false
    })
}

fn create_author_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "email": {"type": "string", "format": "email"}
        },
        "required": ["name"]
    })
}

fn add_analytics_report_schema(schemas: &mut std::collections::HashMap<String, serde_json::Value>) {
    let analytics_schema = create_analytics_report_schema();
    schemas.insert("analytics_report".to_string(), analytics_schema);
}

fn create_analytics_report_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "report_id": {"type": "string"},
            "period": create_period_schema(),
            "metrics": create_metrics_schema(),
            "top_pages": create_top_pages_schema()
        },
        "required": ["report_id", "period", "metrics"],
        "additionalProperties": false
    })
}

fn create_period_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "start_date": {"type": "string", "format": "date"},
            "end_date": {"type": "string", "format": "date"}
        },
        "required": ["start_date", "end_date"]
    })
}

fn create_metrics_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "page_views": {"type": "integer", "minimum": 0},
            "unique_visitors": {"type": "integer", "minimum": 0},
            "bounce_rate": {"type": "number", "minimum": 0, "maximum": 1},
            "conversion_rate": {"type": "number", "minimum": 0, "maximum": 1}
        },
        "required": ["page_views", "unique_visitors"]
    })
}

fn create_top_pages_schema() -> serde_json::Value {
    json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "url": {"type": "string"},
                "views": {"type": "integer", "minimum": 0}
            },
            "required": ["url", "views"]
        },
        "maxItems": 10
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_schemas() {
        let schemas = create_example_schemas();
        assert!(schemas.contains_key("product"));
        assert!(schemas.contains_key("article"));
        assert!(schemas.contains_key("analytics_report"));

        // Test product schema validation
        let product_schema = schemas.get("product").unwrap();
        let spec = JsonSchemaSpec::new("product", product_schema.clone());

        let valid_product = json!({
            "id": "prod-123",
            "name": "Wireless Headphones",
            "price": 99.99,
            "category": "electronics",
            "in_stock": true,
            "tags": ["audio", "wireless", "bluetooth"]
        });

        let validation = spec.validate(&valid_product);
        assert!(validation.is_valid);
    }

    #[test]
    fn test_person_struct_serialization() {
        let person = Person {
            name: "Test User".to_string(),
            age: 30,
            occupation: "Developer".to_string(),
            skills: vec!["Rust".to_string(), "Python".to_string()],
        };

        let json_value = serde_json::to_value(&person).unwrap();
        let parsed_person: Person = serde_json::from_value(json_value).unwrap();

        assert_eq!(person.name, parsed_person.name);
        assert_eq!(person.age, parsed_person.age);
        assert_eq!(person.skills, parsed_person.skills);
    }
}
