//! # Image Request Builders
//!
//! Builder patterns for creating image generation, editing, and variation requests
//! with fluent APIs and method chaining.

use super::common::ImageRequestCommon;
use super::requests::{ImageEditRequest, ImageGenerationRequest, ImageVariationRequest};
use super::types::{ImageModels, ImageQuality, ImageResponseFormat, ImageSize, ImageStyle};
use crate::models::common_builder::{Builder, WithFormat, WithN, WithQuality, WithSize, WithUser};
use crate::{
    impl_builder, impl_format_methods, impl_image_size_methods, impl_with_format, impl_with_n,
    impl_with_quality, impl_with_size, impl_with_user,
};

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

    /// Set the user identifier
    #[must_use]
    pub fn user(self, user: impl Into<String>) -> Self {
        <Self as crate::models::common_builder::WithUser<ImageGenerationRequest>>::user(self, user)
    }

    /// Build the request
    pub fn build(self) -> ImageGenerationRequest {
        <Self as crate::models::common_builder::Builder<ImageGenerationRequest>>::build(self)
    }
}

// Apply common builder traits
impl_builder!(ImageGenerationBuilder, ImageGenerationRequest, request);
impl_with_n!(
    ImageGenerationBuilder,
    ImageGenerationRequest,
    request,
    |n: u32| n.clamp(1, 10)
);
impl_with_format!(
    ImageGenerationBuilder,
    ImageGenerationRequest,
    request,
    ImageResponseFormat
);
impl_with_size!(
    ImageGenerationBuilder,
    ImageGenerationRequest,
    request,
    ImageSize
);
impl_with_user!(ImageGenerationBuilder, ImageGenerationRequest, request);
impl_with_quality!(
    ImageGenerationBuilder,
    ImageGenerationRequest,
    request,
    ImageQuality
);

// Generate format and size convenience methods
impl_format_methods!(ImageGenerationBuilder, ImageResponseFormat, request);
impl_image_size_methods!(ImageGenerationBuilder, request);

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

    /// Set the number of images to generate
    #[must_use]
    pub fn n(self, n: u32) -> Self {
        <Self as crate::models::common_builder::WithN<ImageEditRequest>>::n(self, n)
    }

    /// Set the size of the generated images
    #[must_use]
    pub fn size(self, size: ImageSize) -> Self {
        <Self as crate::models::common_builder::WithSize<ImageEditRequest, ImageSize>>::size(
            self, size,
        )
    }

    /// Set the user identifier
    #[must_use]
    pub fn user(self, user: impl Into<String>) -> Self {
        <Self as crate::models::common_builder::WithUser<ImageEditRequest>>::user(self, user)
    }

    /// Build the request
    pub fn build(self) -> ImageEditRequest {
        <Self as crate::models::common_builder::Builder<ImageEditRequest>>::build(self)
    }
}

// Apply common builder traits
impl_builder!(ImageEditBuilder, ImageEditRequest, request);
impl_with_n!(ImageEditBuilder, ImageEditRequest, request, |n: u32| n
    .clamp(1, 10));
impl_with_format!(
    ImageEditBuilder,
    ImageEditRequest,
    request,
    ImageResponseFormat
);
impl_with_size!(ImageEditBuilder, ImageEditRequest, request, ImageSize);
impl_with_user!(ImageEditBuilder, ImageEditRequest, request);

// Generate format convenience methods
impl_format_methods!(ImageEditBuilder, ImageResponseFormat, request);

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

    /// Set the number of images to generate
    #[must_use]
    pub fn n(self, n: u32) -> Self {
        <Self as crate::models::common_builder::WithN<ImageVariationRequest>>::n(self, n)
    }

    /// Set the size of the generated images
    #[must_use]
    pub fn size(self, size: ImageSize) -> Self {
        <Self as crate::models::common_builder::WithSize<ImageVariationRequest, ImageSize>>::size(
            self, size,
        )
    }

    /// Build the request
    pub fn build(self) -> ImageVariationRequest {
        <Self as crate::models::common_builder::Builder<ImageVariationRequest>>::build(self)
    }
}

// Apply common builder traits
impl_builder!(ImageVariationBuilder, ImageVariationRequest, request);
impl_with_n!(
    ImageVariationBuilder,
    ImageVariationRequest,
    request,
    |n: u32| n.clamp(1, 10)
);
impl_with_format!(
    ImageVariationBuilder,
    ImageVariationRequest,
    request,
    ImageResponseFormat
);
impl_with_size!(
    ImageVariationBuilder,
    ImageVariationRequest,
    request,
    ImageSize
);
impl_with_user!(ImageVariationBuilder, ImageVariationRequest, request);

// Generate format convenience methods
impl_format_methods!(ImageVariationBuilder, ImageResponseFormat, request);

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
