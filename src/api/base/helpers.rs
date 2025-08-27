//! Helper functions and utilities for HTTP client operations

// Re-export common validation functionality
pub use crate::api::base::config::{validate_request, Validate};
pub use crate::api::base::error::{
    handle_error_response_with_json, handle_simple_error_response, map_parse_error,
    map_request_error,
};

/// URL building utilities
pub mod url {
    /// Build a URL from the base URL and path
    #[must_use]
    pub fn build_simple_url(base_url: &str, path: &str) -> String {
        format!("{}{}", base_url, path)
    }

    /// Build URL with path and optional query parameters
    #[must_use]
    pub fn build_url_with_query(
        base_url: &str,
        path: &str,
        query_params: &[(String, String)],
    ) -> String {
        let mut url = format!("{}{}", base_url, path);

        if !query_params.is_empty() {
            url.push('?');
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            url.push_str(&query_string);
        }

        url
    }
}

#[cfg(test)]
mod tests {
    use super::url::*;

    #[test]
    fn test_build_simple_url() {
        let result = build_simple_url("https://api.example.com", "/v1/test");
        assert_eq!(result, "https://api.example.com/v1/test");
    }

    #[test]
    fn test_build_url_with_query() {
        let params = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];
        let result = build_url_with_query("https://api.example.com", "/v1/test", &params);
        assert_eq!(
            result,
            "https://api.example.com/v1/test?key1=value1&key2=value2"
        );
    }

    #[test]
    fn test_build_url_with_empty_query() {
        let result = build_url_with_query("https://api.example.com", "/v1/test", &[]);
        assert_eq!(result, "https://api.example.com/v1/test");
    }
}
