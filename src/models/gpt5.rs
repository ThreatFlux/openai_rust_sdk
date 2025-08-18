//! GPT-5 model constants and configuration

use serde::{Deserialize, Serialize};

/// GPT-5 model constants
pub mod models {
    /// GPT-5 models - Latest reasoning models
    pub const GPT_5: &str = "gpt-5";
    /// GPT-5 Mini - Smaller, faster version of GPT-5
    pub const GPT_5_MINI: &str = "gpt-5-mini";
    /// GPT-5 Nano - Smallest, fastest version of GPT-5
    pub const GPT_5_NANO: &str = "gpt-5-nano";
    /// GPT-5 Chat Latest - Latest chat-optimized GPT-5 model
    pub const GPT_5_CHAT_LATEST: &str = "gpt-5-chat-latest";

    /// Model snapshots with dates
    pub const GPT_5_2025_01_01: &str = "gpt-5-2025-01-01";
    /// GPT-5 Mini snapshot from 2025-01-01
    pub const GPT_5_MINI_2025_01_01: &str = "gpt-5-mini-2025-01-01";
    /// GPT-5 Nano snapshot from 2025-01-01
    pub const GPT_5_NANO_2025_01_01: &str = "gpt-5-nano-2025-01-01";

    /// GPT-4.1 models
    pub const GPT_4_1: &str = "gpt-4.1";
    /// GPT-4.1 Mini - Smaller version of GPT-4.1
    pub const GPT_4_1_MINI: &str = "gpt-4.1-mini";
    /// GPT-4.1 Nano - Smallest version of GPT-4.1
    pub const GPT_4_1_NANO: &str = "gpt-4.1-nano";

    /// GPT-4 models
    pub const GPT_4: &str = "gpt-4";
    /// GPT-4 Turbo - Previous generation turbo model
    pub const GPT_4_TURBO: &str = "gpt-4-turbo";

    /// GPT-3.5 models
    pub const GPT_3_5_TURBO: &str = "gpt-3.5-turbo";

    /// O-series reasoning models (legacy)
    pub const O3: &str = "o3";
    /// O4 Mini - Legacy reasoning model
    pub const O4_MINI: &str = "o4-mini";
}

/// Reasoning effort levels for GPT-5 models
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    /// Very few reasoning tokens for fastest time-to-first-token
    Minimal,
    /// Favors speed and fewer tokens (default for o3-like behavior)
    Low,
    /// Balanced reasoning (default)
    Medium,
    /// More thorough reasoning for complex tasks
    High,
}

impl Default for ReasoningEffort {
    fn default() -> Self {
        Self::Medium
    }
}

/// Verbosity levels for GPT-5 output
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    /// Concise answers with minimal commentary
    Low,
    /// Balanced output (default)
    Medium,
    /// Thorough explanations and detailed responses
    High,
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::Medium
    }
}

/// Reasoning configuration for GPT-5 models
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReasoningConfig {
    /// The effort level for reasoning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
}

impl ReasoningConfig {
    /// Create a new reasoning config with specified effort
    #[must_use]
    pub fn new(effort: ReasoningEffort) -> Self {
        Self {
            effort: Some(effort),
        }
    }

    /// Create minimal reasoning config for fastest responses
    #[must_use]
    pub fn minimal() -> Self {
        Self::new(ReasoningEffort::Minimal)
    }

    /// Create low reasoning config for speed
    #[must_use]
    pub fn low() -> Self {
        Self::new(ReasoningEffort::Low)
    }

    /// Create medium reasoning config (default)
    #[must_use]
    pub fn medium() -> Self {
        Self::new(ReasoningEffort::Medium)
    }

    /// Create high reasoning config for complex tasks
    #[must_use]
    pub fn high() -> Self {
        Self::new(ReasoningEffort::High)
    }
}

/// Text output configuration for GPT-5 models
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextConfig {
    /// The verbosity level for output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<Verbosity>,

    /// Format for the text output (for structured outputs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<serde_json::Value>,
}

impl TextConfig {
    /// Create a new text config with specified verbosity
    #[must_use]
    pub fn new(verbosity: Verbosity) -> Self {
        Self {
            verbosity: Some(verbosity),
            format: None,
        }
    }

    /// Create low verbosity config for concise responses
    #[must_use]
    pub fn low() -> Self {
        Self::new(Verbosity::Low)
    }

    /// Create medium verbosity config (default)
    #[must_use]
    pub fn medium() -> Self {
        Self::new(Verbosity::Medium)
    }

    /// Create high verbosity config for detailed responses
    #[must_use]
    pub fn high() -> Self {
        Self::new(Verbosity::High)
    }

    /// Set the format for structured outputs
    #[must_use]
    pub fn with_format(mut self, format: serde_json::Value) -> Self {
        self.format = Some(format);
        self
    }
}

/// GPT-5 model selection helper
pub struct GPT5ModelSelector;

impl GPT5ModelSelector {
    /// Select the best model for complex reasoning tasks
    #[must_use]
    pub fn for_complex_reasoning() -> &'static str {
        models::GPT_5
    }

    /// Select the best model for cost-optimized reasoning
    #[must_use]
    pub fn for_cost_optimized() -> &'static str {
        models::GPT_5_MINI
    }

    /// Select the best model for high-throughput tasks
    #[must_use]
    pub fn for_high_throughput() -> &'static str {
        models::GPT_5_NANO
    }

    /// Select the best model for coding tasks
    #[must_use]
    pub fn for_coding() -> &'static str {
        models::GPT_5
    }

    /// Select the best model for chat applications
    #[must_use]
    pub fn for_chat() -> &'static str {
        models::GPT_5_CHAT_LATEST
    }

    /// Get migration recommendation from an older model
    #[must_use]
    pub fn migration_from(old_model: &str) -> &'static str {
        match old_model {
            "o3" => models::GPT_5,
            "gpt-4.1" | "gpt-4" | "gpt-4-turbo" => models::GPT_5,
            "o4-mini" | "gpt-4.1-mini" => models::GPT_5_MINI,
            "gpt-4.1-nano" | "gpt-3.5-turbo" => models::GPT_5_NANO,
            _ => models::GPT_5,
        }
    }
}
