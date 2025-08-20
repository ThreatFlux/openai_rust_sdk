#![allow(clippy::pedantic)]
//! Performance benchmarks for YARA rule validation
//!
//! These benchmarks measure the performance of various validation operations
//! to ensure the SDK performs well under different workloads.

#[cfg(not(feature = "yara"))]
fn main() {
    println!("This benchmark requires the 'yara' feature to be enabled.");
    println!("Run with: cargo bench --features yara");
}

#[cfg(feature = "yara")]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
#[cfg(feature = "yara")]
use openai_rust_sdk::testing::{YaraTestCases, YaraValidator};
#[cfg(feature = "yara")]
use std::time::Duration;

#[cfg(feature = "yara")]
fn benchmark_simple_rule_validation(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let simple_rule = r"
        rule simple_benchmark {
            condition: true
        }
    ";

    c.bench_function("validate_simple_rule", |b| {
        b.iter(|| validator.validate_rule(black_box(simple_rule)).unwrap())
    });
}

#[cfg(feature = "yara")]
fn benchmark_complex_rule_validation(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let complex_rule = r#"
        rule complex_benchmark {
            meta:
                author = "benchmark"
                description = "Complex rule for performance testing"
                version = "1.0"
            strings:
                $str1 = "hello"
                $str2 = "world"
                $str3 = "benchmark"
                $hex1 = { 4D 5A }
                $hex2 = { 50 45 }
                $hex3 = { FF FE }
                $regex1 = /test[0-9]+/
                $regex2 = /[a-zA-Z]+@[a-zA-Z]+\.[a-zA-Z]+/
            condition:
                (any of ($str*) and any of ($hex*)) or any of ($regex*)
        }
    "#;

    c.bench_function("validate_complex_rule", |b| {
        b.iter(|| validator.validate_rule(black_box(complex_rule)).unwrap())
    });
}

#[cfg(feature = "yara")]
fn benchmark_rule_validation_by_size(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let mut group = c.benchmark_group("validation_by_rule_size");

    let rule_sizes = vec![
        (1, generate_rule_with_strings(1)),
        (5, generate_rule_with_strings(5)),
        (10, generate_rule_with_strings(10)),
        (25, generate_rule_with_strings(25)),
        (50, generate_rule_with_strings(50)),
    ];

    for (string_count, rule) in rule_sizes {
        group.bench_with_input(
            BenchmarkId::new("strings", string_count),
            &rule,
            |b, rule| b.iter(|| validator.validate_rule(black_box(rule)).unwrap()),
        );
    }

    group.finish();
}

#[cfg(feature = "yara")]
fn benchmark_concurrent_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_validation");

    let rule = r#"
        rule concurrent_test {
            strings:
                $test = "concurrent"
            condition:
                $test
        }
    "#;

    let thread_counts = vec![1, 2, 4, 8];

    for thread_count in thread_counts {
        group.bench_with_input(
            BenchmarkId::new("threads", thread_count),
            &thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let handles: Vec<_> = (0..thread_count)
                        .map(|_| {
                            let validator = YaraValidator::new();
                            let rule_copy = rule.to_string();
                            std::thread::spawn(move || validator.validate_rule(&rule_copy).unwrap())
                        })
                        .collect();

                    for handle in handles {
                        handle.join().unwrap();
                    }
                })
            },
        );
    }

    group.finish();
}

#[cfg(feature = "yara")]
fn benchmark_test_suite_execution(c: &mut Criterion) {
    let test_cases = YaraTestCases::new();

    c.bench_function("run_all_tests", |b| {
        b.iter(|| test_cases.run_all_tests().unwrap())
    });
}

#[cfg(feature = "yara")]
fn benchmark_pattern_testing(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let mut group = c.benchmark_group("pattern_testing");

    let patterns = vec![
        (
            "simple_string",
            r#"rule test { strings: $s = "test" condition: $s }"#,
        ),
        (
            "hex_pattern",
            r"rule test { strings: $h = { 4D 5A } condition: $h }",
        ),
        (
            "regex_pattern",
            r"rule test { strings: $r = /test[0-9]+/ condition: $r }",
        ),
        (
            "multiple_patterns",
            r#"
            rule test {
                strings:
                    $s1 = "test1"
                    $s2 = "test2"
                    $h1 = { 4D 5A }
                    $h2 = { 50 45 }
                condition:
                    any of them
            }
        "#,
        ),
    ];

    for (pattern_type, rule) in patterns {
        group.bench_with_input(
            BenchmarkId::new("pattern", pattern_type),
            rule,
            |b, rule| b.iter(|| validator.validate_rule(black_box(rule)).unwrap()),
        );
    }

    group.finish();
}

#[cfg(feature = "yara")]
fn benchmark_feature_analysis(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let mut group = c.benchmark_group("feature_analysis");

    let rules = vec![
        ("minimal", "rule minimal { condition: true }"),
        (
            "with_metadata",
            r#"
            rule with_metadata {
                meta:
                    author = "test"
                    description = "test rule"
                condition: true
            }
        "#,
        ),
        (
            "with_strings",
            r#"
            rule with_strings {
                strings:
                    $s1 = "test1"
                    $s2 = "test2"
                condition: any of them
            }
        "#,
        ),
        (
            "comprehensive",
            r#"
            import "pe"
            rule comprehensive {
                meta:
                    author = "test"
                    version = "1.0"
                strings:
                    $str = "test"
                    $hex = { 4D 5A }
                    $regex = /test[0-9]+/
                condition:
                    filesize > 100 and any of them
            }
        "#,
        ),
    ];

    // Note: analyze_features is now private, so we'll benchmark validate instead
    for (rule_type, rule) in rules {
        group.bench_function(rule_type, |b| {
            b.iter(|| {
                // Benchmark the full validation instead
                validator.validate_rule(black_box(rule))
            })
        });
    }

    group.finish();
}

#[cfg(feature = "yara")]
fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("validator_creation", |b| {
        b.iter(|| black_box(YaraValidator::new()))
    });

    group.bench_function("multiple_validators", |b| {
        b.iter(|| {
            let validators: Vec<_> = (0..100).map(|_| YaraValidator::new()).collect();
            black_box(validators)
        })
    });

    group.finish();
}

#[cfg(feature = "yara")]
fn benchmark_error_handling(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let mut group = c.benchmark_group("error_handling");

    let invalid_rules = vec![
        ("syntax_error", "rule invalid { condition invalid_syntax }"),
        (
            "missing_condition",
            r#"rule incomplete { strings: $s = "test" }"#,
        ),
        ("empty_rule", ""),
        ("malformed", "this is not a YARA rule "),
    ];

    for (error_type, rule) in invalid_rules {
        group.bench_function(error_type, |b| {
            b.iter(|| validator.validate_rule(black_box(rule)).unwrap())
        });
    }

    group.finish();
}

// Helper function to generate rules with varying numbers of strings
#[cfg(feature = "yara")]
fn generate_rule_with_strings(string_count: usize) -> String {
    let mut rule = String::from(
        r"rule generated_rule {
    strings:
",
    );

    for i in 0..string_count {
        rule.push_str(&format!(
            r#"        $str{} = "test_string_{}"
"#,
            i, i
        ));
    }

    rule.push_str(
        r"    condition:
        any of them
}",
    );
    rule
}

#[cfg(feature = "yara")]
criterion_group!(
    benches,
    benchmark_simple_rule_validation,
    benchmark_complex_rule_validation,
    benchmark_rule_validation_by_size,
    benchmark_concurrent_validation,
    benchmark_test_suite_execution,
    benchmark_pattern_testing,
    benchmark_feature_analysis,
    benchmark_memory_usage,
    benchmark_error_handling
);

#[cfg(feature = "yara")]
criterion_main!(benches);
