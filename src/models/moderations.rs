//! # Moderations Models
//!
//! Data structures for the OpenAI Moderations API to classify content according to OpenAI's usage policies

use serde::{Deserialize, Serialize};

/// Request for content moderation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationRequest {
    /// Input text to moderate (string or array of strings)
    pub input: ModerationInput,

    /// ID of the model to use (e.g., "text-moderation-stable", "text-moderation-latest")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Input for moderation requests
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModerationInput {
    /// Single text string
    String(String),
    /// Array of text strings
    StringArray(Vec<String>),
}

/// Response from moderations API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationResponse {
    /// Unique identifier for the moderation request
    pub id: String,

    /// Model used for moderation
    pub model: String,

    /// List of moderation results
    pub results: Vec<ModerationResult>,
}

/// Individual moderation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationResult {
    /// Whether the content violates `OpenAI`'s usage policies
    pub flagged: bool,

    /// Category-specific flags
    pub categories: ModerationCategories,

    /// Category-specific confidence scores
    pub category_scores: CategoryScores,
}

/// Category flags indicating which policies are violated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationCategories {
    /// Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste
    pub hate: bool,

    /// Content that expresses, incites, or promotes harassing language towards any target
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,

    /// Content that promotes, encourages, or depicts acts of self-harm
    #[serde(rename = "self-harm")]
    pub self_harm: bool,

    /// Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,

    /// Content that encourages performing acts of self-harm
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,

    /// Content meant to arouse sexual excitement
    pub sexual: bool,

    /// Sexual content that includes an individual who is under 18 years old
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,

    /// Content that depicts death, violence, or physical injury
    pub violence: bool,

    /// Content that depicts death, violence, or physical injury in graphic detail
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,

    /// Content intended to harass, threaten, or bully an individual
    pub harassment: bool,

    /// Harassment content that also includes violence or serious harm towards any target
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
}

/// Confidence scores for each category (0.0 to 1.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl ModerationRequest {
    /// Create a new moderation request with a single string
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: ModerationInput::String(input.into()),
            model: None,
        }
    }

    /// Create a new moderation request with multiple strings
    #[must_use]
    pub fn new_batch(inputs: Vec<String>) -> Self {
        Self {
            input: ModerationInput::StringArray(inputs),
            model: None,
        }
    }

    /// Set the model to use for moderation
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
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
        let mut violations = Vec::new();

        if self.categories.hate {
            violations.push("hate".to_string());
        }
        if self.categories.hate_threatening {
            violations.push("hate/threatening".to_string());
        }
        if self.categories.self_harm {
            violations.push("self-harm".to_string());
        }
        if self.categories.self_harm_intent {
            violations.push("self-harm/intent".to_string());
        }
        if self.categories.self_harm_instructions {
            violations.push("self-harm/instructions".to_string());
        }
        if self.categories.sexual {
            violations.push("sexual".to_string());
        }
        if self.categories.sexual_minors {
            violations.push("sexual/minors".to_string());
        }
        if self.categories.violence {
            violations.push("violence".to_string());
        }
        if self.categories.violence_graphic {
            violations.push("violence/graphic".to_string());
        }
        if self.categories.harassment {
            violations.push("harassment".to_string());
        }
        if self.categories.harassment_threatening {
            violations.push("harassment/threatening".to_string());
        }

        violations
    }

    /// Get the highest confidence score for this result
    #[must_use]
    pub fn max_confidence_score(&self) -> f64 {
        self.category_scores.max_score()
    }
}

impl CategoryScores {
    /// Get the maximum score across all categories
    #[must_use]
    pub fn max_score(&self) -> f64 {
        [
            self.hate,
            self.hate_threatening,
            self.self_harm,
            self.self_harm_intent,
            self.self_harm_instructions,
            self.sexual,
            self.sexual_minors,
            self.violence,
            self.violence_graphic,
            self.harassment,
            self.harassment_threatening,
        ]
        .iter()
        .fold(0.0, |a, &b| f64::max(a, b))
    }

    /// Get scores above a certain threshold
    #[must_use]
    pub fn scores_above_threshold(&self, threshold: f64) -> Vec<(String, f64)> {
        let mut high_scores = Vec::new();

        if self.hate >= threshold {
            high_scores.push(("hate".to_string(), self.hate));
        }
        if self.hate_threatening >= threshold {
            high_scores.push(("hate/threatening".to_string(), self.hate_threatening));
        }
        if self.self_harm >= threshold {
            high_scores.push(("self-harm".to_string(), self.self_harm));
        }
        if self.self_harm_intent >= threshold {
            high_scores.push(("self-harm/intent".to_string(), self.self_harm_intent));
        }
        if self.self_harm_instructions >= threshold {
            high_scores.push((
                "self-harm/instructions".to_string(),
                self.self_harm_instructions,
            ));
        }
        if self.sexual >= threshold {
            high_scores.push(("sexual".to_string(), self.sexual));
        }
        if self.sexual_minors >= threshold {
            high_scores.push(("sexual/minors".to_string(), self.sexual_minors));
        }
        if self.violence >= threshold {
            high_scores.push(("violence".to_string(), self.violence));
        }
        if self.violence_graphic >= threshold {
            high_scores.push(("violence/graphic".to_string(), self.violence_graphic));
        }
        if self.harassment >= threshold {
            high_scores.push(("harassment".to_string(), self.harassment));
        }
        if self.harassment_threatening >= threshold {
            high_scores.push((
                "harassment/threatening".to_string(),
                self.harassment_threatening,
            ));
        }

        high_scores
    }
}

/// Common moderation models
pub struct ModerationModels;

impl ModerationModels {
    /// Stable moderation model (recommended for production)
    pub const STABLE: &'static str = "text-moderation-stable";

    /// Latest moderation model (may change over time)
    pub const LATEST: &'static str = "text-moderation-latest";
}

/// Builder for creating moderation requests
pub struct ModerationBuilder {
    request: ModerationRequest,
}

impl ModerationBuilder {
    /// Create a new builder with a single input
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            request: ModerationRequest::new(input),
        }
    }

    /// Create a new builder with multiple inputs
    #[must_use]
    pub fn new_batch(inputs: Vec<String>) -> Self {
        Self {
            request: ModerationRequest::new_batch(inputs),
        }
    }

    /// Set the model to use
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.request.model = Some(model.into());
        self
    }

    /// Use the stable model
    #[must_use]
    pub fn stable_model(mut self) -> Self {
        self.request.model = Some(ModerationModels::STABLE.to_string());
        self
    }

    /// Use the latest model
    #[must_use]
    pub fn latest_model(mut self) -> Self {
        self.request.model = Some(ModerationModels::LATEST.to_string());
        self
    }

    /// Build the request
    #[must_use]
    pub fn build(self) -> ModerationRequest {
        self.request
    }
}

/// Safety threshold presets for interpreting confidence scores
pub struct SafetyThresholds;

impl SafetyThresholds {
    /// Conservative threshold - flags content with lower confidence scores
    pub const CONSERVATIVE: f64 = 0.1;

    /// Moderate threshold - balanced approach
    pub const MODERATE: f64 = 0.3;

    /// Permissive threshold - only flags content with high confidence scores
    pub const PERMISSIVE: f64 = 0.7;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moderation_request_creation() {
        let req = ModerationRequest::new("Test content");
        matches!(req.input, ModerationInput::String(s) if s == "Test content");
        assert!(req.model.is_none());
    }

    #[test]
    fn test_moderation_builder() {
        let req = ModerationBuilder::new("Test content")
            .stable_model()
            .build();

        assert_eq!(req.model, Some(ModerationModels::STABLE.to_string()));
    }

    #[test]
    fn test_batch_moderation() {
        let inputs = vec!["First".to_string(), "Second".to_string()];
        let req = ModerationRequest::new_batch(inputs.clone());

        match req.input {
            ModerationInput::StringArray(arr) => assert_eq!(arr, inputs),
            _ => panic!("Expected StringArray"),
        }
    }

    #[test]
    fn test_category_scores_max() {
        let scores = CategoryScores {
            hate: 0.1,
            hate_threatening: 0.2,
            self_harm: 0.9,
            self_harm_intent: 0.0,
            self_harm_instructions: 0.0,
            sexual: 0.3,
            sexual_minors: 0.0,
            violence: 0.4,
            violence_graphic: 0.0,
            harassment: 0.5,
            harassment_threatening: 0.0,
        };

        assert_eq!(scores.max_score(), 0.9);
    }

    #[test]
    fn test_scores_above_threshold() {
        let scores = CategoryScores {
            hate: 0.1,
            hate_threatening: 0.8,
            self_harm: 0.9,
            self_harm_intent: 0.0,
            self_harm_instructions: 0.0,
            sexual: 0.3,
            sexual_minors: 0.0,
            violence: 0.4,
            violence_graphic: 0.0,
            harassment: 0.5,
            harassment_threatening: 0.0,
        };

        let high_scores = scores.scores_above_threshold(0.7);
        assert_eq!(high_scores.len(), 2);
        assert!(high_scores.contains(&("hate/threatening".to_string(), 0.8)));
        assert!(high_scores.contains(&("self-harm".to_string(), 0.9)));
    }

    #[test]
    fn test_moderation_result_violation_checks() {
        let categories = ModerationCategories {
            hate: true,
            hate_threatening: false,
            self_harm: false,
            self_harm_intent: false,
            self_harm_instructions: false,
            sexual: false,
            sexual_minors: false,
            violence: true,
            violence_graphic: false,
            harassment: false,
            harassment_threatening: false,
        };

        let scores = CategoryScores {
            hate: 0.8,
            hate_threatening: 0.1,
            self_harm: 0.1,
            self_harm_intent: 0.0,
            self_harm_instructions: 0.0,
            sexual: 0.1,
            sexual_minors: 0.0,
            violence: 0.9,
            violence_graphic: 0.1,
            harassment: 0.1,
            harassment_threatening: 0.0,
        };

        let result = ModerationResult {
            flagged: true,
            categories,
            category_scores: scores,
        };

        assert!(result.has_hate_violations());
        assert!(!result.has_self_harm_violations());
        assert!(!result.has_sexual_violations());
        assert!(result.has_violence_violations());
        assert!(!result.has_harassment_violations());

        let violations = result.violated_categories();
        assert_eq!(violations.len(), 2);
        assert!(violations.contains(&"hate".to_string()));
        assert!(violations.contains(&"violence".to_string()));
    }
}
