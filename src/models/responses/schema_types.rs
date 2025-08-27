use crate::schema::JsonSchema;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// Response format enforcement options
#[derive(Debug, Clone, Ser, De, PartialEq)]
#[serde(tag = "type")]
#[derive(Default)]
pub enum ResponseFormat {
    /// Standard text response
    #[serde(rename = "text")]
    #[default]
    Text,
    /// JSON object response (legacy JSON mode)
    #[serde(rename = "json_object")]
    JsonObject,
    /// JSON schema-enforced response
    #[serde(rename = "json_schema")]
    JsonSchema {
        /// The JSON schema definition
        json_schema: JsonSchemaSpec,
        /// Whether to enable strict mode
        #[serde(default)]
        strict: bool,
    },
}

/// JSON Schema specification for structured outputs
#[derive(Debug, Clone, Ser, De, PartialEq)]
pub struct JsonSchemaSpec {
    /// Name of the schema
    pub name: String,
    /// Description of the schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The JSON schema definition
    pub schema: serde_json::Value,
    /// Whether the schema is strict (additional properties not allowed)
    #[serde(default)]
    pub strict: bool,
}

/// Schema validation result
#[derive(Debug, Clone, Ser, De)]
pub struct SchemaValidationResult {
    /// Whether the data is valid according to the schema
    pub is_valid: bool,
    /// Validation errors if any
    pub errors: Vec<String>,
    /// The validated data
    pub data: Option<serde_json::Value>,
}

impl ResponseFormat {
    /// Create a JSON object response format
    #[must_use]
    pub fn json_object() -> Self {
        ResponseFormat::JsonObject
    }

    /// Create a JSON schema response format
    pub fn json_schema(name: impl Into<String>, schema: serde_json::Value) -> Self {
        ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema,
                strict: false,
            },
            strict: false,
        }
    }

    /// Create a strict JSON schema response format
    pub fn strict_json_schema(name: impl Into<String>, schema: serde_json::Value) -> Self {
        ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema,
                strict: true,
            },
            strict: true,
        }
    }

    /// Create from schema builder
    pub fn from_schema_builder(
        name: impl Into<String>,
        builder: crate::schema::SchemaBuilder,
    ) -> Self {
        let schema = builder.build();
        ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema: schema.to_value(),
                strict: false,
            },
            strict: false,
        }
    }

    /// Create strict format from schema builder
    pub fn strict_from_schema_builder(
        name: impl Into<String>,
        builder: crate::schema::SchemaBuilder,
    ) -> Self {
        let schema = builder.build();
        ResponseFormat::JsonSchema {
            json_schema: JsonSchemaSpec {
                name: name.into(),
                description: None,
                schema: schema.to_value(),
                strict: true,
            },
            strict: true,
        }
    }

    /// Check if this is a structured output format
    #[must_use]
    pub fn is_structured(&self) -> bool {
        matches!(
            self,
            ResponseFormat::JsonObject | ResponseFormat::JsonSchema { .. }
        )
    }

    /// Check if this format requires schema validation
    #[must_use]
    pub fn requires_schema_validation(&self) -> bool {
        matches!(self, ResponseFormat::JsonSchema { .. })
    }

    /// Get the schema if available
    #[must_use]
    pub fn schema(&self) -> Option<&JsonSchemaSpec> {
        match self {
            ResponseFormat::JsonSchema { json_schema, .. } => Some(json_schema),
            _ => None,
        }
    }
}

impl JsonSchemaSpec {
    /// Create a new JSON schema specification
    pub fn new(name: impl Into<String>, schema: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            description: None,
            schema,
            strict: false,
        }
    }

    /// Create a strict JSON schema specification
    pub fn strict(name: impl Into<String>, schema: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            description: None,
            schema,
            strict: true,
        }
    }

    /// Create with description
    pub fn with_description(
        name: impl Into<String>,
        description: impl Into<String>,
        schema: serde_json::Value,
    ) -> Self {
        Self {
            name: name.into(),
            description: Some(description.into()),
            schema,
            strict: false,
        }
    }

    /// Set description
    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set strict mode
    #[must_use]
    pub fn set_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Validate data against this schema
    #[must_use]
    pub fn validate(&self, data: &serde_json::Value) -> SchemaValidationResult {
        let json_schema = JsonSchema::new(self.schema.clone());
        match json_schema.validate(data) {
            Ok(()) => SchemaValidationResult {
                is_valid: true,
                errors: vec![],
                data: Some(data.clone()),
            },
            Err(e) => SchemaValidationResult {
                is_valid: false,
                errors: vec![e.to_string()],
                data: Some(data.clone()),
            },
        }
    }

    /// Convert to `JsonSchema` for validation
    #[must_use]
    pub fn to_json_schema(&self) -> JsonSchema {
        JsonSchema::new(self.schema.clone())
    }
}

/// Schema generation utilities
pub struct SchemaUtils;

impl SchemaUtils {
    /// Generate a basic object schema with properties
    #[must_use]
    pub fn object_schema(properties: &[(&str, &str)]) -> serde_json::Value {
        let mut props = serde_json::Map::new();
        let mut required = Vec::new();

        for (name, type_name) in properties {
            props.insert(
                (*name).to_string(),
                serde_json::json!({ "type": type_name }),
            );
            required.push((*name).to_string());
        }

        serde_json::json!({
            "type": "object",
            "properties": props,
            "required": required,
            "additionalProperties": false
        })
    }

    /// Generate an array schema
    #[must_use]
    pub fn array_schema(item_type: &str) -> serde_json::Value {
        serde_json::json!({
            "type": "array",
            "items": { "type": item_type }
        })
    }

    /// Generate an enum schema
    #[must_use]
    pub fn enum_schema(values: &[&str]) -> serde_json::Value {
        serde_json::json!({
            "type": "string",
            "enum": values
        })
    }

    /// Generate a union schema (anyOf)
    #[must_use]
    pub fn union_schema(schemas: &[serde_json::Value]) -> serde_json::Value {
        serde_json::json!({
            "anyOf": schemas
        })
    }
}

/// Helper trait for creating schema-validated structured outputs
pub trait StructuredOutput: serde::Serialize + serde::de::DeserializeOwned {
    /// Get the JSON schema for this type
    fn json_schema() -> serde_json::Value;

    /// Get the schema name
    fn schema_name() -> &'static str;

    /// Create a response format for this type
    #[must_use]
    fn response_format() -> ResponseFormat {
        ResponseFormat::json_schema(Self::schema_name(), Self::json_schema())
    }

    /// Create a strict response format for this type
    #[must_use]
    fn strict_response_format() -> ResponseFormat {
        ResponseFormat::strict_json_schema(Self::schema_name(), Self::json_schema())
    }

    /// Validate and parse from JSON value
    fn from_json_value(value: &serde_json::Value) -> Result<Self, String> {
        let schema = JsonSchema::new(Self::json_schema());
        schema.validate(value).map_err(|e| e.to_string())?;
        serde_json::from_value(value.clone()).map_err(|e| e.to_string())
    }
}
