//! # Image Request Validation
//!
//! Validation logic for different DALL-E models and their specific constraints.

use super::requests::ImageGenerationRequest;
use super::types::ImageSize;

/// Validation error messages
pub mod errors {
    /// Error message for DALL-E 3 multiple image generation
    pub const DALL_E_3_SINGLE_IMAGE: &str = "DALL-E 3 only supports generating 1 image at a time";
    /// Error message for unsupported DALL-E 3 image sizes
    pub const DALL_E_3_SIZE_NOT_SUPPORTED: &str =
        "DALL-E 3 does not support 256x256 or 512x512 sizes";
    /// Error message for using quality parameter with DALL-E 2
    pub const DALL_E_2_NO_QUALITY: &str = "Quality parameter is only available for DALL-E 3";
    /// Error message for using style parameter with DALL-E 2
    pub const DALL_E_2_NO_STYLE: &str = "Style parameter is only available for DALL-E 3";
    /// Error message for unsupported DALL-E 2 image sizes
    pub const DALL_E_2_SIZE_NOT_SUPPORTED: &str =
        "DALL-E 2 does not support 1792x1024 or 1024x1792 sizes";
}

/// Model name constants for validation
pub mod models {
    /// DALL-E 3 model identifier
    pub const DALL_E_3: &str = "dall-e-3";
    /// DALL-E 2 model identifier
    pub const DALL_E_2: &str = "dall-e-2";
}

/// Validate DALL-E 3 specific parameters
pub fn validate_dall_e_3(request: &ImageGenerationRequest) -> Result<(), String> {
    validate_dall_e_3_image_count(request)?;
    validate_dall_e_3_image_size(request)?;
    Ok(())
}

/// Validate DALL-E 2 specific parameters
pub fn validate_dall_e_2(request: &ImageGenerationRequest) -> Result<(), String> {
    validate_dall_e_2_quality_and_style(request)?;
    validate_dall_e_2_image_size(request)?;
    Ok(())
}

/// Validate DALL-E 3 image count parameter
fn validate_dall_e_3_image_count(request: &ImageGenerationRequest) -> Result<(), String> {
    if let Some(n) = request.n {
        if n != 1 {
            return Err(errors::DALL_E_3_SINGLE_IMAGE.to_string());
        }
    }
    Ok(())
}

/// Validate DALL-E 3 image size parameter
fn validate_dall_e_3_image_size(request: &ImageGenerationRequest) -> Result<(), String> {
    if let Some(ImageSize::Size256x256 | ImageSize::Size512x512) = &request.size {
        return Err(errors::DALL_E_3_SIZE_NOT_SUPPORTED.to_string());
    }
    Ok(())
}

/// Validate DALL-E 2 quality and style parameters
fn validate_dall_e_2_quality_and_style(request: &ImageGenerationRequest) -> Result<(), String> {
    if request.quality.is_some() {
        return Err(errors::DALL_E_2_NO_QUALITY.to_string());
    }

    if request.style.is_some() {
        return Err(errors::DALL_E_2_NO_STYLE.to_string());
    }

    Ok(())
}

/// Validate DALL-E 2 image size parameter
fn validate_dall_e_2_image_size(request: &ImageGenerationRequest) -> Result<(), String> {
    if let Some(ImageSize::Size1792x1024 | ImageSize::Size1024x1792) = &request.size {
        return Err(errors::DALL_E_2_SIZE_NOT_SUPPORTED.to_string());
    }
    Ok(())
}

/// Main validation function that routes to appropriate validator based on model
pub fn validate_request(request: &ImageGenerationRequest) -> Result<(), String> {
    if request.model == models::DALL_E_3 {
        validate_dall_e_3(request)?;
    } else if request.model == models::DALL_E_2 {
        validate_dall_e_2(request)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::images::types::{ImageQuality, ImageSize, ImageStyle};

    fn create_test_request(model: &str) -> ImageGenerationRequest {
        ImageGenerationRequest {
            model: model.to_string(),
            prompt: "test".to_string(),
            n: None,
            quality: None,
            response_format: None,
            size: None,
            style: None,
            user: None,
        }
    }

    #[test]
    fn test_dall_e_3_validation() {
        let mut req = create_test_request(models::DALL_E_3);

        // DALL-E 3 only supports n=1
        req.n = Some(2);
        assert!(validate_dall_e_3(&req).is_err());

        req.n = Some(1);
        assert!(validate_dall_e_3(&req).is_ok());

        // DALL-E 3 doesn't support small sizes
        req.size = Some(ImageSize::Size256x256);
        assert!(validate_dall_e_3(&req).is_err());

        req.size = Some(ImageSize::Size1024x1024);
        assert!(validate_dall_e_3(&req).is_ok());
    }

    #[test]
    fn test_dall_e_2_validation() {
        let mut req = create_test_request(models::DALL_E_2);

        // DALL-E 2 doesn't support quality
        req.quality = Some(ImageQuality::Hd);
        assert!(validate_dall_e_2(&req).is_err());

        req.quality = None;
        assert!(validate_dall_e_2(&req).is_ok());

        // DALL-E 2 doesn't support style
        req.style = Some(ImageStyle::Vivid);
        assert!(validate_dall_e_2(&req).is_err());

        req.style = None;
        assert!(validate_dall_e_2(&req).is_ok());

        // DALL-E 2 doesn't support large sizes
        req.size = Some(ImageSize::Size1792x1024);
        assert!(validate_dall_e_2(&req).is_err());

        req.size = Some(ImageSize::Size512x512);
        assert!(validate_dall_e_2(&req).is_ok());
    }

    #[test]
    fn test_validate_request_routing() {
        let dall_e_3_req = create_test_request(models::DALL_E_3);
        assert!(validate_request(&dall_e_3_req).is_ok());

        let dall_e_2_req = create_test_request(models::DALL_E_2);
        assert!(validate_request(&dall_e_2_req).is_ok());

        // Test with invalid parameters
        let mut invalid_dall_e_3 = create_test_request(models::DALL_E_3);
        invalid_dall_e_3.n = Some(2);
        assert!(validate_request(&invalid_dall_e_3).is_err());

        let mut invalid_dall_e_2 = create_test_request(models::DALL_E_2);
        invalid_dall_e_2.quality = Some(ImageQuality::Hd);
        assert!(validate_request(&invalid_dall_e_2).is_err());
    }

    #[test]
    fn test_error_messages() {
        let mut req = create_test_request(models::DALL_E_3);
        req.n = Some(2);

        let result = validate_request(&req);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), errors::DALL_E_3_SINGLE_IMAGE);
    }
}
