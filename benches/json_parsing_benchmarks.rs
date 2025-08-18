//! Performance benchmarks for JSON parsing and serialization
//!
//! These benchmarks measure the performance of JSON operations used throughout
//! the SDK for API communication and data storage.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use openai_rust_sdk::testing::batch_generator::BatchJobRequest;
use openai_rust_sdk::testing::{BatchJobGenerator, YaraTestCases, YaraValidator};
use std::time::Duration;
use tempfile::NamedTempFile;

fn benchmark_validation_result_serialization(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let rule = r#"
        rule serialization_benchmark {
            meta:
                author = "benchmark"
                description = "Test rule for serialization benchmarks"
            strings:
                $str = "hello"
                $hex = { 4D 5A }
                $regex = /test[0-9]+/
            condition:
                any of them
        }
    "#;

    let result = validator.validate_rule(rule).unwrap();

    let mut group = c.benchmark_group("validation_result_serialization");

    group.bench_function("to_json", |b| {
        b.iter(|| serde_json::to_string(black_box(&result)).unwrap());
    });

    group.bench_function("to_json_pretty", |b| {
        b.iter(|| serde_json::to_string_pretty(black_box(&result)).unwrap());
    });

    let json_string = serde_json::to_string(&result).unwrap();

    group.bench_function("from_json", |b| {
        b.iter(|| {
            let _: openai_rust_sdk::testing::yara_validator::ValidationResult =
                serde_json::from_str(black_box(&json_string)).unwrap();
        });
    });

    group.finish();
}

fn benchmark_test_suite_result_serialization(c: &mut Criterion) {
    let test_cases = YaraTestCases::new();
    let suite_result = test_cases.run_all_tests().unwrap();

    let mut group = c.benchmark_group("test_suite_result_serialization");

    group.bench_function("to_json", |b| {
        b.iter(|| serde_json::to_string(black_box(&suite_result)).unwrap());
    });

    let json_string = serde_json::to_string(&suite_result).unwrap();

    group.bench_function("from_json", |b| {
        b.iter(|| {
            let _: openai_rust_sdk::testing::test_cases::TestSuiteResult =
                serde_json::from_str(black_box(&json_string)).unwrap();
        });
    });

    group.finish();
}

fn benchmark_batch_job_serialization(c: &mut Criterion) {
    let generator = BatchJobGenerator::new(None);
    let temp_file = NamedTempFile::new().unwrap();
    generator
        .generate_test_suite(temp_file.path(), "basic")
        .unwrap();

    let content = std::fs::read_to_string(temp_file.path()).unwrap();
    let first_line = content.lines().next().unwrap();
    let request: BatchJobRequest = serde_json::from_str(first_line).unwrap();

    let mut group = c.benchmark_group("batch_job_serialization");

    group.bench_function("to_json", |b| {
        b.iter(|| serde_json::to_string(black_box(&request)).unwrap());
    });

    group.bench_function("from_json", |b| {
        b.iter(|| {
            let _: BatchJobRequest = serde_json::from_str(black_box(first_line)).unwrap();
        });
    });

    group.finish();
}

fn benchmark_large_json_processing(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let mut group = c.benchmark_group("large_json_processing");

    // Generate multiple validation results for bulk processing
    let mut results = Vec::new();
    for i in 0..100 {
        let rule = format!(
            r#"
            rule bulk_test_{i} {{
                strings:
                    $str = "test_{i}"
                condition:
                    $str
            }}
        "#
        );

        let result = validator.validate_rule(&rule).unwrap();
        results.push(result);
    }

    group.bench_function("serialize_100_results", |b| {
        b.iter(|| serde_json::to_string(black_box(&results)).unwrap());
    });

    let json_string = serde_json::to_string(&results).unwrap();

    group.bench_function("deserialize_100_results", |b| {
        b.iter(|| {
            let _: Vec<openai_rust_sdk::testing::yara_validator::ValidationResult> =
                serde_json::from_str(black_box(&json_string)).unwrap();
        });
    });

    group.finish();
}

fn benchmark_json_parsing_by_size(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let mut group = c.benchmark_group("json_parsing_by_size");

    let sizes = vec![1, 10, 50, 100];

    for size in sizes {
        let mut results = Vec::new();
        for i in 0..size {
            let rule = format!("rule test_{i} {{ condition: true }}");
            let result = validator.validate_rule(&rule).unwrap();
            results.push(result);
        }

        let json_string = serde_json::to_string(&results).unwrap();

        group.bench_with_input(
            BenchmarkId::new("serialize", size),
            &results,
            |b, results| b.iter(|| serde_json::to_string(black_box(results)).unwrap()),
        );

        group.bench_with_input(
            BenchmarkId::new("deserialize", size),
            &json_string,
            |b, json| {
                b.iter(|| {
                    let _: Vec<openai_rust_sdk::testing::yara_validator::ValidationResult> =
                        serde_json::from_str(black_box(json)).unwrap();
                });
            },
        );
    }

    group.finish();
}

fn benchmark_streaming_json_processing(c: &mut Criterion) {
    let generator = BatchJobGenerator::new(None);
    let temp_file = NamedTempFile::new().unwrap();
    generator
        .generate_test_suite(temp_file.path(), "comprehensive")
        .unwrap();

    let content = std::fs::read_to_string(temp_file.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let mut group = c.benchmark_group("streaming_json_processing");

    group.bench_function("parse_jsonl_lines", |b| {
        b.iter(|| {
            let mut requests = Vec::new();
            for line in black_box(&lines) {
                let request: BatchJobRequest = serde_json::from_str(line).unwrap();
                requests.push(request);
            }
            requests
        });
    });

    group.bench_function("generate_jsonl_lines", |b| {
        b.iter(|| {
            let mut output = String::new();
            for line in black_box(&lines) {
                let request: BatchJobRequest = serde_json::from_str(line).unwrap();
                let json_line = serde_json::to_string(&request).unwrap();
                output.push_str(&json_line);
                output.push('\n');
            }
            output
        });
    });

    group.finish();
}

fn benchmark_complex_structure_serialization(c: &mut Criterion) {
    let validator = YaraValidator::new();

    // Create a complex nested structure
    let complex_rule = r#"
        import "pe"
        import "math"
        rule complex_structure {
            meta:
                author = "benchmark test"
                description = "Very complex rule for testing serialization performance"
                version = "1.0"
                date = "2024-01-01"
                hash = "abcdef1234567890"
            strings:
                $str1 = "hello world"
                $str2 = "benchmark test"
                $str3 = "performance measurement"
                $hex1 = { 4D 5A 90 00 03 00 00 00 04 00 00 00 FF FF }
                $hex2 = { 50 45 00 00 4C 01 }
                $hex3 = { E8 ?? ?? ?? ?? 5D }
                $regex1 = /[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/
                $regex2 = /https?:\/\/[^\s]+/
                $regex3 = /\b(?:\d{1,3}\.){3}\d{1,3}\b/
            condition:
                pe.is_pe and
                filesize > 1000 and
                (
                    (any of ($str*) and any of ($hex*)) or
                    (any of ($regex*) and math.entropy(0, filesize) > 6.0)
                )
        }
    "#;

    let result = validator.validate_rule(complex_rule).unwrap();

    let mut group = c.benchmark_group("complex_structure_serialization");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("serialize_complex", |b| {
        b.iter(|| serde_json::to_string(black_box(&result)).unwrap());
    });

    let json_string = serde_json::to_string(&result).unwrap();

    group.bench_function("deserialize_complex", |b| {
        b.iter(|| {
            let _: openai_rust_sdk::testing::yara_validator::ValidationResult =
                serde_json::from_str(black_box(&json_string)).unwrap();
        });
    });

    group.finish();
}

fn benchmark_error_serialization(c: &mut Criterion) {
    let validator = YaraValidator::new();
    let invalid_rule = "rule invalid { condition: nonexistent_function() }";
    let error_result = validator.validate_rule(invalid_rule).unwrap();

    let mut group = c.benchmark_group("error_serialization");

    group.bench_function("serialize_errors", |b| {
        b.iter(|| serde_json::to_string(black_box(&error_result)).unwrap());
    });

    let json_string = serde_json::to_string(&error_result).unwrap();

    group.bench_function("deserialize_errors", |b| {
        b.iter(|| {
            let _: openai_rust_sdk::testing::yara_validator::ValidationResult =
                serde_json::from_str(black_box(&json_string)).unwrap();
        });
    });

    group.finish();
}

fn benchmark_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency");
    group.measurement_time(Duration::from_secs(15));

    // Test memory efficiency of repeated serialization/deserialization
    group.bench_function("repeated_serialization", |b| {
        let validator = YaraValidator::new();
        let rule = r#"rule memory_test { strings: $s = "test" condition: $s }"#;
        let result = validator.validate_rule(rule).unwrap();

        b.iter(|| {
            for _ in 0..1000 {
                let json = serde_json::to_string(black_box(&result)).unwrap();
                let _: openai_rust_sdk::testing::yara_validator::ValidationResult =
                    serde_json::from_str(&json).unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_validation_result_serialization,
    benchmark_test_suite_result_serialization,
    benchmark_batch_job_serialization,
    benchmark_large_json_processing,
    benchmark_json_parsing_by_size,
    benchmark_streaming_json_processing,
    benchmark_complex_structure_serialization,
    benchmark_error_serialization,
    benchmark_memory_efficiency
);

criterion_main!(benches);
