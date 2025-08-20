//! # Container Models
//!
//! Data structures for container management and code execution

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Container configuration for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    /// Container name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Container metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// Python version to use (e.g., "3.9", "3.10", "3.11")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_version: Option<String>,

    /// Pre-installed libraries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<String>>,

    /// Memory limit in MB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_mb: Option<u32>,

    /// CPU limit (0.5, 1.0, 2.0, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_limit: Option<f32>,

    /// Container expiration time in minutes (default: 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_minutes: Option<u32>,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            name: None,
            metadata: None,
            python_version: Some("3.11".to_string()),
            libraries: None,
            memory_limit_mb: None,
            cpu_limit: None,
            expiration_minutes: None,
        }
    }
}

/// Container instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    /// Unique container ID
    pub id: String,

    /// Container object type
    pub object: String,

    /// Container name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Container status
    pub status: ContainerStatus,

    /// Creation timestamp
    pub created_at: u64,

    /// Last activity timestamp
    pub last_activity_at: u64,

    /// Expiration timestamp
    pub expires_at: u64,

    /// Python version
    pub python_version: String,

    /// Installed libraries
    #[serde(default)]
    pub libraries: Vec<String>,

    /// Container metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,

    /// Files in the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<ContainerFile>>,

    /// Memory usage in MB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_usage_mb: Option<u32>,

    /// CPU usage percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_usage_percent: Option<f32>,
}

/// Container status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContainerStatus {
    /// Container is being created
    Creating,
    /// Container is ready for use
    Active,
    /// Container is idle (no recent activity)
    Idle,
    /// Container is expired
    Expired,
    /// Container is being deleted
    Deleting,
    /// Container has an error
    Error,
}

/// File in a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerFile {
    /// File ID
    pub id: String,

    /// File object type
    pub object: String,

    /// File name
    pub filename: String,

    /// File size in bytes
    pub size: u64,

    /// File MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// Creation timestamp
    pub created_at: u64,

    /// Last modified timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<u64>,

    /// File path in container
    pub path: String,

    /// Whether file is readable
    #[serde(default = "default_true")]
    pub readable: bool,

    /// Whether file is writable
    #[serde(default = "default_true")]
    pub writable: bool,

    /// Whether file is executable
    #[serde(default)]
    pub executable: bool,
}

fn default_true() -> bool {
    true
}

/// List of containers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerList {
    /// List object type
    pub object: String,

    /// List of containers
    pub data: Vec<Container>,

    /// First container ID in the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,

    /// Last container ID in the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Whether there are more containers
    pub has_more: bool,
}

/// List of files in a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerFileList {
    /// List object type
    pub object: String,

    /// List of files
    pub data: Vec<ContainerFile>,

    /// Total number of files
    pub total: u32,
}

/// Code execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionRequest {
    /// Python code to execute
    pub code: String,

    /// Execution timeout in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u32>,

    /// Whether to include output in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_output: Option<bool>,
}

/// Code execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionResult {
    /// Execution ID
    pub id: String,

    /// Execution status
    pub status: ExecutionStatus,

    /// Standard output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,

    /// Standard error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,

    /// Exit code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,

    /// Execution time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_ms: Option<u32>,

    /// Files created during execution
    #[serde(default)]
    pub created_files: Vec<ContainerFile>,

    /// Files modified during execution
    #[serde(default)]
    pub modified_files: Vec<ContainerFile>,

    /// Execution error if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Memory used during execution in MB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_used_mb: Option<u32>,

    /// Citations for code output
    #[serde(default)]
    pub citations: Vec<CodeCitation>,
}

/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    /// Code is being executed
    Running,
    /// Execution completed successfully
    Success,
    /// Execution failed with error
    Failed,
    /// Execution timed out
    Timeout,
    /// Execution was cancelled
    Cancelled,
}

/// Code citation for tracking sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeCitation {
    /// Citation type (e.g., "file", "url", "library")
    pub citation_type: String,

    /// Source of the citation
    pub source: String,

    /// Line numbers referenced (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<u32>>,

    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Parameters for listing containers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListContainersParams {
    /// Maximum number of containers to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Starting container ID for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Ending container ID for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Filter by status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ContainerStatus>,

    /// Order by creation date ("asc" or "desc")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// Enhanced Code Interpreter configuration with container support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedCodeInterpreterConfig {
    /// Container mode: "auto" or "explicit"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_mode: Option<ContainerMode>,

    /// Container ID for explicit mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,

    /// Container configuration for auto mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_config: Option<ContainerConfig>,

    /// Programming language (currently only Python)
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

/// Container mode for Code Interpreter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ContainerMode {
    /// Automatically create and manage containers
    #[default]
    Auto,
    /// Use explicitly created containers
    Explicit,
}

/// Builder for container configuration
pub struct ContainerBuilder {
    /// The container configuration being built
    config: ContainerConfig,
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerBuilder {
    /// Create a new container builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ContainerConfig::default(),
        }
    }

    /// Set container name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.config.name = Some(name.into());
        self
    }

    /// Set Python version
    pub fn python_version(mut self, version: impl Into<String>) -> Self {
        self.config.python_version = Some(version.into());
        self
    }

    /// Add libraries
    #[must_use]
    pub fn libraries(mut self, libs: Vec<String>) -> Self {
        self.config.libraries = Some(libs);
        self
    }

    /// Add a single library
    pub fn add_library(mut self, lib: impl Into<String>) -> Self {
        let libs = self.config.libraries.get_or_insert_with(Vec::new);
        libs.push(lib.into());
        self
    }

    /// Set memory limit in MB
    #[must_use]
    pub fn memory_limit_mb(mut self, limit: u32) -> Self {
        self.config.memory_limit_mb = Some(limit);
        self
    }

    /// Set CPU limit
    #[must_use]
    pub fn cpu_limit(mut self, limit: f32) -> Self {
        self.config.cpu_limit = Some(limit);
        self
    }

    /// Set expiration time in minutes
    #[must_use]
    pub fn expiration_minutes(mut self, minutes: u32) -> Self {
        self.config.expiration_minutes = Some(minutes);
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        let metadata = self.config.metadata.get_or_insert_with(HashMap::new);
        metadata.insert(key.into(), value);
        self
    }

    /// Build the container configuration
    #[must_use]
    pub fn build(self) -> ContainerConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_builder() {
        let config = ContainerBuilder::new()
            .name("test-container")
            .python_version("3.11")
            .add_library("numpy")
            .add_library("pandas")
            .memory_limit_mb(1024)
            .cpu_limit(2.0)
            .build();

        assert_eq!(config.name, Some("test-container".to_string()));
        assert_eq!(config.python_version, Some("3.11".to_string()));
        assert_eq!(
            config.libraries,
            Some(vec!["numpy".to_string(), "pandas".to_string()])
        );
        assert_eq!(config.memory_limit_mb, Some(1024));
        assert_eq!(config.cpu_limit, Some(2.0));
    }
}
