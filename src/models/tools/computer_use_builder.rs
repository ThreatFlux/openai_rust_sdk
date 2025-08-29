//! Builder for computer use tool configurations

use super::{ComputerUseConfig, EnhancedTool};

/// Builder for computer use tools
pub struct ComputerUseBuilder {
    /// The computer use configuration being built
    config: ComputerUseConfig,
}

impl Default for ComputerUseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputerUseBuilder {
    /// Create a new ComputerUseBuilder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ComputerUseConfig {
                resolution: None,
                os_type: None,
                applications: None,
                max_actions: None,
            },
        }
    }

    /// Set the screen resolution (e.g., "1920x1080")
    pub fn resolution(mut self, res: impl Into<String>) -> Self {
        self.config.resolution = Some(res.into());
        self
    }

    /// Set the operating system type (e.g., "windows", "macos", "linux")
    pub fn os_type(mut self, os: impl Into<String>) -> Self {
        self.config.os_type = Some(os.into());
        self
    }

    /// Set the applications available for computer use
    #[must_use]
    pub fn applications(mut self, apps: Vec<String>) -> Self {
        self.config.applications = Some(apps);
        self
    }

    /// Set the maximum number of actions allowed
    #[must_use]
    pub fn max_actions(mut self, max: u32) -> Self {
        self.config.max_actions = Some(max);
        self
    }

    /// Build the configured computer use tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::ComputerUse(self.config)
    }
}
