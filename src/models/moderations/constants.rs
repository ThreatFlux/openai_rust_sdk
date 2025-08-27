//! Constants and mappings for moderation categories

use super::{CategoryGetter, CategoryScores, ModerationCategories, ScoreGetter};

/// Mapping of category names to their getter functions
pub const CATEGORY_MAPPINGS: &[(&str, CategoryGetter)] = &[
    ("hate", |c| c.hate),
    ("hate/threatening", |c| c.hate_threatening),
    ("self-harm", |c| c.self_harm),
    ("self-harm/intent", |c| c.self_harm_intent),
    ("self-harm/instructions", |c| c.self_harm_instructions),
    ("sexual", |c| c.sexual),
    ("sexual/minors", |c| c.sexual_minors),
    ("violence", |c| c.violence),
    ("violence/graphic", |c| c.violence_graphic),
    ("harassment", |c| c.harassment),
    ("harassment/threatening", |c| c.harassment_threatening),
];

/// Mapping of category names to their score getter functions
pub const SCORE_MAPPINGS: &[(&str, ScoreGetter)] = &[
    ("hate", |s| s.hate),
    ("hate/threatening", |s| s.hate_threatening),
    ("self-harm", |s| s.self_harm),
    ("self-harm/intent", |s| s.self_harm_intent),
    ("self-harm/instructions", |s| s.self_harm_instructions),
    ("sexual", |s| s.sexual),
    ("sexual/minors", |s| s.sexual_minors),
    ("violence", |s| s.violence),
    ("violence/graphic", |s| s.violence_graphic),
    ("harassment", |s| s.harassment),
    ("harassment/threatening", |s| s.harassment_threatening),
];

/// Common moderation models
pub struct ModerationModels;

impl ModerationModels {
    /// Stable moderation model (recommended for production)
    pub const STABLE: &'static str = "text-moderation-stable";

    /// Latest moderation model (may change over time)
    pub const LATEST: &'static str = "text-moderation-latest";
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
