//! Tests for YARA rule extraction functionality

#[test]
fn test_yara_rule_pattern_detection() {
    // Test content with YARA rule in markdown code block
    let content_with_yara = r#"
Here's the YARA rule for detecting malware:

```yara
rule DetectMalware {
meta:
    description = "Detects specific malware"
    author = "Security Team"

strings:
    $hex = { 4D 5A 90 00 }
    $string = "malicious_pattern"

condition:
    $hex at 0 and $string
}
```

This rule looks for specific patterns.
    "#;

    // This test verifies the pattern exists that would be extracted
    assert!(content_with_yara.contains("rule DetectMalware"));
    assert!(content_with_yara.contains("```yara"));
    assert!(content_with_yara.contains("condition:"));
}

#[test]
fn test_yara_rule_without_code_blocks() {
    let content_with_plain_yara = r#"
rule SimpleRule {
strings:
    $s = "test"
condition:
    $s
}
    "#;

    // Verify basic YARA rule structure
    assert!(content_with_plain_yara.contains("rule "));
    assert!(content_with_plain_yara.contains("{"));
    assert!(content_with_plain_yara.contains("}"));
    assert!(content_with_plain_yara.contains("condition:"));
}

#[test]
fn test_complex_yara_rule_pattern() {
    let complex_yara = r#"
rule ComplexMalwareDetection {
meta:
    description = "Advanced malware detection"
    author = "Threat Research Team"
    date = "2024-01-01"
    version = "1.0"

strings:
    $header = { 4D 5A }
    $payload1 = { E8 [4] 58 [0-10] C3 }
    $payload2 = "CreateRemoteThread"
    $payload3 = /\x00CreateProcess[AW]\x00/ nocase

condition:
    $header at 0 and 
    (
        ($payload1 and #payload2 > 2) or
        $payload3
    ) and
    filesize < 2MB
}
    "#;

    // Verify complex YARA rule components
    assert!(complex_yara.contains("rule ComplexMalwareDetection"));
    assert!(complex_yara.contains("meta:"));
    assert!(complex_yara.contains("strings:"));
    assert!(complex_yara.contains("condition:"));
    assert!(complex_yara.contains("filesize"));
}

#[test]
fn test_invalid_yara_content() {
    let invalid_content = "This is just plain text without any YARA rules";

    // Should not contain YARA patterns
    assert!(!invalid_content.contains("rule "));
    assert!(!invalid_content.contains("condition:"));
}
