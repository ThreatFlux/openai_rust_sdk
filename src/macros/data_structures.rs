//! Data structure macros for generating common patterns in request/response types

/// Macro to implement ListQueryParams trait for pagination structures
#[macro_export]
macro_rules! impl_list_query_params {
    ($struct_name:ident, $order_type:ty) => {
        impl crate::api::common::ListQueryParams for $struct_name {
            fn limit(&self) -> Option<u32> {
                self.limit
            }

            fn order_str(&self) -> Option<&str> {
                self.order.as_ref().map(|o| match o {
                    $crate::models::threads::types::SortOrder::Asc => "asc",
                    $crate::models::threads::types::SortOrder::Desc => "desc",
                })
            }

            fn after(&self) -> Option<&String> {
                self.after.as_ref()
            }

            fn before(&self) -> Option<&String> {
                self.before.as_ref()
            }
        }
    };
}

/// Macro to generate list parameter structures with pagination support
#[macro_export]
macro_rules! impl_list_params {
    ($struct_name:ident, $doc_prefix:literal) => {
        #[doc = concat!("Parameters for listing ", $doc_prefix)]
        #[derive(Debug, Clone, PartialEq, $crate::macros::Ser, $crate::macros::De)]
        pub struct $struct_name {
            /// A limit on the number of objects to be returned (1-100, default 20)
            #[serde(skip_serializing_if = "Option::is_none")]
            pub limit: Option<u32>,
            /// Sort order by the `created_at` timestamp (asc or desc, default desc)
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order: Option<String>,
            /// A cursor for use in pagination. after is an object ID that defines your place in the list
            #[serde(skip_serializing_if = "Option::is_none")]
            pub after: Option<String>,
            /// A cursor for use in pagination. before is an object ID that defines your place in the list
            #[serde(skip_serializing_if = "Option::is_none")]
            pub before: Option<String>,
        }

        impl ListParams for $struct_name {
            fn get_limit(&self) -> Option<u32> {
                self.limit
            }

            fn get_order(&self) -> Option<&String> {
                self.order.as_ref()
            }

            fn get_after(&self) -> Option<&String> {
                self.after.as_ref()
            }

            fn get_before(&self) -> Option<&String> {
                self.before.as_ref()
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                let (limit, order, after, before) = create_default_list_params();
                Self {
                    limit,
                    order,
                    after,
                    before,
                }
            }
        }

        impl $struct_name {
            /// Build query parameters for the API request
            #[must_use]
            pub fn to_query_params(&self) -> Vec<(String, String)> {
                self.build_query_params()
            }
        }
    };
}

/// Macro to generate list response structures
#[macro_export]
macro_rules! impl_list_response {
    ($struct_name:ident, $data_type:ty, $doc_prefix:literal) => {
        #[doc = concat!("Response containing a list of ", $doc_prefix)]
        #[derive(Debug, Clone, PartialEq, $crate::macros::Ser, $crate::macros::De)]
        pub struct $struct_name {
            /// The object type, which is always `list`
            pub object: String,
            #[doc = concat!("The list of ", $doc_prefix)]
            pub data: Vec<$data_type>,
            /// The first ID in the list
            pub first_id: Option<String>,
            /// The last ID in the list
            pub last_id: Option<String>,
            /// Whether there are more results available
            pub has_more: bool,
        }
    };
}

/// Macro to generate success/failure response constructors
#[macro_export]
macro_rules! impl_response_constructors {
    ($struct_name:ident, $id_field:ident, $object_type:literal) => {
        impl $struct_name {
            /// Create a successful response
            #[must_use]
            pub fn success($id_field: String) -> Self {
                Self {
                    $id_field,
                    object: $object_type.to_string(),
                    deleted: true,
                }
            }

            /// Create a failed response
            #[must_use]
            pub fn failure($id_field: String) -> Self {
                Self {
                    $id_field,
                    object: $object_type.to_string(),
                    deleted: false,
                }
            }

            /// Check if the operation was successful
            #[must_use]
            pub fn is_success(&self) -> bool {
                self.deleted
            }
        }
    };
}

/// Macro to generate list response filtering methods for responses with data field
#[macro_export]
macro_rules! impl_list_response_methods {
    ($struct_name:ident, $item_type:ty, $id_field:ident) => {
        impl $struct_name {
            /// Create a new empty list response
            #[must_use]
            pub fn empty() -> Self {
                Self {
                    object: "list".to_string(),
                    data: Vec::new(),
                    first_id: None,
                    last_id: None,
                    has_more: false,
                }
            }

            /// Create a new list response with data
            #[must_use]
            pub fn with_data(data: Vec<$item_type>) -> Self {
                let first_id = data.first().map(|item| item.$id_field.clone());
                let last_id = data.last().map(|item| item.$id_field.clone());

                Self {
                    object: "list".to_string(),
                    data,
                    first_id,
                    last_id,
                    has_more: false,
                }
            }

            /// Get the count of items
            #[must_use]
            pub fn count(&self) -> usize {
                self.data.len()
            }

            /// Check if the response is empty
            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.data.is_empty()
            }
        }
    };
}

/// Macro to generate status filtering methods for list responses
#[macro_export]
macro_rules! impl_status_filters {
    ($struct_name:ident, $item_type:ty, $status_type:ty, $status_field:ident) => {
        impl $struct_name {
            /// Get items by status
            #[must_use]
            pub fn by_status(&self, status: &$status_type) -> Vec<&$item_type> {
                self.data
                    .iter()
                    .filter(|item| item.$status_field == *status)
                    .collect()
            }
        }
    };
}

/// Macro to generate common list parameter types with builder pattern methods
/// This eliminates massive duplication in vector stores, fine tuning, and similar modules
#[macro_export]
macro_rules! impl_list_params_with_builder {
    (
        $struct_name:ident {
            base_fields: {
                limit: $limit_doc:literal,
                order: $order_doc:literal,
                after: $after_doc:literal,
                before: $before_doc:literal
            }
            $(, extra_fields: {
                $($extra_field:ident: $extra_type:ty = $extra_doc:literal),*
            })?
        }
    ) => {
        /// List parameters with pagination support
        #[derive(Debug, Clone, Default)]
        pub struct $struct_name {
            #[doc = $limit_doc]
            pub limit: Option<u32>,
            #[doc = $order_doc]
            pub order: Option<String>,
            #[doc = $after_doc]
            pub after: Option<String>,
            #[doc = $before_doc]
            pub before: Option<String>,
            $($(
                #[doc = $extra_doc]
                pub $extra_field: Option<$extra_type>,
            )*)?
        }

        impl $struct_name {
            /// Create new parameters with default values
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }

            /// Set the limit
            #[must_use]
            pub fn with_limit(mut self, limit: u32) -> Self {
                self.limit = Some(limit);
                self
            }

            /// Set the sort order
            pub fn with_order(mut self, order: impl Into<String>) -> Self {
                self.order = Some(order.into());
                self
            }

            /// Set the after cursor for pagination
            pub fn with_after(mut self, after: impl Into<String>) -> Self {
                self.after = Some(after.into());
                self
            }

            /// Set the before cursor for pagination
            pub fn with_before(mut self, before: impl Into<String>) -> Self {
                self.before = Some(before.into());
                self
            }

            $($(
                /// Set extra field
                #[must_use]
                pub fn $extra_field(mut self, value: $extra_type) -> Self {
                    self.$extra_field = Some(value);
                    self
                }
            )*)?

            /// Check if pagination parameters are set
            #[must_use]
            pub fn has_pagination(&self) -> bool {
                self.after.is_some() || self.before.is_some()
            }

            $(
                /// Check if filtering parameters are set
                #[must_use]
                pub fn has_filters(&self) -> bool {
                    $(self.$extra_field.is_some() ||)* false
                }
            )?
        }

        impl $crate::models::vector_stores::common_types::QueryParamBuilder for $struct_name {
            fn to_query_params(&self) -> Vec<(String, String)> {
                let mut params = Vec::new();

                if let Some(limit) = self.limit {
                    params.push(("limit".to_string(), limit.to_string()));
                }

                if let Some(order) = &self.order {
                    params.push(("order".to_string(), order.clone()));
                }

                if let Some(after) = &self.after {
                    params.push(("after".to_string(), after.clone()));
                }

                if let Some(before) = &self.before {
                    params.push(("before".to_string(), before.clone()));
                }

                $($(
                    if let Some(value) = &self.$extra_field {
                        params.push((stringify!($extra_field).to_string(), value.to_string()));
                    }
                )*)?

                params
            }

            fn is_empty(&self) -> bool {
                self.limit.is_none()
                    && self.order.is_none()
                    && self.after.is_none()
                    && self.before.is_none()
                    $($(
                        && self.$extra_field.is_none()
                    )*)?
            }
        }
    };
}

/// Macro to generate fine-tuning job parameter types with pagination
#[macro_export]
macro_rules! impl_fine_tuning_params {
    ($struct_name:ident, $doc_prefix:literal) => {
        #[doc = concat!("Parameters for listing ", $doc_prefix)]
        #[derive(Debug, Clone, Default)]
        pub struct $struct_name {
            /// Identifier for the last item from the previous pagination request
            pub after: Option<String>,
            /// Number of items to retrieve (1-100, default: 20)
            pub limit: Option<u32>,
        }

        impl $struct_name {
            /// Create new list parameters
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }

            /// Set the after cursor for pagination
            pub fn after(mut self, after: impl Into<String>) -> Self {
                self.after = Some(after.into());
                self
            }

            /// Set the limit for number of items to return
            #[must_use]
            pub fn limit(mut self, limit: u32) -> Self {
                self.limit = Some(limit);
                self
            }

            /// Build query parameters for the API request
            #[must_use]
            pub fn to_query_params(&self) -> Vec<(String, String)> {
                let mut params = Vec::new();
                if let Some(after) = &self.after {
                    params.push(("after".to_string(), after.clone()));
                }
                if let Some(limit) = self.limit {
                    params.push(("limit".to_string(), limit.to_string()));
                }
                params
            }
        }
    };
}

/// Macro to generate status checking methods for entities
#[macro_export]
macro_rules! impl_status_methods {
    ($struct_name:ty, $status_type:ty, {
        $($method_name:ident => $status_variant:ident),* $(,)?
    }) => {
        impl $struct_name {
            $(
                /// Check status
                #[must_use]
                pub fn $method_name(&self) -> bool {
                    matches!(self.status, <$status_type>::$status_variant)
                }
            )*
        }
    };
}

/// Macro to generate comprehensive test suites for API clients and models
#[macro_export]
macro_rules! impl_api_test_suite {
    (
        $api_type:ty,
        $request_type:ty,
        $response_type:ty,
        tests: {
            api_creation: {
                api_key: $api_key:literal,
                base_url: $base_url:literal
            },
            builder: {
                required_fields: [$($req_field:ident: $req_value:expr),*],
                optional_fields: [$($opt_field:ident: $opt_value:expr),*]
            },
            serialization: $serialize_test:literal
        }
    ) => {
        #[test]
        fn test_api_creation() {
            let api = <$api_type>::new($api_key).unwrap();
            assert_eq!(api.api_key(), $api_key);
            assert_eq!(api.base_url(), $base_url);
        }

        #[test]
        fn test_api_custom_base_url() {
            let api = <$api_type>::new_with_base_url($api_key, "https://custom.openai.com").unwrap();
            assert_eq!(api.api_key(), $api_key);
            assert_eq!(api.base_url(), "https://custom.openai.com");
        }

        #[test]
        fn test_request_builder() {
            let request = <$request_type>::builder()
                $(
                    .$req_field($req_value)
                )*
                $(
                    .$opt_field($opt_value)
                )*
                .build();

            match request {
                Ok(req) => {
                    $(
                        // Test required fields are set
                        assert_eq!(req.$req_field, $req_value);
                    )*
                },
                Err(e) => panic!("Builder should succeed: {}", e)
            }
        }

        #[test]
        fn test_request_serialization() {
            let request = <$request_type>::builder()
                $(
                    .$req_field($req_value)
                )*
                .build()
                .unwrap();

            let json = serde_json::to_string(&request).unwrap();
            assert!($serialize_test);

            let deserialized: $request_type = serde_json::from_str(&json).unwrap();
            $(
                assert_eq!(deserialized.$req_field, request.$req_field);
            )*
        }
    };
}

/// Macro to generate common test helper functions for request/response testing
#[macro_export]
macro_rules! impl_test_helpers {
    (
        $module_name:ident,
        helpers: {
            $($helper_name:ident: $helper_type:ty = $helper_impl:expr),*
        }
    ) => {
        mod $module_name {
            use super::*;

            $(
                pub fn $helper_name() -> $helper_type {
                    $helper_impl
                }
            )*
        }
    };
}

/// Macro to generate comprehensive test suites that can replace entire test modules
#[macro_export]
macro_rules! impl_comprehensive_test_suite {
    (
        $module_prefix:literal,
        api: $api_type:ty,
        request: $request_type:ty,
        status_enum: $status_enum:ty,
        config: {
            api_key: $api_key:literal,
            base_url: $base_url:literal
        },
        builder_tests: {
            required: [$($req_field:ident: $req_val:expr => $req_assertion:expr),*],
            optional: [$($opt_field:ident: $opt_val:expr => $opt_assertion:expr),*]
        },
        status_tests: [$($status_variant:ident),*],
        serialization: {
            contains: [$($json_fragment:literal),*]
        }
    ) => {
        #[test]
        fn test_api_creation() {
            let api = <$api_type>::new($api_key).unwrap();
            assert_eq!(api.api_key(), $api_key);
            assert_eq!(api.base_url(), $base_url);
        }

        #[test]
        fn test_api_custom_base_url() {
            let custom_url = "https://custom.openai.com";
            let api = <$api_type>::new_with_base_url($api_key, custom_url).unwrap();
            assert_eq!(api.api_key(), $api_key);
            assert_eq!(api.base_url(), custom_url);
        }

        #[test]
        fn test_builder_required_fields() {
            let request = <$request_type>::builder()
                $(.$req_field($req_val))*
                .build()
                .unwrap();

            $(
                assert!($req_assertion);
            )*
        }

        #[test]
        fn test_builder_optional_fields() {
            let request = <$request_type>::builder()
                $(.$req_field($req_val))*
                $(.$opt_field($opt_val))*
                .build()
                .unwrap();

            $(
                assert!($opt_assertion);
            )*
        }

        #[test]
        fn test_builder_missing_required_fields() {
            let result = <$request_type>::builder().build();
            assert!(result.is_err());
        }

        $(
            #[test]
            fn paste::paste! { test_status_[<$status_variant:lower>]() } {
                // This would test each status variant - simplified for demo
                let status = <$status_enum>::$status_variant;
                // Status-specific assertions would go here
                assert!(format!("{:?}", status).contains(stringify!($status_variant)));
            }
        )*

        #[test]
        fn test_serialization_round_trip() {
            let request = <$request_type>::builder()
                $(.$req_field($req_val))*
                .build()
                .unwrap();

            let json = serde_json::to_string(&request).unwrap();
            
            $(
                assert!(json.contains($json_fragment), "JSON should contain: {}", $json_fragment);
            )*

            let deserialized: $request_type = serde_json::from_str(&json).unwrap();
            
            // Verify round-trip integrity
            let json2 = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(json, json2, "Serialization should be deterministic");
        }

        #[test]
        fn test_default_values() {
            let request = <$request_type>::builder()
                $(.$req_field($req_val))*
                .build()
                .unwrap();
            
            // Test that default values are properly set
            let json = serde_json::to_string(&request).unwrap();
            let deserialized: $request_type = serde_json::from_str(&json).unwrap();
            
            $(
                assert!($req_assertion);
            )*
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
        }
    };
}

/// Macro to generate parameter testing suites with pagination
#[macro_export]
macro_rules! impl_params_test_suite {
    (
        $params_type:ty,
        tests: {
            default: $default_test:expr,
            builder_chain: {
                methods: [$($method:ident($value:expr)),*],
                assertions: [$($assertion:expr),*]
            },
            query_params: {
                expected_count: $expected_count:expr,
                contains: [$($param_check:expr),*]
            },
            pagination: {
                methods: [$($page_method:ident($page_val:expr)),*]
            }
        }
    ) => {
        #[test]
        fn test_params_default() {
            let params = <$params_type>::new();
            assert!($default_test);
        }

        #[test]
        fn test_params_builder_chaining() {
            let params = <$params_type>::new()
                $(.$method($value))*;

            $(
                assert!($assertion);
            )*
        }

        #[test]
        fn test_params_to_query_params() {
            let params = <$params_type>::new()
                $(.$method($value))*;

            let query_params = params.to_query_params();
            assert_eq!(query_params.len(), $expected_count);
            
            $(
                assert!($param_check);
            )*
        }

        #[test]
        fn test_params_pagination() {
            let params = <$params_type>::new()
                $(.$page_method($page_val))*;

            // Test pagination functionality
            let query_params = params.to_query_params();
            assert!(!query_params.is_empty());
        }

        #[test]
        fn test_params_empty() {
            let empty_params = <$params_type>::new();
            let query_params = empty_params.to_query_params();
            assert!(query_params.is_empty() || query_params.len() == 0);
        }
    };
}
