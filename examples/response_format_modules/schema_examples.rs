//! Example schemas for different use cases

use serde_json::{json, Value};
use std::collections::HashMap;

/// Helper function to create example schemas for different use cases
#[must_use]
pub fn create_example_schemas() -> HashMap<String, Value> {
    let mut schemas = HashMap::new();

    add_product_schema(&mut schemas);
    add_article_schema(&mut schemas);
    add_analytics_report_schema(&mut schemas);

    schemas
}

/// Add product schema to the collection
fn add_product_schema(schemas: &mut HashMap<String, Value>) {
    let product_schema = create_product_schema();
    schemas.insert("product".to_string(), product_schema);
}

/// Create a product schema for e-commerce applications
fn create_product_schema() -> Value {
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

/// Add article schema to the collection
fn add_article_schema(schemas: &mut HashMap<String, Value>) {
    let article_schema = create_article_schema();
    schemas.insert("article".to_string(), article_schema);
}

/// Create an article schema for content management
fn create_article_schema() -> Value {
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

/// Create an author schema for article authors
fn create_author_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "email": {"type": "string", "format": "email"}
        },
        "required": ["name"]
    })
}

/// Add analytics report schema to the collection
fn add_analytics_report_schema(schemas: &mut HashMap<String, Value>) {
    let analytics_schema = create_analytics_report_schema();
    schemas.insert("analytics_report".to_string(), analytics_schema);
}

/// Create an analytics report schema for data analysis
fn create_analytics_report_schema() -> Value {
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

/// Create a period schema for date ranges
fn create_period_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "start_date": {"type": "string", "format": "date"},
            "end_date": {"type": "string", "format": "date"}
        },
        "required": ["start_date", "end_date"]
    })
}

/// Create a metrics schema for analytics data
fn create_metrics_schema() -> Value {
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

/// Create a top pages schema for popular page tracking
fn create_top_pages_schema() -> Value {
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
    use openai_rust_sdk::models::responses::JsonSchemaSpec;

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
}
