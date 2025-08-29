//! Builder for web search tool configurations

use super::{EnhancedTool, SearchFilters, WebSearchConfig};

/// Builder for web search configuration
pub struct WebSearchBuilder {
    /// The web search configuration being built
    config: WebSearchConfig,
}

impl Default for WebSearchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSearchBuilder {
    /// Create a new WebSearchBuilder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: WebSearchConfig {
                max_results: None,
                filters: None,
                time_range: None,
            },
        }
    }

    /// Set the maximum number of search results to return
    #[must_use]
    pub fn max_results(mut self, max: u32) -> Self {
        self.config.max_results = Some(max);
        self
    }

    /// Include only results from specified domains
    #[must_use]
    pub fn include_domains(mut self, domains: Vec<String>) -> Self {
        let filters = self.config.filters.get_or_insert(SearchFilters {
            include_domains: None,
            exclude_domains: None,
            language: None,
            region: None,
        });
        filters.include_domains = Some(domains);
        self
    }

    /// Exclude results from specified domains
    #[must_use]
    pub fn exclude_domains(mut self, domains: Vec<String>) -> Self {
        let filters = self.config.filters.get_or_insert(SearchFilters {
            include_domains: None,
            exclude_domains: None,
            language: None,
            region: None,
        });
        filters.exclude_domains = Some(domains);
        self
    }

    /// Set the time range for search results (e.g., "past_week", "past_month")
    pub fn time_range(mut self, range: impl Into<String>) -> Self {
        self.config.time_range = Some(range.into());
        self
    }

    /// Build the configured web search tool
    #[must_use]
    pub fn build(self) -> EnhancedTool {
        EnhancedTool::WebSearch(self.config)
    }
}
