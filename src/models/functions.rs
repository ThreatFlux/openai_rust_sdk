use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use serde_json::Value;

/// Function tool definition with JSON schema parameters
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
pub struct FunctionTool {
    /// Name of the function
    pub name: String,
    /// Description of what the function does
    pub description: String,
    /// JSON schema for the function parameters
    pub parameters: Value,
    /// Whether to use strict mode for reliable schema adherence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// A function call made by the model
#[derive(Debug, Clone, Ser, De)]
pub struct FunctionCall {
    /// Unique identifier for this function call
    pub call_id: String,
    /// Name of the function being called
    pub name: String,
    /// JSON string containing the function arguments
    pub arguments: String,
}

/// Output from a function call execution
#[derive(Debug, Clone, Ser, De)]
pub struct FunctionCallOutput {
    /// The `call_id` this output corresponds to
    pub call_id: String,
    /// The output content from the function execution
    pub output: String,
}

/// Different types of tools that can be used
#[derive(Debug, Clone, Ser, De)]
#[serde(tag = "type")]
pub enum Tool {
    /// Function tool
    #[serde(rename = "function")]
    Function {
        /// The function definition
        function: FunctionTool,
    },
    /// Custom tool (for extensibility)
    #[serde(rename = "custom")]
    Custom {
        /// Custom tool definition
        custom_tool: CustomTool,
    },
}

/// Custom tool definition without explicit schema
#[derive(Debug, Clone, Ser, De)]
pub struct CustomTool {
    /// Name of the custom tool
    pub name: String,
    /// Description of what the tool does
    pub description: String,
    /// Optional grammar specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grammar: Option<Grammar>,
}

/// Grammar specification for custom tools
#[derive(Debug, Clone, Ser, De)]
#[serde(tag = "type")]
pub enum Grammar {
    /// Lark grammar syntax
    #[serde(rename = "lark")]
    Lark {
        /// The Lark grammar definition
        definition: String,
    },
    /// Regular expression syntax
    #[serde(rename = "regex")]
    Regex {
        /// The regex pattern
        pattern: String,
        /// Optional flags for the regex
        #[serde(skip_serializing_if = "Option::is_none")]
        flags: Option<Vec<String>>,
    },
}

/// Tool choice configuration
#[derive(Debug, Clone, Ser, De)]
#[serde(untagged)]
pub enum ToolChoice {
    /// Let the model choose automatically
    Auto,
    /// Require the model to call a tool
    Required,
    /// Don't use any tools
    None,
    /// Force a specific function to be called
    Function {
        /// Type must be "function"
        r#type: String,
        /// The function to force
        function: FunctionSelector,
    },
    /// Only allow specific tools
    AllowedTools {
        /// List of allowed tool names
        allowed_tools: Vec<String>,
    },
}

/// Function selector for tool choice
#[derive(Debug, Clone, Ser, De)]
pub struct FunctionSelector {
    /// Name of the function to select
    pub name: String,
}

impl FunctionTool {
    /// Create a new function tool
    pub fn new(name: impl Into<String>, description: impl Into<String>, parameters: Value) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters,
            strict: None,
        }
    }

    /// Enable strict mode for this function
    #[must_use]
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = Some(strict);
        self
    }

    /// Create a simple function with no parameters
    pub fn simple(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(
            name,
            description,
            serde_json::json!({
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false
            }),
        )
    }
}

impl Tool {
    /// Create a function tool
    #[must_use]
    pub fn function(function: FunctionTool) -> Self {
        Self::Function { function }
    }

    /// Create a custom tool
    #[must_use]
    pub fn custom(custom_tool: CustomTool) -> Self {
        Self::Custom { custom_tool }
    }

    /// Get the name of this tool
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Function { function } => &function.name,
            Self::Custom { custom_tool } => &custom_tool.name,
        }
    }

    /// Get the description of this tool
    #[must_use]
    pub fn description(&self) -> &str {
        match self {
            Self::Function { function } => &function.description,
            Self::Custom { custom_tool } => &custom_tool.description,
        }
    }
}

impl CustomTool {
    /// Create a new custom tool
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            grammar: None,
        }
    }

    /// Add a Lark grammar to this tool
    pub fn with_lark_grammar(mut self, definition: impl Into<String>) -> Self {
        self.grammar = Some(Grammar::Lark {
            definition: definition.into(),
        });
        self
    }

    /// Add a regex grammar to this tool
    pub fn with_regex_grammar(
        mut self,
        pattern: impl Into<String>,
        flags: Option<Vec<String>>,
    ) -> Self {
        self.grammar = Some(Grammar::Regex {
            pattern: pattern.into(),
            flags,
        });
        self
    }
}

impl Grammar {
    /// Create a Lark grammar
    pub fn lark(definition: impl Into<String>) -> Self {
        Self::Lark {
            definition: definition.into(),
        }
    }

    /// Create a regex grammar
    pub fn regex(pattern: impl Into<String>, flags: Option<Vec<String>>) -> Self {
        Self::Regex {
            pattern: pattern.into(),
            flags,
        }
    }
}

impl ToolChoice {
    /// Auto tool choice
    #[must_use]
    pub fn auto() -> Self {
        Self::Auto
    }

    /// Required tool choice
    #[must_use]
    pub fn required() -> Self {
        Self::Required
    }

    /// No tools
    #[must_use]
    pub fn none() -> Self {
        Self::None
    }

    /// Force a specific function
    pub fn function(name: impl Into<String>) -> Self {
        Self::Function {
            r#type: "function".to_string(),
            function: FunctionSelector { name: name.into() },
        }
    }

    /// Only allow specific tools
    #[must_use]
    pub fn allowed_tools(tools: Vec<String>) -> Self {
        Self::AllowedTools {
            allowed_tools: tools,
        }
    }
}

impl FunctionCall {
    /// Create a new function call
    pub fn new(
        call_id: impl Into<String>,
        name: impl Into<String>,
        arguments: impl Into<String>,
    ) -> Self {
        Self {
            call_id: call_id.into(),
            name: name.into(),
            arguments: arguments.into(),
        }
    }

    /// Parse the arguments as JSON
    pub fn parse_arguments<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.arguments)
    }

    /// Get arguments as a JSON Value
    pub fn arguments_json(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_str(&self.arguments)
    }
}

impl FunctionCallOutput {
    /// Create a new function call output
    pub fn new(call_id: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            call_id: call_id.into(),
            output: output.into(),
        }
    }

    /// Create output from a serializable value
    pub fn from_value<T: Serialize>(
        call_id: impl Into<String>,
        value: &T,
    ) -> Result<Self, serde_json::Error> {
        let output = serde_json::to_string(value)?;
        Ok(Self::new(call_id, output))
    }

    /// Create output from a JSON value
    pub fn from_json(call_id: impl Into<String>, value: &Value) -> Result<Self, serde_json::Error> {
        let output = serde_json::to_string(&value)?;
        Ok(Self::new(call_id, output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_tool_creation() {
        let func = FunctionTool::new(
            "get_weather",
            "Get weather for a location",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "location": {"type": "string"},
                    "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]}
                },
                "required": ["location"]
            }),
        );

        assert_eq!(func.name, "get_weather");
        assert_eq!(func.description, "Get weather for a location");
        assert!(func.strict.is_none());
    }

    #[test]
    fn test_function_tool_with_strict() {
        let func = FunctionTool::simple("test", "Test function").with_strict(true);
        assert_eq!(func.strict, Some(true));
    }

    #[test]
    fn test_tool_creation() {
        let func_tool = FunctionTool::simple("test", "Test");
        let tool = Tool::function(func_tool);

        assert_eq!(tool.name(), "test");
        assert_eq!(tool.description(), "Test");
    }

    #[test]
    fn test_custom_tool_with_grammar() {
        let tool =
            CustomTool::new("parser", "Parse text").with_lark_grammar("start: word+\nword: /\\w+/");

        assert_eq!(tool.name, "parser");
        assert!(tool.grammar.is_some());

        if let Some(Grammar::Lark { definition }) = &tool.grammar {
            assert!(definition.contains("start: word+"));
        } else {
            panic!("Expected Lark grammar");
        }
    }

    #[test]
    fn test_tool_choice_variants() {
        let auto = ToolChoice::auto();
        let required = ToolChoice::required();
        let none = ToolChoice::none();
        let function = ToolChoice::function("get_weather");
        let allowed = ToolChoice::allowed_tools(vec!["tool1".to_string(), "tool2".to_string()]);

        // Just test that they can be created without panicking
        assert!(matches!(auto, ToolChoice::Auto));
        assert!(matches!(required, ToolChoice::Required));
        assert!(matches!(none, ToolChoice::None));
        assert!(matches!(function, ToolChoice::Function { .. }));
        assert!(matches!(allowed, ToolChoice::AllowedTools { .. }));
    }

    #[test]
    fn test_function_call_arguments() {
        let call = FunctionCall::new(
            "call-123",
            "get_weather",
            r#"{"location": "San Francisco", "unit": "celsius"}"#,
        );

        let args: Value = call.arguments_json().unwrap();
        assert_eq!(args["location"], "San Francisco");
        assert_eq!(args["unit"], "celsius");
    }

    #[test]
    fn test_function_call_output() {
        let output = FunctionCallOutput::new("call-123", "Temperature: 22°C");
        assert_eq!(output.call_id, "call-123");
        assert_eq!(output.output, "Temperature: 22°C");

        let json_output = FunctionCallOutput::from_json(
            "call-456",
            &serde_json::json!({"temperature": 22, "unit": "celsius"}),
        )
        .unwrap();

        assert_eq!(json_output.call_id, "call-456");
        assert!(json_output.output.contains("temperature"));
    }
}
