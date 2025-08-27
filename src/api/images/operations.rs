//! Core image operations and API methods

use super::{ImageFormBuilder, ImageRequestBuilder, ImagesApi};
use crate::error::{OpenAIError, Result};
use crate::models::images::{
    ImageEditRequest, ImageGenerationRequest, ImageModels, ImageQuality, ImageResponse,
    ImageResponseFormat, ImageSize, ImageVariationRequest,
};
use std::path::Path;
use tokio::fs;

impl ImagesApi {
    /// Generate images from text prompts
    pub async fn create_image(&self, request: &ImageGenerationRequest) -> Result<ImageResponse> {
        // Validate request before sending
        request.validate().map_err(OpenAIError::invalid_request)?;

        self.http_client
            .post("/v1/images/generations", request)
            .await
    }

    /// Edit images with masks
    pub async fn create_image_edit(
        &self,
        request: &ImageEditRequest,
        image_data: Vec<u8>,
        mask_data: Option<Vec<u8>>,
    ) -> Result<ImageResponse> {
        // Create multipart form
        let mut form = ImageFormBuilder::create_image_multipart_form(
            "image",
            image_data,
            &request.image,
            &request.model,
        )?
        .text("prompt", request.prompt.clone());

        // Add mask if provided
        if let (Some(mask_bytes), Some(mask_name)) = (mask_data, &request.mask) {
            form = ImageFormBuilder::add_image_part(form, "mask", mask_bytes, mask_name)?;
        }

        form = ImageFormBuilder::add_optional_form_fields(
            form,
            request.n,
            request.response_format.as_ref(),
            request.size.as_ref(),
            request.user.as_ref(),
        )?;

        self.http_client
            .post_multipart("/v1/images/edits", form)
            .await
    }

    /// Create variations of images
    pub async fn create_image_variation(
        &self,
        request: &ImageVariationRequest,
        image_data: Vec<u8>,
    ) -> Result<ImageResponse> {
        // Create multipart form
        let mut form = ImageFormBuilder::create_image_multipart_form(
            "image",
            image_data,
            &request.image,
            &request.model,
        )?;

        form = ImageFormBuilder::add_optional_form_fields(
            form,
            request.n,
            request.response_format.as_ref(),
            request.size.as_ref(),
            request.user.as_ref(),
        )?;

        self.http_client
            .post_multipart("/v1/images/variations", form)
            .await
    }

    // Convenience methods with file support

    /// Edit image from file paths
    pub async fn edit_image_from_files(
        &self,
        image_path: impl AsRef<Path>,
        mask_path: Option<impl AsRef<Path>>,
        request: &ImageEditRequest,
    ) -> Result<ImageResponse> {
        let image_data = crate::helpers::read_bytes(image_path).await?;

        let mask_data = if let Some(mask_path) = mask_path {
            Some(crate::helpers::read_bytes(mask_path).await?)
        } else {
            None
        };

        self.create_image_edit(request, image_data, mask_data).await
    }

    /// Create variation from file path
    pub async fn create_variation_from_file(
        &self,
        image_path: impl AsRef<Path>,
        request: &ImageVariationRequest,
    ) -> Result<ImageResponse> {
        let image_data = crate::helpers::read_bytes(image_path).await?;

        self.create_image_variation(request, image_data).await
    }

    // High-level convenience methods

    /// Generate a single image with simple parameters
    pub async fn generate_image(
        &self,
        prompt: impl Into<String>,
        model: Option<&str>,
        size: Option<ImageSize>,
        quality: Option<ImageQuality>,
    ) -> Result<ImageResponse> {
        let model = model.unwrap_or(ImageModels::DALL_E_3);
        let request = ImageRequestBuilder::build_generation_request(model, prompt, size, quality);
        self.create_image(&request).await
    }

    /// Generate images and save to files
    pub async fn generate_and_save_images(
        &self,
        prompt: impl Into<String>,
        output_dir: impl AsRef<Path>,
        model: Option<&str>,
        size: Option<ImageSize>,
        quality: Option<ImageQuality>,
        count: Option<u32>,
    ) -> Result<Vec<String>> {
        let model = model.unwrap_or(ImageModels::DALL_E_2); // Use DALL-E 2 for multiple images
        let mut request =
            ImageRequestBuilder::build_generation_request(model, prompt, size, quality)
                .with_response_format(ImageResponseFormat::B64Json);

        if let Some(count) = count {
            request = request.with_n(count);
        }

        let response = self.create_image(&request).await?;
        let output_dir = output_dir.as_ref();

        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir).await.map_err(|e| {
            OpenAIError::FileError(format!("Failed to create output directory: {e}"))
        })?;

        let mut saved_files = Vec::new();

        for (i, image_data) in response.data.iter().enumerate() {
            if let Some(b64_data) = &image_data.b64_json {
                let filename = format!("generated_image_{}.png", i + 1);
                let file_path = output_dir.join(&filename);

                use base64::{engine::general_purpose, Engine as _};
                let decoded_data = general_purpose::STANDARD.decode(b64_data).map_err(|e| {
                    OpenAIError::ParseError(format!("Failed to decode base64: {e}"))
                })?;

                crate::helpers::write_bytes(&file_path, &decoded_data).await?;

                saved_files.push(file_path.to_string_lossy().to_string());
            }
        }

        Ok(saved_files)
    }

    /// Download image from URL and save to file
    pub async fn download_image(&self, url: &str, output_path: impl AsRef<Path>) -> Result<()> {
        let response = self
            .http_client
            .client()
            .get(url)
            .send()
            .await
            .map_err(crate::request_err!("Failed to download image: {}"))?;

        if !response.status().is_success() {
            return Err(OpenAIError::RequestError(format!(
                "Failed to download image: HTTP {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(crate::request_err!("Failed to read image data: {}"))?;

        crate::helpers::write_bytes(output_path, &bytes).await?;

        Ok(())
    }

    /// Quick edit with simple parameters
    pub async fn edit_image(
        &self,
        image_path: impl AsRef<Path>,
        prompt: impl Into<String>,
        mask_path: Option<impl AsRef<Path>>,
        model: Option<&str>,
    ) -> Result<ImageResponse> {
        let model = model.unwrap_or(ImageModels::DALL_E_2);
        let image_filename = ImageRequestBuilder::extract_filename(&image_path, "image.png");

        let mut request = ImageEditRequest::new(model, image_filename, prompt);

        if let Some(mask_path) = &mask_path {
            let mask_filename = ImageRequestBuilder::extract_filename(mask_path, "mask.png");
            request = request.with_mask(mask_filename);
        }

        self.edit_image_from_files(image_path, mask_path, &request)
            .await
    }

    /// Quick variation with simple parameters
    pub async fn create_variation(
        &self,
        image_path: impl AsRef<Path>,
        count: Option<u32>,
        size: Option<ImageSize>,
        model: Option<&str>,
    ) -> Result<ImageResponse> {
        let model = model.unwrap_or(ImageModels::DALL_E_2);
        let image_filename = ImageRequestBuilder::extract_filename(&image_path, "image.png");

        let mut request = ImageVariationRequest::new(model, image_filename);

        if let Some(count) = count {
            request = request.with_n(count);
        }

        if let Some(size) = size {
            request = request.with_size(size);
        }

        self.create_variation_from_file(image_path, &request).await
    }
}
