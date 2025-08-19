//! # Images Models
//!
//! Data structures for the OpenAI Images API including image generation,
//! editing, and variation endpoints using DALL-E models.

use serde::{Deserialize, Serialize};

/// Request for generating images from text prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    /// The model to use for image generation (e.g., "dall-e-3", "dall-e-2")
    pub model: String,

    /// A text description of the desired image(s)
    pub prompt: String,

    /// The number of images to generate (1-10 for dall-e-2, only 1 for dall-e-3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// The quality of the image (standard or hd, dall-e-3 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,

    /// The format in which the generated images are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// The size of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// The style of the generated images (vivid or natural, dall-e-3 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Request for editing images with masks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEditRequest {
    /// The model to use for image editing (e.g., "dall-e-2")
    pub model: String,

    /// The image to edit (PNG only, up to 4MB, must be square)
    #[serde(skip_serializing)]
    pub image: String,

    /// An additional image whose fully transparent areas indicate where image should be edited (PNG only, same size as image)
    #[serde(skip_serializing)]
    pub mask: Option<String>,

    /// A text description of the desired image(s)
    pub prompt: String,

    /// The number of images to generate (1-10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// The format in which the generated images are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// The size of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Request for creating variations of images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageVariationRequest {
    /// The model to use for creating variations (e.g., "dall-e-2")
    pub model: String,

    /// The image to create variations of (PNG only, up to 4MB, must be square)
    #[serde(skip_serializing)]
    pub image: String,

    /// The number of images to generate (1-10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// The format in which the generated images are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// The size of the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// A unique identifier representing your end-user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Image generation quality levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImageQuality {
    /// Standard quality (faster and cheaper)
    Standard,
    /// High definition quality (more detailed but slower and more expensive)
    Hd,
}

/// Image response formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    /// Return image URLs that expire after 1 hour
    Url,
    /// Return base64-encoded image data
    B64Json,
}

/// Image sizes supported by DALL-E
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImageSize {
    /// 256x256 pixels (dall-e-2 only)
    #[serde(rename = "256x256")]
    Size256x256,
    /// 512x512 pixels (dall-e-2 only)
    #[serde(rename = "512x512")]
    Size512x512,
    /// 1024x1024 pixels (dall-e-2 and dall-e-3)
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    /// 1792x1024 pixels (dall-e-3 only)
    #[serde(rename = "1792x1024")]
    Size1792x1024,
    /// 1024x1792 pixels (dall-e-3 only)
    #[serde(rename = "1024x1792")]
    Size1024x1792,
}

/// Image generation styles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImageStyle {
    /// More vivid and dramatic images
    Vivid,
    /// More natural and less hyper-real images
    Natural,
}

/// Response from image generation endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResponse {
    /// The Unix timestamp when the images were created
    pub created: u64,

    /// The generated images
    pub data: Vec<ImageData>,
}

/// Individual image data in the response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    /// The URL of the generated image (if `response_format` is url)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The base64-encoded image data (if `response_format` is `b64_json`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,

    /// The revised prompt used for generation (DALL-E 3 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revised_prompt: Option<String>,
}

impl ImageGenerationRequest {
    /// Create a new image generation request
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            n: None,
            quality: None,
            response_format: None,
            size: None,
            style: None,
            user: None,
        }
    }

    /// Set the number of images to generate
    #[must_use]
    pub fn with_n(mut self, n: u32) -> Self {
        self.n = Some(n.clamp(1, 10));
        self
    }

    /// Set the image quality
    #[must_use]
    pub fn with_quality(mut self, quality: ImageQuality) -> Self {
        self.quality = Some(quality);
        self
    }

    /// Set the response format
    #[must_use]
    pub fn with_response_format(mut self, format: ImageResponseFormat) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set the image size
    #[must_use]
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the image style
    #[must_use]
    pub fn with_style(mut self, style: ImageStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Set the user identifier
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Validate the request parameters
    pub fn validate(&self) -> Result<(), String> {
        // DALL-E 3 specific validations
        if self.model == "dall-e-3" {
            if let Some(n) = self.n {
                if n != 1 {
                    return Err("DALL-E 3 only supports generating 1 image at a time".to_string());
                }
            }

            if let Some(ImageSize::Size256x256 | ImageSize::Size512x512) = &self.size {
                return Err(
                    "DALL-E 3 does not support 256x256 or 512x512 sizes".to_string()
                );
            }
        }

        // DALL-E 2 specific validations
        if self.model == "dall-e-2" {
            if self.quality.is_some() {
                return Err("Quality parameter is only available for DALL-E 3".to_string());
            }

            if self.style.is_some() {
                return Err("Style parameter is only available for DALL-E 3".to_string());
            }

            if let Some(ImageSize::Size1792x1024 | ImageSize::Size1024x1792) = &self.size {
                return Err(
                    "DALL-E 2 does not support 1792x1024 or 1024x1792 sizes".to_string()
                );
            }
        }

        Ok(())
    }
}

impl ImageEditRequest {
    /// Create a new image edit request
    pub fn new(
        model: impl Into<String>,
        image: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Self {
        Self {
            model: model.into(),
            image: image.into(),
            mask: None,
            prompt: prompt.into(),
            n: None,
            response_format: None,
            size: None,
            user: None,
        }
    }

    /// Set the mask image
    pub fn with_mask(mut self, mask: impl Into<String>) -> Self {
        self.mask = Some(mask.into());
        self
    }

    /// Set the number of images to generate
    #[must_use]
    pub fn with_n(mut self, n: u32) -> Self {
        self.n = Some(n.clamp(1, 10));
        self
    }

    /// Set the response format
    #[must_use]
    pub fn with_response_format(mut self, format: ImageResponseFormat) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set the image size
    #[must_use]
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the user identifier
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl ImageVariationRequest {
    /// Create a new image variation request
    pub fn new(model: impl Into<String>, image: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            image: image.into(),
            n: None,
            response_format: None,
            size: None,
            user: None,
        }
    }

    /// Set the number of images to generate
    #[must_use]
    pub fn with_n(mut self, n: u32) -> Self {
        self.n = Some(n.clamp(1, 10));
        self
    }

    /// Set the response format
    #[must_use]
    pub fn with_response_format(mut self, format: ImageResponseFormat) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set the image size
    #[must_use]
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the user identifier
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

impl ImageData {
    /// Check if this image data contains a URL
    #[must_use]
    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    /// Check if this image data contains base64 data
    #[must_use]
    pub fn has_b64_json(&self) -> bool {
        self.b64_json.is_some()
    }

    /// Get the image URL if available
    #[must_use]
    pub fn get_url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    /// Get the base64 image data if available
    #[must_use]
    pub fn get_b64_json(&self) -> Option<&str> {
        self.b64_json.as_deref()
    }

    /// Get the revised prompt if available (DALL-E 3 only)
    #[must_use]
    pub fn get_revised_prompt(&self) -> Option<&str> {
        self.revised_prompt.as_deref()
    }

    /// Decode base64 image data to bytes
    pub fn decode_b64_json(&self) -> Result<Vec<u8>, String> {
        match &self.b64_json {
            Some(b64_data) => {
                use base64::{Engine as _, engine::general_purpose};
                general_purpose::STANDARD
                    .decode(b64_data)
                    .map_err(|e| format!("Failed to decode base64: {e}"))
            }
            None => Err("No base64 data available".to_string()),
        }
    }

    /// Save base64 image data to file
    pub async fn save_b64_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), String> {
        let data = self.decode_b64_json()?;
        tokio::fs::write(path, data)
            .await
            .map_err(|e| format!("Failed to save file: {e}"))
    }
}

impl ImageResponse {
    /// Get the number of generated images
    #[must_use]
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Check if any images have URLs
    #[must_use]
    pub fn has_urls(&self) -> bool {
        self.data.iter().any(ImageData::has_url)
    }

    /// Check if any images have base64 data
    #[must_use]
    pub fn has_b64_json(&self) -> bool {
        self.data.iter().any(ImageData::has_b64_json)
    }

    /// Get all image URLs
    #[must_use]
    pub fn get_urls(&self) -> Vec<&str> {
        self.data.iter().filter_map(|img| img.get_url()).collect()
    }

    /// Get the first image data
    #[must_use]
    pub fn first_image(&self) -> Option<&ImageData> {
        self.data.first()
    }

    /// Get all revised prompts (DALL-E 3 only)
    #[must_use]
    pub fn get_revised_prompts(&self) -> Vec<&str> {
        self.data
            .iter()
            .filter_map(|img| img.get_revised_prompt())
            .collect()
    }
}

/// Common image models
pub struct ImageModels;

impl ImageModels {
    /// DALL-E 3 model (latest, highest quality)
    pub const DALL_E_3: &'static str = "dall-e-3";

    /// DALL-E 2 model (faster, supports more features like editing and variations)
    pub const DALL_E_2: &'static str = "dall-e-2";
}

/// Builder for creating image generation requests
pub struct ImageGenerationBuilder {
    /// The underlying image generation request being built
    request: ImageGenerationRequest,
}

impl ImageGenerationBuilder {
    /// Create a new image generation builder
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            request: ImageGenerationRequest::new(model, prompt),
        }
    }

    /// Use DALL-E 3 model
    pub fn dall_e_3(prompt: impl Into<String>) -> Self {
        Self::new(ImageModels::DALL_E_3, prompt)
    }

    /// Use DALL-E 2 model
    pub fn dall_e_2(prompt: impl Into<String>) -> Self {
        Self::new(ImageModels::DALL_E_2, prompt)
    }

    /// Set number of images (1-10 for DALL-E 2, only 1 for DALL-E 3)
    #[must_use]
    pub fn n(mut self, n: u32) -> Self {
        self.request.n = Some(n.clamp(1, 10));
        self
    }

    /// Set quality to standard
    #[must_use]
    pub fn standard_quality(mut self) -> Self {
        self.request.quality = Some(ImageQuality::Standard);
        self
    }

    /// Set quality to HD
    #[must_use]
    pub fn hd_quality(mut self) -> Self {
        self.request.quality = Some(ImageQuality::Hd);
        self
    }

    /// Return URLs
    #[must_use]
    pub fn url_format(mut self) -> Self {
        self.request.response_format = Some(ImageResponseFormat::Url);
        self
    }

    /// Return base64 JSON
    #[must_use]
    pub fn b64_json_format(mut self) -> Self {
        self.request.response_format = Some(ImageResponseFormat::B64Json);
        self
    }

    /// Set size to 256x256
    #[must_use]
    pub fn size_256x256(mut self) -> Self {
        self.request.size = Some(ImageSize::Size256x256);
        self
    }

    /// Set size to 512x512
    #[must_use]
    pub fn size_512x512(mut self) -> Self {
        self.request.size = Some(ImageSize::Size512x512);
        self
    }

    /// Set size to 1024x1024
    #[must_use]
    pub fn size_1024x1024(mut self) -> Self {
        self.request.size = Some(ImageSize::Size1024x1024);
        self
    }

    /// Set size to 1792x1024 (landscape)
    #[must_use]
    pub fn size_1792x1024(mut self) -> Self {
        self.request.size = Some(ImageSize::Size1792x1024);
        self
    }

    /// Set size to 1024x1792 (portrait)
    #[must_use]
    pub fn size_1024x1792(mut self) -> Self {
        self.request.size = Some(ImageSize::Size1024x1792);
        self
    }

    /// Set style to vivid
    #[must_use]
    pub fn vivid_style(mut self) -> Self {
        self.request.style = Some(ImageStyle::Vivid);
        self
    }

    /// Set style to natural
    #[must_use]
    pub fn natural_style(mut self) -> Self {
        self.request.style = Some(ImageStyle::Natural);
        self
    }

    /// Set user identifier
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.request.user = Some(user.into());
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> ImageGenerationRequest {
        self.request
    }
}

/// Builder for creating image edit requests
pub struct ImageEditBuilder {
    /// The underlying image edit request being built
    request: ImageEditRequest,
}

impl ImageEditBuilder {
    /// Create a new image edit builder
    pub fn new(
        model: impl Into<String>,
        image: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Self {
        Self {
            request: ImageEditRequest::new(model, image, prompt),
        }
    }

    /// Use DALL-E 2 model
    pub fn dall_e_2(image: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self::new(ImageModels::DALL_E_2, image, prompt)
    }

    /// Set mask image
    pub fn mask(mut self, mask: impl Into<String>) -> Self {
        self.request.mask = Some(mask.into());
        self
    }

    /// Set number of images
    #[must_use]
    pub fn n(mut self, n: u32) -> Self {
        self.request.n = Some(n.clamp(1, 10));
        self
    }

    /// Return URLs
    #[must_use]
    pub fn url_format(mut self) -> Self {
        self.request.response_format = Some(ImageResponseFormat::Url);
        self
    }

    /// Return base64 JSON
    #[must_use]
    pub fn b64_json_format(mut self) -> Self {
        self.request.response_format = Some(ImageResponseFormat::B64Json);
        self
    }

    /// Set size
    #[must_use]
    pub fn size(mut self, size: ImageSize) -> Self {
        self.request.size = Some(size);
        self
    }

    /// Set user identifier
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.request.user = Some(user.into());
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> ImageEditRequest {
        self.request
    }
}

/// Builder for creating image variation requests
pub struct ImageVariationBuilder {
    /// The underlying image variation request being built
    request: ImageVariationRequest,
}

impl ImageVariationBuilder {
    /// Create a new image variation builder
    pub fn new(model: impl Into<String>, image: impl Into<String>) -> Self {
        Self {
            request: ImageVariationRequest::new(model, image),
        }
    }

    /// Use DALL-E 2 model
    pub fn dall_e_2(image: impl Into<String>) -> Self {
        Self::new(ImageModels::DALL_E_2, image)
    }

    /// Set number of images
    #[must_use]
    pub fn n(mut self, n: u32) -> Self {
        self.request.n = Some(n.clamp(1, 10));
        self
    }

    /// Return URLs
    #[must_use]
    pub fn url_format(mut self) -> Self {
        self.request.response_format = Some(ImageResponseFormat::Url);
        self
    }

    /// Return base64 JSON
    #[must_use]
    pub fn b64_json_format(mut self) -> Self {
        self.request.response_format = Some(ImageResponseFormat::B64Json);
        self
    }

    /// Set size
    #[must_use]
    pub fn size(mut self, size: ImageSize) -> Self {
        self.request.size = Some(size);
        self
    }

    /// Set user identifier
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.request.user = Some(user.into());
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> ImageVariationRequest {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_generation_request_creation() {
        let req = ImageGenerationRequest::new("dall-e-3", "A beautiful sunset");
        assert_eq!(req.model, "dall-e-3");
        assert_eq!(req.prompt, "A beautiful sunset");
        assert_eq!(req.n, None);
    }

    #[test]
    fn test_image_generation_builder() {
        let req = ImageGenerationBuilder::dall_e_3("Test image")
            .hd_quality()
            .vivid_style()
            .size_1024x1024()
            .b64_json_format()
            .build();

        assert_eq!(req.model, ImageModels::DALL_E_3);
        assert_eq!(req.prompt, "Test image");
        assert_eq!(req.quality, Some(ImageQuality::Hd));
        assert_eq!(req.style, Some(ImageStyle::Vivid));
        assert_eq!(req.size, Some(ImageSize::Size1024x1024));
        assert_eq!(req.response_format, Some(ImageResponseFormat::B64Json));
    }

    #[test]
    fn test_image_edit_request_creation() {
        let req = ImageEditRequest::new("dall-e-2", "image.png", "Add a cat");
        assert_eq!(req.model, "dall-e-2");
        assert_eq!(req.image, "image.png");
        assert_eq!(req.prompt, "Add a cat");
        assert_eq!(req.mask, None);
    }

    #[test]
    fn test_image_variation_request_creation() {
        let req = ImageVariationRequest::new("dall-e-2", "image.png");
        assert_eq!(req.model, "dall-e-2");
        assert_eq!(req.image, "image.png");
        assert_eq!(req.n, None);
    }

    #[test]
    fn test_image_size_serialization() {
        assert_eq!(
            serde_json::to_string(&ImageSize::Size256x256).unwrap(),
            "\"256x256\""
        );
        assert_eq!(
            serde_json::to_string(&ImageSize::Size512x512).unwrap(),
            "\"512x512\""
        );
        assert_eq!(
            serde_json::to_string(&ImageSize::Size1024x1024).unwrap(),
            "\"1024x1024\""
        );
        assert_eq!(
            serde_json::to_string(&ImageSize::Size1792x1024).unwrap(),
            "\"1792x1024\""
        );
        assert_eq!(
            serde_json::to_string(&ImageSize::Size1024x1792).unwrap(),
            "\"1024x1792\""
        );
    }

    #[test]
    fn test_image_quality_serialization() {
        assert_eq!(
            serde_json::to_string(&ImageQuality::Standard).unwrap(),
            "\"standard\""
        );
        assert_eq!(serde_json::to_string(&ImageQuality::Hd).unwrap(), "\"hd\"");
    }

    #[test]
    fn test_image_style_serialization() {
        assert_eq!(
            serde_json::to_string(&ImageStyle::Vivid).unwrap(),
            "\"vivid\""
        );
        assert_eq!(
            serde_json::to_string(&ImageStyle::Natural).unwrap(),
            "\"natural\""
        );
    }

    #[test]
    fn test_response_format_serialization() {
        assert_eq!(
            serde_json::to_string(&ImageResponseFormat::Url).unwrap(),
            "\"url\""
        );
        assert_eq!(
            serde_json::to_string(&ImageResponseFormat::B64Json).unwrap(),
            "\"b64_json\""
        );
    }

    #[test]
    fn test_dall_e_3_validation() {
        let mut req = ImageGenerationRequest::new("dall-e-3", "test");

        // DALL-E 3 only supports n=1
        req.n = Some(2);
        assert!(req.validate().is_err());

        req.n = Some(1);
        assert!(req.validate().is_ok());

        // DALL-E 3 doesn't support small sizes
        req.size = Some(ImageSize::Size256x256);
        assert!(req.validate().is_err());

        req.size = Some(ImageSize::Size1024x1024);
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_dall_e_2_validation() {
        let mut req = ImageGenerationRequest::new("dall-e-2", "test");

        // DALL-E 2 doesn't support quality
        req.quality = Some(ImageQuality::Hd);
        assert!(req.validate().is_err());

        req.quality = None;
        assert!(req.validate().is_ok());

        // DALL-E 2 doesn't support style
        req.style = Some(ImageStyle::Vivid);
        assert!(req.validate().is_err());

        req.style = None;
        assert!(req.validate().is_ok());

        // DALL-E 2 doesn't support large sizes
        req.size = Some(ImageSize::Size1792x1024);
        assert!(req.validate().is_err());

        req.size = Some(ImageSize::Size512x512);
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_n_clamping() {
        let req = ImageGenerationBuilder::dall_e_2("test")
            .n(15) // Should be clamped to 10
            .build();
        assert_eq!(req.n, Some(10));

        let req2 = ImageGenerationBuilder::dall_e_2("test")
            .n(0) // Should be clamped to 1
            .build();
        assert_eq!(req2.n, Some(1));
    }

    #[test]
    fn test_image_data_methods() {
        let image_data = ImageData {
            url: Some("https://example.com/image.png".to_string()),
            b64_json: None,
            revised_prompt: Some("A revised prompt".to_string()),
        };

        assert!(image_data.has_url());
        assert!(!image_data.has_b64_json());
        assert_eq!(image_data.get_url(), Some("https://example.com/image.png"));
        assert_eq!(image_data.get_revised_prompt(), Some("A revised prompt"));
    }

    #[test]
    fn test_image_response_methods() {
        let response = ImageResponse {
            created: 1234567890,
            data: vec![
                ImageData {
                    url: Some("https://example.com/image1.png".to_string()),
                    b64_json: None,
                    revised_prompt: None,
                },
                ImageData {
                    url: Some("https://example.com/image2.png".to_string()),
                    b64_json: None,
                    revised_prompt: None,
                },
            ],
        };

        assert_eq!(response.count(), 2);
        assert!(response.has_urls());
        assert!(!response.has_b64_json());
        assert_eq!(response.get_urls().len(), 2);
        assert!(response.first_image().is_some());
    }

    #[test]
    fn test_request_serialization() {
        let req = ImageGenerationRequest::new("dall-e-3", "A beautiful image")
            .with_quality(ImageQuality::Hd)
            .with_size(ImageSize::Size1024x1024)
            .with_style(ImageStyle::Natural)
            .with_response_format(ImageResponseFormat::Url);

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"model\":\"dall-e-3\""));
        assert!(json.contains("\"prompt\":\"A beautiful image\""));
        assert!(json.contains("\"quality\":\"hd\""));
        assert!(json.contains("\"size\":\"1024x1024\""));
        assert!(json.contains("\"style\":\"natural\""));
        assert!(json.contains("\"response_format\":\"url\""));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "created": 1234567890,
            "data": [
                {
                    "url": "https://example.com/image.png",
                    "revised_prompt": "A revised prompt"
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
        assert_eq!(
            response.data[0].revised_prompt,
            Some("A revised prompt".to_string())
        );
    }
}
