//! # OpenAI Assistants API Models
//!
//! This module provides data structures for OpenAI's Assistants API, which allows you to
//! build AI assistants within your applications that can call models and tools.
//!
//! ## Overview
//!
//! The Assistants API supports:
//! - **Custom Instructions**: Provide specific instructions for how the assistant should behave
//! - **Tools**: Code Interpreter, Retrieval, and Function calling capabilities
//! - **File Management**: Attach files for retrieval and code interpreter usage
//! - **Model Selection**: Choose which OpenAI model powers your assistant
//! - **Metadata**: Store custom metadata for tracking and organization
//!
//! ## Assistant Tools
//!
//! Assistants can use three types of tools:
//! - `CodeInterpreter`: Execute Python code in a sandboxed environment
//! - `Retrieval`: Search and retrieve information from uploaded files
//! - `Function`: Call custom functions you define
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::assistants::{AssistantRequest, AssistantTool};
//! use std::collections::HashMap;
//!
//! // Create a code interpreter assistant
//! let assistant_request = AssistantRequest::builder()
//!     .name("Data Analyst")
//!     .description("Analyzes data and creates visualizations")
//!     .model("gpt-4")
//!     .instructions("You are a data analyst. Help users analyze data and create visualizations.")
//!     .tool(AssistantTool::CodeInterpreter)
//!     .build();
//! ```

use crate::api::base::Validate;
use crate::models::functions::FunctionTool;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// Tools that can be used by an assistant
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AssistantTool {
    /// Code interpreter tool for executing Python code
    CodeInterpreter,
    /// Retrieval tool for searching through uploaded files
    Retrieval,
    /// Function calling tool for custom functions
    Function {
        /// The function definition
        function: FunctionTool,
    },
}

impl AssistantTool {
    /// Create a new code interpreter tool
    #[must_use]
    pub fn code_interpreter() -> Self {
        Self::CodeInterpreter
    }

    /// Create a new retrieval tool
    #[must_use]
    pub fn retrieval() -> Self {
        Self::Retrieval
    }

    /// Create a new function tool
    #[must_use]
    pub fn function(function: FunctionTool) -> Self {
        Self::Function { function }
    }

    /// Get the tool type as a string
    #[must_use]
    pub fn tool_type(&self) -> &'static str {
        match self {
            Self::CodeInterpreter => "code_interpreter",
            Self::Retrieval => "retrieval",
            Self::Function { .. } => "function",
        }
    }
}

/// An assistant represents an entity that can be configured to respond to users' messages
/// using various settings and tools
#[derive(Debug, Clone, Ser, De)]
pub struct Assistant {
    /// The identifier of the assistant
    pub id: String,
    /// The object type, which is always "assistant"
    #[serde(default = "default_object_type")]
    pub object: String,
    /// The Unix timestamp (in seconds) when the assistant was created
    pub created_at: i64,
    /// The name of the assistant (max 256 characters)
    pub name: Option<String>,
    /// The description of the assistant (max 512 characters)
    pub description: Option<String>,
    /// The model used by the assistant
    pub model: String,
    /// The system instructions that the assistant uses
    pub instructions: Option<String>,
    /// A list of tools enabled on the assistant
    #[serde(default)]
    pub tools: Vec<AssistantTool>,
    /// A list of file IDs attached to this assistant
    #[serde(default)]
    pub file_ids: Vec<String>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

crate::impl_default_object_type!(default_object_type, "assistant");

/// Request to create or modify an assistant
#[derive(Debug, Clone, Ser, De)]
pub struct AssistantRequest {
    /// The model used by the assistant
    pub model: String,
    /// The name of the assistant (max 256 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the assistant (max 512 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The system instructions that the assistant uses (max 32768 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// A list of tools enabled on the assistant
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<AssistantTool>,
    /// A list of file IDs attached to this assistant (max 20 files)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_ids: Vec<String>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

impl AssistantRequest {
    /// Create a new assistant request builder
    #[must_use]
    pub fn builder() -> AssistantRequestBuilder {
        AssistantRequestBuilder::new()
    }

    /// Create a new assistant request with just a model
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            name: None,
            description: None,
            instructions: None,
            tools: Vec::new(),
            file_ids: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Validate the assistant request
    pub fn validate(&self) -> Result<(), String> {
        self.validate_text_fields()?;
        self.validate_collections()?;
        self.validate_metadata()?;
        Ok(())
    }

    /// Validate text field lengths
    fn validate_text_fields(&self) -> Result<(), String> {
        self.validate_name()?;
        self.validate_description()?;
        self.validate_instructions()?;
        Ok(())
    }

    /// Validate name length
    fn validate_name(&self) -> Result<(), String> {
        if let Some(name) = &self.name {
            if name.len() > 256 {
                return Err("Assistant name cannot exceed 256 characters".to_string());
            }
        }
        Ok(())
    }

    /// Validate description length
    fn validate_description(&self) -> Result<(), String> {
        if let Some(description) = &self.description {
            if description.len() > 512 {
                return Err("Assistant description cannot exceed 512 characters".to_string());
            }
        }
        Ok(())
    }

    /// Validate instructions length
    fn validate_instructions(&self) -> Result<(), String> {
        if let Some(instructions) = &self.instructions {
            if instructions.len() > 32768 {
                return Err("Assistant instructions cannot exceed 32768 characters".to_string());
            }
        }
        Ok(())
    }

    /// Validate collection sizes
    fn validate_collections(&self) -> Result<(), String> {
        self.validate_tools_count()?;
        self.validate_file_ids_count()?;
        Ok(())
    }

    /// Validate tools count
    fn validate_tools_count(&self) -> Result<(), String> {
        if self.tools.len() > 128 {
            return Err("Assistant cannot have more than 128 tools".to_string());
        }
        Ok(())
    }

    /// Validate file IDs count
    fn validate_file_ids_count(&self) -> Result<(), String> {
        if self.file_ids.len() > 20 {
            return Err("Assistant cannot have more than 20 file IDs".to_string());
        }
        Ok(())
    }

    /// Validate metadata
    fn validate_metadata(&self) -> Result<(), String> {
        self.validate_metadata_count()?;
        self.validate_metadata_entries()?;
        Ok(())
    }

    /// Validate metadata count
    fn validate_metadata_count(&self) -> Result<(), String> {
        if self.metadata.len() > 16 {
            return Err("Assistant cannot have more than 16 metadata pairs".to_string());
        }
        Ok(())
    }

    /// Validate metadata key/value lengths
    fn validate_metadata_entries(&self) -> Result<(), String> {
        for (key, value) in &self.metadata {
            if key.len() > 64 {
                return Err("Metadata key cannot exceed 64 characters".to_string());
            }
            if value.len() > 512 {
                return Err("Metadata value cannot exceed 512 characters".to_string());
            }
        }
        Ok(())
    }
}

/// Implementation of Validate trait for AssistantRequest
impl Validate for AssistantRequest {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

/// Builder for creating assistant requests
#[derive(Debug, Clone, Default)]
pub struct AssistantRequestBuilder {
    /// The model ID to use
    model: Option<String>,
    /// The name of the assistant
    name: Option<String>,
    /// The description of the assistant
    description: Option<String>,
    /// Instructions that the assistant uses
    instructions: Option<String>,
    /// A list of tools enabled on the assistant
    tools: Vec<AssistantTool>,
    /// A list of file IDs attached to this assistant
    file_ids: Vec<String>,
    /// Metadata for the assistant
    metadata: HashMap<String, String>,
}

impl AssistantRequestBuilder {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the model
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set the name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the instructions
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Add a tool
    #[must_use]
    pub fn tool(mut self, tool: AssistantTool) -> Self {
        self.tools.push(tool);
        self
    }

    /// Add multiple tools
    #[must_use]
    pub fn tools(mut self, tools: Vec<AssistantTool>) -> Self {
        self.tools.extend(tools);
        self
    }

    /// Add a file ID
    pub fn file_id(mut self, file_id: impl Into<String>) -> Self {
        self.file_ids.push(file_id.into());
        self
    }

    /// Add multiple file IDs
    #[must_use]
    pub fn file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids.extend(file_ids);
        self
    }

    /// Add metadata
    pub fn metadata_pair(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Set all metadata
    #[must_use]
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }
}

// Generate the build method for AssistantRequestBuilder
crate::impl_builder_build! {
    AssistantRequestBuilder => AssistantRequest {
        required: [model: "Model is required"],
        optional: [name, description, instructions, tools, file_ids, metadata],
        validate: true
    }
}

/// Response from listing assistants
#[derive(Debug, Clone, Ser, De)]
pub struct ListAssistantsResponse {
    /// The object type, which is always "list"
    #[serde(default = "default_list_object")]
    pub object: String,
    /// List of assistant objects
    pub data: Vec<Assistant>,
    /// ID of the first item in the list
    pub first_id: Option<String>,
    /// ID of the last item in the list
    pub last_id: Option<String>,
    /// Whether there are more items available
    pub has_more: bool,
}

crate::impl_default_object_type!(default_list_object, "list");

/// Parameters for listing assistants
#[derive(Debug, Clone, Default, Ser, De)]
pub struct ListAssistantsParams {
    /// Number of assistants to retrieve (1-100, default: 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order for the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SortOrder>,
    /// Cursor for pagination (assistant ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Cursor for reverse pagination (assistant ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl ListAssistantsParams {
    /// Create new list parameters
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit.clamp(1, 100));
        self
    }

    /// Set the sort order
    #[must_use]
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }

    /// Set the after cursor for pagination
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the before cursor for reverse pagination
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Build query parameters for the API request
    #[must_use]
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(order) = &self.order {
            let order_str = match order {
                SortOrder::Asc => "asc",
                SortOrder::Desc => "desc",
            };
            params.push(("order".to_string(), order_str.to_string()));
        }
        if let Some(after) = &self.after {
            params.push(("after".to_string(), after.clone()));
        }
        if let Some(before) = &self.before {
            params.push(("before".to_string(), before.clone()));
        }
        params
    }
}

impl crate::api::common::ListQueryParams for ListAssistantsParams {
    fn limit(&self) -> Option<u32> {
        self.limit
    }

    fn order_str(&self) -> Option<&str> {
        self.order.as_ref().map(|o| match o {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        })
    }

    fn after(&self) -> Option<&String> {
        self.after.as_ref()
    }

    fn before(&self) -> Option<&String> {
        self.before.as_ref()
    }
}

/// Sort order for listing results
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum SortOrder {
    /// Ascending order (oldest first)
    Asc,
    /// Descending order (newest first)
    #[default]
    Desc,
}

/// Response from deleting an assistant
#[derive(Debug, Clone, Ser, De)]
pub struct DeletionStatus {
    /// The ID of the deleted assistant
    pub id: String,
    /// The object type, which is always "assistant.deleted"
    #[serde(default = "default_deletion_object")]
    pub object: String,
    /// Whether the deletion was successful
    pub deleted: bool,
}

crate::impl_default_object_type!(default_deletion_object, "assistant.deleted");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assistant_tool_creation() {
        let code_interpreter = AssistantTool::code_interpreter();
        assert_eq!(code_interpreter.tool_type(), "code_interpreter");

        let retrieval = AssistantTool::retrieval();
        assert_eq!(retrieval.tool_type(), "retrieval");
    }

    #[test]
    fn test_assistant_request_builder() {
        let request = AssistantRequest::builder()
            .model("gpt-4")
            .name("Test Assistant")
            .description("A test assistant")
            .instructions("You are a helpful assistant")
            .tool(AssistantTool::code_interpreter())
            .file_id("file-123")
            .metadata_pair("purpose", "testing")
            .build()
            .unwrap();

        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.name, Some("Test Assistant".to_string()));
        assert_eq!(request.tools.len(), 1);
        assert_eq!(request.file_ids.len(), 1);
        assert_eq!(request.metadata.len(), 1);
    }

    #[test]
    fn test_assistant_request_validation() {
        // Test name length validation
        let long_name = "a".repeat(257);
        let request = AssistantRequest::builder()
            .model("gpt-4")
            .name(long_name)
            .build();
        assert!(request.is_err());

        // Test valid request
        let request = AssistantRequest::builder()
            .model("gpt-4")
            .name("Valid Name")
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_list_params_limit_clamping() {
        let params = ListAssistantsParams::new().limit(150);
        assert_eq!(params.limit, Some(100));

        let params = ListAssistantsParams::new().limit(0);
        assert_eq!(params.limit, Some(1));
    }

    #[test]
    fn test_sort_order_default() {
        let order = SortOrder::default();
        assert_eq!(order, SortOrder::Desc);
    }
}
