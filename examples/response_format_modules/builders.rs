//! Schema builders and advanced schema construction

use openai_rust_sdk::{
    api::ResponsesApi,
    models::responses::{JsonSchemaSpec, ResponseRequest},
    schema::SchemaBuilder,
};
use serde_json::json;

/// Container for all task list schemas
#[allow(dead_code)]
pub struct TaskListSchemas {
    pub task_schema: SchemaBuilder,
    pub priority_schema: SchemaBuilder,
    pub task_list_schema: SchemaBuilder,
}

/// Build all schemas needed for task list demonstrations
pub fn build_task_list_schemas() -> TaskListSchemas {
    let task_schema = build_task_schema();
    let priority_schema = build_priority_schema();
    let task_list_schema = build_task_list_schema(&task_schema, &priority_schema);

    TaskListSchemas {
        task_schema,
        priority_schema,
        task_list_schema,
    }
}

/// Build schema for individual tasks
pub fn build_task_schema() -> SchemaBuilder {
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

/// Build schema for priority enumeration
pub fn build_priority_schema() -> SchemaBuilder {
    SchemaBuilder::string()
        .enum_values(&[
            json!("low"),
            json!("medium"),
            json!("high"),
            json!("critical"),
        ])
        .description("Task priority level")
}

/// Build the main task list schema
pub fn build_task_list_schema(
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

/// Create a request with the task list schema
pub fn create_task_list_request(task_list_schema: SchemaBuilder) -> ResponseRequest {
    ResponseRequest::new_text(
        "gpt-4",
        "Create a task list for developing a web application with 3 tasks",
    )
    .with_schema_builder("task_list", task_list_schema)
}

/// Demonstrate the schema output structure
pub fn demonstrate_schema_output(
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

/// Validate example data against the task list schema
pub fn validate_example_data(
    task_list_schema: &SchemaBuilder,
) -> Result<(), Box<dyn std::error::Error>> {
    let schema_json = task_list_schema.clone().build().to_value();
    let spec = JsonSchemaSpec::new("task_list", schema_json);
    let example_task_list = create_example_task_list();

    let validation = spec.validate(&example_task_list);
    println!("✅ Complex schema validation: {:?}", validation.is_valid);

    Ok(())
}

/// Create example task list data
pub fn create_example_task_list() -> serde_json::Value {
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

/// Demonstrate complex schema building
pub async fn demo_complex_schema_builder(
    _client: &ResponsesApi,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Building complex task list schema...");

    let schemas = build_task_list_schemas();
    let request = create_task_list_request(schemas.task_list_schema.clone());
    demonstrate_schema_output(&request, &schemas.task_list_schema)?;
    validate_example_data(&schemas.task_list_schema)?;

    Ok(())
}
