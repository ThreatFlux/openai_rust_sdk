//! Image request builders and form creation utilities

use crate::api::shared_utilities::{FormBuilder, MultipartFormBuilder, RequestValidator};
use crate::error::{OpenAIError, Result};
use crate::models::images::{
    ImageEditRequest, ImageGenerationRequest, ImageQuality, ImageResponseFormat, ImageSize,
    ImageVariationRequest,
};
use reqwest::multipart;
use std::path::Path;

/// Image request building utilities
pub struct ImageRequestBuilder;

impl ImageRequestBuilder {
    /// Build an image generation request with optional parameters
    pub fn build_generation_request(
        model: &str,
        prompt: impl Into<String>,
        size: Option<ImageSize>,
        quality: Option<ImageQuality>,
    ) -> ImageGenerationRequest {
        let mut request = ImageGenerationRequest::new(model, prompt);

        if let Some(size) = size {
            request = request.with_size(size);
        }

        if let Some(quality) = quality {
            request = request.with_quality(quality);
        }

        request
    }

    /// Extract filename from path with default fallback
    pub fn extract_filename(path: impl AsRef<Path>, default: &str) -> String {
        path.as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(default)
            .to_string()
    }
}

/// Form building utilities for image API requests
pub struct ImageFormBuilder;

impl ImageFormBuilder {
    /// Create a multipart form with an image part using shared utilities
    pub fn create_image_multipart_form(
        field_name: &str,
        image_data: Vec<u8>,
        file_name: &str,
        model: &str,
    ) -> Result<multipart::Form> {
        // Validate inputs
        RequestValidator::validate_file_not_empty(&image_data)?;
        RequestValidator::validate_required_string(model, "model")?;

        let form = FormBuilder::create_base_image_form(model.to_string());
        FormBuilder::add_png_image_part(
            form,
            field_name.to_string(),
            image_data,
            file_name.to_string(),
        )
    }

    /// Add an image part to an existing form using shared utilities
    pub fn add_image_part(
        form: multipart::Form,
        field_name: &str,
        image_data: Vec<u8>,
        file_name: &str,
    ) -> Result<multipart::Form> {
        RequestValidator::validate_file_not_empty(&image_data)?;
        FormBuilder::add_png_image_part(
            form,
            field_name.to_string(),
            image_data,
            file_name.to_string(),
        )
    }

    /// Helper to add optional form fields using shared utilities
    pub fn add_optional_form_fields(
        mut form: multipart::Form,
        n: Option<u32>,
        response_format: Option<&ImageResponseFormat>,
        size: Option<&ImageSize>,
        user: Option<&String>,
    ) -> Result<multipart::Form> {
        form = FormBuilder::add_optional_numeric_field(form, "n".to_string(), n);

        // Convert borrowed options to owned options for serialization
        let response_format_owned = response_format.cloned();
        form = FormBuilder::add_optional_serializable_field(
            form,
            "response_format".to_string(),
            response_format_owned.as_ref(),
        )?;

        let size_owned = size.cloned();
        form = FormBuilder::add_optional_serializable_field(
            form,
            "size".to_string(),
            size_owned.as_ref(),
        )?;

        let user_owned = user.cloned();
        form = FormBuilder::add_optional_text_field(form, "user".to_string(), user_owned.as_ref());

        Ok(form)
    }
}
