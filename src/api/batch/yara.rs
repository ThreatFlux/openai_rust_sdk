//! YARA processing utilities for batch results

/// YARA processing utilities
pub struct YaraProcessor;

impl YaraProcessor {
    /// Extracts YARA rule content from AI response text
    pub fn extract_yara_rule(content: &str) -> Option<String> {
        // Look for content between ```yara and next ```
        if let Some(start) = content.find("```yara") {
            let rule_start = start + 7; // Skip "```yara"
            let rule_start = content[rule_start..]
                .find('\n')
                .map_or(rule_start, |newline| rule_start + newline + 1);

            if let Some(end) = content[rule_start..].find("```") {
                let rule_end = rule_start + end;
                return Some(content[rule_start..rule_end].trim().to_string());
            }
        }

        // Look for content between ``` (generic code blocks)
        if let Some(start) = content.find("```") {
            let after_first = start + 3;
            let rule_start = content[after_first..]
                .find('\n')
                .map_or(after_first, |i| after_first + i + 1);

            if let Some(end) = content[rule_start..].find("```") {
                let rule_end = rule_start + end;
                let potential_rule = content[rule_start..rule_end].trim();

                if potential_rule.contains("rule ")
                    && potential_rule.contains('{')
                    && potential_rule.contains('}')
                {
                    return Some(potential_rule.to_string());
                }
            }
        }

        // Look for rule patterns without code blocks
        if content.contains("rule ") && content.contains('{') && content.contains('}') {
            if let Some(rule_start) = content.find("rule ") {
                let remaining = &content[rule_start..];
                let mut brace_count = 0;
                let mut rule_end = remaining.len();

                for (i, ch) in remaining.char_indices() {
                    match ch {
                        '{' => brace_count += 1,
                        '}' => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                rule_end = i + 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                if brace_count == 0 {
                    let rule_text = &remaining[..rule_end];
                    return Some(rule_text.trim().to_string());
                }
            }
        }

        None
    }

    /// Validate YARA rule syntax (basic validation)
    pub fn validate_yara_rule(rule_content: &str) -> bool {
        // Basic validation - check for required components
        rule_content.contains("rule ")
            && rule_content.contains('{')
            && rule_content.contains('}')
            && rule_content.contains("condition")
    }

    /// Extract rule name from YARA rule content
    pub fn extract_rule_name(rule_content: &str) -> Option<String> {
        if let Some(rule_pos) = rule_content.find("rule ") {
            let after_rule = &rule_content[rule_pos + 5..]; // Skip "rule "
            if let Some(space_or_brace) = after_rule.find(|c: char| c.is_whitespace() || c == '{') {
                let name = after_rule[..space_or_brace].trim();
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yara_processor_extract_rule() {
        let content = r#"Here's a YARA rule:

```yara
rule TestRule {
    strings:
        $a = "test"
    condition:
        $a
}
```

Hope this helps!"#;

        let result = YaraProcessor::extract_yara_rule(content);
        assert!(result.is_some());
        let rule = result.unwrap();
        assert!(rule.contains("rule TestRule"));
        assert!(rule.contains("strings:"));
        assert!(rule.contains("condition:"));
    }

    #[test]
    fn test_yara_processor_no_rule() {
        let content = "This is just regular text with no YARA rules.";
        let result = YaraProcessor::extract_yara_rule(content);
        assert!(result.is_none());
    }

    #[test]
    fn test_yara_processor_generic_code_block() {
        let content = r#"Here's a rule:

```
rule GenericRule {
    condition:
        true
}
```"#;

        let result = YaraProcessor::extract_yara_rule(content);
        assert!(result.is_some());
        let rule = result.unwrap();
        assert!(rule.contains("rule GenericRule"));
    }

    #[test]
    fn test_yara_processor_validate_rule() {
        let valid_rule = r#"rule TestRule {
    condition:
        true
}"#;

        let invalid_rule = "not a yara rule";

        assert!(YaraProcessor::validate_yara_rule(valid_rule));
        assert!(!YaraProcessor::validate_yara_rule(invalid_rule));
    }

    #[test]
    fn test_extract_rule_name() {
        let rule = r#"rule MyTestRule {
    condition:
        true
}"#;

        let name = YaraProcessor::extract_rule_name(rule);
        assert_eq!(name, Some("MyTestRule".to_string()));

        let rule_with_space = "rule AnotherRule   {";
        let name = YaraProcessor::extract_rule_name(rule_with_space);
        assert_eq!(name, Some("AnotherRule".to_string()));
    }
}
