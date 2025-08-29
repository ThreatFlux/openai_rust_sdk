use crate::error::{OpenAIError, Result};
use crate::models::functions::FunctionCall;
use serde_json::Value;

/// Helper struct for extracting tool call data with reduced complexity
#[derive(Debug)]
pub(crate) struct ToolCallExtractor<'a> {
    /// The tool call value to extract from
    call: &'a Value,
}

impl<'a> ToolCallExtractor<'a> {
    /// Create a new extractor for a tool call
    pub(crate) fn new(call: &'a Value) -> Self {
        Self { call }
    }

    /// Extract all required fields in a single operation
    pub(crate) fn extract_all(&self) -> Result<(&'a str, &'a str, &'a str)> {
        let id = self.extract_id()?;
        let function = self.extract_function()?;
        let name = self.extract_name(function)?;
        let arguments = self.extract_arguments(function)?;
        Ok((id, name, arguments))
    }

    /// Extract tool call ID
    fn extract_id(&self) -> Result<&'a str> {
        self.call
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenAIError::parsing("Tool call missing id"))
    }

    /// Extract function object
    fn extract_function(&self) -> Result<&'a Value> {
        self.call
            .get("function")
            .ok_or_else(|| OpenAIError::parsing("Tool call missing function"))
    }

    /// Extract function name
    fn extract_name(&self, function: &'a Value) -> Result<&'a str> {
        function
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenAIError::parsing("Function missing name"))
    }

    /// Extract function arguments
    fn extract_arguments(&self, function: &'a Value) -> Result<&'a str> {
        function
            .get("arguments")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenAIError::parsing("Function missing arguments"))
    }
}

/// Parse a single tool call from the API response using structured extraction
pub(crate) fn parse_single_tool_call(call: &Value) -> Result<FunctionCall> {
    let extractor = ToolCallExtractor::new(call);
    let (id, name, arguments) = extractor.extract_all()?;
    Ok(FunctionCall::new(id, name, arguments))
}

/// Parse tool calls from the API response using functional approach
pub(crate) fn parse_tool_calls(tool_calls: &[Value]) -> Result<Vec<FunctionCall>> {
    tool_calls.iter().map(parse_single_tool_call).collect()
}
