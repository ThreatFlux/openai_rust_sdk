//! Response structures for the moderations API

use super::{constants::CATEGORY_MAPPINGS, CategoryScores, ModerationCategories};
use crate::{De, Ser};

/// Response from moderations API
#[derive(Debug, Clone, Ser, De)]
pub struct ModerationResponse {
    /// Unique identifier for the moderation request
    pub id: String,

    /// Model used for moderation
    pub model: String,

    /// List of moderation results
    pub results: Vec<ModerationResult>,
}

/// Individual moderation result
#[derive(Debug, Clone, Ser, De)]
pub struct ModerationResult {
    /// Whether the content violates `OpenAI`'s usage policies
    pub flagged: bool,

    /// Category-specific flags
    pub categories: ModerationCategories,

    /// Category-specific confidence scores
    pub category_scores: CategoryScores,
}

impl ModerationResponse {
    /// Check if any of the results are flagged
    #[must_use]
    pub fn has_violations(&self) -> bool {
        self.results.iter().any(|result| result.flagged)
    }

    /// Get the highest confidence score across all categories and results
    pub fn max_confidence_score(&self) -> f64 {
        self.results
            .iter()
            .map(|result| result.category_scores.max_score())
            .fold(0.0, f64::max)
    }

    /// Get results that are flagged
    #[must_use]
    pub fn flagged_results(&self) -> Vec<&ModerationResult> {
        self.results
            .iter()
            .filter(|result| result.flagged)
            .collect()
    }

    /// Get the number of inputs processed
    #[must_use]
    pub fn input_count(&self) -> usize {
        self.results.len()
    }
}

impl ModerationResult {
    /// Check if this result has any violations in specific categories
    #[must_use]
    pub fn has_hate_violations(&self) -> bool {
        self.categories.hate || self.categories.hate_threatening
    }

    /// Check if this result has any self-harm related violations
    #[must_use]
    pub fn has_self_harm_violations(&self) -> bool {
        self.categories.self_harm
            || self.categories.self_harm_intent
            || self.categories.self_harm_instructions
    }

    /// Check if this result has any sexual content violations
    #[must_use]
    pub fn has_sexual_violations(&self) -> bool {
        self.categories.sexual || self.categories.sexual_minors
    }

    /// Check if this result has any violence related violations
    #[must_use]
    pub fn has_violence_violations(&self) -> bool {
        self.categories.violence || self.categories.violence_graphic
    }

    /// Check if this result has any harassment related violations
    #[must_use]
    pub fn has_harassment_violations(&self) -> bool {
        self.categories.harassment || self.categories.harassment_threatening
    }

    /// Get a list of violated categories as strings
    #[must_use]
    pub fn violated_categories(&self) -> Vec<String> {
        self.get_category_violations()
            .into_iter()
            .filter_map(|(name, is_violated)| {
                if is_violated {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Helper method to get all category names with their violation status
    fn get_category_violations(&self) -> Vec<(&'static str, bool)> {
        CATEGORY_MAPPINGS
            .iter()
            .map(|&(name, getter)| (name, getter(&self.categories)))
            .collect()
    }

    /// Get the highest confidence score for this result
    #[must_use]
    pub fn max_confidence_score(&self) -> f64 {
        self.category_scores.max_score()
    }
}
