//! # Model Helpers
//!
//! Utility functions and helpers for working with models.

use crate::models::models::{Model, ModelCapabilities};

/// Utility functions for working with models
pub struct ModelUtils;

impl ModelUtils {
    /// Extract the base model name from a versioned model ID
    ///
    /// # Examples
    ///
    /// ```
    /// use openai_rust_sdk::api::models::helpers::ModelUtils;
    ///
    /// assert_eq!(
    ///     ModelUtils::extract_base_model_name("gpt-3.5-turbo-0613"),
    ///     "gpt-3.5-turbo"
    /// );
    /// assert_eq!(ModelUtils::extract_base_model_name("gpt-4-32k"), "gpt-4");
    /// ```
    #[must_use]
    pub fn extract_base_model_name(model_id: &str) -> String {
        // Remove common version suffixes
        let suffixes = [
            "-0613",
            "-0314",
            "-0301",
            "-16k",
            "-32k",
            "-instruct",
            "-chat",
        ];

        let mut base_name = model_id.to_string();
        for suffix in &suffixes {
            if let Some(pos) = base_name.rfind(suffix) {
                base_name = base_name[..pos].to_string();
            }
        }

        base_name
    }

    /// Check if two models are from the same family
    ///
    /// # Examples
    ///
    /// ```
    /// use openai_rust_sdk::api::models::helpers::ModelUtils;
    ///
    /// assert!(ModelUtils::are_same_family(
    ///     "gpt-3.5-turbo",
    ///     "gpt-3.5-turbo-16k"
    /// ));
    /// assert!(!ModelUtils::are_same_family("gpt-4", "dall-e-3"));
    /// ```
    #[must_use]
    pub fn are_same_family(model_id_1: &str, model_id_2: &str) -> bool {
        ModelCapabilities::classify_family(model_id_1)
            == ModelCapabilities::classify_family(model_id_2)
    }

    /// Get the newest model from a list by creation timestamp
    #[must_use]
    pub fn get_newest_model(models: &[Model]) -> Option<&Model> {
        models.iter().max_by_key(|model| model.created)
    }

    /// Get the oldest model from a list by creation timestamp
    #[must_use]
    pub fn get_oldest_model(models: &[Model]) -> Option<&Model> {
        models.iter().min_by_key(|model| model.created)
    }

    /// Sort models by preference (non-deprecated, premium tier, higher context)
    ///
    /// This function sorts models in place, prioritizing:
    /// 1. Non-deprecated models first
    /// 2. Higher tier models
    /// 3. Models with higher maximum token limits
    /// 4. Newer models by creation date
    pub fn sort_by_preference(models: &mut [Model]) {
        models.sort_by(|a, b| {
            let a_caps = a.capabilities();
            let b_caps = b.capabilities();

            // First: non-deprecated models
            match (a.is_deprecated(), b.is_deprecated()) {
                (false, true) => return std::cmp::Ordering::Less,
                (true, false) => return std::cmp::Ordering::Greater,
                _ => {}
            }

            // Second: tier
            match a_caps.tier.partial_cmp(&b_caps.tier) {
                Some(std::cmp::Ordering::Equal) => {}
                Some(ordering) => return ordering.reverse(), // Higher tier first
                None => {}
            }

            // Third: max tokens
            match (a_caps.max_tokens, b_caps.max_tokens) {
                (Some(a_tokens), Some(b_tokens)) => {
                    match a_tokens.cmp(&b_tokens) {
                        std::cmp::Ordering::Equal => {}
                        ordering => return ordering.reverse(), // Higher context first
                    }
                }
                (Some(_), None) => return std::cmp::Ordering::Less,
                (None, Some(_)) => return std::cmp::Ordering::Greater,
                _ => {}
            }

            // Finally: creation date (newer first)
            b.created.cmp(&a.created)
        });
    }

    /// Filter models by a minimum context window size
    ///
    /// Returns models that have at least the specified number of maximum tokens.
    #[must_use]
    pub fn filter_by_min_context(models: &[Model], min_tokens: u32) -> Vec<&Model> {
        models
            .iter()
            .filter(|model| {
                model
                    .capabilities()
                    .max_tokens
                    .is_some_and(|tokens| tokens >= min_tokens)
            })
            .collect()
    }

    /// Get models that support function calling
    #[must_use]
    pub fn get_function_calling_models(models: &[Model]) -> Vec<&Model> {
        models
            .iter()
            .filter(|model| model.capabilities().supports_function_calling)
            .collect()
    }

    /// Get models that support vision/image analysis
    #[must_use]
    pub fn get_vision_models(models: &[Model]) -> Vec<&Model> {
        models
            .iter()
            .filter(|model| model.capabilities().supports_vision)
            .collect()
    }

    /// Calculate the average creation timestamp for a set of models
    #[must_use]
    pub fn average_creation_time(models: &[Model]) -> Option<i64> {
        if models.is_empty() {
            return None;
        }

        let sum: i64 = models.iter().map(|m| m.created as i64).sum();
        Some(sum / models.len() as i64)
    }
}
