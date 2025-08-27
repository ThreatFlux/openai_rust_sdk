//! # Models API Types
//!
//! Type definitions and data structures for the Models API.

use std::collections::HashMap;

/// Statistics about available models
#[derive(Debug, Clone)]
pub struct ModelStatistics {
    /// Total number of models
    pub total_models: usize,
    /// Number of available (non-deprecated) models
    pub available_models: usize,
    /// Number of deprecated models
    pub deprecated_models: usize,
    /// Distribution of models by family
    pub family_distribution: HashMap<String, usize>,
    /// Distribution of models by tier
    pub tier_distribution: HashMap<String, usize>,
    /// Distribution of models by completion type support
    pub completion_type_distribution: HashMap<String, usize>,
}

impl ModelStatistics {
    /// Get the deprecation rate as a percentage
    #[must_use]
    pub fn deprecation_rate(&self) -> f64 {
        if self.total_models == 0 {
            0.0
        } else {
            (self.deprecated_models as f64 / self.total_models as f64) * 100.0
        }
    }

    /// Get the availability rate as a percentage
    #[must_use]
    pub fn availability_rate(&self) -> f64 {
        if self.total_models == 0 {
            0.0
        } else {
            (self.available_models as f64 / self.total_models as f64) * 100.0
        }
    }

    /// Get the most common model family
    #[must_use]
    pub fn most_common_family(&self) -> Option<(&String, &usize)> {
        self.family_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
    }

    /// Get the most common model tier
    #[must_use]
    pub fn most_common_tier(&self) -> Option<(&String, &usize)> {
        self.tier_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
    }
}
