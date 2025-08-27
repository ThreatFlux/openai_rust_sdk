//! Confidence scores for each category

use super::constants::SCORE_MAPPINGS;
use crate::{De, Ser};

/// Confidence scores for each category (0.0 to 1.0)
#[derive(Debug, Clone, Ser, De)]
pub struct CategoryScores {
    /// Hate confidence score
    pub hate: f64,

    /// Hate/threatening confidence score
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,

    /// Self-harm confidence score
    #[serde(rename = "self-harm")]
    pub self_harm: f64,

    /// Self-harm/intent confidence score
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f64,

    /// Self-harm/instructions confidence score
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f64,

    /// Sexual confidence score
    pub sexual: f64,

    /// Sexual/minors confidence score
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,

    /// Violence confidence score
    pub violence: f64,

    /// Violence/graphic confidence score
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,

    /// Harassment confidence score
    pub harassment: f64,

    /// Harassment/threatening confidence score
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f64,
}

impl CategoryScores {
    /// Get the maximum score across all categories
    #[must_use]
    pub fn max_score(&self) -> f64 {
        SCORE_MAPPINGS
            .iter()
            .map(|(_, getter)| getter(self))
            .fold(0.0, f64::max)
    }

    /// Get scores above a certain threshold
    #[must_use]
    pub fn scores_above_threshold(&self, threshold: f64) -> Vec<(String, f64)> {
        self.get_category_scores()
            .into_iter()
            .filter(|(_, score)| *score >= threshold)
            .map(|(name, score)| (name.to_string(), score))
            .collect()
    }

    /// Helper method to get all category scores with their names
    fn get_category_scores(&self) -> Vec<(&'static str, f64)> {
        SCORE_MAPPINGS
            .iter()
            .map(|&(name, getter)| (name, getter(self)))
            .collect()
    }
}
