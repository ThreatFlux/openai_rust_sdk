//! Core types and input handling for moderations

use crate::{De, Ser};

/// Input for moderation requests
#[derive(Debug, Clone, PartialEq, Ser, De)]
#[serde(untagged)]
pub enum ModerationInput {
    /// Single text string
    String(String),
    /// Array of text strings
    StringArray(Vec<String>),
}

/// Type alias for category getter function
pub type CategoryGetter = fn(&super::ModerationCategories) -> bool;

/// Type alias for score getter function
pub type ScoreGetter = fn(&super::CategoryScores) -> f64;
