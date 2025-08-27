//! # Moderations Models - Modular Version
//!
//! Data structures for the OpenAI Moderations API to classify content according to OpenAI's usage policies

use super::moderations::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create test category scores
    fn create_test_category_scores() -> CategoryScores {
        CategoryScores {
            hate: 0.1,
            hate_threatening: 0.1,
            self_harm: 0.1,
            self_harm_intent: 0.1,
            self_harm_instructions: 0.1,
            sexual: 0.1,
            sexual_minors: 0.1,
            violence: 0.1,
            violence_graphic: 0.1,
            harassment: 0.1,
            harassment_threatening: 0.1,
        }
    }

    /// Helper function to create categories with no violations
    fn create_no_violations_categories() -> ModerationCategories {
        ModerationCategories {
            hate: false,
            hate_threatening: false,
            self_harm: false,
            self_harm_intent: false,
            self_harm_instructions: false,
            sexual: false,
            sexual_minors: false,
            violence: false,
            violence_graphic: false,
            harassment: false,
            harassment_threatening: false,
        }
    }

    /// Helper function to create categories with all violations
    fn create_all_violations_categories() -> ModerationCategories {
        ModerationCategories {
            hate: true,
            hate_threatening: true,
            self_harm: true,
            self_harm_intent: true,
            self_harm_instructions: true,
            sexual: true,
            sexual_minors: true,
            violence: true,
            violence_graphic: true,
            harassment: true,
            harassment_threatening: true,
        }
    }

    /// Helper function to verify all expected categories are present
    fn verify_all_categories_present(violations_list: &[String]) {
        let expected_categories = vec![
            "hate",
            "hate/threatening",
            "self-harm",
            "self-harm/intent",
            "self-harm/instructions",
            "sexual",
            "sexual/minors",
            "violence",
            "violence/graphic",
            "harassment",
            "harassment/threatening",
        ];

        for category in expected_categories {
            assert!(violations_list.contains(&category.to_string()));
        }
    }

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

    #[test]
    fn test_violated_categories_comprehensive() {
        let scores = create_test_category_scores();

        // Test cases: (flagged, categories_creator, expected_count, test_description)
        let test_cases = [
            (
                false,
                create_no_violations_categories as fn() -> ModerationCategories,
                0,
                "no violations",
            ),
            (
                true,
                create_all_violations_categories as fn() -> ModerationCategories,
                11,
                "all violations",
            ),
        ];

        for (flagged, categories_creator, expected_count, description) in test_cases {
            let result = ModerationResult {
                flagged,
                categories: categories_creator(),
                category_scores: scores.clone(),
            };

            let violations = result.violated_categories();
            assert_eq!(
                violations.len(),
                expected_count,
                "Failed test case: {}",
                description
            );

            if expected_count > 0 {
                verify_all_categories_present(&violations);
            }
        }
    }
}
