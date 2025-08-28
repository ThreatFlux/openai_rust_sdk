//! Request parameter types for vector store operations
//!
//! This module contains parameter types for listing and filtering vector store
//! operations, providing convenient builders for API requests.

use crate::models::vector_stores::common_types::QueryParamBuilder;
use crate::models::vector_stores::status_types::VectorStoreFileStatus;
use crate::{impl_list_params_with_builder, impl_query_params};

// Generate ListVectorStoresParams using our macro
impl_list_params_with_builder! {
    ListVectorStoresParams {
        base_fields: {
            limit: "Maximum number of vector stores to return (default 20, max 100)",
            order: "Sort order for the results (desc for descending, asc for ascending)",
            after: "Pagination cursor - list vector stores after this ID",
            before: "Pagination cursor - list vector stores before this ID"
        }
    }
}

// Generate ListVectorStoreFilesParams using our macro
impl_list_params_with_builder! {
    ListVectorStoreFilesParams {
        base_fields: {
            limit: "Maximum number of files to return (default 20, max 100)",
            order: "Sort order for the results (desc for descending, asc for ascending)",
            after: "Pagination cursor - list files after this ID",
            before: "Pagination cursor - list files before this ID"
        },
        extra_fields: {
            filter: VectorStoreFileStatus = "Filter files by status"
        }
    }
}

impl ListVectorStoreFilesParams {
    /// Set the status filter (custom method for better API)
    #[must_use]
    pub fn with_filter(mut self, filter: VectorStoreFileStatus) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Common query parameter building utilities
pub mod utils {
    /// Build a URL query string from parameters
    #[must_use]
    pub fn build_query_string(params: &[(String, String)]) -> String {
        if params.is_empty() {
            return String::new();
        }

        let query_parts: Vec<String> = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect();

        format!("?{}", query_parts.join("&"))
    }

    /// Validate limit parameter (must be between 1 and 100)
    #[must_use]
    pub fn validate_limit(limit: u32) -> bool {
        (1..=100).contains(&limit)
    }

    /// Validate order parameter (must be "asc" or "desc")
    #[must_use]
    pub fn validate_order(order: &str) -> bool {
        matches!(order, "asc" | "desc")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_vector_stores_params() {
        let params = ListVectorStoresParams::new()
            .with_limit(50)
            .with_order("desc")
            .with_after("vs-123");

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&("limit".to_string(), "50".to_string())));
        assert!(query_params.contains(&("order".to_string(), "desc".to_string())));
        assert!(query_params.contains(&("after".to_string(), "vs-123".to_string())));
        assert!(!params.is_empty());
    }

    #[test]
    fn test_list_vector_store_files_params() {
        let params = ListVectorStoreFilesParams::new()
            .with_limit(25)
            .with_filter(VectorStoreFileStatus::Completed)
            .with_order("asc");

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&("limit".to_string(), "25".to_string())));
        assert!(query_params.contains(&("filter".to_string(), "completed".to_string())));
        assert!(query_params.contains(&("order".to_string(), "asc".to_string())));
        assert!(!params.is_empty());
        assert!(!params.has_pagination());
        assert!(params.has_filters());
    }

    #[test]
    fn test_list_params_pagination() {
        let params = ListVectorStoreFilesParams::new()
            .with_after("file-123")
            .with_before("file-456");

        assert!(params.has_pagination());
        assert!(!params.has_filters());

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 2);
    }

    #[test]
    fn test_empty_params() {
        let empty_stores_params = ListVectorStoresParams::new();
        assert!(empty_stores_params.is_empty());
        assert!(empty_stores_params.to_query_params().is_empty());

        let empty_files_params = ListVectorStoreFilesParams::new();
        assert!(empty_files_params.is_empty());
        assert!(empty_files_params.to_query_params().is_empty());
        assert!(!empty_files_params.has_pagination());
        assert!(!empty_files_params.has_filters());
    }

    #[test]
    fn test_params_fluent_interface() {
        let params = ListVectorStoresParams::new()
            .with_limit(10)
            .with_order("asc")
            .with_after("vs-001")
            .with_before("vs-999");

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 4);
    }

    #[test]
    fn test_utils_build_query_string() {
        let params = vec![
            ("limit".to_string(), "25".to_string()),
            ("order".to_string(), "desc".to_string()),
        ];
        let query_string = utils::build_query_string(&params);
        assert_eq!(query_string, "?limit=25&order=desc");

        let empty_params: Vec<(String, String)> = vec![];
        let empty_query = utils::build_query_string(&empty_params);
        assert!(empty_query.is_empty());
    }

    #[test]
    fn test_utils_validation() {
        // Test limit validation
        assert!(utils::validate_limit(1));
        assert!(utils::validate_limit(50));
        assert!(utils::validate_limit(100));
        assert!(!utils::validate_limit(0));
        assert!(!utils::validate_limit(101));

        // Test order validation
        assert!(utils::validate_order("asc"));
        assert!(utils::validate_order("desc"));
        assert!(!utils::validate_order("ascending"));
        assert!(!utils::validate_order("descending"));
        assert!(!utils::validate_order(""));
    }

    #[test]
    fn test_params_builder_chaining() {
        // Test that builder methods can be chained in any order
        let params1 = ListVectorStoresParams::new()
            .with_limit(25)
            .with_order("desc");

        let params2 = ListVectorStoresParams::new()
            .with_order("desc")
            .with_limit(25);

        assert_eq!(params1.limit, params2.limit);
        assert_eq!(params1.order, params2.order);
    }

    #[test]
    fn test_file_params_status_filter() {
        let params = ListVectorStoreFilesParams::new().with_filter(VectorStoreFileStatus::Failed);

        assert!(params.has_filters());
        let query_params = params.to_query_params();
        assert!(query_params.contains(&("filter".to_string(), "failed".to_string())));
    }
}
