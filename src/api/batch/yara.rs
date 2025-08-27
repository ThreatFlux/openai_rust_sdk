//! YARA processing utilities for batch results

/// Extraction strategy for YARA rules
#[derive(Debug, PartialEq)]
enum ExtractionStrategy {
    YaraCodeBlock,
    GenericCodeBlock,
    PlainText,
}

/// YARA processing utilities
pub struct YaraProcessor;

impl YaraProcessor {
    /// Extracts YARA rule content from AI response text
    pub fn extract_yara_rule(content: &str) -> Option<String> {
        let strategies = [
            ExtractionStrategy::YaraCodeBlock,
            ExtractionStrategy::GenericCodeBlock,
            ExtractionStrategy::PlainText,
        ];

        strategies
            .iter()
            .find_map(|strategy| Self::try_extraction_strategy(content, strategy))
    }

    /// Attempts extraction using a specific strategy
    fn try_extraction_strategy(content: &str, strategy: &ExtractionStrategy) -> Option<String> {
        match strategy {
            ExtractionStrategy::YaraCodeBlock => Self::extract_from_yara_block(content),
            ExtractionStrategy::GenericCodeBlock => Self::extract_from_generic_block(content),
            ExtractionStrategy::PlainText => Self::extract_from_plain_text(content),
        }
    }

    /// Extract rule from ```yara code block
    fn extract_from_yara_block(content: &str) -> Option<String> {
        let start = content.find("```yara")? + 7;
        let rule_start = Self::skip_to_next_line(content, start);
        let end = content[rule_start..].find("```")? + rule_start;
        Some(content[rule_start..end].trim().to_string())
    }

    /// Extract rule from generic ``` code block
    fn extract_from_generic_block(content: &str) -> Option<String> {
        let start = content.find("```")? + 3;
        let rule_start = Self::skip_to_next_line(content, start);
        let end = content[rule_start..].find("```")? + rule_start;
        let rule = content[rule_start..end].trim();
        Self::is_valid_yara_structure(rule).then(|| rule.to_string())
    }

    /// Extract rule from plain text using brace matching
    fn extract_from_plain_text(content: &str) -> Option<String> {
        if !Self::is_valid_yara_structure(content) {
            return None;
        }
        let rule_start = content.find("rule ")?;
        Self::extract_balanced_braces(&content[rule_start..])
    }

    /// Helper to skip to next line after position
    fn skip_to_next_line(content: &str, pos: usize) -> usize {
        content[pos..].find('\n').map_or(pos, |i| pos + i + 1)
    }

    /// Check if content has basic YARA structure
    fn is_valid_yara_structure(content: &str) -> bool {
        content.contains("rule ") && content.contains('{') && content.contains('}')
    }

    /// Extract content with balanced braces
    fn extract_balanced_braces(content: &str) -> Option<String> {
        let mut brace_count = 0;
        for (i, ch) in content.char_indices() {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        return Some(content[..=i].trim().to_string());
                    }
                }
                _ => {}
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
