use crate::error::{OpenAIError, Result};
use crate::models::functions::{CustomTool, Grammar};
use serde_json::Value;
use std::collections::HashMap;

// Lookup table for grammar type constants to reduce duplication
const GRAMMAR_TYPE_LARK: &str = "lark";
const GRAMMAR_TYPE_REGEX: &str = "regex";
const GRAMMAR_TYPE_CFG: &str = "cfg";

/// Custom tools API for advanced tool definitions
#[derive(Debug)]
pub struct CustomToolsApi {
    /// Registry of custom tools
    tools: HashMap<String, CustomTool>,
    /// Grammar validators
    validators: HashMap<String, Box<dyn GrammarValidator>>,
}

/// Trait for validating grammar-based inputs
pub trait GrammarValidator: Send + Sync + std::fmt::Debug {
    /// Validate input against the grammar
    fn validate(&self, input: &str) -> Result<bool>;

    /// Parse and extract structured data from input
    fn parse(&self, input: &str) -> Result<Value>;

    /// Get the grammar type
    fn grammar_type(&self) -> &str;
}

/// Lark grammar validator
#[derive(Debug)]
pub struct LarkValidator {
    /// The grammar definition
    definition: String,
}

/// Regex grammar validator
#[derive(Debug)]
pub struct RegexValidator {
    /// The regex pattern
    pattern: String,
    /// Optional flags
    flags: Option<Vec<String>>,
    /// Compiled regex (would use regex crate in real implementation)
    _compiled: String,
}

/// Context-free grammar (CFG) validator
#[derive(Debug)]
pub struct CfgValidator {
    /// The CFG rules
    rules: HashMap<String, Vec<String>>,
    /// Start symbol
    start_symbol: String,
}

impl Clone for CustomToolsApi {
    fn clone(&self) -> Self {
        Self {
            tools: self.tools.clone(),
            validators: HashMap::new(), // Validators can't be cloned, so create empty
        }
    }
}

impl CustomToolsApi {
    /// Create a new custom tools API
    #[must_use]
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            validators: HashMap::new(),
        }
    }

    /// Register a custom tool
    pub fn register_tool(&mut self, tool: CustomTool) -> Result<()> {
        let name = tool.name.clone();

        // Create validator if grammar is provided
        if let Some(grammar) = &tool.grammar {
            let validator = Self::create_validator(grammar)?;
            self.validators.insert(name.clone(), validator);
        }

        self.tools.insert(name, tool);
        Ok(())
    }

    /// Get a registered tool by name
    #[must_use]
    pub fn get_tool(&self, name: &str) -> Option<&CustomTool> {
        self.tools.get(name)
    }

    /// List all registered tool names
    #[must_use]
    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// Validate input for a tool with grammar
    pub fn validate_input(&self, tool_name: &str, input: &str) -> Result<bool> {
        self.validators
            .get(tool_name)
            .map_or_else(|| Ok(true), |validator| validator.validate(input))
    }

    /// Parse input for a tool with grammar
    pub fn parse_input(&self, tool_name: &str, input: &str) -> Result<Value> {
        self.validators.get(tool_name).map_or_else(
            || Ok(Value::String(input.to_string())),
            |validator| validator.parse(input),
        )
    }

    /// Create a validator from grammar specification
    fn create_validator(grammar: &Grammar) -> Result<Box<dyn GrammarValidator>> {
        match grammar {
            Grammar::Lark { definition } => Ok(Box::new(LarkValidator::new(definition.clone())?)),
            Grammar::Regex { pattern, flags } => Ok(Box::new(RegexValidator::new(
                pattern.clone(),
                flags.clone(),
            )?)),
        }
    }

    /// Remove a tool from the registry
    pub fn remove_tool(&mut self, name: &str) -> Option<CustomTool> {
        self.validators.remove(name);
        self.tools.remove(name)
    }

    /// Clear all registered tools
    pub fn clear(&mut self) {
        self.tools.clear();
        self.validators.clear();
    }

    /// Get the number of registered tools
    #[must_use]
    pub fn tool_count(&self) -> usize {
        self.tools.len()
    }

    /// Check if a tool is registered
    #[must_use]
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
}

impl LarkValidator {
    /// Create a new Lark validator
    pub fn new(definition: String) -> Result<Self> {
        // In a real implementation, this would compile the Lark grammar
        // For now, we'll just validate that it's not empty
        if definition.trim().is_empty() {
            return Err(OpenAIError::validation(
                "Lark grammar definition cannot be empty",
            ));
        }

        Ok(Self { definition })
    }

    /// Get the grammar definition
    #[must_use]
    pub fn definition(&self) -> &str {
        &self.definition
    }
}

impl GrammarValidator for LarkValidator {
    fn validate(&self, input: &str) -> Result<bool> {
        // Simplified validation - in real implementation would use Lark parser
        // For now, just check that input is not empty
        Ok(!input.trim().is_empty())
    }

    fn parse(&self, input: &str) -> Result<Value> {
        // Simplified parsing - in real implementation would use Lark to build AST
        // For now, return the input as a structured object
        Ok(serde_json::json!({
            "input": input,
            "grammar_type": GRAMMAR_TYPE_LARK,
            "parsed": true
        }))
    }

    fn grammar_type(&self) -> &'static str {
        GRAMMAR_TYPE_LARK
    }
}

impl RegexValidator {
    /// Create a new regex validator
    pub fn new(pattern: String, flags: Option<Vec<String>>) -> Result<Self> {
        // In a real implementation, this would compile the regex
        if pattern.trim().is_empty() {
            return Err(OpenAIError::validation("Regex pattern cannot be empty"));
        }

        Ok(Self {
            pattern: pattern.clone(),
            flags,
            _compiled: pattern, // Simplified - would use regex crate
        })
    }

    /// Get the regex pattern
    #[must_use]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Get the regex flags
    #[must_use]
    pub const fn flags(&self) -> Option<&Vec<String>> {
        self.flags.as_ref()
    }
}

impl GrammarValidator for RegexValidator {
    fn validate(&self, input: &str) -> Result<bool> {
        // Simplified validation - in real implementation would use regex crate
        // For now, just check basic pattern matching
        if self.pattern == ".*" {
            Ok(true)
        } else if self.pattern.contains("\\d+") {
            Ok(input.chars().any(|c| c.is_ascii_digit()))
        } else if self.pattern.contains("\\w+") {
            Ok(input.chars().any(char::is_alphanumeric))
        } else {
            // Default to true for other patterns
            Ok(true)
        }
    }

    fn parse(&self, input: &str) -> Result<Value> {
        // Simplified parsing - in real implementation would use regex capture groups
        Ok(serde_json::json!({
            "input": input,
            "pattern": self.pattern,
            "grammar_type": GRAMMAR_TYPE_REGEX,
            "flags": self.flags
        }))
    }

    fn grammar_type(&self) -> &'static str {
        GRAMMAR_TYPE_REGEX
    }
}

impl CfgValidator {
    /// Create a new CFG validator
    pub fn new(rules: HashMap<String, Vec<String>>, start_symbol: String) -> Result<Self> {
        if rules.is_empty() {
            return Err(OpenAIError::validation("CFG rules cannot be empty"));
        }

        if !rules.contains_key(&start_symbol) {
            return Err(OpenAIError::validation("Start symbol must exist in rules"));
        }

        Ok(Self {
            rules,
            start_symbol,
        })
    }

    /// Get the CFG rules
    #[must_use]
    pub const fn rules(&self) -> &HashMap<String, Vec<String>> {
        &self.rules
    }

    /// Get the start symbol
    #[must_use]
    pub fn start_symbol(&self) -> &str {
        &self.start_symbol
    }
}

impl GrammarValidator for CfgValidator {
    fn validate(&self, input: &str) -> Result<bool> {
        // Simplified CFG validation - in real implementation would use a proper parser
        // For now, just check that input is not empty
        Ok(!input.trim().is_empty())
    }

    fn parse(&self, input: &str) -> Result<Value> {
        // Simplified CFG parsing
        Ok(serde_json::json!({
            "input": input,
            "start_symbol": self.start_symbol,
            "grammar_type": GRAMMAR_TYPE_CFG,
            "rules_count": self.rules.len()
        }))
    }

    fn grammar_type(&self) -> &'static str {
        GRAMMAR_TYPE_CFG
    }
}

impl Default for CustomToolsApi {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating custom tools with fluent API
#[derive(Debug, Default)]
pub struct CustomToolBuilder {
    /// Tool name
    name: Option<String>,
    /// Tool description
    description: Option<String>,
    /// Grammar specification
    grammar: Option<Grammar>,
}

impl CustomToolBuilder {
    /// Create a new custom tool builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the tool name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the tool description
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add a Lark grammar
    #[must_use]
    pub fn lark_grammar(mut self, definition: impl Into<String>) -> Self {
        self.grammar = Some(Grammar::lark(definition));
        self
    }

    /// Add a regex grammar
    #[must_use]
    pub fn regex_grammar(mut self, pattern: impl Into<String>, flags: Option<Vec<String>>) -> Self {
        self.grammar = Some(Grammar::regex(pattern, flags));
        self
    }

    /// Build the custom tool
    pub fn build(self) -> Result<CustomTool> {
        let name = self
            .name
            .ok_or_else(|| OpenAIError::validation("Custom tool name is required"))?;

        let description = self
            .description
            .ok_or_else(|| OpenAIError::validation("Custom tool description is required"))?;

        let mut tool = CustomTool::new(name, description);
        if let Some(grammar) = self.grammar {
            tool.grammar = Some(grammar);
        }

        Ok(tool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_tools_api() {
        let mut api = CustomToolsApi::new();
        assert_eq!(api.tool_count(), 0);

        let tool = CustomTool::new("test_tool", "A test tool");
        api.register_tool(tool).unwrap();

        assert_eq!(api.tool_count(), 1);
        assert!(api.has_tool("test_tool"));
        assert!(!api.has_tool("nonexistent"));

        let retrieved = api.get_tool("test_tool").unwrap();
        assert_eq!(retrieved.name, "test_tool");
    }

    #[test]
    fn test_lark_validator() {
        let validator = LarkValidator::new("start: word+".to_string()).unwrap();
        assert_eq!(validator.grammar_type(), "lark");
        assert!(validator.validate("test input").unwrap());

        let parsed = validator.parse("test").unwrap();
        assert_eq!(parsed["grammar_type"], "lark");
    }

    #[test]
    fn test_regex_validator() {
        let validator = RegexValidator::new("\\d+".to_string(), None).unwrap();
        assert_eq!(validator.grammar_type(), "regex");
        assert!(validator.validate("123").unwrap());

        let parsed = validator.parse("123").unwrap();
        assert_eq!(parsed["grammar_type"], "regex");
    }

    #[test]
    fn test_cfg_validator() {
        let mut rules = HashMap::new();
        rules.insert("S".to_string(), vec!["A B".to_string()]);
        rules.insert("A".to_string(), vec!["a".to_string()]);
        rules.insert("B".to_string(), vec!["b".to_string()]);

        let validator = CfgValidator::new(rules, "S".to_string()).unwrap();
        assert_eq!(validator.grammar_type(), "cfg");
        assert!(validator.validate("a b").unwrap());
    }

    #[test]
    fn test_custom_tool_builder() {
        let tool = CustomToolBuilder::new()
            .name("parser")
            .description("Parse structured text")
            .lark_grammar("start: word+\nword: /\\w+/")
            .build()
            .unwrap();

        assert_eq!(tool.name, "parser");
        assert_eq!(tool.description, "Parse structured text");
        assert!(tool.grammar.is_some());
    }

    #[test]
    fn test_builder_validation() {
        let result = CustomToolBuilder::new().description("Missing name").build();

        assert!(result.is_err());
    }

    #[test]
    fn test_empty_grammar_validation() {
        let result = LarkValidator::new("".to_string());
        assert!(result.is_err());

        let result = RegexValidator::new("".to_string(), None);
        assert!(result.is_err());
    }
}
