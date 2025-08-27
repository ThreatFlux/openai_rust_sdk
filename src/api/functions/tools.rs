use crate::error::{OpenAIError, Result};
use crate::models::functions::{FunctionCall, FunctionCallOutput, Tool};
use serde_json::{json, Value};

use super::client::FunctionsApi;

impl FunctionsApi {
    /// Register a custom tool
    pub fn register_custom_tool(
        &mut self,
        tool: crate::models::functions::CustomTool,
    ) -> Result<()> {
        self.custom_tools.register_tool(tool)
    }

    /// Validate function call arguments against the schema
    pub fn validate_function_call(&self, call: &FunctionCall, tools: &[Tool]) -> Result<()> {
        // Find the tool definition
        let tool = tools
            .iter()
            .find(|t| t.name() == call.name)
            .ok_or_else(|| OpenAIError::validation(format!("Unknown function: {}", call.name)))?;

        match tool {
            Tool::Function { function: _ } => {
                // Parse arguments as JSON
                let args: Value = serde_json::from_str(&call.arguments)
                    .map_err(crate::validation_err!("Invalid JSON arguments: {}"))?;

                // TODO: Add proper JSON schema validation here
                // For now, just check that it's a valid JSON object
                if !args.is_object() {
                    return Err(OpenAIError::validation(
                        "Function arguments must be a JSON object",
                    ));
                }

                Ok(())
            }
            Tool::Custom { custom_tool } => {
                // Validate with custom tool grammar if available
                self.custom_tools
                    .validate_input(&custom_tool.name, &call.arguments)?;
                Ok(())
            }
        }
    }

    /// Execute a function call (placeholder - actual execution is application-specific)
    pub async fn execute_function_call(&self, call: &FunctionCall) -> Result<FunctionCallOutput> {
        // This is a placeholder implementation
        // In practice, applications would provide their own function executors

        match call.name.as_str() {
            "get_weather" => {
                let args: Value = call.arguments_json()?;
                let location = args
                    .get("location")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");

                let result = json!({
                    "location": location,
                    "temperature": 22,
                    "unit": "celsius",
                    "description": "Sunny"
                });

                Ok(FunctionCallOutput::from_json(&call.call_id, &result)?)
            }
            "get_time" => {
                let result = json!({
                    "current_time": chrono::Utc::now().to_rfc3339(),
                    "timezone": "UTC"
                });

                Ok(FunctionCallOutput::from_json(&call.call_id, &result)?)
            }
            _ => Err(OpenAIError::invalid_request(format!(
                "Unknown function: {}",
                call.name
            ))),
        }
    }

    /// Convert tools to API format
    #[allow(clippy::unused_self)]
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn serialize_tools(&self, tools: &[Tool]) -> Result<Vec<Value>> {
        let mut serialized = Vec::new();

        for tool in tools {
            match tool {
                Tool::Function { function } => {
                    serialized.push(json!({
                        "type": "function",
                        "function": {
                            "name": function.name,
                            "description": function.description,
                            "parameters": function.parameters,
                            "strict": function.strict
                        }
                    }));
                }
                Tool::Custom { custom_tool } => {
                    // Convert custom tool to function format
                    serialized.push(json!({
                        "type": "function",
                        "function": {
                            "name": custom_tool.name,
                            "description": custom_tool.description,
                            "parameters": {
                                "type": "object",
                                "properties": {},
                                "required": []
                            }
                        }
                    }));
                }
            }
        }

        Ok(serialized)
    }
}
