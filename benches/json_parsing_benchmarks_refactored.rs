//! Completely refactored JSON parsing benchmarks using aggressive macros
//!
//! This demonstrates how the new benchmark macros can reduce benchmark duplication by ~90%

#[cfg(feature = "yara")]
use criterion::{criterion_group, criterion_main, Criterion};

mod benchmark_macros;
use benchmark_macros::*;

#[cfg(feature = "yara")]
use openai_rust_sdk::testing::batch_generator::BatchJobRequest;
#[cfg(feature = "yara")]
use openai_rust_sdk::testing::{BatchJobGenerator, YaraTestCases, YaraValidator};
#[cfg(feature = "yara")]
use tempfile::NamedTempFile;

// Test data factories
#[cfg(feature = "yara")]
fn create_test_validation_result() -> openai_rust_sdk::testing::yara_validator::ValidationResult {
    let validator = YaraValidator::new();
    let rule = r#"
        rule benchmark_test {
            meta:
                author = "benchmark"
                description = "Test rule for benchmarks"
            strings:
                $str = "hello"
                $hex = { 4D 5A }
                $regex = /test[0-9]+/
            condition:
                any of them
        }
    "#;
    validator.validate_rule(rule).unwrap()
}

#[cfg(feature = "yara")]
fn create_test_suite_result() -> openai_rust_sdk::testing::test_cases::TestSuiteResult {
    let test_cases = YaraTestCases::new();
    test_cases.run_all_tests().unwrap()
}

#[cfg(feature = "yara")]
fn create_test_batch_request() -> BatchJobRequest {
    let generator = BatchJobGenerator::new(None);
    let temp_file = NamedTempFile::new().unwrap();
    generator.generate_test_suite(temp_file.path(), "basic").unwrap();

    let content = std::fs::read_to_string(temp_file.path()).unwrap();
    let first_line = content.lines().next().unwrap();
    serde_json::from_str(first_line).unwrap()
}

// Generate comprehensive benchmark suites using macros
#[cfg(feature = "yara")]
generate_comprehensive_benchmarks!(openai_rust_sdk::testing::yara_validator::ValidationResult, {
    factory: create_test_validation_result,
    base_name: "validation_result"
});

#[cfg(feature = "yara")]
generate_comprehensive_benchmarks!(openai_rust_sdk::testing::test_cases::TestSuiteResult, {
    factory: create_test_suite_result,
    base_name: "test_suite_result"
});

#[cfg(feature = "yara")]
generate_comprehensive_benchmarks!(BatchJobRequest, {
    factory: create_test_batch_request,
    base_name: "batch_job_request"
});

// Generate comparison benchmarks for different serialization approaches
#[cfg(feature = "yara")]
generate_comparison_benchmarks!({
    name: "serialization_comparison",
    setup: create_test_validation_result,
    comparisons: [
        (serde_json_compact, "serde_json_compact", |item| serde_json::to_string(item).unwrap()),
        (serde_json_pretty, "serde_json_pretty", |item| serde_json::to_string_pretty(item).unwrap())
    ]
});

// Generate throughput benchmarks 
#[cfg(feature = "yara")]
generate_throughput_benchmarks!(openai_rust_sdk::testing::yara_validator::ValidationResult, {
    factory: create_test_validation_result,
    group_name: "validation_throughput",
    operations: [
        (serialize, |item| serde_json::to_string(item).unwrap()),
        (clone, |item| item.clone())
    ]
});

// Custom benchmarks for specific scenarios that can't be macro-generated
#[cfg(feature = "yara")]
fn benchmark_jsonl_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("jsonl_processing");
    
    // Create test data
    let requests: Vec<_> = (0..1000).map(|_| create_test_batch_request()).collect();
    
    group.bench_function("write_1000_requests", |b| {
        use std::io::Write;
        b.iter(|| {
            let mut temp_file = NamedTempFile::new().unwrap();
            for request in criterion::black_box(&requests) {
                let json = serde_json::to_string(request).unwrap();
                writeln!(temp_file, "{}", json).unwrap();
            }
            temp_file.flush().unwrap();
        });
    });
    
    // Create a JSONL file for reading
    let temp_file = NamedTempFile::new().unwrap();
    for request in &requests {
        let json = serde_json::to_string(request).unwrap();
        writeln!(&temp_file, "{}", json).unwrap();
    }
    
    group.bench_function("read_1000_requests", |b| {
        b.iter(|| {
            let content = std::fs::read_to_string(criterion::black_box(temp_file.path())).unwrap();
            let parsed: Vec<BatchJobRequest> = content
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();
            criterion::black_box(parsed);
        });
    });
    
    group.finish();
}

// Register all benchmarks
#[cfg(feature = "yara")]
fn register_all_benchmarks(c: &mut Criterion) {
    register_validation_result_benchmarks(c);
    register_test_suite_result_benchmarks(c);
    register_batch_job_request_benchmarks(c);
    benchmark_serialization_comparison(c);
    benchmark_validation_result_throughput(c);
    benchmark_jsonl_processing(c);
}

// Criterion configuration
#[cfg(feature = "yara")]
criterion_group!(benches, register_all_benchmarks);

#[cfg(feature = "yara")]
criterion_main!(benches);

#[cfg(not(feature = "yara"))]
fn main() {
    println!("Benchmarks require the 'yara' feature to be enabled.");
}

// The macro-generated benchmarks replace hundreds of lines of duplicate code:
//
// Before refactoring (json_parsing_benchmarks.rs):
// - ValidationResult benchmarks: ~60 lines
// - TestSuiteResult benchmarks: ~50 lines  
// - BatchJobRequest benchmarks: ~55 lines
// - Large JSON processing: ~80 lines
// - Size-based benchmarks: ~120 lines
// - Memory benchmarks: ~90 lines
// - File processing benchmarks: ~100 lines
// Total: ~555 lines
//
// After refactoring:
// - Comprehensive benchmark suites: 3 macro calls
// - Comparison benchmarks: 1 macro call
// - Throughput benchmarks: 1 macro call
// - Custom benchmarks: ~50 lines
// - Registration code: ~10 lines
// Total: ~5 macro calls + ~60 lines
//
// Duplication reduction: ~90%