//! Code interpreter tool configuration and types

use crate::{De, Ser};

/// Code interpreter configuration
#[derive(Debug, Clone, Ser, De)]
pub struct CodeInterpreterConfig {
    /// Container mode: "auto" or "explicit"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_mode: Option<crate::models::containers::ContainerMode>,

    /// Container ID for explicit mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,

    /// Container configuration for auto mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_config: Option<crate::models::containers::ContainerConfig>,

    /// Programming language to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Maximum execution time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_execution_time_ms: Option<u32>,

    /// Available libraries/packages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<String>>,

    /// File IDs accessible to the interpreter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    /// Whether to persist container after execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist_container: Option<bool>,

    /// Whether to include file citations in responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_citations: Option<bool>,
}
