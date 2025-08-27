//! Types and enums for the OpenAI Batch API

use crate::{De, Ser};
use serde::{Deserialize, Serialize};

/// Batch status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    /// The input file is being validated before the batch can begin
    Validating,
    /// The input file has failed the validation process
    Failed,
    /// The input file was successfully validated and the batch is currently being run
    InProgress,
    /// The batch has completed and the results are being prepared
    Finalizing,
    /// The batch has been completed and the results are ready
    Completed,
    /// The batch was not able to be completed within the 24-hour time window
    Expired,
    /// The batch is being cancelled (may take up to 10 minutes)
    Cancelling,
    /// The batch was cancelled
    Cancelled,
}

impl std::fmt::Display for BatchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Self::Validating => "validating",
            Self::Failed => "failed",
            Self::InProgress => "in_progress",
            Self::Finalizing => "finalizing",
            Self::Completed => "completed",
            Self::Expired => "expired",
            Self::Cancelling => "cancelling",
            Self::Cancelled => "cancelled",
        };
        write!(f, "{status}")
    }
}

/// Information about a YARA rule extracted from batch results
#[derive(Debug, Clone)]
pub struct YaraRuleInfo {
    /// Custom ID from the batch request
    pub custom_id: String,
    /// The extracted YARA rule content
    pub rule_content: String,
}

impl YaraRuleInfo {
    /// Create a new YARA rule info
    pub fn new(custom_id: String, rule_content: String) -> Self {
        Self {
            custom_id,
            rule_content,
        }
    }
}

/// Request counts for batch processing
#[derive(Debug, Clone, Ser, De)]
pub struct BatchRequestCounts {
    /// Total number of requests in the batch
    pub total: u32,
    /// Number of completed requests
    pub completed: u32,
    /// Number of failed requests
    pub failed: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_status_display() {
        assert_eq!(BatchStatus::Validating.to_string(), "validating");
        assert_eq!(BatchStatus::InProgress.to_string(), "in_progress");
        assert_eq!(BatchStatus::Completed.to_string(), "completed");
    }

    #[test]
    fn test_yara_rule_info() {
        let rule_info = YaraRuleInfo::new(
            "test_rule".to_string(),
            "rule TestRule { condition: true }".to_string(),
        );
        assert_eq!(rule_info.custom_id, "test_rule");
        assert!(rule_info.rule_content.contains("TestRule"));
    }
}
