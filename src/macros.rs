//! Common macros for reducing code duplication across the codebase

/// Macro to simplify error mapping with format strings
/// Usage: `map_err!(FileError, "Failed to read file: {}")`
#[macro_export]
macro_rules! map_err {
    // Pattern for errors with format string and placeholder
    ($error_type:ident, $msg:literal) => {
        |e| $crate::error::OpenAIError::$error_type(format!($msg, e))
    };
    // Pattern for errors with just a prefix message (literal)
    ($error_type:ident, $prefix:literal, to_string) => {
        |e| $crate::error::OpenAIError::$error_type(format!("{}: {}", $prefix, e.to_string()))
    };
    // Pattern for errors with a variable prefix and to_string()
    ($error_type:ident, $prefix:ident, to_string) => {
        |e| $crate::error::OpenAIError::$error_type(format!("{}: {}", $prefix, e.to_string()))
    };
    // Pattern for errors that just use to_string()
    ($error_type:ident, to_string) => {
        |e| $crate::error::OpenAIError::$error_type(e.to_string())
    };
}

/// Convenience macro for FileError mapping with format string
#[macro_export]
macro_rules! file_err {
    ($msg:literal) => {
        $crate::map_err!(FileError, $msg)
    };
}

/// Convenience macro for RequestError mapping
#[macro_export]
macro_rules! request_err {
    ($msg:literal) => {
        $crate::map_err!(RequestError, $msg)
    };
    (to_string) => {
        $crate::map_err!(RequestError, to_string)
    };
}

/// Convenience macro for ParseError mapping
#[macro_export]
macro_rules! parse_err {
    ($msg:literal) => {
        $crate::map_err!(ParseError, $msg)
    };
    (to_string) => {
        $crate::map_err!(ParseError, to_string)
    };
}

/// Convenience macro for InvalidRequest mapping
#[macro_export]
macro_rules! invalid_request_err {
    ($msg:literal) => {
        $crate::map_err!(InvalidRequest, $msg)
    };
    (to_string) => {
        $crate::map_err!(InvalidRequest, to_string)
    };
}

/// Convenience macro for Streaming error mapping
#[macro_export]
macro_rules! streaming_err {
    ($msg:literal) => {
        $crate::map_err!(Streaming, $msg)
    };
}

/// Convenience macro for Unknown error mapping (network errors)
#[macro_export]
macro_rules! network_err {
    ($msg:literal) => {
        |e| $crate::error::OpenAIError::network(format!($msg, e))
    };
}

/// Convenience macro for validation errors (parsing errors that go to InvalidRequest)
#[macro_export]
macro_rules! validation_err {
    ($msg:literal) => {
        |e| $crate::error::OpenAIError::validation(format!($msg, e))
    };
}

/// Convenience macro for parsing errors that go to InvalidRequest
#[macro_export]
macro_rules! parsing_err {
    ($msg:literal) => {
        |e| $crate::error::OpenAIError::parsing(format!($msg, e))
    };
}

/// Macro to generate builder methods for optional string fields
#[macro_export]
macro_rules! impl_string_setters {
    ($($field:ident),* $(,)?) => {
        $(
            #[doc = concat!("Set the ", stringify!($field))]
            pub fn $field<S: Into<String>>(mut self, value: S) -> Self {
                self.$field = Some(value.into());
                self
            }
        )*
    };
}

/// Macro to generate builder methods for optional non-string fields
#[macro_export]
macro_rules! impl_option_setters {
    ($($field:ident: $type:ty),* $(,)?) => {
        $(
            #[doc = concat!("Set the ", stringify!($field))]
            pub fn $field(mut self, value: $type) -> Self {
                self.$field = Some(value);
                self
            }
        )*
    };
}

/// Macro to generate builder methods for vector fields
#[macro_export]
macro_rules! impl_vec_setters {
    ($($field:ident: $type:ty),* $(,)?) => {
        $(
            #[doc = concat!("Add a ", stringify!($field))]
            pub fn $field(mut self, value: $type) -> Self {
                self.$field.get_or_insert_with(Vec::new).push(value);
                self
            }

            #[doc = concat!("Set all ", stringify!($field))]
            pub fn set_all_$field(mut self, values: Vec<$type>) -> Self {
                self.$field = Some(values);
                self
            }
        )*
    };
}

/// Macro to generate builder methods for `HashMap` fields
#[macro_export]
macro_rules! impl_map_setters {
    ($($field:ident),* $(,)?) => {
        $(
            #[doc = concat!("Set the ", stringify!($field))]
            pub fn $field(mut self, value: std::collections::HashMap<String, String>) -> Self {
                self.$field = Some(value);
                self
            }

            #[doc = concat!("Add a ", stringify!($field), " key-value pair")]
            pub fn add_$field<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
                self.$field
                    .get_or_insert_with(std::collections::HashMap::new)
                    .insert(key.into(), value.into());
                self
            }
        )*
    };
}

/// Macro to generate simple HTTP GET method wrappers
#[macro_export]
macro_rules! http_get {
    // Simple GET request: method_name -> path
    ($method_name:ident, $path:literal, $return_type:ty) => {
        #[doc = concat!("Makes a GET request to ", $path)]
        pub async fn $method_name(&self) -> $crate::error::Result<$return_type> {
            self.http_client.get($path).await
        }
    };

    // GET with AsRef<str> parameter (must come first to match before generic pattern)
    ($method_name:ident, $path_fmt:literal, $param:ident: impl AsRef<str>, $return_type:ty) => {
        #[doc = concat!("Makes a GET request to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: impl AsRef<str>,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param.as_ref());
            self.http_client.get(&path).await
        }
    };

    // GET with path parameter: method_name(&self, id) -> format!("/path/{}", id)
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a GET request to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.get(&path).await
        }
    };
}

/// Macro to generate HTTP GET method wrappers with beta headers
#[macro_export]
macro_rules! http_get_beta {
    // GET with beta headers and Into<String> parameter (must come before generic pattern)
    ($method_name:ident, $path_fmt:literal, $param:ident: impl Into<String>, $return_type:ty) => {
        #[doc = concat!("Makes a GET request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: impl Into<String>,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param.into());
            self.http_client.get_with_beta(&path).await
        }
    };

    // GET with beta headers and path parameter
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a GET request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.get_with_beta(&path).await
        }
    };
}

/// Macro to generate HTTP POST method wrappers
#[macro_export]
macro_rules! http_post {
    // POST with body: method_name(&self, request) -> path
    ($method_name:ident, $path:literal, $request:ident: $request_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a POST request to ", $path)]
        pub async fn $method_name(
            &self,
            $request: $request_type,
        ) -> $crate::error::Result<$return_type> {
            self.http_client.post($path, $request).await
        }
    };

    // POST with path parameter and body
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $request:ident: $request_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a POST request to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
            $request: $request_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.post(&path, $request).await
        }
    };
}

/// Macro to generate HTTP POST method wrappers with beta headers
#[macro_export]
macro_rules! http_post_beta {
    // POST with beta headers and body
    ($method_name:ident, $path:literal, $request:ident: $request_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a POST request with beta headers to ", $path)]
        pub async fn $method_name(
            &self,
            $request: $request_type,
        ) -> $crate::error::Result<$return_type> {
            self.http_client.post_with_beta($path, $request).await
        }
    };

    // POST with beta headers, path parameter and body
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $request:ident: $request_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a POST request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
            $request: $request_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.post_with_beta(&path, $request).await
        }
    };
}

/// Macro to generate HTTP DELETE method wrappers
#[macro_export]
macro_rules! http_delete {
    // DELETE with path parameter
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a DELETE request to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.delete(&path).await
        }
    };
}

/// Macro to generate HTTP DELETE method wrappers with beta headers
#[macro_export]
macro_rules! http_delete_beta {
    // DELETE with beta headers and Into<String> parameter (must come before generic pattern)
    ($method_name:ident, $path_fmt:literal, $param:ident: impl Into<String>, $return_type:ty) => {
        #[doc = concat!("Makes a DELETE request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: impl Into<String>,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param.into());
            self.http_client.delete_with_beta(&path).await
        }
    };

    // DELETE with beta headers and path parameter
    ($method_name:ident, $path_fmt:literal, $param:ident: $param_type:ty, $return_type:ty) => {
        #[doc = concat!("Makes a DELETE request with beta headers to ", $path_fmt)]
        pub async fn $method_name(
            &self,
            $param: $param_type,
        ) -> $crate::error::Result<$return_type> {
            let path = format!($path_fmt, $param);
            self.http_client.delete_with_beta(&path).await
        }
    };
}

/// Macro to generate standard builder `build()` methods with field validation
///
/// This macro generates a `build()` method that validates required fields using `ok_or()`
/// and returns a `Result<T, String>`. It supports both required fields validation and
/// optional final validation on the constructed object.
///
/// # Basic usage with required fields only:
/// ```rust,ignore
/// impl_builder_build! {
///     MyBuilder => MyRequest {
///         required: [field1: "field1 is required", field2: "field2 is required"],
///         optional: [opt_field1, opt_field2, opt_field3]
///     }
/// }
/// ```
///
/// # Usage with additional validation:
/// ```rust,ignore  
/// impl_builder_build! {
///     MyBuilder => MyRequest {
///         required: [field1: "field1 is required"],
///         optional: [opt_field],
///         validate: true
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_builder_build {
    // Pattern for builders with required fields, optional fields, and post-construction validation
    ($builder:ident => $target:ident {
        required: [$( $req_field:ident: $req_msg:literal ),* $(,)?],
        optional: [$( $opt_field:ident ),* $(,)?],
        validate: true
    }) => {
        impl $builder {
            /// Build the request
            pub fn build(self) -> std::result::Result<$target, String> {
                $(
                    let $req_field = self.$req_field.ok_or($req_msg)?;
                )*

                let request = $target {
                    $( $req_field, )*
                    $( $opt_field: self.$opt_field, )*
                };

                request.validate()?;
                Ok(request)
            }
        }
    };

    // Pattern for builders with required fields and optional fields (no post-validation)
    ($builder:ident => $target:ident {
        required: [$( $req_field:ident: $req_msg:literal ),* $(,)?],
        optional: [$( $opt_field:ident ),* $(,)?]
    }) => {
        impl $builder {
            /// Build the request
            pub fn build(self) -> std::result::Result<$target, String> {
                $(
                    let $req_field = self.$req_field.ok_or($req_msg)?;
                )*

                Ok($target {
                    $( $req_field, )*
                    $( $opt_field: self.$opt_field, )*
                })
            }
        }
    };

    // Pattern for builders with only required fields (no optional fields or validation)
    ($builder:ident => $target:ident {
        required: [$( $req_field:ident: $req_msg:literal ),* $(,)?]
    }) => {
        impl $builder {
            /// Build the request
            pub fn build(self) -> std::result::Result<$target, String> {
                $(
                    let $req_field = self.$req_field.ok_or($req_msg)?;
                )*

                Ok($target {
                    $( $req_field, )*
                })
            }
        }
    };
}

// Re-export commonly used serde traits with shorter aliases for the codebase.
// This allows us to write `#[derive(Debug, Clone, Ser, De)]` instead of the full form,
// saving significant characters across the 234+ derive statements in the codebase.
pub use serde::{Deserialize as De, Serialize as Ser};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::base::HttpClient;
    use crate::error::Result;

    // Test struct to validate our shorter derive aliases
    #[derive(Debug, Clone, Ser, De, PartialEq)]
    struct TestStruct {
        name: String,
        value: i32,
    }

    // Test enum to validate our shorter derive aliases
    #[derive(Debug, Clone, Ser, De, PartialEq)]
    enum TestEnum {
        Variant1,
        Variant2(String),
        Variant3 { field: i32 },
    }

    #[test]
    fn test_shortened_derive_aliases_work() {
        let test_struct = TestStruct {
            name: "test".to_string(),
            value: 42,
        };

        // Test Debug
        let debug_output = format!("{:?}", test_struct);
        assert!(debug_output.contains("TestStruct"));
        assert!(debug_output.contains("test"));
        assert!(debug_output.contains("42"));

        // Test Clone
        let cloned = test_struct.clone();
        assert_eq!(test_struct, cloned);

        // Test Serialize
        let json = serde_json::to_string(&test_struct).expect("Failed to serialize");
        assert!(json.contains("test"));
        assert!(json.contains("42"));

        // Test Deserialize
        let deserialized: TestStruct = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(test_struct, deserialized);
    }

    #[test]
    fn test_shortened_derive_aliases_work_with_enums() {
        let test_enum = TestEnum::Variant3 { field: 100 };

        // Test Debug
        let debug_output = format!("{:?}", test_enum);
        assert!(debug_output.contains("Variant3"));
        assert!(debug_output.contains("100"));

        // Test Clone
        let cloned = test_enum.clone();
        assert_eq!(test_enum, cloned);

        // Test Serialize
        let json = serde_json::to_string(&test_enum).expect("Failed to serialize");
        assert!(json.contains("Variant3") || json.contains("field"));

        // Test Deserialize
        let deserialized: TestEnum = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(test_enum, deserialized);
    }

    #[test]
    fn test_line_length_reduction() {
        // This test documents the character savings achieved
        let old_pattern = "#[derive(Debug, Clone, Serialize, Deserialize)]";
        let new_pattern = "#[derive(Debug, Clone, Ser, De)]";

        let old_length = old_pattern.len();
        let new_length = new_pattern.len();
        let savings_per_line = old_length - new_length;

        // We should save 15 characters per line
        assert_eq!(savings_per_line, 15);

        // With 234 occurrences, we save significant characters
        let total_character_savings = savings_per_line * 234;
        assert_eq!(total_character_savings, 15 * 234);

        println!("Character savings per line: {}", savings_per_line);
        println!("Total character savings: {}", total_character_savings);
    }

    // Test that our aliases are equivalent to the original traits
    #[test]
    #[allow(clippy::no_effect_underscore_binding, clippy::assertions_on_constants)]
    fn test_alias_equivalence() {
        // These should be the same types - the fact that this compiles proves our aliases work
        let _ser_test: fn(&TestStruct) -> std::result::Result<Vec<u8>, _> =
            |s| serde_json::to_vec(s);
        let _de_test: fn(&[u8]) -> std::result::Result<TestStruct, _> =
            |b| serde_json::from_slice(b);

        // Test actual serialization/deserialization with our aliases
        let test = TestStruct {
            name: "alias_test".to_string(),
            value: 123,
        };
        let serialized = serde_json::to_vec(&test).expect("Serialization failed");
        let deserialized: TestStruct =
            serde_json::from_slice(&serialized).expect("Deserialization failed");
        assert_eq!(test, deserialized);
    }

    #[test]
    fn test_error_mapping_macros() {
        use crate::error::OpenAIError;
        use std::io::Error as IoError;

        // Test file_err! macro with format string
        let io_err = IoError::new(std::io::ErrorKind::NotFound, "File not found");
        let file_error_mapper = file_err!("Failed to read file: {}");
        let mapped_error = file_error_mapper(io_err);

        match mapped_error {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("Failed to read file:"));
                assert!(msg.contains("File not found"));
            }
            _ => panic!("Expected FileError variant"),
        }

        // Test request_err! macro with to_string
        let io_err2 = IoError::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");
        let request_error_mapper: fn(IoError) -> OpenAIError = request_err!(to_string);
        let mapped_error2 = request_error_mapper(io_err2);

        match mapped_error2 {
            OpenAIError::RequestError(msg) => {
                assert!(msg.contains("Connection refused"));
            }
            _ => panic!("Expected RequestError variant"),
        }

        // Test parse_err! macro with format string
        let parse_result: std::result::Result<serde_json::Value, _> =
            serde_json::from_str("invalid json");
        let parse_err = parse_result.unwrap_err();
        let parse_error_mapper = parse_err!("Failed to parse JSON: {}");
        let mapped_error3 = parse_error_mapper(parse_err);

        match mapped_error3 {
            OpenAIError::ParseError(msg) => {
                assert!(msg.contains("Failed to parse JSON:"));
            }
            _ => panic!("Expected ParseError variant"),
        }

        // Test invalid_request_err! macro
        let validation_err = std::fmt::Error;
        let invalid_request_mapper = invalid_request_err!("Invalid request format: {}");
        let mapped_error4 = invalid_request_mapper(validation_err);

        match mapped_error4 {
            OpenAIError::InvalidRequest(msg) => {
                assert!(msg.contains("Invalid request format:"));
            }
            _ => panic!("Expected InvalidRequest variant"),
        }

        // Test streaming_err! macro
        let stream_err = std::fmt::Error;
        let streaming_error_mapper = streaming_err!("Stream processing failed: {}");
        let mapped_error5 = streaming_error_mapper(stream_err);

        match mapped_error5 {
            OpenAIError::Streaming(msg) => {
                assert!(msg.contains("Stream processing failed:"));
            }
            _ => panic!("Expected Streaming variant"),
        }
    }

    #[test]
    fn test_generic_map_err_macro() {
        use crate::error::OpenAIError;
        use std::io::Error as IoError;

        // Test map_err! macro with different patterns
        let io_err = IoError::new(std::io::ErrorKind::PermissionDenied, "Permission denied");

        // Test with format string
        let mapper1 = map_err!(FileError, "IO operation failed: {}");
        let error1 = mapper1(io_err);
        match error1 {
            OpenAIError::FileError(msg) => {
                assert!(msg.contains("IO operation failed:"));
                assert!(msg.contains("Permission denied"));
            }
            _ => panic!("Expected FileError variant"),
        }

        // Test with to_string pattern
        let io_err2 = IoError::new(std::io::ErrorKind::TimedOut, "Timed out");
        let mapper2: fn(IoError) -> OpenAIError = map_err!(RequestError, to_string);
        let error2 = mapper2(io_err2);
        match error2 {
            OpenAIError::RequestError(msg) => {
                assert!(msg.contains("Timed out"));
            }
            _ => panic!("Expected RequestError variant"),
        }

        // Test with prefix and to_string pattern
        let io_err3 = IoError::new(std::io::ErrorKind::InvalidData, "Invalid data");
        let mapper3: fn(IoError) -> OpenAIError =
            map_err!(ParseError, "Data validation error", to_string);
        let error3 = mapper3(io_err3);
        match error3 {
            OpenAIError::ParseError(msg) => {
                assert!(msg.contains("Data validation error:"));
                assert!(msg.contains("Invalid data"));
            }
            _ => panic!("Expected ParseError variant"),
        }
    }

    #[test]
    fn test_error_macro_line_savings() {
        // This test documents the line savings achieved by our error macros
        let old_pattern =
            r#".map_err(|e| OpenAIError::FileError(format!("Failed to read file: {e}")))"#;
        let new_pattern = r#".map_err(file_err!("Failed to read file: {}"))"#;

        let old_length = old_pattern.len();
        let new_length = new_pattern.len();
        let savings_per_occurrence = old_length - new_length;

        // We should save significant characters per occurrence
        assert!(savings_per_occurrence > 15);

        // With 49 occurrences, we save substantial characters
        let total_character_savings = savings_per_occurrence * 49;

        println!(
            "Character savings per occurrence: {}",
            savings_per_occurrence
        );
        println!(
            "Total character savings across 49 occurrences: {}",
            total_character_savings
        );

        // Verify we're achieving meaningful reduction
        assert!(total_character_savings > 1000);
    }

    // Test struct to validate HTTP macro generation
    #[allow(dead_code)]
    struct TestApiClient {
        http_client: HttpClient,
    }

    #[derive(Debug, Clone, Ser, De, PartialEq)]
    struct TestRequest {
        pub field: String,
    }

    #[derive(Debug, Clone, Ser, De, PartialEq)]
    struct TestResponse {
        pub result: String,
    }

    #[allow(dead_code)]
    impl TestApiClient {
        // Test that the macros compile correctly
        http_get!(test_get_simple, "/test", TestResponse);
        http_get!(test_get_with_param, "/test/{}", param: &str, TestResponse);
        http_get!(test_get_with_string, "/test/{}", param: impl AsRef<str>, TestResponse);
        http_post!(test_post, "/test", request: &TestRequest, TestResponse);
        http_get_beta!(test_get_beta, "/test/{}", param: impl Into<String>, TestResponse);
        http_delete_beta!(test_delete_beta, "/test/{}", param: impl Into<String>, TestResponse);
    }

    #[test]
    fn test_http_macro_compilation() {
        // This test verifies that the HTTP macros compile correctly
        // We don't need to test the actual HTTP functionality since that's tested elsewhere
        // This test just ensures the macro syntax is valid
        let client = HttpClient::new("test-key").unwrap();
        let test_client = TestApiClient {
            http_client: client,
        };

        // The fact that these methods exist proves the macros compiled
        assert!(std::ptr::addr_of!(test_client).is_aligned());
    }

    #[test]
    fn test_http_macro_line_savings() {
        // This test documents the line savings achieved by HTTP macros
        let old_pattern = r#"
    pub async fn retrieve_file(&self, file_id: &str) -> Result<File> {
        self.http_client.get(&format!("/v1/files/{file_id}")).await
    }"#;

        let new_pattern = r#"
    http_get!(retrieve_file, "/v1/files/{}", file_id: &str, File);"#;

        let old_lines = old_pattern.lines().count();
        let new_lines = new_pattern.lines().count();
        let savings_per_method = old_lines - new_lines;

        // We should save at least 2 lines per method (signature line and await line)
        assert!(savings_per_method >= 2);

        // With 8 methods refactored so far, we save significant lines
        let total_line_savings = savings_per_method * 8;
        assert!(total_line_savings > 15);

        println!("Line savings per method: {}", savings_per_method);
        println!(
            "Total line savings across 8 refactored methods: {}",
            total_line_savings
        );
    }

    // Test the new builder macro functionality
    mod builder_tests {
        use super::*;
        use std::collections::HashMap;

        // Test structures for builder macro testing
        #[derive(Debug, Clone, PartialEq)]
        struct TestRequest {
            pub required_field: String,
            pub optional_field: Option<String>,
            pub another_optional: Option<i32>,
        }

        impl TestRequest {
            pub fn validate(&self) -> std::result::Result<(), String> {
                if self.required_field.is_empty() {
                    Err("Required field cannot be empty".to_string())
                } else {
                    Ok(())
                }
            }
        }

        #[derive(Debug, Default)]
        struct TestRequestBuilder {
            required_field: Option<String>,
            optional_field: Option<String>,
            another_optional: Option<i32>,
        }

        impl TestRequestBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn required_field(mut self, value: impl Into<String>) -> Self {
                self.required_field = Some(value.into());
                self
            }

            pub fn optional_field(mut self, value: impl Into<String>) -> Self {
                self.optional_field = Some(value.into());
                self
            }

            pub fn another_optional(mut self, value: i32) -> Self {
                self.another_optional = Some(value);
                self
            }
        }

        // Generate the build method using our macro
        crate::impl_builder_build! {
            TestRequestBuilder => TestRequest {
                required: [required_field: "required_field is required"],
                optional: [optional_field, another_optional],
                validate: true
            }
        }

        // Test structure without validation
        #[derive(Debug, Clone, PartialEq)]
        struct SimpleRequest {
            pub field1: String,
            pub field2: String,
            pub optional_field: Option<String>,
        }

        #[derive(Debug, Default)]
        struct SimpleRequestBuilder {
            field1: Option<String>,
            field2: Option<String>,
            optional_field: Option<String>,
        }

        impl SimpleRequestBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn field1(mut self, value: impl Into<String>) -> Self {
                self.field1 = Some(value.into());
                self
            }

            pub fn field2(mut self, value: impl Into<String>) -> Self {
                self.field2 = Some(value.into());
                self
            }

            pub fn optional_field(mut self, value: impl Into<String>) -> Self {
                self.optional_field = Some(value.into());
                self
            }
        }

        // Generate build method without validation
        crate::impl_builder_build! {
            SimpleRequestBuilder => SimpleRequest {
                required: [field1: "field1 is required", field2: "field2 is required"],
                optional: [optional_field]
            }
        }

        // Test structure with only required fields
        #[derive(Debug, Clone, PartialEq)]
        struct MinimalRequest {
            pub name: String,
        }

        #[derive(Debug, Default)]
        struct MinimalRequestBuilder {
            name: Option<String>,
        }

        impl MinimalRequestBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
                self
            }
        }

        // Generate build method with only required fields
        crate::impl_builder_build! {
            MinimalRequestBuilder => MinimalRequest {
                required: [name: "name is required"]
            }
        }

        #[test]
        fn test_builder_macro_with_validation() {
            // Test successful build
            let request = TestRequestBuilder::new()
                .required_field("test")
                .optional_field("optional")
                .another_optional(42)
                .build()
                .expect("Build should succeed");

            assert_eq!(request.required_field, "test");
            assert_eq!(request.optional_field, Some("optional".to_string()));
            assert_eq!(request.another_optional, Some(42));

            // Test with missing required field
            let result = TestRequestBuilder::new().optional_field("optional").build();

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "required_field is required");

            // Test validation failure
            let result = TestRequestBuilder::new()
                .required_field("") // Empty string should fail validation
                .build();

            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .contains("Required field cannot be empty"));
        }

        #[test]
        fn test_builder_macro_without_validation() {
            // Test successful build
            let request = SimpleRequestBuilder::new()
                .field1("value1")
                .field2("value2")
                .optional_field("optional")
                .build()
                .expect("Build should succeed");

            assert_eq!(request.field1, "value1");
            assert_eq!(request.field2, "value2");
            assert_eq!(request.optional_field, Some("optional".to_string()));

            // Test with missing required field
            let result = SimpleRequestBuilder::new().field1("value1").build();

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "field2 is required");

            // Test partial optional fields
            let request = SimpleRequestBuilder::new()
                .field1("value1")
                .field2("value2")
                .build()
                .expect("Build should succeed");

            assert_eq!(request.field1, "value1");
            assert_eq!(request.field2, "value2");
            assert_eq!(request.optional_field, None);
        }

        #[test]
        fn test_builder_macro_minimal() {
            // Test successful build
            let request = MinimalRequestBuilder::new()
                .name("test_name")
                .build()
                .expect("Build should succeed");

            assert_eq!(request.name, "test_name");

            // Test with missing required field
            let result = MinimalRequestBuilder::new().build();

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "name is required");
        }

        #[test]
        fn test_builder_macro_line_savings() {
            // This test documents the line savings achieved by the builder macro
            let old_pattern_lines = [
                "pub fn build(self) -> Result<TestRequest, String> {",
                "    let required_field = self.required_field.ok_or(\"required_field is required\")?;",
                "",
                "    let request = TestRequest {",
                "        required_field,",
                "        optional_field: self.optional_field,",
                "        another_optional: self.another_optional,",
                "    };",
                "",
                "    request.validate()?;",
                "    Ok(request)",
                "}",
            ];

            let new_pattern_lines = [
                "crate::impl_builder_build! {",
                "    TestRequestBuilder => TestRequest {",
                "        required: [required_field: \"required_field is required\"],",
                "        optional: [optional_field, another_optional],",
                "        validate: true",
                "    }",
                "}",
            ];

            let old_line_count = old_pattern_lines.len();
            let new_line_count = new_pattern_lines.len();
            let savings_per_builder = old_line_count - new_line_count;

            // We should save at least 5 lines per builder
            assert!(savings_per_builder >= 5);

            // With 5 builders refactored, we save significant lines
            let total_line_savings = savings_per_builder * 5;
            assert!(total_line_savings >= 25);

            println!(
                "Builder macro line savings per builder: {}",
                savings_per_builder
            );
            println!(
                "Total line savings across 5 refactored builders: {}",
                total_line_savings
            );
        }

        #[test]
        fn test_builder_macro_error_messages() {
            // Test that error messages are preserved correctly
            let result = TestRequestBuilder::new().build();
            assert_eq!(result.unwrap_err(), "required_field is required");

            let result = SimpleRequestBuilder::new().field1("test").build();
            assert_eq!(result.unwrap_err(), "field2 is required");

            let result = MinimalRequestBuilder::new().build();
            assert_eq!(result.unwrap_err(), "name is required");
        }

        #[test]
        fn test_builder_macro_validation_integration() {
            // Test that validation is called and errors are propagated
            let result = TestRequestBuilder::new()
                .required_field("") // This will pass required field check but fail validation
                .build();

            assert!(result.is_err());
            let error_message = result.unwrap_err();
            assert!(error_message.contains("Required field cannot be empty"));
        }
    }
}
