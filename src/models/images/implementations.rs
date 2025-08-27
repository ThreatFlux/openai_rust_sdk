//! # Image Request Implementations
//!
//! Constructor and method implementations for image request and response types.

use super::common::ImageRequestCommon;
use super::requests::{ImageEditRequest, ImageGenerationRequest, ImageVariationRequest};
use super::types::{ImageQuality, ImageResponseFormat, ImageSize, ImageStyle};
use super::validation::validate_request;

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
        self.set_n(n);
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
        self.set_response_format(format);
        self
    }

    /// Set the image size
    #[must_use]
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.set_size(size);
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
        self.set_user(user.into());
        self
    }

    /// Validate the request parameters
    pub fn validate(&self) -> Result<(), String> {
        validate_request(self)
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
        self.set_n(n);
        self
    }

    /// Set the response format
    #[must_use]
    pub fn with_response_format(mut self, format: ImageResponseFormat) -> Self {
        self.set_response_format(format);
        self
    }

    /// Set the image size
    #[must_use]
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.set_size(size);
        self
    }

    /// Set the user identifier
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.set_user(user.into());
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
        self.set_n(n);
        self
    }

    /// Set the response format
    #[must_use]
    pub fn with_response_format(mut self, format: ImageResponseFormat) -> Self {
        self.set_response_format(format);
        self
    }

    /// Set the image size
    #[must_use]
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.set_size(size);
        self
    }

    /// Set the user identifier
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.set_user(user.into());
        self
    }
}
