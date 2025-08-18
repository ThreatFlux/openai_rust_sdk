//! Helper functions for YARA validator to reduce complexity
#![allow(dead_code)]

/// Check if a line is a string pattern definition
pub fn is_string_pattern(line: &str) -> bool {
    line.starts_with('$')
}

/// Check if a line contains hex patterns
pub fn has_hex_pattern(line: &str) -> bool {
    line.contains('{') && line.contains('}')
}

/// Check if a line contains regex patterns
pub fn has_regex_pattern(line: &str) -> bool {
    line.contains('/') && line.matches('/').count() >= 2
}

/// Count string patterns in a rule section
pub fn count_strings_in_section(lines: &[&str], start_idx: usize) -> (usize, bool, bool) {
    let mut string_count = 0;
    let mut has_hex = false;
    let mut has_regex = false;

    for line in &lines[start_idx..] {
        let line = line.trim();

        if line.starts_with("condition:") {
            break;
        }

        if is_string_pattern(line) {
            string_count += 1;

            if has_hex_pattern(line) {
                has_hex = true;
            }

            if has_regex_pattern(line) {
                has_regex = true;
            }
        }
    }

    (string_count, has_hex, has_regex)
}

/// Analyze rule metadata and imports
pub fn analyze_rule_metadata(source: &str) -> (bool, bool, bool, bool) {
    let source_lower = source.to_lowercase();

    let has_strings = source_lower.contains("strings:");
    let has_metadata = source_lower.contains("meta:");
    let has_imports = source_lower.contains("import ");
    let uses_external_vars = source_lower.contains("filesize");

    (has_strings, has_metadata, has_imports, uses_external_vars)
}

/// Check for iterator usage in rule
pub fn uses_iterators(source: &str) -> bool {
    let source_lower = source.to_lowercase();
    source_lower.contains("any of") || source_lower.contains("all of")
}

/// Calculate complexity score based on features
pub fn calculate_complexity_score(string_count: usize) -> u8 {
    (string_count + 1).min(10) as u8
}
