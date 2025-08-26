use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::api::shared_utilities::EnumConverter;
use crate::error::{OpenAIError, Result};
use crate::models::responses::{
    Message, MessageRole, ResponseInput, ResponseRequest, ResponseResult,
};

/// `OpenAI` Responses API client
#[derive(Clone)]
pub struct ResponsesApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl ApiClientConstructors for ResponsesApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl ResponsesApi {
    /// Create a new `ResponsesApi` client with custom base URL
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
        })
    }

    /// Create a response using the /v1/chat/completions endpoint
    pub async fn create_response(&self, request: &ResponseRequest) -> Result<ResponseResult> {
        // Convert our request format to OpenAI's chat completions format
        let openai_request = self.to_openai_format(request)?;

        let mut result: ResponseResult = self
            .http_client
            .post("/v1/chat/completions", &openai_request)
            .await?;

        // Post-process response for structured outputs
        if let Some(response_format) = &request.response_format {
            self.process_structured_response(&mut result, response_format)?;
        }

        Ok(result)
    }

    /// Convert our internal request format to `OpenAI`'s chat completions format
    pub fn to_openai_format(&self, request: &ResponseRequest) -> Result<serde_json::Value> {
        use serde_json::json;

        let messages =
            self.convert_input_to_messages(&request.input, request.instructions.as_ref());

        let mut openai_request = json!({
            "model": request.model,
            "messages": messages
        });

        self.add_optional_parameters(&mut openai_request, request);
        self.add_response_format(&mut openai_request, request.response_format.as_ref());

        Ok(openai_request)
    }

    /// Convert input to messages format
    fn convert_input_to_messages(
        &self,
        input: &ResponseInput,
        instructions: Option<&String>,
    ) -> Vec<serde_json::Value> {
        match input {
            ResponseInput::Text(text) => self.convert_text_to_messages(text, instructions),
            ResponseInput::Messages(msgs) => self.convert_messages_to_openai_format(msgs),
        }
    }

    /// Convert text to messages format
    fn convert_text_to_messages(
        &self,
        text: &str,
        instructions: Option<&String>,
    ) -> Vec<serde_json::Value> {
        use serde_json::json;

        if let Some(instructions) = instructions {
            vec![
                json!({"role": "system", "content": instructions}),
                json!({"role": "user", "content": text}),
            ]
        } else {
            vec![json!({"role": "user", "content": text})]
        }
    }

    /// Convert messages to OpenAI format
    fn convert_messages_to_openai_format(&self, messages: &[Message]) -> Vec<serde_json::Value> {
        use serde_json::json;

        messages
            .iter()
            .map(|msg| {
                let role = self.convert_message_role(&msg.role);
                let content = self.convert_message_content(&msg.content);

                json!({
                    "role": role,
                    "content": content
                })
            })
            .collect()
    }

    /// Convert message role to string
    fn convert_message_role(&self, role: &MessageRole) -> &'static str {
        EnumConverter::message_role_to_string(role)
    }

    /// Convert message content to JSON value
    fn convert_message_content(
        &self,
        content: &crate::models::responses::MessageContentInput,
    ) -> serde_json::Value {
        use serde_json::json;

        match content {
            crate::models::responses::MessageContentInput::Text(text) => json!(text),
            crate::models::responses::MessageContentInput::Array(contents) => {
                json!(contents
                    .iter()
                    .map(|c| self.convert_message_content_item(c))
                    .collect::<Vec<_>>())
            }
        }
    }

    /// Convert message content item to JSON value
    fn convert_message_content_item(
        &self,
        content: &crate::models::responses::MessageContent,
    ) -> serde_json::Value {
        use serde_json::json;

        match content {
            crate::models::responses::MessageContent::Text { text } => json!({
                "type": "text",
                "text": text
            }),
            crate::models::responses::MessageContent::Image { image_url } => {
                self.convert_image_content(image_url)
            }
        }
    }

    /// Convert image content to JSON value
    fn convert_image_content(
        &self,
        image_url: &crate::models::responses::ImageUrl,
    ) -> serde_json::Value {
        use serde_json::json;

        let mut img = json!({
            "type": "image_url",
            "image_url": {
                "url": image_url.url
            }
        });

        if let Some(detail) = &image_url.detail {
            img["image_url"]["detail"] = json!(self.convert_image_detail(detail));
        }

        img
    }

    /// Convert image detail to string
    fn convert_image_detail(&self, detail: &crate::models::responses::ImageDetail) -> &'static str {
        EnumConverter::image_detail_to_string(detail)
    }

    /// Add optional parameters to request
    fn add_optional_parameters(
        &self,
        openai_request: &mut serde_json::Value,
        request: &ResponseRequest,
    ) {
        use serde_json::json;

        if let Some(temp) = request.temperature {
            openai_request["temperature"] = json!(temp);
        }
        if let Some(max_tokens) = request.max_tokens {
            openai_request["max_tokens"] = json!(max_tokens);
        }
        if let Some(top_p) = request.top_p {
            openai_request["top_p"] = json!(top_p);
        }
        if let Some(freq_penalty) = request.frequency_penalty {
            openai_request["frequency_penalty"] = json!(freq_penalty);
        }
        if let Some(pres_penalty) = request.presence_penalty {
            openai_request["presence_penalty"] = json!(pres_penalty);
        }
        if request.stream == Some(true) {
            openai_request["stream"] = json!(true);
        }
    }

    /// Add response format to request
    fn add_response_format(
        &self,
        openai_request: &mut serde_json::Value,
        response_format: Option<&crate::models::responses::ResponseFormat>,
    ) {
        use serde_json::json;

        if let Some(response_format) = response_format {
            match response_format {
                crate::models::responses::ResponseFormat::Text => {}
                crate::models::responses::ResponseFormat::JsonObject => {
                    openai_request["response_format"] = json!({
                        "type": "json_object"
                    });
                }
                crate::models::responses::ResponseFormat::JsonSchema {
                    json_schema,
                    strict,
                } => {
                    openai_request["response_format"] = json!({
                        "type": "json_schema",
                        "json_schema": {
                            "name": json_schema.name,
                            "description": json_schema.description,
                            "schema": json_schema.schema,
                            "strict": json_schema.strict || *strict
                        }
                    });
                }
            }
        }
    }

    /// Create a simple text response
    pub async fn create_text_response(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<String> {
        let request = ResponseRequest::new_text(model, prompt);
        let response = self.create_response(&request).await?;

        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone().unwrap_or_default())
        } else {
            Err(OpenAIError::invalid_request(
                "No choices returned in response",
            ))
        }
    }

    /// Create a response with instructions
    pub async fn create_instructed_response(
        &self,
        model: impl Into<String>,
        input: impl Into<String>,
        instructions: impl Into<String>,
    ) -> Result<String> {
        let request = ResponseRequest::new_text(model, input).with_instructions(instructions);
        let response = self.create_response(&request).await?;

        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone().unwrap_or_default())
        } else {
            Err(OpenAIError::invalid_request(
                "No choices returned in response",
            ))
        }
    }

    /// Create a response from messages
    pub async fn create_chat_response(
        &self,
        model: impl Into<String>,
        messages: Vec<Message>,
    ) -> Result<String> {
        let request = ResponseRequest::new_messages(model, messages);
        let response = self.create_response(&request).await?;

        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone().unwrap_or_default())
        } else {
            Err(OpenAIError::invalid_request(
                "No choices returned in response",
            ))
        }
    }

    /// Create a response with custom parameters
    pub async fn create_custom_response(
        &self,
        model: impl Into<String>,
        input: ResponseInput,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        instructions: Option<String>,
    ) -> Result<ResponseResult> {
        let mut request = match input {
            ResponseInput::Text(text) => ResponseRequest::new_text(model, text),
            ResponseInput::Messages(messages) => ResponseRequest::new_messages(model, messages),
        };

        if let Some(temp) = temperature {
            request = request.with_temperature(temp);
        }
        if let Some(tokens) = max_tokens {
            request = request.with_max_tokens(tokens);
        }
        if let Some(instr) = instructions {
            request = request.with_instructions(instr);
        }

        self.create_response(&request).await
    }

    /// Get the API key (for internal use)
    pub(crate) fn api_key(&self) -> &str {
        self.http_client.api_key()
    }

    /// Get the base URL (for internal use)
    pub(crate) fn base_url(&self) -> &str {
        self.http_client.base_url()
    }

    /// Get the HTTP client (for internal use)
    pub(crate) fn client(&self) -> &reqwest::Client {
        self.http_client.client()
    }

    /// Process structured response based on response format
    fn process_structured_response(
        &self,
        result: &mut ResponseResult,
        response_format: &crate::models::responses::ResponseFormat,
    ) -> Result<()> {
        use crate::models::responses::{ResponseFormat, SchemaValidationResult};

        for choice in &mut result.choices {
            if let Some(content) = &choice.message.content {
                match response_format {
                    ResponseFormat::JsonObject => {
                        // Parse JSON and validate it's an object
                        match serde_json::from_str::<serde_json::Value>(content) {
                            Ok(json_value) => {
                                if json_value.is_object() {
                                    choice.message.structured_data = Some(json_value);
                                    choice.message.schema_validation =
                                        Some(SchemaValidationResult {
                                            is_valid: true,
                                            errors: vec![],
                                            data: choice.message.structured_data.clone(),
                                        });
                                } else {
                                    choice.message.schema_validation =
                                        Some(SchemaValidationResult {
                                            is_valid: false,
                                            errors: vec![
                                                "Response is not a JSON object".to_string()
                                            ],
                                            data: Some(json_value),
                                        });
                                }
                            }
                            Err(e) => {
                                choice.message.schema_validation = Some(SchemaValidationResult {
                                    is_valid: false,
                                    errors: vec![format!("Invalid JSON: {}", e)],
                                    data: None,
                                });
                            }
                        }
                    }
                    ResponseFormat::JsonSchema { json_schema, .. } => {
                        // Parse JSON and validate against schema
                        match serde_json::from_str::<serde_json::Value>(content) {
                            Ok(json_value) => {
                                let validation_result = json_schema.validate(&json_value);
                                choice.message.structured_data = Some(json_value);
                                choice.message.schema_validation = Some(validation_result);
                            }
                            Err(e) => {
                                choice.message.schema_validation = Some(SchemaValidationResult {
                                    is_valid: false,
                                    errors: vec![format!("Invalid JSON: {}", e)],
                                    data: None,
                                });
                            }
                        }
                    }
                    ResponseFormat::Text => {
                        // No processing needed for text format
                    }
                }
            }
        }

        Ok(())
    }

    /// Create a structured response with schema validation
    pub async fn create_structured_response<T>(&self, request: &ResponseRequest) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let result = self.create_response(request).await?;

        if let Some(structured_data) = result.structured_data() {
            serde_json::from_value(structured_data.clone()).map_err(|e| {
                OpenAIError::invalid_request(format!("Failed to parse structured response: {e}"))
            })
        } else if result.has_valid_structured_data() {
            // Try parsing from content if no structured data but validation passed
            let content = result.output_text();
            serde_json::from_str(&content).map_err(|e| {
                OpenAIError::invalid_request(format!("Failed to parse response content: {e}"))
            })
        } else {
            Err(OpenAIError::invalid_request(
                "No valid structured data in response",
            ))
        }
    }

    /// Create a response with JSON object format
    pub async fn create_json_response(
        &self,
        model: impl Into<String>,
        input: crate::models::responses::ResponseInput,
    ) -> Result<serde_json::Value> {
        let request = match input {
            crate::models::responses::ResponseInput::Text(text) => {
                ResponseRequest::new_text(model, text).with_json_mode()
            }
            crate::models::responses::ResponseInput::Messages(messages) => {
                ResponseRequest::new_messages(model, messages).with_json_mode()
            }
        };

        let result = self.create_response(&request).await?;

        if let Some(structured_data) = result.structured_data() {
            Ok(structured_data.clone())
        } else {
            // Fallback: try parsing content as JSON
            let content = result.output_text();
            serde_json::from_str(&content).map_err(|e| {
                OpenAIError::invalid_request(format!("Response is not valid JSON: {e}"))
            })
        }
    }

    /// Create a response with schema validation
    pub async fn create_schema_validated_response(
        &self,
        model: impl Into<String>,
        input: crate::models::responses::ResponseInput,
        schema_name: impl Into<String>,
        schema: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let request = match input {
            crate::models::responses::ResponseInput::Text(text) => {
                ResponseRequest::new_text(model, text).with_json_schema(schema_name, schema)
            }
            crate::models::responses::ResponseInput::Messages(messages) => {
                ResponseRequest::new_messages(model, messages).with_json_schema(schema_name, schema)
            }
        };

        let result = self.create_response(&request).await?;

        if result.has_valid_structured_data() {
            if let Some(structured_data) = result.structured_data() {
                Ok(structured_data.clone())
            } else {
                Err(OpenAIError::invalid_request(
                    "No structured data in validated response",
                ))
            }
        } else {
            let errors = result.schema_validation_errors();
            Err(OpenAIError::invalid_request(format!(
                "Schema validation failed: {}",
                errors.join(", ")
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responses_api_creation() {
        let api = ResponsesApi::new("test-key").unwrap();
        assert_eq!(api.api_key(), "test-key");
        assert_eq!(api.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_responses_api_empty_key() {
        let result = ResponsesApi::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_request_builders() {
        let request = ResponseRequest::new_text("gpt-4", "Hello world")
            .with_instructions("Be helpful")
            .with_temperature(0.7)
            .with_max_tokens(100);

        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.instructions, Some("Be helpful".to_string()));
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.max_tokens, Some(100));
    }

    #[test]
    fn test_message_builders() {
        let user_msg = Message::user("Hello");
        assert_eq!(user_msg.role, MessageRole::User);
        assert_eq!(user_msg.text_content(), "Hello");

        let assistant_msg = Message::assistant("Hi there");
        assert_eq!(assistant_msg.role, MessageRole::Assistant);
        assert_eq!(assistant_msg.text_content(), "Hi there");

        let developer_msg = Message::developer("System prompt");
        assert_eq!(developer_msg.role, MessageRole::Developer);
        assert_eq!(developer_msg.text_content(), "System prompt");
    }
}
