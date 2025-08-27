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
