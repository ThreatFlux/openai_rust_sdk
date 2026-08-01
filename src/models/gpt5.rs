//! GPT-5 model constants and configuration

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};

/// GPT-5 model constants
pub mod models {
    /// GPT-5.6 Sol - latest high-end reasoning model.
    pub const GPT_5_6_SOL: &str = "gpt-5.6-sol";
    /// GPT-5.6 Terra - latest balanced reasoning model.
    pub const GPT_5_6_TERRA: &str = "gpt-5.6-terra";
    /// GPT-5.6 Luna - latest efficient reasoning model.
    pub const GPT_5_6_LUNA: &str = "gpt-5.6-luna";
    /// GPT-5.4 - current general-purpose reasoning model.
    pub const GPT_5_4: &str = "gpt-5.4";
    /// GPT-5.4 Mini - current compact reasoning model.
    pub const GPT_5_4_MINI: &str = "gpt-5.4-mini";
    /// GPT-5.4 Nano - current high-throughput reasoning model.
    pub const GPT_5_4_NANO: &str = "gpt-5.4-nano";
    /// GPT-5.3 Chat Latest - current chat-optimized model.
    pub const GPT_5_3_CHAT_LATEST: &str = "gpt-5.3-chat-latest";

    /// GPT-5 models - Latest reasoning models
    pub const GPT_5: &str = "gpt-5";
    /// GPT-5 Mini - Smaller, faster version of GPT-5
    pub const GPT_5_MINI: &str = "gpt-5-mini";
    /// GPT-5 Nano - Smallest, fastest version of GPT-5
    pub const GPT_5_NANO: &str = "gpt-5-nano";
    /// GPT-5 Chat Latest - Latest chat-optimized GPT-5 model
    pub const GPT_5_CHAT_LATEST: &str = "gpt-5-chat-latest";

    /// Model snapshots with dates
    pub const GPT_5_2025_08_07: &str = "gpt-5-2025-08-07";
    /// GPT-5 snapshot from 2025-01-01 (deprecated; retained for compatibility).
    #[deprecated(note = "use GPT_5_2025_08_07")]
    pub const GPT_5_2025_01_01: &str = "gpt-5-2025-01-01";
    /// GPT-5 Mini snapshot from 2025-08-07.
    pub const GPT_5_MINI_2025_08_07: &str = "gpt-5-mini-2025-08-07";
    /// GPT-5 Mini snapshot from 2025-01-01 (deprecated; retained for compatibility).
    #[deprecated(note = "use GPT_5_MINI_2025_08_07")]
    pub const GPT_5_MINI_2025_01_01: &str = "gpt-5-mini-2025-01-01";
    /// GPT-5 Nano snapshot from 2025-08-07.
    pub const GPT_5_NANO_2025_08_07: &str = "gpt-5-nano-2025-08-07";
    /// GPT-5 Nano snapshot from 2025-01-01 (deprecated; retained for compatibility).
    #[deprecated(note = "use GPT_5_NANO_2025_08_07")]
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
#[derive(Debug, Clone, Copy, Ser, De, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    /// Disable reasoning where supported.
    None,
    /// Very few reasoning tokens for fastest time-to-first-token
    Minimal,
    /// Favors speed and fewer tokens (default for o3-like behavior)
    Low,
    /// Balanced reasoning (default)
    #[default]
    Medium,
    /// More thorough reasoning for complex tasks
    High,
    /// Extra-high reasoning effort.
    XHigh,
    /// Maximum reasoning effort for models that support it.
    Max,
}

/// Reasoning execution mode.
#[derive(Debug, Clone, Copy, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningMode {
    /// Standard reasoning execution.
    Standard,
    /// Pro reasoning execution.
    Pro,
}

/// Reasoning summary detail requested from the model.
#[derive(Debug, Clone, Copy, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummary {
    /// Let the model choose the summary detail.
    Auto,
    /// Return a concise summary.
    Concise,
    /// Return a detailed summary.
    Detailed,
}

/// Which reasoning items should be carried into later turns.
#[derive(Debug, Clone, Copy, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningContext {
    /// Let the model choose the context mode.
    Auto,
    /// Keep only the current turn's reasoning.
    CurrentTurn,
    /// Keep reasoning from all turns.
    AllTurns,
}

/// Prompt-cache retention mode for GPT-5.6 and later.
#[derive(Debug, Clone, Copy, Ser, De, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PromptCacheMode {
    /// Allow the service to create an implicit breakpoint.
    Implicit,
    /// Use only explicit breakpoints.
    Explicit,
}

/// Prompt-cache time-to-live.
#[derive(Debug, Clone, Copy, Ser, De, PartialEq, Eq)]
pub enum PromptCacheTtl {
    /// Retain cache entries for at least thirty minutes.
    #[serde(rename = "30m")]
    ThirtyMinutes,
}

/// Prompt-cache controls for GPT-5.6 and later.
#[derive(Debug, Clone, Ser, De, Default)]
pub struct PromptCacheOptions {
    /// Minimum cache retention period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<PromptCacheTtl>,
    /// Implicit or explicit breakpoint behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<PromptCacheMode>,
}

/// Verbosity levels for GPT-5 output
#[derive(Debug, Clone, Copy, Ser, De, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    /// Concise answers with minimal commentary
    Low,
    /// Balanced output (default)
    #[default]
    Medium,
    /// Thorough explanations and detailed responses
    High,
}

/// Reasoning configuration for GPT-5 models
#[derive(Debug, Clone, Ser, De, Default)]
pub struct ReasoningConfig {
    /// Standard or pro reasoning execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ReasoningMode>,
    /// The effort level for reasoning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
    /// Summary detail to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummary>,
    /// Reasoning context retained across turns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReasoningContext>,
}

impl ReasoningConfig {
    /// Create a new reasoning config with specified effort
    #[must_use]
    pub fn new(effort: ReasoningEffort) -> Self {
        Self {
            mode: None,
            effort: Some(effort),
            summary: None,
            context: None,
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

    /// Create a maximum-effort reasoning config.
    #[must_use]
    pub fn max() -> Self {
        Self::new(ReasoningEffort::Max)
    }

    /// Set the reasoning execution mode.
    #[must_use]
    pub fn with_mode(mut self, mode: ReasoningMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set the reasoning summary detail.
    #[must_use]
    pub fn with_summary(mut self, summary: ReasoningSummary) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Set the cross-turn reasoning context.
    #[must_use]
    pub fn with_context(mut self, context: ReasoningContext) -> Self {
        self.context = Some(context);
        self
    }
}

/// Text output configuration for GPT-5 models
#[derive(Debug, Clone, Ser, De, Default)]
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
        models::GPT_5_6_SOL
    }

    /// Select the best model for cost-optimized reasoning
    #[must_use]
    pub fn for_cost_optimized() -> &'static str {
        models::GPT_5_4_MINI
    }

    /// Select the best model for high-throughput tasks
    #[must_use]
    pub fn for_high_throughput() -> &'static str {
        models::GPT_5_4_NANO
    }

    /// Select the best model for coding tasks
    #[must_use]
    pub fn for_coding() -> &'static str {
        models::GPT_5_4
    }

    /// Select the best model for chat applications
    #[must_use]
    pub fn for_chat() -> &'static str {
        models::GPT_5_3_CHAT_LATEST
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn current_reasoning_configuration_serializes() {
        let config = ReasoningConfig::max()
            .with_mode(ReasoningMode::Pro)
            .with_summary(ReasoningSummary::Detailed)
            .with_context(ReasoningContext::AllTurns);
        let value = serde_json::to_value(config).expect("reasoning JSON");

        assert_eq!(value["effort"], "max");
        assert_eq!(value["mode"], "pro");
        assert_eq!(value["summary"], "detailed");
        assert_eq!(value["context"], "all_turns");
    }

    #[test]
    fn current_prompt_cache_configuration_serializes() {
        let options = PromptCacheOptions {
            ttl: Some(PromptCacheTtl::ThirtyMinutes),
            mode: Some(PromptCacheMode::Explicit),
        };
        let value = serde_json::to_value(options).expect("prompt cache JSON");
        assert_eq!(value, json!({"ttl": "30m", "mode": "explicit"}));
    }

    #[test]
    fn current_model_selector_prefers_latest_models() {
        assert_eq!(
            GPT5ModelSelector::for_complex_reasoning(),
            models::GPT_5_6_SOL
        );
        assert_eq!(
            GPT5ModelSelector::for_cost_optimized(),
            models::GPT_5_4_MINI
        );
        assert_eq!(
            GPT5ModelSelector::for_high_throughput(),
            models::GPT_5_4_NANO
        );
        assert_eq!(GPT5ModelSelector::for_coding(), models::GPT_5_4);
        assert_eq!(GPT5ModelSelector::for_chat(), models::GPT_5_3_CHAT_LATEST);
    }
}
