//! Validation demonstrations and type-safe response handling

use crate::response_format_modules::type_definitions::Person;
use openai_rust_sdk::{
    api::ResponsesApi,
    models::responses::{JsonSchemaSpec, ResponseRequest},
};
use serde_json::{json, Value};

/// Create a strict schema for API response validation
pub fn create_strict_schema() -> Value {
    json!({
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
    })
}

/// Create valid test data for strict mode validation
pub fn create_valid_test_data() -> Value {
    json!({
        "status": "success",
        "data": {
            "id": "item-123",
            "value": 42.5
        },
        "timestamp": "2024-01-01T12:00:00Z"
    })
}

/// Create invalid test data for strict mode validation
pub fn create_invalid_test_data() -> Value {
    json!({
        "status": "success",
        "data": {
            "id": "item-123",
            "value": 42.5,
            "extra_field": "not allowed"  // This should fail in strict mode
        },
        "timestamp": "2024-01-01T12:00:00Z"
    })
}

/// Validate and print results for a given schema specification
pub fn validate_and_print_results(spec: &JsonSchemaSpec, data: &Value, description: &str) {
    let validation = spec.validate(data);
    if validation.is_valid {
        println!("✅ {description}: {}", validation.is_valid);
    } else {
        println!(
            "❌ {description}: {} (errors: {:?})",
            validation.is_valid, validation.errors
        );
    }
}

/// Demonstrate strict mode schema enforcement
pub async fn demo_strict_mode_enforcement(
    _client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating strict mode schema enforcement...");

    let strict_schema = create_strict_schema();

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

    // Test valid data
    let valid_data = create_valid_test_data();
    validate_and_print_results(&spec, &valid_data, "Valid strict data");

    // Test invalid data
    let invalid_data = create_invalid_test_data();
    validate_and_print_results(&spec, &invalid_data, "Invalid strict data");

    Ok(())
}

/// Demonstrate type-safe response parsing
pub async fn demo_type_safe_responses(
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

/// Run all validation demonstrations
pub async fn run_validation_demos(client: &ResponsesApi) -> Result<(), Box<dyn std::error::Error>> {
    // Example 4: Strict Mode Schema Enforcement
    println!("🔒 Example 4: Strict Mode Enforcement");
    println!("-------------------------------------");
    demo_strict_mode_enforcement(client).await?;

    // Example 5: Type-Safe Structured Responses
    println!("\n🎯 Example 5: Type-Safe Responses");
    println!("---------------------------------");
    demo_type_safe_responses(client).await?;

    Ok(())
}
