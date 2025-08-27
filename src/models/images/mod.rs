//! # Images Models
//!
//! Data structures for the OpenAI Images API including image generation,
//! editing, and variation endpoints using DALL-E models.

/// Builder patterns for creating image requests
pub mod builders;
/// Common functionality and traits
pub mod common;
/// Method implementations for request and response types  
pub mod implementations;
/// Request type definitions
pub mod requests;
/// Response type definitions
pub mod responses;
/// Type definitions (enums, constants)
pub mod types;
/// Request validation logic
pub mod validation;

// Re-export all public items to maintain API compatibility
pub use builders::*;
pub use requests::*;
pub use responses::*;
pub use types::*;

// Import traits to enable method implementations
pub use common::ImageRequestCommon;

// Re-export validation for advanced users
pub use validation::validate_request;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_image_generation_request_creation() {
        let req = ImageGenerationRequest::new("dall-e-3", "A beautiful sunset");
        assert_eq!(req.model, "dall-e-3");
        assert_eq!(req.prompt, "A beautiful sunset");
        assert_eq!(req.n, None);
    }

    #[test]
    fn test_integration_image_generation_builder() {
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
    fn test_integration_dall_e_3_validation() {
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
    fn test_integration_dall_e_2_validation() {
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
}
