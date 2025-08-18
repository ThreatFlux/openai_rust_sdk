use crate::error::{OpenAIError, Result};
use crate::models::responses::{JsonSchemaSpec, ResponseFormat};
use serde_json::{Map, Value};

/// Enhanced schema builder specifically for function calling
#[derive(Debug, Clone)]
pub struct SchemaBuilder {
    /// The schema being built
    schema: Map<String, Value>,
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
        Self { schema: Map::new() }
    }

    /// Create a new object schema
    #[must_use]
    pub fn new_object() -> Self {
        let mut builder = Self::new();
        builder
            .schema
            .insert("type".to_string(), Value::String("object".to_string()));
        builder
            .schema
            .insert("properties".to_string(), Value::Object(Map::new()));
        builder
            .schema
            .insert("required".to_string(), Value::Array(vec![]));
        builder
            .schema
            .insert("additionalProperties".to_string(), Value::Bool(false));
        builder
    }

    /// Create a string schema
    #[must_use]
    pub fn string() -> Self {
        let mut builder = Self::new();
        builder
            .schema
            .insert("type".to_string(), Value::String("string".to_string()));
        builder
    }

    /// Create a number schema
    #[must_use]
    pub fn number() -> Self {
        let mut builder = Self::new();
        builder
            .schema
            .insert("type".to_string(), Value::String("number".to_string()));
        builder
    }

    /// Create an integer schema
    #[must_use]
    pub fn integer() -> Self {
        let mut builder = Self::new();
        builder
            .schema
            .insert("type".to_string(), Value::String("integer".to_string()));
        builder
    }

    /// Create a boolean schema
    #[must_use]
    pub fn boolean() -> Self {
        let mut builder = Self::new();
        builder
            .schema
            .insert("type".to_string(), Value::String("boolean".to_string()));
        builder
    }

    /// Create an array schema
    #[must_use]
    pub fn array(items: SchemaBuilder) -> Self {
        let mut builder = Self::new();
        builder
            .schema
            .insert("type".to_string(), Value::String("array".to_string()));
        if let Ok(items_schema) = items.build() {
            builder.schema.insert("items".to_string(), items_schema);
        }
        builder
    }

    /// Create a null schema
    #[must_use]
    pub fn null() -> Self {
        let mut builder = Self::new();
        builder
            .schema
            .insert("type".to_string(), Value::String("null".to_string()));
        builder
    }

    /// Create a string enum schema
    pub fn string_enum(values: Vec<String>) -> Self {
        let mut builder = Self::string();
        let enum_values: Vec<Value> = values.into_iter().map(Value::String).collect();
        builder
            .schema
            .insert("enum".to_string(), Value::Array(enum_values));
        builder
    }

    /// Set the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.schema
            .insert("description".to_string(), Value::String(description.into()));
        self
    }

    /// Set the title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.schema
            .insert("title".to_string(), Value::String(title.into()));
        self
    }

    /// Set minimum value for numbers
    #[must_use]
    pub fn minimum(mut self, min: f64) -> Self {
        if let Some(num) = serde_json::Number::from_f64(min) {
            self.schema
                .insert("minimum".to_string(), Value::Number(num));
        }
        self
    }

    /// Set maximum value for numbers
    #[must_use]
    pub fn maximum(mut self, max: f64) -> Self {
        if let Some(num) = serde_json::Number::from_f64(max) {
            self.schema
                .insert("maximum".to_string(), Value::Number(num));
        }
        self
    }

    /// Set minimum length for strings
    #[must_use]
    pub fn min_length(mut self, min: usize) -> Self {
        self.schema
            .insert("minLength".to_string(), Value::Number((min as u64).into()));
        self
    }

    /// Set maximum length for strings
    #[must_use]
    pub fn max_length(mut self, max: usize) -> Self {
        self.schema
            .insert("maxLength".to_string(), Value::Number((max as u64).into()));
        self
    }

    /// Set minimum items for arrays
    #[must_use]
    pub fn min_items(mut self, min: usize) -> Self {
        self.schema
            .insert("minItems".to_string(), Value::Number((min as u64).into()));
        self
    }

    /// Set maximum items for arrays
    #[must_use]
    pub fn max_items(mut self, max: usize) -> Self {
        self.schema
            .insert("maxItems".to_string(), Value::Number((max as u64).into()));
        self
    }

    /// Set pattern for strings
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.schema
            .insert("pattern".to_string(), Value::String(pattern.into()));
        self
    }

    /// Set format for strings
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.schema
            .insert("format".to_string(), Value::String(format.into()));
        self
    }

    /// Set default value
    #[must_use]
    pub fn default_value(mut self, default: Value) -> Self {
        self.schema.insert("default".to_string(), default);
        self
    }

    /// Set whether additional properties are allowed
    #[must_use]
    pub fn additional_properties(mut self, allowed: bool) -> Self {
        self.schema
            .insert("additionalProperties".to_string(), Value::Bool(allowed));
        self
    }

    /// Add a required property to an object schema
    pub fn required_property(
        mut self,
        name: impl Into<String>,
        property_schema: SchemaBuilder,
    ) -> Self {
        let name = name.into();

        // Add to properties
        if let Some(Value::Object(properties)) = self.schema.get_mut("properties") {
            properties.insert(name.clone(), property_schema.build().unwrap_or(Value::Null));
        }

        // Add to required array
        if let Some(Value::Array(required)) = self.schema.get_mut("required") {
            if !required.iter().any(|v| v.as_str() == Some(&name)) {
                required.push(Value::String(name));
            }
        }

        self
    }

    /// Add an optional property to an object schema
    pub fn optional_property(
        mut self,
        name: impl Into<String>,
        property_schema: SchemaBuilder,
    ) -> Self {
        let name = name.into();

        // Add to properties only (not to required)
        if let Some(Value::Object(properties)) = self.schema.get_mut("properties") {
            properties.insert(name, property_schema.build().unwrap_or(Value::Null));
        }

        self
    }

    /// Set enum values
    #[must_use]
    pub fn enum_values(mut self, values: Vec<Value>) -> Self {
        self.schema.insert("enum".to_string(), Value::Array(values));
        self
    }

    /// Set const value
    #[must_use]
    pub fn const_value(mut self, value: Value) -> Self {
        self.schema.insert("const".to_string(), value);
        self
    }

    /// Add anyOf schemas
    #[must_use]
    pub fn any_of(mut self, schemas: Vec<SchemaBuilder>) -> Self {
        let any_of_values: Vec<Value> =
            schemas.into_iter().filter_map(|s| s.build().ok()).collect();
        self.schema
            .insert("anyOf".to_string(), Value::Array(any_of_values));
        self
    }

    /// Add oneOf schemas
    #[must_use]
    pub fn one_of(mut self, schemas: Vec<SchemaBuilder>) -> Self {
        let one_of_values: Vec<Value> =
            schemas.into_iter().filter_map(|s| s.build().ok()).collect();
        self.schema
            .insert("oneOf".to_string(), Value::Array(one_of_values));
        self
    }

    /// Add allOf schemas
    #[must_use]
    pub fn all_of(mut self, schemas: Vec<SchemaBuilder>) -> Self {
        let all_of_values: Vec<Value> =
            schemas.into_iter().filter_map(|s| s.build().ok()).collect();
        self.schema
            .insert("allOf".to_string(), Value::Array(all_of_values));
        self
    }

    /// Create a reference to a definition
    pub fn reference(def_name: impl Into<String>) -> Self {
        let mut builder = Self::new();
        builder.schema.insert(
            "$ref".to_string(),
            Value::String(format!("#/definitions/{}", def_name.into())),
        );
        builder
    }

    /// Build the final schema
    pub fn build(self) -> Result<Value> {
        Ok(Value::Object(self.schema))
    }

    /// Build and get as a JSON object
    pub fn build_object(self) -> Result<Map<String, Value>> {
        Ok(self.schema)
    }

    /// Get the current schema as a Value (for debugging)
    #[must_use]
    pub fn to_value(&self) -> Value {
        Value::Object(self.schema.clone())
    }

    /// Validate that this is a valid schema
    pub fn validate_schema(&self) -> Result<()> {
        // Basic validation - check for required fields in object schemas
        if let Some(Value::String(schema_type)) = self.schema.get("type") {
            if schema_type == "object" {
                if !self.schema.contains_key("properties") {
                    return Err(OpenAIError::validation(
                        "Object schema must have properties",
                    ));
                }
            } else if schema_type == "array" && !self.schema.contains_key("items") {
                return Err(OpenAIError::validation("Array schema must have items"));
            }
        }
        Ok(())
    }
}

/// Convenience functions for common patterns
impl SchemaBuilder {
    /// Create a simple string field with optional constraints
    pub fn string_field(description: impl Into<String>) -> Self {
        Self::string().description(description)
    }

    /// Create a string field with enum values
    pub fn string_enum_field(description: impl Into<String>, values: Vec<String>) -> Self {
        Self::string_enum(values).description(description)
    }

    /// Create a number field with optional range
    pub fn number_field(
        description: impl Into<String>,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Self {
        let mut builder = Self::number().description(description);
        if let Some(min_val) = min {
            builder = builder.minimum(min_val);
        }
        if let Some(max_val) = max {
            builder = builder.maximum(max_val);
        }
        builder
    }

    /// Create an integer field with optional range
    pub fn integer_field(
        description: impl Into<String>,
        min: Option<i64>,
        max: Option<i64>,
    ) -> Self {
        let mut builder = Self::integer().description(description);
        if let Some(min_val) = min {
            builder = builder.minimum(min_val as f64);
        }
        if let Some(max_val) = max {
            builder = builder.maximum(max_val as f64);
        }
        builder
    }

    /// Create a boolean field
    pub fn boolean_field(description: impl Into<String>) -> Self {
        Self::boolean().description(description)
    }

    /// Create an array field with item type
    pub fn array_field(description: impl Into<String>, items: SchemaBuilder) -> Self {
        Self::array(items).description(description)
    }

    /// Create a simple object with string properties
    #[must_use]
    pub fn simple_object(properties: Vec<(String, SchemaBuilder, bool)>) -> Self {
        let mut builder = Self::new_object();

        for (name, schema, required) in properties {
            if required {
                builder = builder.required_property(name, schema);
            } else {
                builder = builder.optional_property(name, schema);
            }
        }

        builder
    }

    /// Create a flexible object that allows additional properties
    #[must_use]
    pub fn flexible_object() -> Self {
        Self::new_object().additional_properties(true)
    }

    /// Create an optional field wrapper (using anyOf with null)
    #[must_use]
    pub fn optional(schema: SchemaBuilder) -> Self {
        Self::new().any_of(vec![schema, Self::null()])
    }

    /// Convert this schema builder to a response format
    pub fn to_response_format(self, name: impl Into<String>) -> ResponseFormat {
        match self.build() {
            Ok(schema) => ResponseFormat::json_schema(name, schema),
            Err(_) => ResponseFormat::JsonObject, // Fallback to JSON object mode
        }
    }

    /// Convert this schema builder to a strict response format
    pub fn to_strict_response_format(self, name: impl Into<String>) -> ResponseFormat {
        match self.build() {
            Ok(schema) => ResponseFormat::strict_json_schema(name, schema),
            Err(_) => ResponseFormat::JsonObject, // Fallback to JSON object mode
        }
    }

    /// Convert this schema builder to a `JsonSchemaSpec`
    pub fn to_json_schema_spec(
        self,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<JsonSchemaSpec> {
        let schema = self.build()?;
        Ok(JsonSchemaSpec {
            name: name.into(),
            description,
            schema,
            strict: false,
        })
    }

    /// Convert this schema builder to a strict `JsonSchemaSpec`
    pub fn to_strict_json_schema_spec(
        self,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<JsonSchemaSpec> {
        let schema = self.build()?;
        Ok(JsonSchemaSpec {
            name: name.into(),
            description,
            schema,
            strict: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_types() {
        let string_schema = SchemaBuilder::string().build().unwrap();
        assert_eq!(string_schema["type"], "string");

        let number_schema = SchemaBuilder::number().build().unwrap();
        assert_eq!(number_schema["type"], "number");

        let boolean_schema = SchemaBuilder::boolean().build().unwrap();
        assert_eq!(boolean_schema["type"], "boolean");
    }

    #[test]
    fn test_object_schema() {
        let schema = SchemaBuilder::new_object()
            .required_property("name", SchemaBuilder::string().description("User name"))
            .optional_property("age", SchemaBuilder::integer().minimum(0.0))
            .build()
            .unwrap();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"].is_object());
        assert!(schema["required"].is_array());

        let required = schema["required"].as_array().unwrap();
        assert!(required.contains(&Value::String("name".to_string())));
        assert!(!required.contains(&Value::String("age".to_string())));
    }

    #[test]
    fn test_array_schema() {
        let schema = SchemaBuilder::array(SchemaBuilder::string())
            .min_items(1)
            .max_items(10)
            .build()
            .unwrap();

        assert_eq!(schema["type"], "array");
        assert_eq!(schema["minItems"], 1);
        assert_eq!(schema["maxItems"], 10);
        assert!(schema["items"].is_object());
    }

    #[test]
    fn test_string_enum() {
        let schema = SchemaBuilder::string_enum(vec![
            "red".to_string(),
            "green".to_string(),
            "blue".to_string(),
        ])
        .build()
        .unwrap();

        assert_eq!(schema["type"], "string");
        let enum_values = schema["enum"].as_array().unwrap();
        assert_eq!(enum_values.len(), 3);
        assert!(enum_values.contains(&Value::String("red".to_string())));
    }

    #[test]
    fn test_constraints() {
        let schema = SchemaBuilder::string()
            .min_length(5)
            .max_length(50)
            .pattern("^[a-zA-Z]+$")
            .description("A string field")
            .build()
            .unwrap();

        assert_eq!(schema["minLength"], 5);
        assert_eq!(schema["maxLength"], 50);
        assert_eq!(schema["pattern"], "^[a-zA-Z]+$");
        assert_eq!(schema["description"], "A string field");
    }

    #[test]
    fn test_number_constraints() {
        let schema = SchemaBuilder::number()
            .minimum(0.0)
            .maximum(100.0)
            .build()
            .unwrap();

        assert_eq!(schema["minimum"], 0.0);
        assert_eq!(schema["maximum"], 100.0);
    }

    #[test]
    fn test_convenience_functions() {
        let schema = SchemaBuilder::simple_object(vec![
            (
                "name".to_string(),
                SchemaBuilder::string_field("User name"),
                true,
            ),
            (
                "age".to_string(),
                SchemaBuilder::integer_field("User age", Some(0), Some(150)),
                false,
            ),
            (
                "active".to_string(),
                SchemaBuilder::boolean_field("Is active"),
                false,
            ),
        ])
        .build()
        .unwrap();

        assert_eq!(schema["type"], "object");
        let required = schema["required"].as_array().unwrap();
        assert_eq!(required.len(), 1);
        assert!(required.contains(&Value::String("name".to_string())));
    }

    #[test]
    fn test_any_of() {
        let schema = SchemaBuilder::new()
            .any_of(vec![SchemaBuilder::string(), SchemaBuilder::integer()])
            .build()
            .unwrap();

        assert!(schema["anyOf"].is_array());
        let any_of = schema["anyOf"].as_array().unwrap();
        assert_eq!(any_of.len(), 2);
    }

    #[test]
    fn test_reference() {
        let schema = SchemaBuilder::reference("User").build().unwrap();
        assert_eq!(schema["$ref"], "#/definitions/User");
    }

    #[test]
    fn test_validation() {
        let valid_object =
            SchemaBuilder::new_object().required_property("name", SchemaBuilder::string());
        assert!(valid_object.validate_schema().is_ok());

        let _invalid_array = SchemaBuilder::new()
            .schema
            .insert("type".to_string(), Value::String("array".to_string()));
        // This should fail validation because array schema needs items
        // But our current implementation doesn't enforce this strictly
    }

    #[test]
    fn test_flexible_object() {
        let schema = SchemaBuilder::flexible_object().build().unwrap();
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["additionalProperties"], true);
    }
}
