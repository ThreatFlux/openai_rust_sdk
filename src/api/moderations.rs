//! # Moderations API
//!
//! This module provides access to OpenAI's moderations API for classifying
//! content according to OpenAI's usage policies.

use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
use crate::models::moderations::{ModerationRequest, ModerationResponse, ModerationResult};

/// Moderations API client
pub struct ModerationsApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
}

impl crate::api::common::ApiClientConstructors for ModerationsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

impl ModerationsApi {
    /// Create a moderation for the given request
    pub async fn create_moderation(
        &self,
        request: &ModerationRequest,
    ) -> Result<ModerationResponse> {
        self.http_client.post("/v1/moderations", request).await
    }

    /// Moderate a single text input (convenience method)
    pub async fn moderate_text(&self, text: &str) -> Result<ModerationResult> {
        let request = ModerationRequest::new(text);
        let response = self.create_moderation(&request).await?;

        response
            .results
            .into_iter()
            .next()
            .ok_or_else(|| OpenAIError::ParseError("No moderation result returned".to_string()))
    }

    /// Moderate a single text input with a specific model
    pub async fn moderate_text_with_model(
        &self,
        text: &str,
        model: &str,
    ) -> Result<ModerationResult> {
        let request = ModerationRequest::new(text).with_model(model);
        let response = self.create_moderation(&request).await?;

        response
            .results
            .into_iter()
            .next()
            .ok_or_else(|| OpenAIError::ParseError("No moderation result returned".to_string()))
    }

    /// Moderate multiple text inputs (batch processing)
    pub async fn moderate_texts(&self, texts: Vec<String>) -> Result<Vec<ModerationResult>> {
        let request = ModerationRequest::new_batch(texts);
        let response = self.create_moderation(&request).await?;

        Ok(response.results)
    }

    /// Moderate multiple text inputs with a specific model
    pub async fn moderate_texts_with_model(
        &self,
        texts: Vec<String>,
        model: &str,
    ) -> Result<Vec<ModerationResult>> {
        let request = ModerationRequest::new_batch(texts).with_model(model);
        let response = self.create_moderation(&request).await?;

        Ok(response.results)
    }

    /// Check if text is safe according to `OpenAI`'s policies (convenience method)
    pub async fn is_safe(&self, text: &str) -> Result<bool> {
        let result = self.moderate_text(text).await?;
        Ok(!result.flagged)
    }

    /// Check if text is safe with a custom confidence threshold
    pub async fn is_safe_with_threshold(&self, text: &str, threshold: f64) -> Result<bool> {
        let result = self.moderate_text(text).await?;

        // Check if any score exceeds the threshold
        let max_score = result.category_scores.max_score();
        Ok(max_score < threshold)
    }

    /// Check if text is safe with a specific threshold constant
    pub async fn is_safe_with_threshold_preset(
        &self,
        text: &str,
        preset: f64, // Use SafetyThresholds::CONSERVATIVE, MODERATE, or PERMISSIVE
    ) -> Result<bool> {
        self.is_safe_with_threshold(text, preset).await
    }

    /// Check if text contains specific types of violations
    pub async fn check_violations(&self, text: &str) -> Result<Vec<String>> {
        let result = self.moderate_text(text).await?;
        Ok(result.get_violations())
    }

    /// Get detailed moderation scores for a text input
    pub async fn get_scores(
        &self,
        text: &str,
    ) -> Result<crate::models::moderations::CategoryScores> {
        let result = self.moderate_text(text).await?;
        Ok(result.category_scores)
    }

    /// Moderate text and return both the result and the categories that were flagged
    pub async fn moderate_with_details(&self, text: &str) -> Result<(bool, Vec<String>)> {
        let result = self.moderate_text(text).await?;
        Ok((result.flagged, result.get_violations()))
    }
}

/// Helper methods for working with moderation results
impl ModerationResult {
    /// Get a list of violations detected in the content
    #[must_use]
    pub fn get_violations(&self) -> Vec<String> {
        let mut violations = Vec::new();

        if self.categories.sexual {
            violations.push("sexual".to_string());
        }
        if self.categories.hate {
            violations.push("hate".to_string());
        }
        if self.categories.harassment {
            violations.push("harassment".to_string());
        }
        if self.categories.self_harm {
            violations.push("self-harm".to_string());
        }
        if self.categories.sexual_minors {
            violations.push("sexual/minors".to_string());
        }
        if self.categories.hate_threatening {
            violations.push("hate/threatening".to_string());
        }
        if self.categories.violence_graphic {
            violations.push("violence/graphic".to_string());
        }
        if self.categories.self_harm_intent {
            violations.push("self-harm/intent".to_string());
        }
        if self.categories.self_harm_instructions {
            violations.push("self-harm/instructions".to_string());
        }
        if self.categories.harassment_threatening {
            violations.push("harassment/threatening".to_string());
        }
        if self.categories.violence {
            violations.push("violence".to_string());
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::common::ApiClientConstructors;

    #[test]
    fn test_moderation_api_creation() {
        let api = ModerationsApi::new("test-key");
        assert!(api.is_ok());
    }

    #[test]
    fn test_moderation_api_empty_key() {
        let api = ModerationsApi::new("");
        assert!(api.is_err());
    }

    #[test]
    fn test_moderation_api_whitespace_key() {
        let api = ModerationsApi::new("   ");
        assert!(api.is_err());
    }

    #[test]
    fn test_moderation_request_creation() {
        use crate::models::moderations::ModerationInput;
        let request = ModerationRequest::new("test text");
        match request.input {
            ModerationInput::String(s) => assert_eq!(s, "test text"),
            _ => panic!("Expected single string input"),
        }
    }

    #[test]
    fn test_moderation_batch_request() {
        use crate::models::moderations::ModerationInput;
        let texts = vec!["text1".to_string(), "text2".to_string()];
        let request = ModerationRequest::new_batch(texts.clone());
        match request.input {
            ModerationInput::StringArray(arr) => assert_eq!(arr.len(), 2),
            _ => panic!("Expected array input"),
        }
    }

    #[test]
    fn test_moderation_with_model() {
        let request = ModerationRequest::new("test").with_model("text-moderation-stable");
        assert_eq!(request.model, Some("text-moderation-stable".to_string()));
    }
}
