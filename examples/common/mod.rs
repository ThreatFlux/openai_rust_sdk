//! Common utilities for example applications
//!
//! This module provides shared utilities used across multiple example applications
//! to reduce code duplication and improve maintainability.

use std::env;

/// Get the OpenAI API key from the environment variable
///
/// # Returns
///
/// Returns the API key from the `OPENAI_API_KEY` environment variable
///
/// # Panics
///
/// Panics if the `OPENAI_API_KEY` environment variable is not set or is empty
///
/// # Example
///
/// ```rust
/// use common::get_api_key;
///
/// let api_key = get_api_key();
/// println!("API key loaded successfully");
/// ```
#[allow(dead_code)]
pub fn get_api_key() -> String {
    env::var("OPENAI_API_KEY").expect(
        "OPENAI_API_KEY environment variable must be set. \
        Please set it with: export OPENAI_API_KEY=your_api_key_here",
    )
}

/// Get the OpenAI API key from the environment variable with a custom error message
///
/// # Arguments
///
/// * `error_message` - Custom error message to display if the API key is not found
///
/// # Returns
///
/// Returns the API key from the `OPENAI_API_KEY` environment variable
///
/// # Panics
///
/// Panics if the `OPENAI_API_KEY` environment variable is not set or is empty,
/// displaying the custom error message
#[allow(dead_code)]
pub fn get_api_key_with_message(error_message: &str) -> String {
    env::var("OPENAI_API_KEY").expect(error_message)
}

/// Try to get the OpenAI API key from the environment variable
///
/// # Returns
///
/// Returns `Ok(api_key)` if the environment variable is set, or `Err(error_message)`
/// if it's not set or is empty
///
/// # Example
///
/// ```rust
/// use common::try_get_api_key;
///
/// match try_get_api_key() {
///     Ok(api_key) => println!("API key loaded: {}", api_key.len()),
///     Err(e) => {
///         eprintln!("Error: {}", e);
///         std::process::exit(1);
///     }
/// }
/// ```
#[allow(dead_code)]
pub fn try_get_api_key() -> Result<String, String> {
    env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. \
        Please set it with: export OPENAI_API_KEY=your_api_key_here"
            .to_string()
    })
}

/// Get the OpenAI API key or return a default value for testing
///
/// This function is useful for examples that might run in testing environments
/// where a real API key is not available.
///
/// # Arguments
///
/// * `default_key` - Default API key to use if the environment variable is not set
///
/// # Returns
///
/// Returns the API key from the environment variable, or the default key if not set
#[allow(dead_code)]
pub fn get_api_key_or_default(default_key: &str) -> String {
    env::var("OPENAI_API_KEY").unwrap_or_else(|_| default_key.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_api_key_or_default() {
        // Remove the env var if it exists for this test
        let original = env::var("OPENAI_API_KEY").ok();
        env::remove_var("OPENAI_API_KEY");

        let result = get_api_key_or_default("test-key");
        assert_eq!(result, "test-key");

        // Restore original value if it existed
        if let Some(original_key) = original {
            env::set_var("OPENAI_API_KEY", original_key);
        }
    }

    #[test]
    fn test_try_get_api_key_with_key() {
        env::set_var("OPENAI_API_KEY", "test-key");
        let result = try_get_api_key();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-key");
    }
}
