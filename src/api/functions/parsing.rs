use crate::constants::endpoints;
use crate::error::{OpenAIError, Result};
use serde_json::{json, Value};

use super::client::{FunctionConfig, FunctionsApi};
use super::extraction::parse_tool_calls;
use super::helpers::FunctionResponseResult;

impl FunctionsApi {
    /// Build the API request payload
    pub(crate) fn build_payload(
        &self,
        request: &crate::models::responses::ResponseRequest,
    ) -> Result<Value> {
        let mut payload = json!({
            "model": request.model,
            "messages": self.convert_input_to_messages(&request.input)?,
        });

        if let Some(temp) = request.temperature {
            payload["temperature"] = json!(temp);
        }
        if let Some(max_tokens) = request.max_tokens {
            payload["max_tokens"] = json!(max_tokens);
        }
        if let Some(stream) = request.stream {
            payload["stream"] = json!(stream);
        }

        Ok(payload)
    }

    /// Add function calling configuration to the payload
    pub(crate) fn add_function_config(
        &self,
        payload: &mut Value,
        config: &FunctionConfig,
    ) -> Result<()> {
        if !config.tools.is_empty() {
            payload["tools"] = json!(self.serialize_tools(&config.tools)?);
        }

        if let Some(ref tool_choice) = config.tool_choice {
            payload["tool_choice"] = json!(tool_choice);
        }

        if let Some(parallel) = config.parallel_function_calls {
            payload["parallel_tool_calls"] = json!(parallel);
        }

        Ok(())
    }

    /// Convert input to messages format
    #[allow(clippy::unused_self)]
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::match_same_arms)]
    pub(crate) fn convert_input_to_messages(
        &self,
        input: &crate::models::responses::ResponseInput,
    ) -> Result<Vec<Value>> {
        match input {
            crate::models::responses::ResponseInput::Text(text) => Ok(vec![json!({
                "role": "user",
                "content": text
            })]),
            crate::models::responses::ResponseInput::Messages(messages) => Ok(messages
                .iter()
                .map(|msg| {
                    json!({
                        "role": match msg.role {
                            crate::models::responses::MessageRole::User => "user",
                            crate::models::responses::MessageRole::Assistant => "assistant",
                            crate::models::responses::MessageRole::Developer => "system",
                            crate::models::responses::MessageRole::System => "system",
                        },
                        "content": msg.content
                    })
                })
                .collect()),
        }
    }

    /// Send the API request
    pub(crate) async fn send_request(&self, payload: &Value) -> Result<Value> {
        let url = format!("{}{}", self.base_url, endpoints::CHAT_COMPLETIONS);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(payload)
            .send()
            .await
            .map_err(crate::network_err!("Request failed: {}"))?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::api_error(status_code, &error_text));
        }

        let json_response: Value = response
            .json()
            .await
            .map_err(crate::parsing_err!("Failed to parse response: {}"))?;

        Ok(json_response)
    }

    /// Parse the function response from the API
    pub(crate) async fn parse_function_response(
        &self,
        response: Value,
    ) -> Result<FunctionResponseResult> {
        let choices = response
            .get("choices")
            .and_then(|v| v.as_array())
            .ok_or_else(|| OpenAIError::parsing("No choices in response"))?;

        let first_choice = choices
            .first()
            .ok_or_else(|| OpenAIError::parsing("No choices in response"))?;

        let message = first_choice
            .get("message")
            .ok_or_else(|| OpenAIError::parsing("No message in choice"))?;

        let content = message
            .get("content")
            .and_then(|v| v.as_str())
            .map(std::string::ToString::to_string);

        let tool_calls = message
            .get("tool_calls")
            .and_then(|v| v.as_array())
            .map(|calls| parse_tool_calls(calls))
            .transpose()?
            .unwrap_or_default();

        Ok(FunctionResponseResult {
            content,
            function_calls: tool_calls,
            response,
        })
    }
}
