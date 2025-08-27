//! Computer use tool configuration and types

use crate::{De, Ser};

/// Computer use configuration for agentic workflows
#[derive(Debug, Clone, Ser, De)]
pub struct ComputerUseConfig {
    /// Screen resolution for the virtual display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    /// Operating system type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,

    /// Available applications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<String>>,

    /// Maximum action count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_actions: Option<u32>,
}
