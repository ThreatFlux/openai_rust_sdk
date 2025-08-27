//! Builder for code interpreter tool configurations

use super::*;

/// Builder for code interpreter tools
pub struct CodeInterpreterBuilder {
    /// The code interpreter configuration being built
    config: CodeInterpreterConfig,
}

impl Default for CodeInterpreterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeInterpreterBuilder {
    /// Create a new CodeInterpreterBuilder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: CodeInterpreterConfig {
                container_mode: None,
                container_id: None,
                container_config: None,
                language: None,
                max_execution_time_ms: None,
                libraries: None,
                file_ids: None,
                persist_container: None,
                include_citations: None,
            },
        }
    }

    /// Set the container mode for code execution
    #[must_use]
    pub fn container_mode(mut self, mode: crate::models::containers::ContainerMode) -> Self {
        self.config.container_mode = Some(mode);
        self
    }

    /// Set the container ID for explicit container mode
    pub fn container_id(mut self, id: impl Into<String>) -> Self {
        self.config.container_id = Some(id.into());
        self.config.container_mode = Some(crate::models::containers::ContainerMode::Explicit);
        self
    }

    /// Set the container configuration for auto mode
    #[must_use]
    pub fn container_config(mut self, config: crate::models::containers::ContainerConfig) -> Self {
        self.config.container_config = Some(config);
        self.config.container_mode = Some(crate::models::containers::ContainerMode::Auto);
        self
    }

    /// Set the programming language to use
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.config.language = Some(lang.into());
        self
    }

    /// Set the maximum execution time in milliseconds
    #[must_use]
    pub fn max_execution_time_ms(mut self, ms: u32) -> Self {
        self.config.max_execution_time_ms = Some(ms);
        self
    }

    /// Set the libraries to make available in the code interpreter
    #[must_use]
    pub fn libraries(mut self, libs: Vec<String>) -> Self {
        self.config.libraries = Some(libs);
        self
    }

    /// Set the file IDs to make available to the code interpreter
    #[must_use]
    pub fn file_ids(mut self, ids: Vec<String>) -> Self {
        self.config.file_ids = Some(ids);
        self
    }

    /// Enable or disable container persistence between executions
    #[must_use]
    pub fn persist_container(mut self, persist: bool) -> Self {
        self.config.persist_container = Some(persist);
        self
    }

    /// Enable or disable citation inclusion in outputs
    #[must_use]
    pub fn include_citations(mut self, include: bool) -> Self {
        self.config.include_citations = Some(include);
        self
    }

    /// Build the configured code interpreter tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::CodeInterpreter(self.config)
    }
}
