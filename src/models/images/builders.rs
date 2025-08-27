//! # Image Request Builders
//!
//! Builder patterns for creating image generation, editing, and variation requests
//! with fluent APIs and method chaining.

use super::common::ImageRequestCommon;
use super::requests::{ImageEditRequest, ImageGenerationRequest, ImageVariationRequest};
use super::types::{ImageModels, ImageQuality, ImageResponseFormat, ImageSize, ImageStyle};

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
        self.request.set_n(n);
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
        self.request.set_response_format(ImageResponseFormat::Url);
        self
    }

    /// Return base64 JSON
    #[must_use]
    pub fn b64_json_format(mut self) -> Self {
        self.request
            .set_response_format(ImageResponseFormat::B64Json);
        self
    }

    /// Set size to 256x256
    #[must_use]
    pub fn size_256x256(mut self) -> Self {
        self.request.set_size(ImageSize::Size256x256);
        self
    }

    /// Set size to 512x512
    #[must_use]
    pub fn size_512x512(mut self) -> Self {
        self.request.set_size(ImageSize::Size512x512);
        self
    }

    /// Set size to 1024x1024
    #[must_use]
    pub fn size_1024x1024(mut self) -> Self {
        self.request.set_size(ImageSize::Size1024x1024);
        self
    }

    /// Set size to 1792x1024 (landscape)
    #[must_use]
    pub fn size_1792x1024(mut self) -> Self {
        self.request.set_size(ImageSize::Size1792x1024);
        self
    }

    /// Set size to 1024x1792 (portrait)
    #[must_use]
    pub fn size_1024x1792(mut self) -> Self {
        self.request.set_size(ImageSize::Size1024x1792);
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
        self.request.set_user(user.into());
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
        self.request.set_n(n);
        self
    }

    /// Return URLs
    #[must_use]
    pub fn url_format(mut self) -> Self {
        self.request.set_response_format(ImageResponseFormat::Url);
        self
    }

    /// Return base64 JSON
    #[must_use]
    pub fn b64_json_format(mut self) -> Self {
        self.request
            .set_response_format(ImageResponseFormat::B64Json);
        self
    }

    /// Set size
    #[must_use]
    pub fn size(mut self, size: ImageSize) -> Self {
        self.request.set_size(size);
        self
    }

    /// Set user identifier
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.request.set_user(user.into());
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
        self.request.set_n(n);
        self
    }

    /// Return URLs
    #[must_use]
    pub fn url_format(mut self) -> Self {
        self.request.set_response_format(ImageResponseFormat::Url);
        self
    }

    /// Return base64 JSON
    #[must_use]
    pub fn b64_json_format(mut self) -> Self {
        self.request
            .set_response_format(ImageResponseFormat::B64Json);
        self
    }

    /// Set size
    #[must_use]
    pub fn size(mut self, size: ImageSize) -> Self {
        self.request.set_size(size);
        self
    }

    /// Set user identifier
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.request.set_user(user.into());
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
    use crate::models::images::types::{ImageQuality, ImageResponseFormat, ImageSize, ImageStyle};

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
    fn test_image_edit_builder() {
        let req = ImageEditBuilder::dall_e_2("image.png", "Add a cat")
            .mask("mask.png")
            .n(3)
            .url_format()
            .size(ImageSize::Size512x512)
            .build();

        assert_eq!(req.model, ImageModels::DALL_E_2);
        assert_eq!(req.image, "image.png");
        assert_eq!(req.mask, Some("mask.png".to_string()));
        assert_eq!(req.prompt, "Add a cat");
        assert_eq!(req.n, Some(3));
        assert_eq!(req.response_format, Some(ImageResponseFormat::Url));
        assert_eq!(req.size, Some(ImageSize::Size512x512));
    }

    #[test]
    fn test_image_variation_builder() {
        let req = ImageVariationBuilder::dall_e_2("image.png")
            .n(5)
            .b64_json_format()
            .size(ImageSize::Size1024x1024)
            .user("test_user")
            .build();

        assert_eq!(req.model, ImageModels::DALL_E_2);
        assert_eq!(req.image, "image.png");
        assert_eq!(req.n, Some(5));
        assert_eq!(req.response_format, Some(ImageResponseFormat::B64Json));
        assert_eq!(req.size, Some(ImageSize::Size1024x1024));
        assert_eq!(req.user, Some("test_user".to_string()));
    }

    #[test]
    fn test_builder_n_clamping() {
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
    fn test_all_size_methods() {
        let req = ImageGenerationBuilder::dall_e_2("test")
            .size_256x256()
            .build();
        assert_eq!(req.size, Some(ImageSize::Size256x256));

        let req = ImageGenerationBuilder::dall_e_2("test")
            .size_512x512()
            .build();
        assert_eq!(req.size, Some(ImageSize::Size512x512));

        let req = ImageGenerationBuilder::dall_e_3("test")
            .size_1024x1024()
            .build();
        assert_eq!(req.size, Some(ImageSize::Size1024x1024));

        let req = ImageGenerationBuilder::dall_e_3("test")
            .size_1792x1024()
            .build();
        assert_eq!(req.size, Some(ImageSize::Size1792x1024));

        let req = ImageGenerationBuilder::dall_e_3("test")
            .size_1024x1792()
            .build();
        assert_eq!(req.size, Some(ImageSize::Size1024x1792));
    }
}
