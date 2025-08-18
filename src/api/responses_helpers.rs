//! Helper functions for response API to reduce complexity
#![allow(dead_code)]

use crate::models::responses::{
    ImageDetail, MessageContent, MessageContentInput, MessageRole, ResponseFormat,
};
use serde_json::{Value, json};

/// Convert message role to OpenAI format
pub fn role_to_string(role: MessageRole) -> &'static str {
    match role {
        MessageRole::Developer => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::System => "system",
    }
}

/// Convert message content to OpenAI format
pub fn content_to_json(content: &MessageContentInput) -> Value {
    match content {
        MessageContentInput::Text(text) => json!(text),
        MessageContentInput::Array(contents) => {
            json!(
                contents
                    .iter()
                    .map(content_item_to_json)
                    .collect::<Vec<_>>()
            )
        }
    }
}

/// Convert a single content item to JSON
fn content_item_to_json(content: &MessageContent) -> Value {
    match content {
        MessageContent::Text { text } => {
            json!({
                "type": "text",
                "text": text
            })
        }
        MessageContent::Image { image_url } => {
            let mut img = json!({
                "type": "image_url",
                "image_url": {
                    "url": image_url.url
                }
            });

            if let Some(detail) = &image_url.detail {
                img["image_url"]["detail"] = json!(image_detail_to_string(detail));
            }

            img
        }
    }
}

/// Convert image detail enum to string
fn image_detail_to_string(detail: &ImageDetail) -> &'static str {
    match detail {
        ImageDetail::Auto => "auto",
        ImageDetail::Low => "low",
        ImageDetail::High => "high",
    }
}

/// Add optional parameters to request
pub fn add_optional_params(request: &mut Value, params: OptionalParams) {
    if let Some(temp) = params.temperature {
        request["temperature"] = json!(temp);
    }
    if let Some(max_tokens) = params.max_tokens {
        request["max_tokens"] = json!(max_tokens);
    }
    if let Some(top_p) = params.top_p {
        request["top_p"] = json!(top_p);
    }
    if let Some(freq_penalty) = params.frequency_penalty {
        request["frequency_penalty"] = json!(freq_penalty);
    }
    if let Some(pres_penalty) = params.presence_penalty {
        request["presence_penalty"] = json!(pres_penalty);
    }
    if params.stream == Some(true) {
        request["stream"] = json!(true);
    }
}

/// Optional parameters for OpenAI requests
pub struct OptionalParams {
    /// The sampling temperature
    pub temperature: Option<f32>,
    /// The maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Top-p nucleus sampling
    pub top_p: Option<f32>,
    /// Frequency penalty to reduce repetition
    pub frequency_penalty: Option<f32>,
    /// Presence penalty to encourage new topics
    pub presence_penalty: Option<f32>,
    /// Whether to stream the response
    pub stream: Option<bool>,
}

/// Convert response format to OpenAI format
pub fn response_format_to_json(format: &ResponseFormat) -> Option<Value> {
    match format {
        ResponseFormat::Text => None, // Default format
        ResponseFormat::JsonObject => Some(json!({
            "type": "json_object"
        })),
        ResponseFormat::JsonSchema { json_schema, .. } => Some(json!({
            "type": "json_schema",
            "json_schema": json_schema
        })),
    }
}
