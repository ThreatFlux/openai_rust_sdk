//! Web search tool configuration and types

use crate::{De, Ser};

/// Web search configuration
#[derive(Debug, Clone, Ser, De)]
pub struct WebSearchConfig {
    /// Maximum number of search results to include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,

    /// Search query filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<SearchFilters>,

    /// Time range for search results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
}

/// Search filters for web search
#[derive(Debug, Clone, Ser, De)]
pub struct SearchFilters {
    /// Domains to include in search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_domains: Option<Vec<String>>,

    /// Domains to exclude from search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,

    /// Language filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Region filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
