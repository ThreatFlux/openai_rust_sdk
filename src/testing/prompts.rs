//! # Test Suite Prompts
//!
//! This module contains pre-defined prompt sets for different YARA rule generation test suites.
//! Each test suite focuses on specific use cases and complexity levels.

/// Basic test prompts for fundamental YARA rule functionality
pub const BASIC_PROMPTS: &[&str] = &[
    "Create a YARA rule that detects files containing 'Hello World'.",
    "Generate a YARA rule to detect PE headers (MZ signature).",
    "Write a YARA rule for detecting error/warning/debug strings.",
];

/// Malware-focused test prompts for advanced detection scenarios
pub const MALWARE_PROMPTS: &[&str] = &[
    "Generate a YARA rule to detect UPX packed executables.",
    "Create a YARA rule for ransomware detection based on encryption strings.",
    "Write a YARA rule to detect keylogger APIs.",
];

/// Comprehensive test prompts covering all rule types and complexity levels
pub const COMPREHENSIVE_PROMPTS: &[&str] = &[
    "Create a YARA rule that detects files containing 'Hello World'.",
    "Generate a YARA rule to detect PE headers (MZ signature).",
    "Generate a YARA rule to detect UPX packed executables.",
    "Create a YARA rule for ransomware detection.",
    "Write a YARA rule using regex to detect email addresses.",
    "Create a YARA rule to detect cryptocurrency addresses.",
    "Generate a YARA rule with external variables for file size detection.",
    "Write a YARA rule using for loops to detect repeating patterns.",
    "Create a YARA rule that combines multiple modules for comprehensive analysis.",
    "Generate a YARA rule for detecting obfuscated JavaScript code.",
];

/// Registry of available test suites and their corresponding prompts
pub struct PromptsRegistry;

impl PromptsRegistry {
    /// Gets prompts for the specified test suite
    ///
    /// # Arguments
    ///
    /// * `suite_name` - Name of the test suite ("basic", "malware", "comprehensive")
    ///
    /// # Returns
    ///
    /// Returns a slice of prompts for the specified suite or None if the suite doesn't exist
    ///
    /// # Example
    ///
    /// ```
    /// use openai_rust_sdk::testing::prompts::PromptsRegistry;
    ///
    /// let prompts = PromptsRegistry::get_prompts("basic").unwrap();
    /// assert!(!prompts.is_empty());
    /// ```
    pub fn get_prompts(suite_name: &str) -> Option<&'static [&'static str]> {
        match suite_name {
            "basic" => Some(BASIC_PROMPTS),
            "malware" => Some(MALWARE_PROMPTS),
            "comprehensive" => Some(COMPREHENSIVE_PROMPTS),
            _ => None,
        }
    }

    /// Gets all available test suite names
    ///
    /// # Returns
    ///
    /// Returns a vector of all available test suite names
    #[allow(dead_code)]
    pub fn available_suites() -> Vec<&'static str> {
        vec!["basic", "malware", "comprehensive"]
    }

    /// Checks if a test suite exists
    ///
    /// # Arguments
    ///
    /// * `suite_name` - Name of the test suite to check
    ///
    /// # Returns
    ///
    /// Returns true if the suite exists, false otherwise
    #[allow(dead_code)]
    pub fn suite_exists(suite_name: &str) -> bool {
        Self::get_prompts(suite_name).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_prompts_not_empty() {
        // Const arrays always have a fixed size, just verify the expected count
        assert_eq!(BASIC_PROMPTS.len(), 3);
    }

    #[test]
    fn test_malware_prompts_not_empty() {
        // Const arrays always have a fixed size, just verify the expected count
        assert_eq!(MALWARE_PROMPTS.len(), 3);
    }

    #[test]
    fn test_comprehensive_prompts_not_empty() {
        // Const arrays always have a fixed size, just verify the expected count
        assert_eq!(COMPREHENSIVE_PROMPTS.len(), 10);
    }

    #[test]
    fn test_prompts_registry_get_valid_suites() {
        assert!(PromptsRegistry::get_prompts("basic").is_some());
        assert!(PromptsRegistry::get_prompts("malware").is_some());
        assert!(PromptsRegistry::get_prompts("comprehensive").is_some());
    }

    #[test]
    fn test_prompts_registry_get_invalid_suite() {
        assert!(PromptsRegistry::get_prompts("invalid").is_none());
        assert!(PromptsRegistry::get_prompts("").is_none());
    }

    #[test]
    fn test_available_suites() {
        let suites = PromptsRegistry::available_suites();
        assert_eq!(suites.len(), 3);
        assert!(suites.contains(&"basic"));
        assert!(suites.contains(&"malware"));
        assert!(suites.contains(&"comprehensive"));
    }

    #[test]
    fn test_suite_exists() {
        assert!(PromptsRegistry::suite_exists("basic"));
        assert!(PromptsRegistry::suite_exists("malware"));
        assert!(PromptsRegistry::suite_exists("comprehensive"));
        assert!(!PromptsRegistry::suite_exists("invalid"));
        assert!(!PromptsRegistry::suite_exists(""));
    }
}
