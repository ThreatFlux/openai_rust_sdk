//! Exact Responses request fixtures; Chat Completions shapes differ.

use openai_rust_sdk::models::functions::{CustomTool, FunctionTool, Tool, ToolChoice};
use openai_rust_sdk::models::gpt5::{TextConfig, Verbosity};
use openai_rust_sdk::models::responses::{
    ImageDetail, Message, MessageContent, MessageContentInput, MessageRole,
};
use openai_rust_sdk::models::responses_v2::CreateResponseRequest;
use openai_rust_sdk::models::tools::{EnhancedTool, EnhancedToolChoice};
use serde_json::json;

#[test]
fn response_json_schema_is_flat_under_text_format_for_generation_and_counting() {
    let schema = json!({"type":"object","properties":{"answer":{"type":"string"}},
        "required":["answer"],"additionalProperties":false});
    let mut request = CreateResponseRequest::new_text("model_fixture", "input")
        .with_strict_json_schema("Answer", schema.clone());
    request.text = Some(TextConfig {
        verbosity: Some(Verbosity::Low),
        ..Default::default()
    });
    let expected = json!({"model":"model_fixture","input":"input","text":{
        "verbosity":"low","format":{"type":"json_schema","name":"Answer",
            "strict":true,"schema":schema}}});
    assert_eq!(request.to_payload().unwrap(), expected);
    assert_eq!(request.to_input_token_payload().unwrap(), expected);
}

#[test]
fn response_json_mode_uses_text_format() {
    let request = CreateResponseRequest::new_text("model_fixture", "input").with_json_mode();
    assert_eq!(
        request.to_payload().unwrap(),
        json!({"model":"model_fixture","input":"input",
        "text":{"format":{"type":"json_object"}}})
    );
}

#[test]
fn multimodal_response_input_uses_string_image_url_and_top_level_detail() {
    let request = CreateResponseRequest::new_messages(
        "model_fixture",
        vec![Message {
            role: MessageRole::User,
            content: MessageContentInput::Array(vec![
                MessageContent::text("describe"),
                MessageContent::image_url_with_detail(
                    "https://example.com/fixture.png",
                    ImageDetail::High,
                ),
            ]),
        }],
    );
    let expected = json!({"model":"model_fixture","input":[{"type":"message","role":"user",
        "content":[{"type":"input_text","text":"describe"},{"type":"input_image",
            "image_url":"https://example.com/fixture.png","detail":"high"}]}]});
    assert_eq!(request.to_payload().unwrap(), expected);
    assert_eq!(request.to_input_token_payload().unwrap(), expected);
}

#[test]
fn function_and_custom_tools_use_flat_responses_schema() {
    let function = FunctionTool::new("lookup", "look up a fixture", json!({"type":"object"}));
    let custom =
        CustomTool::new("grammar_tool", "parse fixture").with_lark_grammar("start: \"ok\"");
    let request = CreateResponseRequest::new_text("model_fixture", "input")
        .with_tools(vec![
            Tool::Function { function },
            Tool::Custom {
                custom_tool: custom,
            },
        ])
        .with_enhanced_tools(vec![EnhancedTool::WebSearchPreview]);
    assert_eq!(
        request.to_payload().unwrap()["tools"],
        json!([
            {"type":"function","name":"lookup","description":"look up a fixture","parameters":{"type":"object"}},
            {"type":"custom","name":"grammar_tool","description":"parse fixture",
                "format":{"type":"grammar","syntax":"lark","definition":"start: \"ok\""}},
            {"type":"web_search_preview"}
        ])
    );
}

#[test]
fn simple_tool_choices_are_strings_in_generation_and_counting() {
    for (choice, expected) in [
        (ToolChoice::Auto, "auto"),
        (ToolChoice::Required, "required"),
        (ToolChoice::None, "none"),
    ] {
        let request =
            CreateResponseRequest::new_text("model_fixture", "input").with_tool_choice(choice);
        assert_eq!(request.to_payload().unwrap()["tool_choice"], expected);
        assert_eq!(
            request.to_input_token_payload().unwrap()["tool_choice"],
            expected
        );
    }
    for (choice, expected) in [
        (EnhancedToolChoice::Auto, "auto"),
        (EnhancedToolChoice::Required, "required"),
        (EnhancedToolChoice::None, "none"),
    ] {
        let request = CreateResponseRequest::new_text("model_fixture", "input")
            .with_enhanced_tool_choice(choice);
        assert_eq!(request.to_payload().unwrap()["tool_choice"], expected);
    }
}

#[test]
fn legacy_allowed_names_resolve_declared_tool_kinds_without_guessing() {
    let request = CreateResponseRequest::new_text("model_fixture", "input")
        .with_tools(vec![
            Tool::Function {
                function: FunctionTool::new("lookup", "fixture", json!({})),
            },
            Tool::Custom {
                custom_tool: CustomTool::new("custom_fixture", "fixture"),
            },
        ])
        .with_tool_choice(ToolChoice::allowed_tools(vec![
            "lookup".into(),
            "custom_fixture".into(),
        ]));
    assert_eq!(
        request.to_payload().unwrap()["tool_choice"],
        json!({
        "type":"allowed_tools","mode":"auto","tools":[{"type":"function","name":"lookup"},
            {"type":"custom","name":"custom_fixture"}]})
    );
    let undeclared =
        request.with_tool_choice(ToolChoice::allowed_tools(vec!["missing_fixture".into()]));
    assert!(undeclared.to_payload().is_err());
}

#[test]
fn legacy_regex_flags_are_rejected_instead_of_sending_an_invalid_format() {
    let custom = CustomTool::new("regex_fixture", "fixture")
        .with_regex_grammar("ok", Some(vec!["i".into()]));
    let request =
        CreateResponseRequest::new_text("model_fixture", "input").with_tools(vec![Tool::Custom {
            custom_tool: custom,
        }]);
    assert!(request.to_payload().is_err());
}
