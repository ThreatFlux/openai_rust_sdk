//! Shared utilities for API clients to reduce code duplication
//!
//! This module provides common patterns and utilities used across multiple API clients,
//! with a focus on eliminating code duplication while maintaining type safety and readability.

use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
use reqwest::multipart::{Form, Part};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// Trait for building multipart form requests with file uploads
pub trait MultipartFormBuilder {
    /// Add a file part to the form
    fn add_file_part(
        form: Form,
        field_name: String,
        file_data: Vec<u8>,
        filename: String,
    ) -> Result<Form> {
        let part = Part::bytes(file_data)
            .file_name(filename)
            .mime_str("audio/*")
            .map_err(|e| OpenAIError::InvalidRequest(format!("Invalid file: {e}")))?;

        Ok(form.part(field_name, part))
    }

    /// Add a text field to the form if the value is Some
    fn add_optional_text_field(form: Form, field_name: String, value: Option<&String>) -> Form {
        match value {
            Some(val) => form.text(field_name, val.clone()),
            None => form,
        }
    }

    /// Add a serializable field to the form if the value is Some
    fn add_optional_serializable_field<T>(
        form: Form,
        field_name: String,
        value: Option<&T>,
    ) -> Result<Form>
    where
        T: serde::Serialize,
    {
        match value {
            Some(val) => {
                let serialized = serde_json::to_string(val)
                    .map_err(|e| OpenAIError::ParseError(e.to_string()))?
                    .trim_matches('"')
                    .to_string();
                Ok(form.text(field_name, serialized))
            }
            None => Ok(form),
        }
    }

    /// Add temperature field if present
    fn add_temperature_field(form: Form, temperature: Option<f32>) -> Form {
        match temperature {
            Some(temp) => form.text("temperature", temp.to_string()),
            None => form,
        }
    }

    /// Add multiple serializable array fields (like timestamp_granularities)
    fn add_serializable_array_field<T>(
        form: Form,
        field_name: String,
        values: Option<&Vec<T>>,
    ) -> Result<Form>
    where
        T: serde::Serialize,
    {
        match values {
            Some(vals) => {
                let mut new_form = form;
                let array_field_name = format!("{field_name}[]");
                for val in vals {
                    let serialized = serde_json::to_string(val)
                        .map_err(|e| OpenAIError::ParseError(e.to_string()))?
                        .trim_matches('"')
                        .to_string();
                    new_form = new_form.text(array_field_name.clone(), serialized);
                }
                Ok(new_form)
            }
            None => Ok(form),
        }
    }
}

/// Helper struct for building multipart forms
pub struct FormBuilder;

impl MultipartFormBuilder for FormBuilder {}

impl FormBuilder {
    /// Create a new base form with file and model
    pub fn create_base_audio_form(
        file_data: Vec<u8>,
        filename: String,
        model: String,
    ) -> Result<Form> {
        let form = Form::new().text("model", model);
        Self::add_file_part(form, "file".to_string(), file_data, filename)
    }

    /// Create a new base form for image API with model
    pub fn create_base_image_form(model: String) -> Form {
        Form::new().text("model", model)
    }

    /// Add an image part to form with specific field name and MIME type
    pub fn add_image_part(
        form: Form,
        field_name: String,
        image_data: Vec<u8>,
        filename: String,
        mime_type: &str,
    ) -> Result<Form> {
        let part = Part::bytes(image_data)
            .file_name(filename)
            .mime_str(mime_type)
            .map_err(|e| OpenAIError::InvalidRequest(format!("Invalid image: {e}")))?;

        Ok(form.part(field_name, part))
    }

    /// Add image with PNG MIME type (common case)
    pub fn add_png_image_part(
        form: Form,
        field_name: String,
        image_data: Vec<u8>,
        filename: String,
    ) -> Result<Form> {
        Self::add_image_part(form, field_name, image_data, filename, "image/png")
    }

    /// Add an optional numeric field to form
    pub fn add_optional_numeric_field<T: std::fmt::Display>(
        form: Form,
        field_name: String,
        value: Option<T>,
    ) -> Form {
        match value {
            Some(val) => form.text(field_name, val.to_string()),
            None => form,
        }
    }

    /// Build form for audio transcription with all fields
    pub fn build_transcription_form(
        file_data: Vec<u8>,
        filename: String,
        model: String,
        language: Option<&String>,
        prompt: Option<&String>,
        response_format: Option<&impl serde::Serialize>,
        temperature: Option<f32>,
        timestamp_granularities: Option<&Vec<impl serde::Serialize>>,
    ) -> Result<Form> {
        let mut form = Self::create_base_audio_form(file_data, filename, model)?;

        form = Self::add_optional_text_field(form, "language".to_string(), language);
        form = Self::add_optional_text_field(form, "prompt".to_string(), prompt);
        form = Self::add_optional_serializable_field(
            form,
            "response_format".to_string(),
            response_format,
        )?;
        form = Self::add_temperature_field(form, temperature);
        form = Self::add_serializable_array_field(
            form,
            "timestamp_granularities".to_string(),
            timestamp_granularities,
        )?;

        Ok(form)
    }

    /// Build form for audio translation with all fields
    pub fn build_translation_form(
        file_data: Vec<u8>,
        filename: String,
        model: String,
        prompt: Option<&String>,
        response_format: Option<&impl serde::Serialize>,
        temperature: Option<f32>,
    ) -> Result<Form> {
        let mut form = Self::create_base_audio_form(file_data, filename, model)?;

        form = Self::add_optional_text_field(form, "prompt".to_string(), prompt);
        form = Self::add_optional_serializable_field(
            form,
            "response_format".to_string(),
            response_format,
        )?;
        form = Self::add_temperature_field(form, temperature);

        Ok(form)
    }
}

/// Utility for executing multipart HTTP requests to OpenAI API
pub struct MultipartRequestExecutor;

impl MultipartRequestExecutor {
    /// Send a multipart request to the specified endpoint
    pub async fn send_multipart_request(
        http_client: &HttpClient,
        endpoint: &str,
        form: Form,
    ) -> Result<reqwest::Response> {
        let url = format!("{}{}", http_client.base_url(), endpoint);
        let headers = http_client.build_auth_headers()?;

        let response = http_client
            .client()
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        Ok(response)
    }
}

/// Utility for handling audio API responses
pub struct AudioResponseHandler;

impl AudioResponseHandler {
    /// Handle response that can be either JSON or plain text
    pub async fn handle_flexible_response<T>(
        response: reqwest::Response,
        fallback_parser: impl FnOnce(String) -> T,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/json");

        if content_type.contains("application/json") {
            response
                .json::<T>()
                .await
                .map_err(|e| OpenAIError::ParseError(e.to_string()))
        } else {
            let text = response
                .text()
                .await
                .map_err(|e| OpenAIError::ParseError(e.to_string()))?;
            Ok(fallback_parser(text))
        }
    }
}

/// Common request validation utilities
pub struct RequestValidator;

/// Utilities for converting enum values to strings
pub struct EnumConverter;

impl EnumConverter {
    /// Convert message role to string representation
    pub fn message_role_to_string(role: &crate::models::responses::MessageRole) -> &'static str {
        use crate::models::responses::MessageRole;
        match role {
            MessageRole::Developer => "system",
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
            MessageRole::System => "system",
        }
    }

    /// Convert image detail to string representation
    pub fn image_detail_to_string(detail: &crate::models::responses::ImageDetail) -> &'static str {
        use crate::models::responses::ImageDetail;
        match detail {
            ImageDetail::Auto => "auto",
            ImageDetail::Low => "low",
            ImageDetail::High => "high",
        }
    }
}

impl RequestValidator {
    /// Validate that a file is not empty
    pub fn validate_file_not_empty(file_data: &[u8]) -> Result<()> {
        if file_data.is_empty() {
            return Err(OpenAIError::InvalidRequest(
                "File data cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate that required fields are present
    pub fn validate_required_string(value: &str, field_name: &str) -> Result<()> {
        if value.trim().is_empty() {
            return Err(OpenAIError::InvalidRequest(format!(
                "{field_name} cannot be empty"
            )));
        }
        Ok(())
    }

    /// Validate temperature range
    pub fn validate_temperature(temperature: Option<f32>) -> Result<()> {
        if let Some(temp) = temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err(OpenAIError::InvalidRequest(
                    "Temperature must be between 0.0 and 2.0".to_string(),
                ));
            }
        }
        Ok(())
    }
}

/// Configuration for common API parameters
#[derive(Debug, Clone, Default)]
pub struct ApiRequestConfig {
    /// Temperature parameter (0.0 - 2.0)
    pub temperature: Option<f32>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Custom timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Additional headers to include
    pub extra_headers: HashMap<String, String>,
}

impl ApiRequestConfig {
    /// Create a new API request configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the temperature parameter
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the maximum tokens parameter
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set the request timeout
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    /// Add a custom header to the request
    pub fn with_header<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.extra_headers.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_validator() {
        assert!(RequestValidator::validate_file_not_empty(&[1, 2, 3]).is_ok());
        assert!(RequestValidator::validate_file_not_empty(&[]).is_err());

        assert!(RequestValidator::validate_required_string("test", "field").is_ok());
        assert!(RequestValidator::validate_required_string("", "field").is_err());
        assert!(RequestValidator::validate_required_string("   ", "field").is_err());

        assert!(RequestValidator::validate_temperature(Some(0.5)).is_ok());
        assert!(RequestValidator::validate_temperature(Some(2.0)).is_ok());
        assert!(RequestValidator::validate_temperature(Some(2.1)).is_err());
        assert!(RequestValidator::validate_temperature(Some(-0.1)).is_err());
        assert!(RequestValidator::validate_temperature(None).is_ok());
    }

    #[test]
    fn test_form_builder_base() {
        let form = FormBuilder::create_base_audio_form(
            vec![1, 2, 3],
            "test.mp3".to_string(),
            "whisper-1".to_string(),
        );
        assert!(form.is_ok());
    }

    #[test]
    fn test_api_request_config() {
        let config = ApiRequestConfig::new()
            .with_temperature(0.7)
            .with_max_tokens(100)
            .with_timeout(30)
            .with_header("Custom-Header", "value");

        assert_eq!(config.temperature, Some(0.7));
        assert_eq!(config.max_tokens, Some(100));
        assert_eq!(config.timeout_seconds, Some(30));
        assert_eq!(
            config.extra_headers.get("Custom-Header"),
            Some(&"value".to_string())
        );
    }
}
