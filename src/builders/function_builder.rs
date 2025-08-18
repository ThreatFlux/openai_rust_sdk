use crate::error::{OpenAIError, Result};
use crate::models::functions::{FunctionTool, Tool};
use crate::schema::builder::SchemaBuilder;

/// Builder for creating function definitions with fluent API
#[derive(Debug, Clone)]
pub struct FunctionBuilder {
    /// Function name
    name: Option<String>,
    /// Function description
    description: Option<String>,
    /// Schema builder for parameters
    schema_builder: SchemaBuilder,
    /// Whether to use strict mode
    strict: Option<bool>,
}

impl Default for FunctionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionBuilder {
    /// Create a new function builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            schema_builder: SchemaBuilder::new_object(),
            strict: None,
        }
    }

    /// Set the function name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the function description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Enable strict mode for this function
    #[must_use]
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = Some(strict);
        self
    }

    /// Add a required string parameter
    pub fn required_string(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .required_property(name, SchemaBuilder::string().description(description));
        self
    }

    /// Add an optional string parameter
    pub fn optional_string(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .optional_property(name, SchemaBuilder::string().description(description));
        self
    }

    /// Add a required number parameter
    pub fn required_number(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .required_property(name, SchemaBuilder::number().description(description));
        self
    }

    /// Add an optional number parameter
    pub fn optional_number(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .optional_property(name, SchemaBuilder::number().description(description));
        self
    }

    /// Add a required integer parameter
    pub fn required_integer(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .required_property(name, SchemaBuilder::integer().description(description));
        self
    }

    /// Add an optional integer parameter
    pub fn optional_integer(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .optional_property(name, SchemaBuilder::integer().description(description));
        self
    }

    /// Add a required boolean parameter
    pub fn required_boolean(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .required_property(name, SchemaBuilder::boolean().description(description));
        self
    }

    /// Add an optional boolean parameter
    pub fn optional_boolean(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .optional_property(name, SchemaBuilder::boolean().description(description));
        self
    }

    /// Add a required array parameter
    pub fn required_array(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        item_schema: SchemaBuilder,
    ) -> Self {
        self.schema_builder = self.schema_builder.required_property(
            name,
            SchemaBuilder::array(item_schema).description(description),
        );
        self
    }

    /// Add an optional array parameter
    pub fn optional_array(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        item_schema: SchemaBuilder,
    ) -> Self {
        self.schema_builder = self.schema_builder.optional_property(
            name,
            SchemaBuilder::array(item_schema).description(description),
        );
        self
    }

    /// Add a required object parameter
    pub fn required_object(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        object_schema: SchemaBuilder,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .required_property(name, object_schema.description(description));
        self
    }

    /// Add an optional object parameter
    pub fn optional_object(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        object_schema: SchemaBuilder,
    ) -> Self {
        self.schema_builder = self
            .schema_builder
            .optional_property(name, object_schema.description(description));
        self
    }

    /// Add a required enum parameter
    pub fn required_enum(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        values: Vec<String>,
    ) -> Self {
        self.schema_builder = self.schema_builder.required_property(
            name,
            SchemaBuilder::string_enum(values).description(description),
        );
        self
    }

    /// Add an optional enum parameter
    pub fn optional_enum(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        values: Vec<String>,
    ) -> Self {
        self.schema_builder = self.schema_builder.optional_property(
            name,
            SchemaBuilder::string_enum(values).description(description),
        );
        self
    }

    /// Add a parameter with custom schema
    pub fn required_parameter(mut self, name: impl Into<String>, schema: SchemaBuilder) -> Self {
        self.schema_builder = self.schema_builder.required_property(name, schema);
        self
    }

    /// Add an optional parameter with custom schema
    pub fn optional_parameter(mut self, name: impl Into<String>, schema: SchemaBuilder) -> Self {
        self.schema_builder = self.schema_builder.optional_property(name, schema);
        self
    }

    /// Build the function tool
    pub fn build(self) -> Result<FunctionTool> {
        let name = self
            .name
            .ok_or_else(|| OpenAIError::validation("Function name is required"))?;

        let description = self
            .description
            .ok_or_else(|| OpenAIError::validation("Function description is required"))?;

        let parameters = self.schema_builder.build()?;

        let mut function = FunctionTool::new(name, description, parameters);
        if let Some(strict) = self.strict {
            function = function.with_strict(strict);
        }

        Ok(function)
    }

    /// Build as a Tool
    pub fn build_tool(self) -> Result<Tool> {
        Ok(Tool::function(self.build()?))
    }
}

/// Convenience functions for creating common function patterns
impl FunctionBuilder {
    /// Create a function that takes a single location parameter
    pub fn location_function(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new()
            .name(name)
            .description(description)
            .required_string("location", "The location to query")
    }

    /// Create a function that takes location and optional unit
    pub fn weather_function(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new()
            .name(name)
            .description(description)
            .required_string("location", "The location to get weather for")
            .optional_enum(
                "unit",
                "Temperature unit",
                vec!["celsius".to_string(), "fahrenheit".to_string()],
            )
    }

    /// Create a function that searches with a query
    pub fn search_function(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new()
            .name(name)
            .description(description)
            .required_string("query", "The search query")
            .optional_integer("limit", "Maximum number of results")
    }

    /// Create a function that takes an ID parameter
    pub fn id_function(
        name: impl Into<String>,
        description: impl Into<String>,
        id_field: impl Into<String>,
    ) -> Self {
        Self::new()
            .name(name)
            .description(description)
            .required_string(id_field, "The ID to look up")
    }

    /// Create a function with flexible key-value parameters
    pub fn key_value_function(
        name: impl Into<String>,
        description: impl Into<String>,
        required_keys: Vec<(String, String)>, // (key, description) pairs
        optional_keys: Vec<(String, String)>,
    ) -> Self {
        let mut builder = Self::new().name(name).description(description);

        for (key, desc) in required_keys {
            builder = builder.required_string(key, desc);
        }

        for (key, desc) in optional_keys {
            builder = builder.optional_string(key, desc);
        }

        builder
    }
}

/// Builder for nested object schemas
#[derive(Debug, Clone)]
pub struct ObjectSchemaBuilder {
    /// Schema builder for the object
    schema_builder: SchemaBuilder,
}

impl ObjectSchemaBuilder {
    /// Create a new object schema builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            schema_builder: SchemaBuilder::new_object(),
        }
    }

    /// Add a required property
    pub fn required_property(mut self, name: impl Into<String>, schema: SchemaBuilder) -> Self {
        self.schema_builder = self.schema_builder.required_property(name, schema);
        self
    }

    /// Add an optional property
    pub fn optional_property(mut self, name: impl Into<String>, schema: SchemaBuilder) -> Self {
        self.schema_builder = self.schema_builder.optional_property(name, schema);
        self
    }

    /// Build the schema
    #[must_use]
    pub fn build(self) -> SchemaBuilder {
        self.schema_builder
    }
}

impl Default for ObjectSchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_function_builder() {
        let func = FunctionBuilder::new()
            .name("test_function")
            .description("A test function")
            .required_string("param1", "First parameter")
            .optional_number("param2", "Second parameter")
            .build()
            .unwrap();

        assert_eq!(func.name, "test_function");
        assert_eq!(func.description, "A test function");
        assert!(func.strict.is_none());
    }

    #[test]
    fn test_function_builder_with_strict() {
        let func = FunctionBuilder::new()
            .name("strict_function")
            .description("A strict function")
            .strict(true)
            .build()
            .unwrap();

        assert_eq!(func.strict, Some(true));
    }

    #[test]
    fn test_weather_function_pattern() {
        let func = FunctionBuilder::weather_function("get_weather", "Get weather information")
            .build()
            .unwrap();

        assert_eq!(func.name, "get_weather");
        assert_eq!(func.description, "Get weather information");

        // Check that the schema includes location and unit parameters
        let schema = func.parameters;
        assert!(schema.get("properties").is_some());
    }

    #[test]
    fn test_search_function_pattern() {
        let func = FunctionBuilder::search_function("search_docs", "Search documentation")
            .build()
            .unwrap();

        assert_eq!(func.name, "search_docs");
        assert_eq!(func.description, "Search documentation");
    }

    #[test]
    fn test_key_value_function() {
        let func = FunctionBuilder::key_value_function(
            "update_user",
            "Update user information",
            vec![("user_id".to_string(), "User ID".to_string())],
            vec![("name".to_string(), "User name".to_string())],
        )
        .build()
        .unwrap();

        assert_eq!(func.name, "update_user");
        assert_eq!(func.description, "Update user information");
    }

    #[test]
    fn test_enum_parameter() {
        let func = FunctionBuilder::new()
            .name("test_enum")
            .description("Test enum parameter")
            .required_enum(
                "status",
                "Status value",
                vec![
                    "active".to_string(),
                    "inactive".to_string(),
                    "pending".to_string(),
                ],
            )
            .build()
            .unwrap();

        assert_eq!(func.name, "test_enum");
    }

    #[test]
    fn test_nested_object() {
        let address_schema = ObjectSchemaBuilder::new()
            .required_property("street", SchemaBuilder::string())
            .required_property("city", SchemaBuilder::string())
            .optional_property("zip", SchemaBuilder::string())
            .build();

        let func = FunctionBuilder::new()
            .name("update_address")
            .description("Update user address")
            .required_string("user_id", "User ID")
            .required_object("address", "User address", address_schema)
            .build()
            .unwrap();

        assert_eq!(func.name, "update_address");
    }

    #[test]
    fn test_array_parameter() {
        let func = FunctionBuilder::new()
            .name("process_items")
            .description("Process a list of items")
            .required_array("items", "List of items to process", SchemaBuilder::string())
            .build()
            .unwrap();

        assert_eq!(func.name, "process_items");
    }

    #[test]
    fn test_missing_name_validation() {
        let result = FunctionBuilder::new().description("Missing name").build();

        assert!(result.is_err());
    }

    #[test]
    fn test_missing_description_validation() {
        let result = FunctionBuilder::new().name("test").build();

        assert!(result.is_err());
    }

    #[test]
    fn test_build_as_tool() {
        let tool = FunctionBuilder::new()
            .name("test_tool")
            .description("A test tool")
            .build_tool()
            .unwrap();

        assert_eq!(tool.name(), "test_tool");
        assert_eq!(tool.description(), "A test tool");
    }
}
