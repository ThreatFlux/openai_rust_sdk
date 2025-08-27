//! JSON Schema builder for structured outputs

/// Enhanced schema builder for function calling
pub mod builder;

use crate::{
    error::{OpenAIError, Result},
    De, Ser,
};
use indexmap::IndexMap;
use serde_json::{json, Value};
use std::collections::HashMap;

pub use builder::SchemaBuilder as EnhancedSchemaBuilder;

/// A JSON Schema wrapper for validation and compilation
#[derive(Debug, Clone, Ser, De)]
pub struct JsonSchema {
    /// The JSON schema definition
    pub schema: Value,
}

impl JsonSchema {
    /// Create a new JSON schema from a Value
    #[must_use]
    pub fn new(schema: Value) -> Self {
        Self { schema }
    }

    /// Validate data against this schema
    pub fn validate(&self, data: &Value) -> Result<()> {
        let compiled = jsonschema::JSONSchema::compile(&self.schema)
            .map_err(|e| OpenAIError::invalid_request(format!("Failed to compile schema: {e}")))?;

        if let Err(errors) = compiled.validate(data) {
            let error_messages: Vec<String> = errors.map(|e| e.to_string()).collect();
            return Err(OpenAIError::invalid_request(format!(
                "Schema validation failed: {}",
                error_messages.join(", ")
            )));
        }

        Ok(())
    }

    /// Convert the schema to a `serde_json::Value`
    #[must_use]
    pub fn to_value(&self) -> Value {
        self.schema.clone()
    }
}

/// A builder for creating JSON schemas
#[derive(Debug, Clone)]
pub struct SchemaBuilder {
    /// The JSON schema type (string, number, object, etc.)
    schema_type: Option<String>,
    /// Object properties definition
    properties: Option<IndexMap<String, Value>>,
    /// Array items schema definition
    items: Option<Box<Value>>,
    /// Required fields for objects
    required: Option<Vec<String>>,
    /// Enum values for validation
    enum_values: Option<Vec<Value>>,
    /// `AnyOf` schema variants
    any_of: Option<Vec<Value>>,
    /// String pattern for validation
    pattern: Option<String>,
    /// String format (email, date, etc.)
    format: Option<String>,
    /// Minimum value for numbers
    minimum: Option<f64>,
    /// Maximum value for numbers
    maximum: Option<f64>,
    /// Minimum string length
    min_length: Option<usize>,
    /// Maximum string length
    max_length: Option<usize>,
    /// Minimum array items
    min_items: Option<usize>,
    /// Maximum array items
    max_items: Option<usize>,
    /// Whether additional properties are allowed
    additional_properties: Option<bool>,
    /// Schema description
    description: Option<String>,
    /// Schema title
    title: Option<String>,
    /// Default value
    default: Option<Value>,
    /// Schema definitions for references
    definitions: Option<IndexMap<String, Value>>,
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaBuilder {
    /// Create a new empty schema builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            schema_type: None,
            properties: None,
            items: None,
            required: None,
            enum_values: None,
            any_of: None,
            pattern: None,
            format: None,
            minimum: None,
            maximum: None,
            min_length: None,
            max_length: None,
            min_items: None,
            max_items: None,
            additional_properties: None,
            description: None,
            title: None,
            default: None,
            definitions: None,
        }
    }

    /// Create a string schema
    #[must_use]
    pub fn string() -> Self {
        Self::new().schema_type("string")
    }

    /// Create a number schema
    #[must_use]
    pub fn number() -> Self {
        Self::new().schema_type("number")
    }

    /// Create an integer schema
    #[must_use]
    pub fn integer() -> Self {
        Self::new().schema_type("integer")
    }

    /// Create a boolean schema
    #[must_use]
    pub fn boolean() -> Self {
        Self::new().schema_type("boolean")
    }

    /// Create an array schema
    #[must_use]
    pub fn array() -> Self {
        Self::new().schema_type("array")
    }

    /// Create an object schema
    #[must_use]
    pub fn object() -> Self {
        Self::new().schema_type("object")
    }

    /// Create a null schema
    #[must_use]
    pub fn null() -> Self {
        Self::new().schema_type("null")
    }

    /// Set the schema type
    #[must_use]
    pub fn schema_type(mut self, type_name: &str) -> Self {
        self.schema_type = Some(type_name.to_string());
        self
    }

    /// Add a property to object schema
    #[must_use]
    pub fn property(mut self, name: &str, schema: SchemaBuilder) -> Self {
        if self.properties.is_none() {
            self.properties = Some(IndexMap::new());
        }

        if let Some(ref mut props) = self.properties {
            props.insert(name.to_string(), schema.build().schema);
        }
        self
    }

    /// Add a property with raw JSON value
    #[must_use]
    pub fn property_value(mut self, name: &str, schema: Value) -> Self {
        if self.properties.is_none() {
            self.properties = Some(IndexMap::new());
        }

        if let Some(ref mut props) = self.properties {
            props.insert(name.to_string(), schema);
        }
        self
    }

    /// Set items schema for arrays
    #[must_use]
    pub fn items(mut self, items_schema: SchemaBuilder) -> Self {
        self.items = Some(Box::new(items_schema.build().schema));
        self
    }

    /// Set items schema with raw JSON value
    #[must_use]
    pub fn items_value(mut self, items_schema: Value) -> Self {
        self.items = Some(Box::new(items_schema));
        self
    }

    /// Set required fields for object schema
    pub fn required<S: AsRef<str>>(mut self, fields: &[S]) -> Self {
        self.required = Some(fields.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    /// Set enum values for validation
    #[must_use]
    pub fn enum_values(mut self, values: &[Value]) -> Self {
        self.enum_values = Some(values.to_vec());
        self
    }

    /// Set anyOf schemas
    #[must_use]
    pub fn any_of(mut self, schemas: &[SchemaBuilder]) -> Self {
        self.any_of = Some(schemas.iter().map(|s| s.clone().build().schema).collect());
        self
    }

    /// Set anyOf schemas with raw JSON values
    #[must_use]
    pub fn any_of_values(mut self, schemas: &[Value]) -> Self {
        self.any_of = Some(schemas.to_vec());
        self
    }

    /// Set string pattern validation
    #[must_use]
    pub fn pattern(mut self, pattern: &str) -> Self {
        self.pattern = Some(pattern.to_string());
        self
    }

    /// Set string format validation
    #[must_use]
    pub fn format(mut self, format: &str) -> Self {
        self.format = Some(format.to_string());
        self
    }

    /// Set minimum value for numbers
    #[must_use]
    pub fn minimum(mut self, min: f64) -> Self {
        self.minimum = Some(min);
        self
    }

    /// Set maximum value for numbers
    #[must_use]
    pub fn maximum(mut self, max: f64) -> Self {
        self.maximum = Some(max);
        self
    }

    /// Set minimum string length
    #[must_use]
    pub fn min_length(mut self, min: usize) -> Self {
        self.min_length = Some(min);
        self
    }

    /// Set maximum string length
    #[must_use]
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Set minimum array items
    #[must_use]
    pub fn min_items(mut self, min: usize) -> Self {
        self.min_items = Some(min);
        self
    }

    /// Set maximum array items
    #[must_use]
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = Some(max);
        self
    }

    /// Set whether additional properties are allowed
    #[must_use]
    pub fn additional_properties(mut self, allowed: bool) -> Self {
        self.additional_properties = Some(allowed);
        self
    }

    /// Set schema description
    #[must_use]
    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Set schema title
    #[must_use]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set default value
    #[must_use]
    pub fn default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }

    /// Add a schema definition for references
    #[must_use]
    pub fn definition(mut self, name: &str, schema: SchemaBuilder) -> Self {
        if self.definitions.is_none() {
            self.definitions = Some(IndexMap::new());
        }

        if let Some(ref mut defs) = self.definitions {
            defs.insert(name.to_string(), schema.build().schema);
        }
        self
    }

    /// Create a reference to a schema definition
    #[must_use]
    pub fn reference(name: &str) -> Self {
        let mut schema = IndexMap::new();
        schema.insert("$ref".to_string(), json!(format!("#/definitions/{}", name)));

        Self {
            schema_type: None,
            properties: None,
            items: None,
            required: None,
            enum_values: None,
            any_of: None,
            pattern: None,
            format: None,
            minimum: None,
            maximum: None,
            min_length: None,
            max_length: None,
            min_items: None,
            max_items: None,
            additional_properties: None,
            description: None,
            title: None,
            default: None,
            definitions: None,
        }
    }

    /// Build the final JSON schema
    #[must_use]
    pub fn build(self) -> JsonSchema {
        let mut schema = IndexMap::new();

        self.add_basic_properties(&mut schema);
        self.add_object_properties(&mut schema);
        self.add_array_properties(&mut schema);
        self.add_validation_properties(&mut schema);
        self.add_string_properties(&mut schema);
        self.add_numeric_properties(&mut schema);
        self.add_metadata_properties(&mut schema);

        JsonSchema::new(json!(schema))
    }

    /// Add basic properties to schema
    fn add_basic_properties(&self, schema: &mut IndexMap<String, Value>) {
        if let Some(ref schema_type) = self.schema_type {
            schema.insert("type".to_string(), json!(schema_type));
        }

        if let Some(ref enum_values) = self.enum_values {
            schema.insert("enum".to_string(), json!(enum_values));
        }

        if let Some(ref any_of) = self.any_of {
            schema.insert("anyOf".to_string(), json!(any_of));
        }

        if let Some(ref default) = self.default {
            schema.insert("default".to_string(), default.clone());
        }
    }

    /// Add object properties to schema
    fn add_object_properties(&self, schema: &mut IndexMap<String, Value>) {
        if let Some(ref properties) = self.properties {
            let props_map: HashMap<String, Value> = properties.clone().into_iter().collect();
            schema.insert("properties".to_string(), json!(props_map));
        }

        if let Some(ref required) = self.required {
            schema.insert("required".to_string(), json!(required));
        }

        if let Some(additional_properties) = self.additional_properties {
            schema.insert(
                "additionalProperties".to_string(),
                json!(additional_properties),
            );
        }

        if let Some(ref definitions) = self.definitions {
            let defs_map: HashMap<String, Value> = definitions.clone().into_iter().collect();
            schema.insert("definitions".to_string(), json!(defs_map));
        }
    }

    /// Add array properties to schema
    fn add_array_properties(&self, schema: &mut IndexMap<String, Value>) {
        if let Some(ref items) = self.items {
            schema.insert("items".to_string(), items.as_ref().clone());
        }

        if let Some(min_items) = self.min_items {
            schema.insert("minItems".to_string(), json!(min_items));
        }

        if let Some(max_items) = self.max_items {
            schema.insert("maxItems".to_string(), json!(max_items));
        }
    }

    /// Add validation properties to schema
    fn add_validation_properties(&self, schema: &mut IndexMap<String, Value>) {
        if let Some(ref pattern) = self.pattern {
            schema.insert("pattern".to_string(), json!(pattern));
        }

        if let Some(ref format) = self.format {
            schema.insert("format".to_string(), json!(format));
        }
    }

    /// Add string properties to schema
    fn add_string_properties(&self, schema: &mut IndexMap<String, Value>) {
        if let Some(min_length) = self.min_length {
            schema.insert("minLength".to_string(), json!(min_length));
        }

        if let Some(max_length) = self.max_length {
            schema.insert("maxLength".to_string(), json!(max_length));
        }
    }

    /// Add numeric properties to schema
    fn add_numeric_properties(&self, schema: &mut IndexMap<String, Value>) {
        if let Some(minimum) = self.minimum {
            schema.insert("minimum".to_string(), json!(minimum));
        }

        if let Some(maximum) = self.maximum {
            schema.insert("maximum".to_string(), json!(maximum));
        }
    }

    /// Add metadata properties to schema
    fn add_metadata_properties(&self, schema: &mut IndexMap<String, Value>) {
        if let Some(ref description) = self.description {
            schema.insert("description".to_string(), json!(description));
        }

        if let Some(ref title) = self.title {
            schema.insert("title".to_string(), json!(title));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_object_schema() {
        let schema = SchemaBuilder::object()
            .property("name", SchemaBuilder::string())
            .property("age", SchemaBuilder::integer().minimum(0.0))
            .required(&["name", "age"])
            .build();

        let valid_data = json!({
            "name": "John",
            "age": 30
        });

        assert!(schema.validate(&valid_data).is_ok());
    }

    #[test]
    fn test_array_schema() {
        let schema = SchemaBuilder::array()
            .items(SchemaBuilder::string())
            .min_items(1)
            .max_items(10)
            .build();

        let valid_data = json!(["hello", "world"]);
        assert!(schema.validate(&valid_data).is_ok());
    }

    #[test]
    fn test_enum_schema() {
        let schema = SchemaBuilder::string()
            .enum_values(&[json!("red"), json!("green"), json!("blue")])
            .build();

        let valid_data = json!("red");
        assert!(schema.validate(&valid_data).is_ok());

        let invalid_data = json!("yellow");
        assert!(schema.validate(&invalid_data).is_err());
    }
}
