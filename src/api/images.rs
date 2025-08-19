//! # Images API
//!
//! This module provides access to OpenAI's Images API for generating,
//! editing, and creating variations of images using DALL-E models.

use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
#[cfg(test)]
use crate::models::images::{ImageEditBuilder, ImageGenerationBuilder, ImageVariationBuilder};
use crate::models::images::{
    ImageEditRequest, ImageGenerationRequest, ImageModels, ImageQuality, ImageResponse,
    ImageResponseFormat, ImageSize, ImageStyle, ImageVariationRequest,
};
use reqwest::multipart;
use std::path::Path;
use tokio::fs;

/// Images API client
pub struct ImagesApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl crate::api::common::ApiClientConstructors for ImagesApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl ImagesApi {
    /// Create a new Images API client
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        <Self as crate::api::common::ApiClientConstructors>::new(api_key)
    }

    /// Create a new Images API client with custom base URL
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        <Self as crate::api::common::ApiClientConstructors>::new_with_base_url(api_key, base_url)
    }
    /// Helper to add optional form fields
    fn add_optional_form_fields(
        mut form: multipart::Form,
        n: Option<u32>,
        response_format: Option<&ImageResponseFormat>,
        size: Option<&ImageSize>,
        user: Option<&String>,
    ) -> Result<multipart::Form> {
        if let Some(n) = n {
            form = form.text("n", n.to_string());
        }

        if let Some(format) = response_format {
            form = form.text(
                "response_format",
                serde_json::to_string(format)
                    .map_err(|e| OpenAIError::ParseError(e.to_string()))?
                    .trim_matches('"')
                    .to_string(),
            );
        }

        if let Some(size) = size {
            form = form.text(
                "size",
                serde_json::to_string(size)
                    .map_err(|e| OpenAIError::ParseError(e.to_string()))?
                    .trim_matches('"')
                    .to_string(),
            );
        }

        if let Some(user) = user {
            form = form.text("user", user.clone());
        }

        Ok(form)
    }
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
        let mut form = multipart::Form::new()
            .part(
                "image",
                multipart::Part::bytes(image_data)
                    .file_name(request.image.clone())
                    .mime_str("image/png")
                    .map_err(|e| OpenAIError::InvalidRequest(format!("Invalid image: {e}")))?,
            )
            .text("model", request.model.clone())
            .text("prompt", request.prompt.clone());

        // Add mask if provided
        if let (Some(mask_bytes), Some(mask_name)) = (mask_data, &request.mask) {
            form = form.part(
                "mask",
                multipart::Part::bytes(mask_bytes)
                    .file_name(mask_name.clone())
                    .mime_str("image/png")
                    .map_err(|e| OpenAIError::InvalidRequest(format!("Invalid mask: {e}")))?,
            );
        }

        form = Self::add_optional_form_fields(
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
        let mut form = multipart::Form::new()
            .part(
                "image",
                multipart::Part::bytes(image_data)
                    .file_name(request.image.clone())
                    .mime_str("image/png")
                    .map_err(|e| OpenAIError::InvalidRequest(format!("Invalid image: {e}")))?,
            )
            .text("model", request.model.clone());

        form = Self::add_optional_form_fields(
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
        let image_data = fs::read(image_path)
            .await
            .map_err(|e| OpenAIError::FileError(format!("Failed to read image file: {e}")))?;

        let mask_data =
            if let Some(mask_path) = mask_path {
                Some(fs::read(mask_path).await.map_err(|e| {
                    OpenAIError::FileError(format!("Failed to read mask file: {e}"))
                })?)
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
        let image_data = fs::read(image_path)
            .await
            .map_err(|e| OpenAIError::FileError(format!("Failed to read image file: {e}")))?;

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
        let mut request = ImageGenerationRequest::new(model, prompt);

        if let Some(size) = size {
            request = request.with_size(size);
        }

        if let Some(quality) = quality {
            request = request.with_quality(quality);
        }

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
        let mut request = ImageGenerationRequest::new(model, prompt)
            .with_response_format(ImageResponseFormat::B64Json);

        if let Some(size) = size {
            request = request.with_size(size);
        }

        if let Some(quality) = quality {
            request = request.with_quality(quality);
        }

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

                fs::write(&file_path, decoded_data)
                    .await
                    .map_err(|e| OpenAIError::FileError(format!("Failed to save image: {e}")))?;

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
            .map_err(|e| OpenAIError::RequestError(format!("Failed to download image: {e}")))?;

        if !response.status().is_success() {
            return Err(OpenAIError::RequestError(format!(
                "Failed to download image: HTTP {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| OpenAIError::RequestError(format!("Failed to read image data: {e}")))?;

        fs::write(output_path, bytes)
            .await
            .map_err(|e| OpenAIError::FileError(format!("Failed to save image: {e}")))?;

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
        let image_filename = image_path
            .as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image.png")
            .to_string();

        let mut request = ImageEditRequest::new(model, image_filename, prompt);

        if let Some(mask_path) = &mask_path {
            let mask_filename = mask_path
                .as_ref()
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("mask.png")
                .to_string();
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
        let image_filename = image_path
            .as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image.png")
            .to_string();

        let mut request = ImageVariationRequest::new(model, image_filename);

        if let Some(count) = count {
            request = request.with_n(count);
        }

        if let Some(size) = size {
            request = request.with_size(size);
        }

        self.create_variation_from_file(image_path, &request).await
    }

    // Utility methods

    /// Get supported image formats for input
    #[must_use]
    pub fn supported_input_formats() -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg", "webp"]
    }

    /// Validate image file format
    pub fn is_supported_format(file_path: impl AsRef<Path>) -> bool {
        if let Some(extension) = file_path.as_ref().extension() {
            if let Some(ext_str) = extension.to_str() {
                return Self::supported_input_formats().contains(&ext_str.to_lowercase().as_str());
            }
        }
        false
    }

    /// Get recommended settings for different use cases
    #[must_use]
    pub fn recommend_settings(
        use_case: &str,
    ) -> (String, ImageSize, Option<ImageQuality>, Option<ImageStyle>) {
        match use_case.to_lowercase().as_str() {
            "avatar" | "profile" => (
                ImageModels::DALL_E_3.to_string(),
                ImageSize::Size1024x1024,
                Some(ImageQuality::Hd),
                Some(ImageStyle::Natural),
            ),
            "wallpaper" | "background" => (
                ImageModels::DALL_E_3.to_string(),
                ImageSize::Size1792x1024,
                Some(ImageQuality::Hd),
                Some(ImageStyle::Vivid),
            ),
            "poster" | "vertical" => (
                ImageModels::DALL_E_3.to_string(),
                ImageSize::Size1024x1792,
                Some(ImageQuality::Hd),
                Some(ImageStyle::Vivid),
            ),
            "thumbnail" | "icon" => (
                ImageModels::DALL_E_2.to_string(),
                ImageSize::Size256x256,
                None,
                None,
            ),
            "concept" | "draft" => (
                ImageModels::DALL_E_2.to_string(),
                ImageSize::Size512x512,
                None,
                None,
            ),
            _ => (
                ImageModels::DALL_E_3.to_string(),
                ImageSize::Size1024x1024,
                Some(ImageQuality::Standard),
                Some(ImageStyle::Natural),
            ),
        }
    }

    /// Estimate cost for image generation
    /// Based on `OpenAI` pricing (as of 2024)
    #[must_use]
    pub fn estimate_cost(
        model: &str,
        size: &ImageSize,
        quality: Option<&ImageQuality>,
        count: u32,
    ) -> f64 {
        let base_cost = match model {
            "dall-e-3" => {
                match size {
                    ImageSize::Size1024x1024 => {
                        match quality {
                            Some(ImageQuality::Hd) => 0.080, // $0.080 per image
                            _ => 0.040,                      // $0.040 per image (standard)
                        }
                    }
                    ImageSize::Size1792x1024 | ImageSize::Size1024x1792 => {
                        match quality {
                            Some(ImageQuality::Hd) => 0.120, // $0.120 per image
                            _ => 0.080,                      // $0.080 per image (standard)
                        }
                    }
                    _ => 0.040, // Default to standard 1024x1024
                }
            }
            "dall-e-2" => {
                match size {
                    ImageSize::Size1024x1024 => 0.020, // $0.020 per image
                    ImageSize::Size512x512 => 0.018,   // $0.018 per image
                    ImageSize::Size256x256 => 0.016,   // $0.016 per image
                    _ => 0.020,                        // Default to 1024x1024
                }
            }
            _ => 0.040, // Default to DALL-E 3 standard pricing
        };

        base_cost * f64::from(count)
    }

    /// Get the API key (for testing purposes)
    #[cfg(test)]
    pub fn api_key(&self) -> &str {
        self.http_client.api_key()
    }
}

/// Image utilities
pub struct ImageUtils;

impl ImageUtils {
    /// Check if an image is square (required for edits/variations)
    pub async fn is_square_image(image_path: impl AsRef<Path>) -> Result<bool> {
        // This is a basic implementation that would need an image processing library
        // For now, we'll just assume PNG files with "square" in the name are square
        let filename = image_path
            .as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        Ok(filename.contains("square") || filename.contains("1024x1024"))
    }

    /// Estimate image file size in bytes
    #[must_use]
    pub fn estimate_file_size(size: &ImageSize, format: &str) -> u64 {
        let pixel_count = match size {
            ImageSize::Size256x256 => 256 * 256,
            ImageSize::Size512x512 => 512 * 512,
            ImageSize::Size1024x1024 => 1024 * 1024,
            ImageSize::Size1792x1024 => 1792 * 1024,
            ImageSize::Size1024x1792 => 1024 * 1792,
        };

        match format.to_lowercase().as_str() {
            "png" => pixel_count * 3, // Rough estimate: 3 bytes per pixel for PNG
            "jpg" | "jpeg" => pixel_count / 10, // JPEG compression ratio ~10:1
            "webp" => pixel_count / 15, // WebP compression ratio ~15:1
            _ => pixel_count * 3,     // Default to PNG estimate
        }
    }

    /// Generate prompt suggestions for better results
    #[must_use]
    pub fn enhance_prompt(basic_prompt: &str, style_hints: Option<&str>) -> String {
        let mut enhanced = basic_prompt.to_string();

        if let Some(hints) = style_hints {
            enhanced = format!("{enhanced}, {hints}");
        }

        // Add common quality enhancers
        if !enhanced.contains("high quality") && !enhanced.contains("detailed") {
            enhanced = format!("{enhanced}, high quality, detailed");
        }

        enhanced
    }

    /// Validate image requirements for different operations
    pub fn validate_image_requirements(
        operation: &str,
        image_path: impl AsRef<Path>,
    ) -> Result<()> {
        let path = image_path.as_ref();

        // Check file extension
        if !ImagesApi::is_supported_format(path) {
            return Err(OpenAIError::invalid_request(
                "Image must be in PNG, JPG, JPEG, or WebP format",
            ));
        }

        // Check file name for operation-specific requirements
        match operation {
            "edit" | "variation" => {
                // These operations require PNG format and square images
                if let Some(ext) = path.extension() {
                    if ext.to_str().unwrap_or("").to_lowercase() != "png" {
                        return Err(OpenAIError::invalid_request(
                            "Image edits and variations require PNG format",
                        ));
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_images_api_creation() {
        let api = ImagesApi::new("test-key".to_string()).unwrap();
        assert_eq!(api.api_key(), "test-key");
    }

    #[test]
    fn test_empty_api_key() {
        let result = ImagesApi::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_supported_formats() {
        let formats = ImagesApi::supported_input_formats();
        assert!(formats.contains(&"png"));
        assert!(formats.contains(&"jpg"));
        assert!(formats.contains(&"jpeg"));
        assert!(formats.contains(&"webp"));
    }

    #[test]
    fn test_format_validation() {
        assert!(ImagesApi::is_supported_format("test.png"));
        assert!(ImagesApi::is_supported_format("test.jpg"));
        assert!(ImagesApi::is_supported_format("test.jpeg"));
        assert!(ImagesApi::is_supported_format("test.webp"));
        assert!(!ImagesApi::is_supported_format("test.gif"));
        assert!(!ImagesApi::is_supported_format("test.txt"));
    }

    #[test]
    fn test_recommend_settings() {
        let (model, size, quality, style) = ImagesApi::recommend_settings("avatar");
        assert_eq!(model, ImageModels::DALL_E_3);
        assert_eq!(size, ImageSize::Size1024x1024);
        assert_eq!(quality, Some(ImageQuality::Hd));
        assert_eq!(style, Some(ImageStyle::Natural));

        let (model, size, quality, style) = ImagesApi::recommend_settings("wallpaper");
        assert_eq!(model, ImageModels::DALL_E_3);
        assert_eq!(size, ImageSize::Size1792x1024);
        assert_eq!(quality, Some(ImageQuality::Hd));
        assert_eq!(style, Some(ImageStyle::Vivid));

        let (model, size, quality, style) = ImagesApi::recommend_settings("thumbnail");
        assert_eq!(model, ImageModels::DALL_E_2);
        assert_eq!(size, ImageSize::Size256x256);
        assert_eq!(quality, None);
        assert_eq!(style, None);
    }

    #[test]
    fn test_cost_estimation() {
        // DALL-E 3 costs
        let cost = ImagesApi::estimate_cost(
            "dall-e-3",
            &ImageSize::Size1024x1024,
            Some(&ImageQuality::Hd),
            1,
        );
        assert_eq!(cost, 0.080);

        let cost = ImagesApi::estimate_cost(
            "dall-e-3",
            &ImageSize::Size1024x1024,
            Some(&ImageQuality::Standard),
            1,
        );
        assert_eq!(cost, 0.040);

        let cost = ImagesApi::estimate_cost(
            "dall-e-3",
            &ImageSize::Size1792x1024,
            Some(&ImageQuality::Hd),
            1,
        );
        assert_eq!(cost, 0.120);

        // DALL-E 2 costs
        let cost = ImagesApi::estimate_cost("dall-e-2", &ImageSize::Size1024x1024, None, 1);
        assert_eq!(cost, 0.020);

        let cost = ImagesApi::estimate_cost("dall-e-2", &ImageSize::Size512x512, None, 1);
        assert_eq!(cost, 0.018);

        let cost = ImagesApi::estimate_cost("dall-e-2", &ImageSize::Size256x256, None, 5);
        assert_eq!(cost, 0.080); // 0.016 * 5
    }

    #[test]
    fn test_file_size_estimation() {
        let size = ImageUtils::estimate_file_size(&ImageSize::Size1024x1024, "png");
        assert!(size > 0);

        let jpg_size = ImageUtils::estimate_file_size(&ImageSize::Size1024x1024, "jpg");
        assert!(jpg_size < size); // JPEG should be smaller than PNG

        let webp_size = ImageUtils::estimate_file_size(&ImageSize::Size1024x1024, "webp");
        assert!(webp_size < jpg_size); // WebP should be smaller than JPEG
    }

    #[test]
    fn test_prompt_enhancement() {
        let basic = "A cat";
        let enhanced = ImageUtils::enhance_prompt(basic, Some("realistic, furry"));
        assert!(enhanced.contains("A cat"));
        assert!(enhanced.contains("realistic, furry"));
        assert!(enhanced.contains("high quality"));

        let already_detailed = "A cat, high quality, detailed";
        let enhanced2 = ImageUtils::enhance_prompt(already_detailed, None);
        // Should not add "high quality, detailed" again
        assert_eq!(enhanced2, already_detailed);

        // Test with "detailed" in style hints
        let enhanced3 = ImageUtils::enhance_prompt("A dog", Some("realistic, detailed fur"));
        assert!(enhanced3.contains("A dog"));
        assert!(enhanced3.contains("realistic, detailed fur"));
        // Should not add quality enhancers because "detailed" is already present
        assert!(!enhanced3.contains("high quality"));
    }

    #[test]
    fn test_image_requirements_validation() {
        let png_path = PathBuf::from("test.png");
        let jpg_path = PathBuf::from("test.jpg");
        let gif_path = PathBuf::from("test.gif");

        // Generation should accept any supported format
        assert!(ImageUtils::validate_image_requirements("generation", &png_path).is_ok());
        assert!(ImageUtils::validate_image_requirements("generation", &jpg_path).is_ok());

        // Edit requires PNG
        assert!(ImageUtils::validate_image_requirements("edit", &png_path).is_ok());
        assert!(ImageUtils::validate_image_requirements("edit", &jpg_path).is_err());

        // Variation requires PNG
        assert!(ImageUtils::validate_image_requirements("variation", &png_path).is_ok());
        assert!(ImageUtils::validate_image_requirements("variation", &jpg_path).is_err());

        // Unsupported format should fail
        assert!(ImageUtils::validate_image_requirements("generation", &gif_path).is_err());
    }

    #[tokio::test]
    async fn test_square_image_detection() {
        let square_path = PathBuf::from("square_image.png");
        let normal_path = PathBuf::from("normal_image.png");
        let size_path = PathBuf::from("image_1024x1024.png");

        // This is a mock implementation - in reality you'd use an image processing library
        assert!(ImageUtils::is_square_image(&square_path).await.unwrap());
        assert!(!ImageUtils::is_square_image(&normal_path).await.unwrap());
        assert!(ImageUtils::is_square_image(&size_path).await.unwrap());
    }

    #[tokio::test]
    async fn test_request_serialization() {
        let request = ImageGenerationRequest::new("dall-e-3", "A beautiful landscape")
            .with_quality(ImageQuality::Hd)
            .with_size(ImageSize::Size1024x1024)
            .with_style(ImageStyle::Natural)
            .with_response_format(ImageResponseFormat::Url);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"dall-e-3\""));
        assert!(json.contains("\"prompt\":\"A beautiful landscape\""));
        assert!(json.contains("\"quality\":\"hd\""));
        assert!(json.contains("\"size\":\"1024x1024\""));
        assert!(json.contains("\"style\":\"natural\""));
        assert!(json.contains("\"response_format\":\"url\""));
    }

    #[tokio::test]
    async fn test_response_parsing() {
        let json = r#"{
            "created": 1234567890,
            "data": [
                {
                    "url": "https://example.com/image.png",
                    "revised_prompt": "A beautiful landscape with mountains and lakes"
                }
            ]
        }"#;

        let response: ImageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.created, 1234567890);
        assert_eq!(response.data.len(), 1);
        assert_eq!(
            response.data[0].url,
            Some("https://example.com/image.png".to_string())
        );
        assert!(response.has_urls());
        assert!(!response.has_b64_json());
    }

    #[tokio::test]
    async fn test_builder_patterns() {
        // Test ImageGenerationBuilder
        let gen_req = ImageGenerationBuilder::dall_e_3("Test image")
            .hd_quality()
            .vivid_style()
            .size_1024x1024()
            .b64_json_format()
            .n(1)
            .user("test-user")
            .build();

        assert_eq!(gen_req.model, ImageModels::DALL_E_3);
        assert_eq!(gen_req.prompt, "Test image");
        assert_eq!(gen_req.quality, Some(ImageQuality::Hd));
        assert_eq!(gen_req.style, Some(ImageStyle::Vivid));
        assert_eq!(gen_req.size, Some(ImageSize::Size1024x1024));
        assert_eq!(gen_req.response_format, Some(ImageResponseFormat::B64Json));
        assert_eq!(gen_req.n, Some(1));
        assert_eq!(gen_req.user, Some("test-user".to_string()));

        // Test ImageEditBuilder
        let edit_req = ImageEditBuilder::dall_e_2("image.png", "Add a rainbow")
            .mask("mask.png")
            .n(2)
            .url_format()
            .size(ImageSize::Size512x512)
            .build();

        assert_eq!(edit_req.model, ImageModels::DALL_E_2);
        assert_eq!(edit_req.image, "image.png");
        assert_eq!(edit_req.prompt, "Add a rainbow");
        assert_eq!(edit_req.mask, Some("mask.png".to_string()));
        assert_eq!(edit_req.n, Some(2));
        assert_eq!(edit_req.response_format, Some(ImageResponseFormat::Url));
        assert_eq!(edit_req.size, Some(ImageSize::Size512x512));

        // Test ImageVariationBuilder
        let var_req = ImageVariationBuilder::dall_e_2("image.png")
            .n(3)
            .b64_json_format()
            .size(ImageSize::Size256x256)
            .user("variation-user")
            .build();

        assert_eq!(var_req.model, ImageModels::DALL_E_2);
        assert_eq!(var_req.image, "image.png");
        assert_eq!(var_req.n, Some(3));
        assert_eq!(var_req.response_format, Some(ImageResponseFormat::B64Json));
        assert_eq!(var_req.size, Some(ImageSize::Size256x256));
        assert_eq!(var_req.user, Some("variation-user".to_string()));
    }
}
