//! Operation methods and convenience functions for the Moderations API

use super::client::ModerationsApi;
use crate::error::{OpenAIError, Result};
use crate::models::moderations::{ModerationRequest, ModerationResult};

impl ModerationsApi {
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
