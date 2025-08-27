//! Type extensions and utilities for moderations API

use crate::models::moderations::ModerationResult;

/// Helper methods for working with moderation results
impl ModerationResult {
    /// Get a list of violations detected in the content
    #[must_use]
    pub fn get_violations(&self) -> Vec<String> {
        // Use the same data-driven approach as in the models module
        self.violated_categories()
    }
}
