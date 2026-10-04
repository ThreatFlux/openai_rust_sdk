//! Adapt compatibility tools to the current Responses request schema.

use crate::models::functions::{CustomTool, Grammar, ToolChoice};
use crate::models::tools::EnhancedToolChoice;
use serde_json::{Value, json};

/// Convert legacy choice variants without changing their public serde behavior.
pub(super) fn convert_tool_choice(
    choice: &ToolChoice,
    tools: Option<&Value>,
) -> serde_json::Result<Value> {
    match choice {
        ToolChoice::Auto => Ok(json!("auto")),
        ToolChoice::Required => Ok(json!("required")),
        ToolChoice::None => Ok(json!("none")),
        ToolChoice::Function(selection) => serde_json::to_value(selection),
        ToolChoice::AllowedTools(selection) => {
            let selected: serde_json::Result<Vec<_>> = selection
                .allowed_tools
                .iter()
                .map(|name| select_declared_tool(name, tools))
                .collect();
            Ok(json!({"type":"allowed_tools","mode":"auto","tools":selected?}))
        }
    }
}

/// Resolve legacy names only when a matching function/custom tool was declared.
fn select_declared_tool(name: &str, tools: Option<&Value>) -> serde_json::Result<Value> {
    let tool = tools
        .and_then(Value::as_array)
        .and_then(|tools| {
            tools
                .iter()
                .find(|tool| tool.get("name").and_then(Value::as_str) == Some(name))
        })
        .ok_or_else(|| {
            invalid("Allowed tool names must identify declared function or custom tools")
        })?;
    let kind = tool
        .get("type")
        .and_then(Value::as_str)
        .filter(|kind| matches!(*kind, "function" | "custom"))
        .ok_or_else(|| invalid("Legacy allowed-tool names support function or custom tools"))?;
    Ok(json!({"type":kind,"name":name}))
}

/// Translate simple enhanced choices to their current string discriminators.
pub(super) fn convert_enhanced_tool_choice(
    choice: &EnhancedToolChoice,
) -> serde_json::Result<Value> {
    match choice {
        EnhancedToolChoice::Auto => Ok(json!("auto")),
        EnhancedToolChoice::Required => Ok(json!("required")),
        EnhancedToolChoice::None => Ok(json!("none")),
        EnhancedToolChoice::Specific(selection) => serde_json::to_value(selection),
    }
}

/// Flatten custom definitions and adapt the legacy grammar variants.
pub(super) fn convert_custom_tool(tool: &CustomTool) -> serde_json::Result<Value> {
    let mut payload = json!({"type":"custom","name":tool.name,"description":tool.description});
    if let Some(grammar) = &tool.grammar {
        payload["format"] = match grammar {
            Grammar::Lark { definition } => {
                json!({"type":"grammar","syntax":"lark","definition":definition})
            }
            Grammar::Regex { pattern, flags } => {
                if flags.as_ref().is_some_and(|flags| !flags.is_empty()) {
                    return Err(invalid(
                        "Responses grammar format does not accept separate regex flags",
                    ));
                }
                json!({"type":"grammar","syntax":"regex","definition":pattern})
            }
        };
    }
    Ok(payload)
}

/// Report an unsupported request shape through the existing JSON error result.
fn invalid(message: &str) -> serde_json::Error {
    <serde_json::Error as serde::ser::Error>::custom(message)
}
