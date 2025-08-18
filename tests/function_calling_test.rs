use openai_rust_sdk::{
    builders::function_builder::FunctionBuilder,
    models::functions::{FunctionCall, FunctionCallOutput, Tool, ToolChoice},
};
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_builder() {
        let function = FunctionBuilder::new()
            .name("get_weather")
            .description("Get the current weather")
            .required_string("location", "City and country")
            .optional_string("units", "Temperature units")
            .strict(true)
            .build()
            .unwrap();

        assert_eq!(function.name, "get_weather");
        assert_eq!(function.description, "Get the current weather");
        assert!(function.strict.unwrap_or(false));

        let params = function.parameters.as_object().unwrap();
        assert!(params.contains_key("properties"));
        assert!(params.contains_key("required"));
    }

    #[test]
    fn test_function_call_serialization() {
        let call = FunctionCall {
            call_id: "call_abc".to_string(),
            name: "get_weather".to_string(),
            arguments: json!({"location": "Paris"}).to_string(),
        };

        let json = serde_json::to_value(&call).unwrap();
        assert_eq!(json["call_id"], "call_abc");
        assert_eq!(json["name"], "get_weather");
        assert!(json["arguments"].is_string());
    }

    #[test]
    fn test_function_call_output() {
        let output = FunctionCallOutput {
            call_id: "call_abc".to_string(),
            output: "15°C, sunny".to_string(),
        };

        let json = serde_json::to_value(&output).unwrap();
        assert_eq!(json["call_id"], "call_abc");
        assert_eq!(json["output"], "15°C, sunny");
    }

    #[test]
    fn test_tool_choice_variants() {
        use openai_rust_sdk::models::functions::FunctionSelector;

        let auto = ToolChoice::Auto;
        let required = ToolChoice::Required;
        let none = ToolChoice::None;
        let specific = ToolChoice::Function {
            r#type: "function".to_string(),
            function: FunctionSelector {
                name: "get_weather".to_string(),
            },
        };

        // These serialize as simple strings
        assert!(serde_json::to_value(&auto).is_ok());
        assert!(serde_json::to_value(&required).is_ok());
        assert!(serde_json::to_value(&none).is_ok());

        let specific_json = serde_json::to_value(&specific).unwrap();
        assert_eq!(specific_json["type"], "function");
        assert_eq!(specific_json["function"]["name"], "get_weather");
    }

    #[test]
    fn test_tool_serialization() {
        use openai_rust_sdk::models::functions::CustomTool;

        let function = FunctionBuilder::new()
            .name("test_fn")
            .description("Test function")
            .build()
            .unwrap();

        let function_tool = Tool::Function { function };

        let custom_tool = Tool::Custom {
            custom_tool: CustomTool {
                name: "code_exec".to_string(),
                description: "Execute code".to_string(),
                grammar: None,
            },
        };

        let fn_json = serde_json::to_value(&function_tool).unwrap();
        assert_eq!(fn_json["type"], "function");
        assert_eq!(fn_json["function"]["name"], "test_fn");

        let custom_json = serde_json::to_value(&custom_tool).unwrap();
        assert_eq!(custom_json["type"], "custom");
        assert_eq!(custom_json["custom_tool"]["name"], "code_exec");
    }

    #[test]
    fn test_allowed_tools() {
        let allowed = ToolChoice::AllowedTools {
            allowed_tools: vec!["get_weather".to_string(), "code_exec".to_string()],
        };

        let json = serde_json::to_value(&allowed).unwrap();
        assert!(json["allowed_tools"].is_array());

        let tools = json["allowed_tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
        assert_eq!(tools[0], "get_weather");
        assert_eq!(tools[1], "code_exec");
    }

    #[test]
    fn test_weather_function_builder() {
        let function =
            FunctionBuilder::weather_function("get_weather", "Get current weather conditions")
                .build()
                .unwrap();

        assert_eq!(function.name, "get_weather");
        assert_eq!(function.description, "Get current weather conditions");

        let params = function.parameters.as_object().unwrap();
        let properties = params["properties"].as_object().unwrap();
        assert!(properties.contains_key("location"));
        assert!(properties.contains_key("unit"));
    }

    #[test]
    fn test_search_function_builder() {
        let function =
            FunctionBuilder::search_function("search_web", "Search the web for information")
                .build()
                .unwrap();

        assert_eq!(function.name, "search_web");
        assert_eq!(function.description, "Search the web for information");

        let params = function.parameters.as_object().unwrap();
        let properties = params["properties"].as_object().unwrap();
        assert!(properties.contains_key("query"));
    }

    #[test]
    fn test_grammar_variants() {
        use openai_rust_sdk::models::functions::Grammar;

        let lark = Grammar::Lark {
            definition: "start: /hello/".to_string(),
        };

        let regex = Grammar::Regex {
            pattern: r"^\d{3}-\d{3}-\d{4}$".to_string(),
            flags: None,
        };

        let lark_json = serde_json::to_value(&lark).unwrap();
        assert_eq!(lark_json["type"], "lark");
        assert_eq!(lark_json["definition"], "start: /hello/");

        let regex_json = serde_json::to_value(&regex).unwrap();
        assert_eq!(regex_json["type"], "regex");
        assert_eq!(regex_json["pattern"], r"^\d{3}-\d{3}-\d{4}$");
    }

    #[test]
    fn test_custom_tool_with_grammar() {
        use openai_rust_sdk::models::functions::{CustomTool, Grammar};

        let tool = Tool::Custom {
            custom_tool: CustomTool {
                name: "math_expr".to_string(),
                description: "Evaluate mathematical expressions".to_string(),
                grammar: Some(Grammar::Regex {
                    pattern: r"^\d+\s*[+\-*/]\s*\d+$".to_string(),
                    flags: None,
                }),
            },
        };

        let json = serde_json::to_value(&tool).unwrap();
        assert_eq!(json["type"], "custom");
        assert_eq!(json["custom_tool"]["name"], "math_expr");
        assert_eq!(json["custom_tool"]["grammar"]["type"], "regex");
    }
}
