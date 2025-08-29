//! Comprehensive benchmark macros to eliminate benchmark duplication
//!
//! This module provides powerful macros that can generate complete benchmark suites
//! for serialization, deserialization, and bulk processing scenarios.

/// Generate complete serialization benchmark suite for any type
///
/// This macro creates comprehensive serialization benchmarks including:
/// - JSON serialization (compact)
/// - JSON serialization (pretty)  
/// - JSON deserialization
/// - Round-trip benchmarks
/// - Memory usage benchmarks
///
/// # Usage
/// ```rust
/// generate_serialization_benchmarks!(ValidationResult, {
///     factory: create_test_validation_result,
///     group_name: "validation_result_serialization"
/// });
/// ```
#[macro_export]
macro_rules! generate_serialization_benchmarks {
    ($type:ty, {
        factory: $factory:ident,
        group_name: $group_name:expr
    }) => {
        paste::paste! {
            fn [<benchmark_ $type:snake _serialization>](c: &mut criterion::Criterion) {
                let item = $factory();
                let mut group = c.benchmark_group($group_name);

                // Basic JSON serialization
                group.bench_function("to_json", |b| {
                    b.iter(|| {
                        serde_json::to_string(criterion::black_box(&item)).unwrap()
                    });
                });

                // Pretty JSON serialization
                group.bench_function("to_json_pretty", |b| {
                    b.iter(|| {
                        serde_json::to_string_pretty(criterion::black_box(&item)).unwrap()
                    });
                });

                // JSON deserialization
                let json_string = serde_json::to_string(&item).unwrap();
                group.bench_function("from_json", |b| {
                    b.iter(|| {
                        let _: $type = serde_json::from_str(
                            criterion::black_box(&json_string)
                        ).unwrap();
                    });
                });

                // Round-trip benchmark
                group.bench_function("round_trip", |b| {
                    b.iter(|| {
                        let json = serde_json::to_string(criterion::black_box(&item)).unwrap();
                        let _: $type = serde_json::from_str(&json).unwrap();
                    });
                });

                // Vector to JSON (useful for checking JSON arrays)
                let items = vec![item.clone(), item.clone(), item.clone()];
                group.bench_function("vec_to_json", |b| {
                    b.iter(|| {
                        serde_json::to_string(criterion::black_box(&items)).unwrap()
                    });
                });

                group.finish();
            }
        }
    };
}

/// Generate bulk processing benchmarks for different scales
///
/// Creates benchmarks for bulk operations with different data sizes.
///
/// # Usage
/// ```rust
/// generate_bulk_benchmarks!(ValidationResult, {
///     factory: create_test_validation_result,
///     group_name: "validation_result_bulk",
///     sizes: [1, 10, 100, 1000]
/// });
/// ```
#[macro_export]
macro_rules! generate_bulk_benchmarks {
    ($type:ty, {
        factory: $factory:ident,
        group_name: $group_name:expr,
        sizes: [$($size:expr),+]
    }) => {
        paste::paste! {
            fn [<benchmark_ $type:snake _bulk_processing>](c: &mut criterion::Criterion) {
                let mut group = c.benchmark_group($group_name);

                $(
                    // Generate test data for this size
                    let items: Vec<_> = (0..$size).map(|_| $factory()).collect();

                    // Serialization benchmark for this size
                    group.bench_with_input(
                        criterion::BenchmarkId::new("serialize", $size),
                        &items,
                        |b, items| {
                            b.iter(|| {
                                serde_json::to_string(criterion::black_box(items)).unwrap()
                            });
                        }
                    );

                    // Deserialization benchmark for this size
                    let json_string = serde_json::to_string(&items).unwrap();
                    group.bench_with_input(
                        criterion::BenchmarkId::new("deserialize", $size),
                        &json_string,
                        |b, json| {
                            b.iter(|| {
                                let _: Vec<$type> = serde_json::from_str(
                                    criterion::black_box(json)
                                ).unwrap();
                            });
                        }
                    );

                    // Processing benchmark (iterate through items)
                    group.bench_with_input(
                        criterion::BenchmarkId::new("iterate", $size),
                        &items,
                        |b, items| {
                            b.iter(|| {
                                for item in criterion::black_box(items) {
                                    criterion::black_box(item);
                                }
                            });
                        }
                    );
                )+

                group.finish();
            }
        }
    };
}

/// Generate file processing benchmarks
///
/// Creates benchmarks for file I/O operations.
///
/// # Usage
/// ```rust
/// generate_file_benchmarks!(BatchJobRequest, {
///     factory: create_test_batch_request,
///     group_name: "batch_job_file_processing"
/// });
/// ```
#[macro_export]
macro_rules! generate_file_benchmarks {
    ($type:ty, {
        factory: $factory:ident,
        group_name: $group_name:expr
    }) => {
        paste::paste! {
            fn [<benchmark_ $type:snake _file_processing>](c: &mut criterion::Criterion) {
                use std::io::Write;
                use tempfile::NamedTempFile;

                let item = $factory();
                let mut group = c.benchmark_group($group_name);

                // Write to file benchmark
                group.bench_function("write_to_file", |b| {
                    b.iter(|| {
                        let mut temp_file = NamedTempFile::new().unwrap();
                        let json = serde_json::to_string(criterion::black_box(&item)).unwrap();
                        temp_file.write_all(json.as_bytes()).unwrap();
                        temp_file.flush().unwrap();
                    });
                });

                // Read from file benchmark
                let temp_file = NamedTempFile::new().unwrap();
                let json = serde_json::to_string(&item).unwrap();
                std::fs::write(temp_file.path(), &json).unwrap();

                group.bench_function("read_from_file", |b| {
                    b.iter(|| {
                        let content = std::fs::read_to_string(
                            criterion::black_box(temp_file.path())
                        ).unwrap();
                        let _: $type = serde_json::from_str(&content).unwrap();
                    });
                });

                // Write multiple items to JSONL
                let items: Vec<_> = (0..100).map(|_| $factory()).collect();
                group.bench_function("write_jsonl", |b| {
                    b.iter(|| {
                        let mut temp_file = NamedTempFile::new().unwrap();
                        for item in criterion::black_box(&items) {
                            let json = serde_json::to_string(item).unwrap();
                            writeln!(temp_file, "{}", json).unwrap();
                        }
                        temp_file.flush().unwrap();
                    });
                });

                // Read JSONL file
                let temp_file = NamedTempFile::new().unwrap();
                for item in &items {
                    let json = serde_json::to_string(item).unwrap();
                    writeln!(&temp_file, "{}", json).unwrap();
                }

                group.bench_function("read_jsonl", |b| {
                    b.iter(|| {
                        let content = std::fs::read_to_string(
                            criterion::black_box(temp_file.path())
                        ).unwrap();

                        let parsed_items: Vec<$type> = content
                            .lines()
                            .map(|line| serde_json::from_str(line).unwrap())
                            .collect();

                        criterion::black_box(parsed_items);
                    });
                });

                group.finish();
            }
        }
    };
}

/// Generate memory usage benchmarks
///
/// Creates benchmarks focused on memory allocation and usage patterns.
///
/// # Usage
/// ```rust
/// generate_memory_benchmarks!(ValidationResult, {
///     factory: create_test_validation_result,
///     group_name: "validation_result_memory"
/// });
/// ```
#[macro_export]
macro_rules! generate_memory_benchmarks {
    ($type:ty, {
        factory: $factory:ident,
        group_name: $group_name:expr
    }) => {
        paste::paste! {
            fn [<benchmark_ $type:snake _memory_usage>](c: &mut criterion::Criterion) {
                let mut group = c.benchmark_group($group_name);

                // Clone benchmark
                let item = $factory();
                group.bench_function("clone", |b| {
                    b.iter(|| {
                        criterion::black_box(criterion::black_box(&item).clone())
                    });
                });

                // Drop benchmark (measuring destructor performance)
                group.bench_function("drop", |b| {
                    b.iter_batched(
                        || $factory(),
                        |item| drop(criterion::black_box(item)),
                        criterion::BatchSize::SmallInput
                    );
                });

                // Vec creation and population
                group.bench_function("vec_creation_1000", |b| {
                    b.iter(|| {
                        let mut items = Vec::with_capacity(1000);
                        for _ in 0..1000 {
                            items.push($factory());
                        }
                        criterion::black_box(items);
                    });
                });

                // Vec clear and refill
                let mut base_vec: Vec<$type> = (0..1000).map(|_| $factory()).collect();
                group.bench_function("vec_clear_refill", |b| {
                    b.iter(|| {
                        base_vec.clear();
                        for _ in 0..1000 {
                            base_vec.push($factory());
                        }
                        criterion::black_box(&base_vec);
                    });
                });

                group.finish();
            }
        }
    };
}

/// Generate validation benchmarks for types with validation logic
///
/// Creates benchmarks for validation operations.
///
/// # Usage  
/// ```rust
/// generate_validation_benchmarks!(FineTuningJobRequest, {
///     valid_factory: create_valid_request,
///     invalid_factory: create_invalid_request,
///     group_name: "fine_tuning_validation"
/// });
/// ```
#[macro_export]
macro_rules! generate_validation_benchmarks {
    ($type:ty, {
        valid_factory: $valid_factory:ident,
        invalid_factory: $invalid_factory:ident,
        group_name: $group_name:expr
    }) => {
        paste::paste! {
            fn [<benchmark_ $type:snake _validation>](c: &mut criterion::Criterion) {
                let valid_item = $valid_factory();
                let invalid_item = $invalid_factory();
                let mut group = c.benchmark_group($group_name);

                // Validate valid item
                group.bench_function("validate_valid", |b| {
                    b.iter(|| {
                        // Assuming validation through serialization
                        let _json = serde_json::to_string(
                            criterion::black_box(&valid_item)
                        ).unwrap();
                    });
                });

                // Validate invalid item (expect error)
                group.bench_function("validate_invalid", |b| {
                    b.iter(|| {
                        let result = serde_json::to_string(
                            criterion::black_box(&invalid_item)
                        );
                        criterion::black_box(result);
                    });
                });

                group.finish();
            }
        }
    };
}

/// Generate comprehensive benchmark suite combining all benchmark types
///
/// Creates a complete benchmark suite with serialization, bulk processing,
/// file operations, and memory benchmarks.
///
/// # Usage
/// ```rust
/// generate_comprehensive_benchmarks!(ValidationResult, {
///     factory: create_test_validation_result,
///     base_name: "validation_result"
/// });
/// ```
#[macro_export]
macro_rules! generate_comprehensive_benchmarks {
    ($type:ty, {
        factory: $factory:ident,
        base_name: $base_name:expr
    }) => {
        // Generate all benchmark types
        generate_serialization_benchmarks!($type, {
            factory: $factory,
            group_name: concat!($base_name, "_serialization")
        });

        generate_bulk_benchmarks!($type, {
            factory: $factory,
            group_name: concat!($base_name, "_bulk"),
            sizes: [1, 10, 100, 500]
        });

        generate_file_benchmarks!($type, {
            factory: $factory,
            group_name: concat!($base_name, "_file")
        });

        generate_memory_benchmarks!($type, {
            factory: $factory,
            group_name: concat!($base_name, "_memory")
        });

        paste::paste! {
            // Create a combined benchmark function that registers all benchmarks
            pub fn [<register_ $type:snake _benchmarks>](c: &mut criterion::Criterion) {
                [<benchmark_ $type:snake _serialization>](c);
                [<benchmark_ $type:snake _bulk_processing>](c);
                [<benchmark_ $type:snake _file_processing>](c);
                [<benchmark_ $type:snake _memory_usage>](c);
            }
        }
    };
}

/// Generate benchmark comparison suite
///
/// Creates benchmarks that compare performance between different implementations
/// or configurations.
///
/// # Usage
/// ```rust
/// generate_comparison_benchmarks!({
///     name: "serialization_comparison",
///     comparisons: [
///         (serde_json, "serde_json", |item| serde_json::to_string(item)),
///         (custom_json, "custom_json", |item| custom_serialize(item))
///     ]
/// });
/// ```
#[macro_export]
macro_rules! generate_comparison_benchmarks {
    ({
        name: $name:expr,
        setup: $setup:expr,
        comparisons: [$(($id:ident, $label:expr, $func:expr)),+]
    }) => {
        paste::paste! {
            fn [<benchmark_ $name>](c: &mut criterion::Criterion) {
                let test_data = $setup();
                let mut group = c.benchmark_group($name);

                $(
                    group.bench_function($label, |b| {
                        let func = $func;
                        b.iter(|| {
                            criterion::black_box(func(criterion::black_box(&test_data)));
                        });
                    });
                )+

                group.finish();
            }
        }
    };
}

/// Generate throughput benchmarks
///
/// Creates benchmarks that measure throughput (operations per second)
/// rather than latency.
///
/// # Usage
/// ```rust
/// generate_throughput_benchmarks!(ValidationResult, {
///     factory: create_test_validation_result,
///     group_name: "validation_throughput",
///     operations: [
///         (serialize, |item| serde_json::to_string(item).unwrap()),
///         (validate, |item| item.is_valid())
///     ]
/// });
/// ```
#[macro_export]
macro_rules! generate_throughput_benchmarks {
    ($type:ty, {
        factory: $factory:ident,
        group_name: $group_name:expr,
        operations: [$(($op_name:ident, $op_func:expr)),+]
    }) => {
        paste::paste! {
            fn [<benchmark_ $type:snake _throughput>](c: &mut criterion::Criterion) {
                let mut group = c.benchmark_group($group_name);
                group.throughput(criterion::Throughput::Elements(1));

                $(
                    let items: Vec<_> = (0..1000).map(|_| $factory()).collect();
                    group.bench_function(stringify!($op_name), |b| {
                        let op = $op_func;
                        b.iter(|| {
                            for item in criterion::black_box(&items) {
                                criterion::black_box(op(item));
                            }
                        });
                    });
                )+

                group.finish();
            }
        }
    };
}
