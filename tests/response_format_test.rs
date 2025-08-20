#![allow(clippy::pedantic, clippy::nursery)]
//! Tests for response format enforcement and JSON schema validation

use openai_rust_sdk::models::responses::{
    JsonSchemaSpec, ResponseFormat, ResponseRequest, SchemaUtils,
};
use openai_rust_sdk::schema::SchemaBuilder;
use serde_json::json;

#[test]
fn test_response_format_creation() {
    // Test JSON object format
    let json_format = ResponseFormat::json_object();
    assert!(matches!(json_format, ResponseFormat::JsonObject));

    // Test JSON schema format
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "integer"}
        },
        "required": ["name"]
    });

    let schema_format = ResponseFormat::json_schema("test_schema", schema.clone());
    assert!(matches!(schema_format, ResponseFormat::JsonSchema { .. }));

    if let ResponseFormat::JsonSchema {
        json_schema,
        strict,
    } = schema_format
    {
        assert_eq!(json_schema.name, "test_schema");
        assert_eq!(json_schema.schema, schema);
        assert!(!strict);
    }

    // Test strict JSON schema format
    let strict_format = ResponseFormat::strict_json_schema("strict_schema", schema.clone());
    if let ResponseFormat::JsonSchema {
        json_schema,
        strict,
    } = strict_format
    {
        assert_eq!(json_schema.name, "strict_schema");
        assert!(strict);
        assert!(json_schema.strict);
    }
}

#[test]
fn test_response_format_utility_methods() {
    let format = ResponseFormat::JsonObject;
    assert!(format.is_structured());
    assert!(!format.requires_schema_validation());
    assert!(format.schema().is_none());

    let schema_format = ResponseFormat::json_schema("test", json!({"type": "object"}));
    assert!(schema_format.is_structured());
    assert!(schema_format.requires_schema_validation());
    assert!(schema_format.schema().is_some());
}

#[test]
fn test_json_schema_spec_creation() {
    let schema = json!({
        "type": "object",
        "properties": {"name": {"type": "string"}},
        "required": ["name"]
    });

    // Basic creation
    let spec = JsonSchemaSpec::new("test_spec", schema.clone());
    assert_eq!(spec.name, "test_spec");
    assert_eq!(spec.schema, schema);
    assert!(!spec.strict);
    assert!(spec.description.is_none());

    // Strict creation
    let strict_spec = JsonSchemaSpec::strict("strict_spec", schema.clone());
    assert_eq!(strict_spec.name, "strict_spec");
    assert!(strict_spec.strict);

    // With description
    let described_spec = JsonSchemaSpec::with_description(
        "described_spec",
        "A test schema with description",
        schema.clone(),
    );
    assert_eq!(
        described_spec.description,
        Some("A test schema with description".to_string())
    );

    // Method chaining
    let chained_spec = JsonSchemaSpec::new("chained", schema.clone())
        .set_description("Chained description")
        .set_strict(true);
    assert_eq!(
        chained_spec.description,
        Some("Chained description".to_string())
    );
    assert!(chained_spec.strict);
}

#[test]
fn test_json_schema_validation() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "integer", "minimum": 0}
        },
        "required": ["name"],
        "additionalProperties": false
    });

    let spec = JsonSchemaSpec::new("person", schema);

    // Valid data
    let valid_data = json!({"name": "John", "age": 30});
    let result = spec.validate(&valid_data);
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
    assert_eq!(result.data, Some(valid_data.clone()));

    // Invalid data - missing required field
    let invalid_data = json!({"age": 30});
    let result = spec.validate(&invalid_data);
    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());

    // Invalid data - wrong type
    let wrong_type_data = json!({"name": "John", "age": "thirty"});
    let result = spec.validate(&wrong_type_data);
    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());

    // Invalid data - additional properties (strict mode)
    let extra_props_data = json!({"name": "John", "age": 30, "city": "New York"});
    let result = spec.validate(&extra_props_data);
    assert!(!result.is_valid); // Should fail due to additionalProperties: false
}

#[test]
fn test_response_request_with_formats() {
    // Test with JSON mode
    let request = ResponseRequest::new_text("gpt-4", "Generate a JSON object").with_json_mode();

    assert!(matches!(
        request.response_format,
        Some(ResponseFormat::JsonObject)
    ));

    // Test with JSON schema
    let schema = json!({
        "type": "object",
        "properties": {"result": {"type": "string"}},
        "required": ["result"]
    });

    let schema_request = ResponseRequest::new_text("gpt-4", "Generate structured data")
        .with_json_schema("result_schema", schema.clone());

    if let Some(ResponseFormat::JsonSchema { json_schema, .. }) = &schema_request.response_format {
        assert_eq!(json_schema.name, "result_schema");
        assert_eq!(json_schema.schema, schema);
    }

    // Test with strict schema
    let strict_request = ResponseRequest::new_text("gpt-4", "Generate strict data")
        .with_strict_json_schema("strict_schema", schema.clone());

    if let Some(ResponseFormat::JsonSchema {
        json_schema,
        strict,
    }) = &strict_request.response_format
    {
        assert_eq!(json_schema.name, "strict_schema");
        assert!(json_schema.strict);
        assert!(*strict);
    }
}

#[test]
fn test_response_request_with_schema_builder() {
    let builder = SchemaBuilder::object()
        .property("name", SchemaBuilder::string())
        .property("age", SchemaBuilder::integer().minimum(0.0))
        .description("Person schema");

    let request = ResponseRequest::new_text("gpt-4", "Create a person")
        .with_schema_builder("person", builder.clone());

    assert!(request.response_format.is_some());
    if let Some(ResponseFormat::JsonSchema { json_schema, .. }) = &request.response_format {
        assert_eq!(json_schema.name, "person");
        assert!(!json_schema.strict);
    }

    // Test strict builder
    let strict_builder = SchemaBuilder::object()
        .property("name", SchemaBuilder::string())
        .property("age", SchemaBuilder::integer().minimum(0.0))
        .description("Person schema");
    let strict_request = ResponseRequest::new_text("gpt-4", "Create a strict person")
        .with_strict_schema_builder("strict_person", strict_builder);

    if let Some(ResponseFormat::JsonSchema {
        json_schema,
        strict,
    }) = &strict_request.response_format
    {
        assert_eq!(json_schema.name, "strict_person");
        assert!(json_schema.strict);
        assert!(*strict);
    }
}

#[test]
fn test_schema_utils() {
    // Test object schema generation
    let properties = &[
        ("name", "string"),
        ("age", "integer"),
        ("active", "boolean"),
    ];
    let schema = SchemaUtils::object_schema(properties);

    assert_eq!(schema["type"], "object");
    assert!(schema["properties"].is_object());
    assert!(schema["required"].is_array());
    assert_eq!(schema["additionalProperties"], false);

    let props = schema["properties"].as_object().unwrap();
    assert_eq!(props["name"]["type"], "string");
    assert_eq!(props["age"]["type"], "integer");
    assert_eq!(props["active"]["type"], "boolean");

    // Test array schema
    let array_schema = SchemaUtils::array_schema("string");
    assert_eq!(array_schema["type"], "array");
    assert_eq!(array_schema["items"]["type"], "string");

    // Test enum schema
    let enum_values = &["red", "green", "blue"];
    let enum_schema = SchemaUtils::enum_schema(enum_values);
    assert_eq!(enum_schema["type"], "string");
    let enum_array = enum_schema["enum"].as_array().unwrap();
    assert_eq!(enum_array.len(), 3);
    assert!(enum_array.contains(&json!("red")));

    // Test union schema
    let schemas = &[json!({"type": "string"}), json!({"type": "integer"})];
    let union_schema = SchemaUtils::union_schema(schemas);
    assert!(union_schema["anyOf"].is_array());
    let any_of = union_schema["anyOf"].as_array().unwrap();
    assert_eq!(any_of.len(), 2);
}

#[test]
fn test_detailed_response_format() {
    let schema = json!({
        "type": "object",
        "properties": {
            "status": {"type": "string", "enum": ["success", "error"]},
            "data": {"type": "object"},
            "message": {"type": "string"}
        },
        "required": ["status"]
    });

    let request = ResponseRequest::new_text("gpt-4", "Generate API response")
        .with_detailed_json_schema(
            "api_response",
            Some("Schema for API response format".to_string()),
            schema.clone(),
            true, // strict mode
        );

    if let Some(ResponseFormat::JsonSchema {
        json_schema,
        strict,
    }) = &request.response_format
    {
        assert_eq!(json_schema.name, "api_response");
        assert_eq!(
            json_schema.description,
            Some("Schema for API response format".to_string())
        );
        assert_eq!(json_schema.schema, schema);
        assert!(json_schema.strict);
        assert!(*strict);
    }
}

#[test]
fn test_complex_schema_validation() {
    // Create a complex nested schema
    let schema = json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "id": {"type": "integer"},
                    "name": {"type": "string"},
                    "email": {"type": "string", "format": "email"}
                },
                "required": ["id", "name"]
            },
            "permissions": {
                "type": "array",
                "items": {"type": "string", "enum": ["read", "write", "admin"]},
                "minItems": 1
            },
            "settings": {
                "type": "object",
                "additionalProperties": true
            }
        },
        "required": ["user", "permissions"]
    });

    let spec = JsonSchemaSpec::new("complex_user", schema);

    // Valid complex data
    let valid_data = json!({
        "user": {
            "id": 123,
            "name": "John Doe",
            "email": "john@example.com"
        },
        "permissions": ["read", "write"],
        "settings": {
            "theme": "dark",
            "notifications": true
        }
    });

    let result = spec.validate(&valid_data);
    assert!(result.is_valid, "Valid complex data should pass validation");

    // Invalid complex data - missing required nested field
    let invalid_data = json!({
        "user": {
            "id": 123
            // missing "name"
        },
        "permissions": ["read"]
    });

    let result = spec.validate(&invalid_data);
    assert!(!result.is_valid, "Invalid data should fail validation");
    assert!(!result.errors.is_empty());
}

#[test]
fn test_response_format_from_builder() {
    // Test conversion from builder to response format
    let builder = SchemaBuilder::object()
        .property(
            "task",
            SchemaBuilder::string().description("Task description"),
        )
        .property(
            "priority",
            SchemaBuilder::string()
                .enum_values(&[json!("low"), json!("medium"), json!("high")])
                .description("Task priority"),
        )
        .property(
            "completed",
            SchemaBuilder::boolean().description("Task completion status"),
        )
        .required(&["task", "priority"]);

    let schema_json = builder.build().to_value();
    let response_format = ResponseFormat::json_schema("task_schema", schema_json.clone());
    assert!(matches!(response_format, ResponseFormat::JsonSchema { .. }));

    if let ResponseFormat::JsonSchema {
        json_schema,
        strict,
    } = response_format
    {
        assert_eq!(json_schema.name, "task_schema");
        assert!(!strict);

        // Verify the schema structure
        assert_eq!(json_schema.schema["type"], "object");
        let properties = json_schema.schema["properties"].as_object().unwrap();
        assert!(properties.contains_key("task"));
        assert!(properties.contains_key("priority"));
        assert!(properties.contains_key("completed"));

        let required = json_schema.schema["required"].as_array().unwrap();
        assert!(required.contains(&json!("task")));
        assert!(required.contains(&json!("priority")));
        assert!(!required.contains(&json!("completed")));
    }
}

// Example usage tests showing how to use the structured output system
#[cfg(test)]
mod integration_tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        email: Option<String>,
    }

    #[test]
    fn test_typed_schema_creation() {
        // Create schema for Person struct manually
        let person_schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer", "minimum": 0},
                "email": {"type": "string", "format": "email"}
            },
            "required": ["name", "age"],
            "additionalProperties": false
        });

        let spec = JsonSchemaSpec::new("person", person_schema);

        // Test valid person data
        let valid_person = json!({
            "name": "Alice",
            "age": 25,
            "email": "alice@example.com"
        });

        let result = spec.validate(&valid_person);
        assert!(result.is_valid);

        // Test deserialization
        let person: Person = serde_json::from_value(valid_person).unwrap();
        assert_eq!(person.name, "Alice");
        assert_eq!(person.age, 25);
        assert_eq!(person.email, Some("alice@example.com".to_string()));
    }

    #[test]
    fn test_builder_pattern_for_complex_types() {
        // Build a schema for a task management system
        let task_schema = SchemaBuilder::object()
            .property(
                "id",
                SchemaBuilder::string().description("Unique task identifier"),
            )
            .property(
                "title",
                SchemaBuilder::string().min_length(1).max_length(100),
            )
            .property(
                "status",
                SchemaBuilder::string().enum_values(&[
                    json!("todo"),
                    json!("in_progress"),
                    json!("done"),
                ]),
            )
            .property("description", SchemaBuilder::string().max_length(500))
            .property(
                "tags",
                SchemaBuilder::array().items(SchemaBuilder::string()),
            )
            .property(
                "assignee",
                SchemaBuilder::object()
                    .property("id", SchemaBuilder::string())
                    .property("name", SchemaBuilder::string())
                    .property("email", SchemaBuilder::string().format("email")),
            )
            .description("Task object schema")
            .build()
            .to_value();

        let spec = JsonSchemaSpec::new("task", task_schema);

        // Test with complete task data
        let complete_task = json!({
            "id": "task-123",
            "title": "Implement feature X",
            "status": "in_progress",
            "description": "Add new functionality to the system",
            "tags": ["feature", "backend"],
            "assignee": {
                "id": "user-456",
                "name": "Bob Developer",
                "email": "bob@company.com"
            }
        });

        let result = spec.validate(&complete_task);
        assert!(result.is_valid, "Complete task should be valid");

        // Test with minimal task data
        let minimal_task = json!({
            "id": "task-124",
            "title": "Fix bug Y",
            "status": "todo"
        });

        let result = spec.validate(&minimal_task);
        assert!(result.is_valid, "Minimal task should be valid");

        // Test with invalid status
        let invalid_task = json!({
            "id": "task-125",
            "title": "Invalid task",
            "status": "invalid_status"
        });

        let result = spec.validate(&invalid_task);
        assert!(!result.is_valid, "Task with invalid status should fail");
    }
}
