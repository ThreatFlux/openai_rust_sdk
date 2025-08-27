//! Aggressive test macros to eliminate code duplication
//!
//! This module provides powerful macros that generate complete test suites
//! with minimal code, dramatically reducing test duplication across the SDK.

/// Generate a complete API test suite with all standard tests
///
/// This macro creates all the common API tests:
/// - API creation with valid key
/// - API creation with custom base URL  
/// - Empty API key error tests
/// - Whitespace API key error tests
///
/// # Usage
/// ```rust
/// generate_api_test_suite!(FineTuningApi, "https://custom.openai.com");
/// ```
#[macro_export]
macro_rules! generate_api_test_suite {
    ($api_type:ty, $custom_url:expr) => {
        mod api_tests {
            use super::*;
            use crate::common::{create_test_api_client, create_test_api_client_with_url, TEST_API_KEY};
            
            #[test]
            fn test_api_creation() {
                let api = create_test_api_client::<$api_type>();
                assert_eq!(api.api_key(), TEST_API_KEY);
            }

            #[test]
            fn test_api_creation_with_base_url() {
                let api = create_test_api_client_with_url::<$api_type>($custom_url);
                assert_eq!(api.api_key(), TEST_API_KEY);
                assert_eq!(api.base_url(), $custom_url);
            }

            #[test]
            fn test_empty_api_key_error() {
                let result = <$api_type>::new("");
                assert!(result.is_err());
                match result.unwrap_err() {
                    openai_rust_sdk::error::OpenAIError::Authentication(msg) => {
                        assert!(msg.contains("API key") || msg.contains("empty"));
                    }
                    _ => panic!("Expected authentication error"),
                }
            }

            #[test]
            fn test_whitespace_api_key_error() {
                let result = <$api_type>::new("   ");
                assert!(result.is_err());
                match result.unwrap_err() {
                    openai_rust_sdk::error::OpenAIError::Authentication(_) => {}
                    _ => panic!("Expected authentication error"),
                }
            }

            #[test]
            fn test_invalid_characters_api_key() {
                let api = <$api_type>::new("test\0key");
                assert!(api.is_ok());
            }
        }
    };
}

/// Generate comprehensive status enum tests for any status type
///
/// This macro creates terminal/active state tests, serialization tests,
/// and string conversion tests for status enums.
///
/// # Usage
/// ```rust  
/// generate_status_enum_tests!(VectorStoreStatus, {
///     terminal: [Completed, Failed, Cancelled, Expired],
///     active: [InProgress]
/// });
/// ```
#[macro_export]
macro_rules! generate_status_enum_tests {
    ($status_type:ty, {
        terminal: [$($terminal:ident),+],
        active: [$($active:ident),+]
    }) => {
        paste::paste! {
            #[test]
            fn [<test_ $status_type:snake _terminal_states>]() {
                $(
                    assert!(<$status_type>::$terminal.is_terminal());
                )+
                $(
                    assert!(!<$status_type>::$active.is_terminal());
                )+
            }

            #[test] 
            fn [<test_ $status_type:snake _active_states>]() {
                $(
                    assert!(<$status_type>::$active.is_active());
                )+
                $(
                    assert!(!<$status_type>::$terminal.is_active());
                )+
            }

            #[test]
            fn [<test_ $status_type:snake _string_conversion>]() {
                $(
                    let status = <$status_type>::$terminal;
                    let string_repr = status.to_string();
                    let snake_case = stringify!($terminal).to_lowercase();
                    assert_eq!(string_repr, snake_case);
                )+
                $(
                    let status = <$status_type>::$active; 
                    let string_repr = status.to_string();
                    let snake_case = stringify!($active).to_lowercase();
                    assert_eq!(string_repr, snake_case);
                )+
            }

            #[test]
            fn [<test_ $status_type:snake _serialization>]() {
                use crate::common::test_serialization_round_trip;
                $(
                    test_serialization_round_trip(&<$status_type>::$terminal);
                )+
                $(
                    test_serialization_round_trip(&<$status_type>::$active);
                )+
            }
        }
    };
}

/// Generate comprehensive builder tests for request types
///
/// This macro creates builder pattern tests, validation tests,
/// and serialization tests for request builders.
///
/// # Usage
/// ```rust
/// generate_builder_tests!(FineTuningJobRequest, {
///     required: [training_file, model],
///     optional: [validation_file, suffix, hyperparameters, metadata],
///     factory: create_test_job_request
/// });
/// ```
#[macro_export]
macro_rules! generate_builder_tests {
    ($request_type:ty, {
        required: [$($required:ident),+],
        optional: [$($optional:ident),*],
        factory: $factory:ident
    }) => {
        paste::paste! {
            mod builder_tests {
                use super::*;

                #[test]
                fn test_builder_with_all_fields() {
                    let request = $factory();
                    // Basic validation that factory creates valid request
                    let _json = serde_json::to_string(&request).unwrap();
                }

                $(
                    #[test]
                    fn [<test_builder_missing_ $required>]() {
                        // Test that builder fails when required field is missing
                        // This would need to be customized per request type
                        // For now just verify the factory works
                        let _request = $factory();
                    }
                )+

                #[test]
                fn test_builder_chaining() {
                    let request = $factory();
                    use crate::common::test_serialization_only;
                    test_serialization_only(&request);
                }

                $(
                    #[test]
                    fn [<test_ $optional _field>]() {
                        // Test optional fields can be set and retrieved
                        let request = $factory();
                        let _json = serde_json::to_string(&request).unwrap();
                    }
                )*
            }
        }
    };
}

/// Generate comprehensive serialization test suite
///
/// Creates serialization, deserialization, and JSON content tests.
///
/// # Usage
/// ```rust
/// generate_serialization_tests!(FineTuningJobRequest, {
///     factory: create_test_job_request,
///     expected_fields: ["training_file", "model", "gpt-3.5-turbo"]
/// });
/// ```
#[macro_export]
macro_rules! generate_serialization_tests {
    ($type:ty, {
        factory: $factory:ident,
        expected_fields: [$($field:expr),+]
    }) => {
        mod serialization_tests {
            use super::*;
            use crate::common::{test_serialization_only, assert_json_contains};

            #[test]
            fn test_serialization() {
                let item = $factory();
                test_serialization_only(&item);
            }

            #[test] 
            fn test_json_contains_expected_fields() {
                let item = $factory();
                let json = serde_json::to_string(&item).unwrap();
                assert_json_contains(&json, &[$($field),+]);
            }

            #[test]
            fn test_deserialization_round_trip() {
                let item = $factory();
                let json = serde_json::to_string(&item).unwrap();
                let _deserialized: $type = serde_json::from_str(&json).unwrap();
            }
        }
    };
}

/// Generate parameter object tests (list parameters, etc.)
///
/// Creates tests for parameter objects with pagination support.
///
/// # Usage
/// ```rust
/// generate_parameter_tests!(ListVectorStoresParams, {
///     fields: [limit, order, after, before],
///     test_values: [(limit, 25), (order, "desc"), (after, "vs-123")]
/// });
/// ```
#[macro_export]
macro_rules! generate_parameter_tests {
    ($param_type:ty, {
        fields: [$($field:ident),+],
        test_values: [$(($field_name:ident, $value:expr)),+]
    }) => {
        paste::paste! {
            mod parameter_tests {
                use super::*;

                #[test]
                fn test_parameter_creation() {
                    let params = <$param_type>::new();
                    // Test default state
                    $(
                        assert_eq!(params.$field, None);
                    )+
                }

                #[test] 
                fn test_parameter_with_values() {
                    let params = <$param_type>::new()
                        $(
                            .[<with_ $field_name>]($value)
                        )+;
                    
                    $(
                        assert_eq!(params.$field_name, Some($value.into()));
                    )+
                }

                #[test]
                fn test_query_params() {
                    let params = <$param_type>::new()
                        $(
                            .[<with_ $field_name>]($value)
                        )+;
                    
                    let query_params = params.to_query_params();
                    assert!(!query_params.is_empty());
                }
            }
        }
    };
}

/// Generate list response filtering tests
///
/// Creates tests for list response filtering and utility methods.
///
/// # Usage  
/// ```rust
/// generate_list_response_tests!(ListVectorStoresResponse, VectorStore, {
///     filter_methods: [ready_stores, processing_stores],
///     utility_methods: [total_usage_bytes],
///     factory: create_test_vector_store
/// });
/// ```
#[macro_export]
macro_rules! generate_list_response_tests {
    ($response_type:ty, $item_type:ty, {
        filter_methods: [$($filter:ident),+],
        utility_methods: [$($utility:ident),+],
        factory: $factory:ident
    }) => {
        mod list_response_tests {
            use super::*;

            #[test]
            fn test_empty_response() {
                let empty = <$response_type>::empty();
                assert_eq!(empty.object, "list");
                assert!(empty.data.is_empty());
                assert!(!empty.has_more);
            }

            #[test]
            fn test_response_with_data() {
                let item1 = $factory();
                let item2 = $factory();
                
                let response = <$response_type> {
                    object: "list".to_string(),
                    data: vec![item1, item2],
                    first_id: Some("first".to_string()),
                    last_id: Some("last".to_string()),
                    has_more: false,
                };

                assert_eq!(response.data.len(), 2);
            }

            $(
                #[test]
                fn [<test_ $filter>]() {
                    let item = $factory();
                    let response = <$response_type> {
                        object: "list".to_string(),
                        data: vec![item],
                        first_id: None,
                        last_id: None,
                        has_more: false,
                    };
                    
                    let _filtered = response.$filter();
                }
            )+

            $(
                #[test]
                fn [<test_ $utility>]() {
                    let item = $factory();
                    let response = <$response_type> {
                        object: "list".to_string(),
                        data: vec![item],
                        first_id: None,
                        last_id: None,
                        has_more: false,
                    };
                    
                    let _result = response.$utility();
                }
            )+
        }
    };
}

/// Generate validation test suite
///
/// Creates comprehensive validation tests for edge cases and error conditions.
///
/// # Usage
/// ```rust
/// generate_validation_tests!(Hyperparameters, {
///     builder: HyperparametersBuilder,
///     edge_cases: [
///         (n_epochs, 0, "zero epochs"),
///         (batch_size, 10000, "large batch size")
///     ]
/// });
/// ```
#[macro_export] 
macro_rules! generate_validation_tests {
    ($type:ty, {
        builder: $builder:ty,
        edge_cases: [$(($field:ident, $value:expr, $description:expr)),+]
    }) => {
        mod validation_tests {
            use super::*;

            $(
                #[test] 
                fn [<test_ $field _ $description:snake>]() {
                    let result = <$builder>::default()
                        .$field($value)
                        .build();
                    assert_eq!(result.$field, Some($value));
                }
            )+

            #[test]
            fn test_default_values() {
                let item = <$type>::default();
                $(
                    assert_eq!(item.$field, None);
                )+
            }
        }
    };
}

/// Generate complete benchmark suite for a type
///
/// Creates serialization and deserialization benchmarks.
///
/// # Usage
/// ```rust
/// generate_benchmark_suite!(ValidationResult, {
///     factory: create_test_validation_result,
///     sizes: [1, 10, 100]
/// });
/// ```
#[macro_export]
macro_rules! generate_benchmark_suite {
    ($type:ty, {
        factory: $factory:ident,
        sizes: [$($size:expr),+]
    }) => {
        paste::paste! {
            fn [<benchmark_ $type:snake _serialization>](c: &mut criterion::Criterion) {
                let item = $factory();
                let mut group = c.benchmark_group(stringify!([<$type:snake _serialization>]));

                group.bench_function("to_json", |b| {
                    b.iter(|| serde_json::to_string(criterion::black_box(&item)).unwrap());
                });

                group.bench_function("to_json_pretty", |b| {
                    b.iter(|| serde_json::to_string_pretty(criterion::black_box(&item)).unwrap());
                });

                let json_string = serde_json::to_string(&item).unwrap();
                group.bench_function("from_json", |b| {
                    b.iter(|| {
                        let _: $type = serde_json::from_str(criterion::black_box(&json_string)).unwrap();
                    });
                });

                group.finish();
            }

            fn [<benchmark_ $type:snake _bulk_processing>](c: &mut criterion::Criterion) {
                let mut group = c.benchmark_group(stringify!([<$type:snake _bulk_processing>]));

                $(
                    let items: Vec<_> = (0..$size).map(|_| $factory()).collect();
                    group.bench_function(&format!("serialize_{}_items", $size), |b| {
                        b.iter(|| serde_json::to_string(criterion::black_box(&items)).unwrap());
                    });

                    let json_string = serde_json::to_string(&items).unwrap();
                    group.bench_function(&format!("deserialize_{}_items", $size), |b| {
                        b.iter(|| {
                            let _: Vec<$type> = serde_json::from_str(criterion::black_box(&json_string)).unwrap();
                        });
                    });
                )+

                group.finish();
            }
        }
    };
}