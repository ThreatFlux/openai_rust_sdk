//! Error handling and validation failure demonstrations

use openai_rust_sdk::{api::ResponsesApi, models::responses::JsonSchemaSpec};
use serde_json::{json, Value};

/// Create a user validation schema for testing error cases
pub fn create_user_validation_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "email": {"type": "string", "format": "email"},
            "age": {"type": "integer", "minimum": 18, "maximum": 100},
            "category": {"type": "string", "enum": ["student", "teacher", "admin"]}
        },
        "required": ["email", "age", "category"],
        "additionalProperties": false
    })
}

/// Create validation test cases with expected failures
pub fn create_validation_test_cases() -> Vec<(&'static str, Value)> {
    vec![
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
    ]
}

/// Test validation failure cases
pub fn test_validation_failure_cases(spec: &JsonSchemaSpec) {
    let test_cases = create_validation_test_cases();

    for (description, test_data) in test_cases {
        let validation = spec.validate(&test_data);
        println!("Test: {} -> Valid: {}", description, validation.is_valid);
        if !validation.is_valid {
            println!("  Errors: {:?}", validation.errors);
        }
    }
}

/// Test a valid case to ensure the schema works correctly
pub fn test_valid_case(spec: &JsonSchemaSpec) {
    let valid_data = json!({
        "email": "student@university.edu",
        "age": 20,
        "category": "student"
    });

    let validation = spec.validate(&valid_data);
    println!("✅ Valid case: {} (no errors)", validation.is_valid);
}

/// Demonstrate error handling and validation failures
pub async fn demo_error_handling(_client: &ResponsesApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating error handling and validation failures...");

    let schema = create_user_validation_schema();
    let spec = JsonSchemaSpec::new("user", schema);

    test_validation_failure_cases(&spec);
    test_valid_case(&spec);

    Ok(())
}

/// Run error handling demonstrations
pub async fn run_error_handling_demos(
    client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("❌ Example 6: Error Handling");
    println!("----------------------------");
    demo_error_handling(client).await?;

    Ok(())
}
