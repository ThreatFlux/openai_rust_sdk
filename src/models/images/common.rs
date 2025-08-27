//! # Common Image Functionality
//!
//! Shared traits and functionality for image request types.

use super::requests::{ImageEditRequest, ImageGenerationRequest, ImageVariationRequest};
use super::types::{ImageResponseFormat, ImageSize};

/// Common trait for image request builders
pub trait ImageRequestCommon {
    /// Set the number of images to generate
    fn set_n(&mut self, n: u32);
    /// Set the response format
    fn set_response_format(&mut self, format: ImageResponseFormat);
    /// Set the image size
    fn set_size(&mut self, size: ImageSize);
    /// Set the user identifier
    fn set_user(&mut self, user: String);
}

/// Helper function to clamp the number of images to valid range (1-10)
pub fn clamp_image_count(n: u32) -> u32 {
    n.clamp(1, 10)
}

impl ImageRequestCommon for ImageGenerationRequest {
    fn set_n(&mut self, n: u32) {
        self.n = Some(clamp_image_count(n));
    }

    fn set_response_format(&mut self, format: ImageResponseFormat) {
        self.response_format = Some(format);
    }

    fn set_size(&mut self, size: ImageSize) {
        self.size = Some(size);
    }

    fn set_user(&mut self, user: String) {
        self.user = Some(user);
    }
}

impl ImageRequestCommon for ImageEditRequest {
    fn set_n(&mut self, n: u32) {
        self.n = Some(clamp_image_count(n));
    }

    fn set_response_format(&mut self, format: ImageResponseFormat) {
        self.response_format = Some(format);
    }

    fn set_size(&mut self, size: ImageSize) {
        self.size = Some(size);
    }

    fn set_user(&mut self, user: String) {
        self.user = Some(user);
    }
}

impl ImageRequestCommon for ImageVariationRequest {
    fn set_n(&mut self, n: u32) {
        self.n = Some(clamp_image_count(n));
    }

    fn set_response_format(&mut self, format: ImageResponseFormat) {
        self.response_format = Some(format);
    }

    fn set_size(&mut self, size: ImageSize) {
        self.size = Some(size);
    }

    fn set_user(&mut self, user: String) {
        self.user = Some(user);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::images::types::{ImageResponseFormat, ImageSize};

    #[test]
    fn test_clamp_image_count() {
        assert_eq!(clamp_image_count(0), 1);
        assert_eq!(clamp_image_count(5), 5);
        assert_eq!(clamp_image_count(15), 10);
    }

    #[test]
    fn test_image_generation_request_common() {
        let mut req = ImageGenerationRequest {
            model: "dall-e-2".to_string(),
            prompt: "test".to_string(),
            n: None,
            quality: None,
            response_format: None,
            size: None,
            style: None,
            user: None,
        };

        req.set_n(15);
        assert_eq!(req.n, Some(10)); // Should be clamped

        req.set_response_format(ImageResponseFormat::B64Json);
        assert_eq!(req.response_format, Some(ImageResponseFormat::B64Json));

        req.set_size(ImageSize::Size512x512);
        assert_eq!(req.size, Some(ImageSize::Size512x512));

        req.set_user("test_user".to_string());
        assert_eq!(req.user, Some("test_user".to_string()));
    }

    #[test]
    fn test_n_clamping_in_trait() {
        let mut req = ImageGenerationRequest {
            model: "dall-e-2".to_string(),
            prompt: "test".to_string(),
            n: None,
            quality: None,
            response_format: None,
            size: None,
            style: None,
            user: None,
        };

        req.set_n(15); // Should be clamped to 10
        assert_eq!(req.n, Some(10));

        req.set_n(0); // Should be clamped to 1
        assert_eq!(req.n, Some(1));
    }
}
