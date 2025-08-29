//! Shared traits and implementations to eliminate model duplication
//!
//! This module provides common traits and generic implementations that can be
//! shared across different model types, dramatically reducing code duplication.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Common trait for status enums across different APIs
pub trait StatusEnum: Clone + PartialEq + Serialize + for<'de> Deserialize<'de> + Display {
    /// Check if the status represents a terminal state
    fn is_terminal(&self) -> bool;

    /// Check if the status represents an active/in-progress state  
    fn is_active(&self) -> bool;

    /// Check if the status represents a failed state
    fn is_failed(&self) -> bool {
        false // Default implementation, can be overridden
    }

    /// Check if the status represents a completed/successful state
    fn is_completed(&self) -> bool {
        false // Default implementation, can be overridden
    }
}

/// Generic macro to implement StatusEnum for snake_case serialized enums
#[macro_export]
macro_rules! impl_status_enum {
    ($enum_type:ty, {
        terminal: [$($terminal:ident),+],
        active: [$($active:ident),+],
        $(failed: [$($failed:ident),+],)?
        $(completed: [$($completed:ident),+],)?
    }) => {
        impl $crate::models::shared_traits::StatusEnum for $enum_type {
            fn is_terminal(&self) -> bool {
                matches!(self, $(Self::$terminal)|+)
            }

            fn is_active(&self) -> bool {
                matches!(self, $(Self::$active)|+)
            }

            $(
                fn is_failed(&self) -> bool {
                    matches!(self, $(Self::$failed)|+)
                }
            )?

            $(
                fn is_completed(&self) -> bool {
                    matches!(self, $(Self::$completed)|+)
                }
            )?
        }

        impl std::fmt::Display for $enum_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let snake_case = match self {
                    $(Self::$terminal => stringify!($terminal),)+
                    $(Self::$active => stringify!($active),)+
                };
                // Convert CamelCase to snake_case
                let mut result = String::new();
                for (i, ch) in snake_case.chars().enumerate() {
                    if ch.is_uppercase() && i > 0 {
                        result.push('_');
                    }
                    result.push(ch.to_lowercase().next().unwrap());
                }
                write!(f, "{}", result)
            }
        }
    };
}

/// Trait for builder patterns with metadata support
pub trait MetadataBuilder<T> {
    /// Add a metadata entry
    fn add_metadata<K: Into<String>, V: Into<String>>(self, key: K, value: V) -> Self;

    /// Set all metadata at once
    fn with_metadata(self, metadata: HashMap<String, String>) -> Self;

    /// Add a metadata entry (alias for add_metadata)
    fn metadata_entry<K: Into<String>, V: Into<String>>(self, key: K, value: V) -> Self
    where
        Self: Sized,
    {
        self.add_metadata(key, value)
    }

    /// Add a metadata pair (alias for add_metadata)
    fn metadata_pair<K: Into<String>, V: Into<String>>(self, key: K, value: V) -> Self
    where
        Self: Sized,
    {
        self.add_metadata(key, value)
    }
}

/// Generic macro to implement MetadataBuilder for any builder type
#[macro_export]
macro_rules! impl_metadata_builder {
    ($builder_type:ty, $field_name:ident) => {
        impl $crate::models::shared_traits::MetadataBuilder<$builder_type> for $builder_type {
            fn add_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
                if self.$field_name.is_none() {
                    self.$field_name = Some(std::collections::HashMap::new());
                }
                self.$field_name
                    .as_mut()
                    .unwrap()
                    .insert(key.into(), value.into());
                self
            }

            fn with_metadata(
                mut self,
                metadata: std::collections::HashMap<String, String>,
            ) -> Self {
                self.$field_name = Some(metadata);
                self
            }
        }
    };
}

/// Trait for list parameters with pagination support
pub trait PaginationParams {
    /// Set the limit for the number of items to return
    fn with_limit(self, limit: i32) -> Self;

    /// Set the cursor for pagination (after)
    fn with_after<S: Into<String>>(self, after: S) -> Self;

    /// Set the cursor for pagination (before)
    fn with_before<S: Into<String>>(self, before: S) -> Self;

    /// Convert to query parameters for API requests
    fn to_query_params(&self) -> Vec<(String, String)>;
}

/// Generic macro to implement PaginationParams
#[macro_export]
macro_rules! impl_pagination_params {
    ($param_type:ty, {
        limit: $limit_field:ident,
        after: $after_field:ident,
        $(before: $before_field:ident,)?
        $(order: $order_field:ident,)?
        $(filter: $filter_field:ident,)?
    }) => {
        impl $crate::models::shared_traits::PaginationParams for $param_type {
            fn with_limit(mut self, limit: i32) -> Self {
                self.$limit_field = Some(limit);
                self
            }

            fn with_after<S: Into<String>>(mut self, after: S) -> Self {
                self.$after_field = Some(after.into());
                self
            }

            $(
                fn with_before<S: Into<String>>(mut self, before: S) -> Self {
                    self.$before_field = Some(before.into());
                    self
                }
            )?

            fn to_query_params(&self) -> Vec<(String, String)> {
                let mut params = Vec::new();

                if let Some(limit) = self.$limit_field {
                    params.push(("limit".to_string(), limit.to_string()));
                }

                if let Some(ref after) = self.$after_field {
                    params.push(("after".to_string(), after.clone()));
                }

                $(
                    if let Some(ref before) = self.$before_field {
                        params.push(("before".to_string(), before.clone()));
                    }
                )?

                $(
                    if let Some(ref order) = self.$order_field {
                        params.push(("order".to_string(), order.clone()));
                    }
                )?

                $(
                    if let Some(ref filter) = self.$filter_field {
                        params.push(("filter".to_string(), filter.to_string()));
                    }
                )?

                params
            }
        }

        $(
            impl $param_type {
                pub fn with_order<S: Into<String>>(mut self, order: S) -> Self {
                    self.$order_field = Some(order.into());
                    self
                }
            }
        )?

        $(
            impl $param_type {
                pub fn with_filter<F>(mut self, filter: F) -> Self
                where
                    F: std::fmt::Display,
                {
                    self.$filter_field = Some(filter);
                    self
                }
            }
        )?
    };
}

/// Trait for list responses with filtering capabilities
pub trait FilterableListResponse<T> {
    /// Get all items from the response
    fn items(&self) -> &[T];

    /// Check if response is empty
    fn is_empty(&self) -> bool {
        self.items().is_empty()
    }

    /// Get the number of items
    fn len(&self) -> usize {
        self.items().len()
    }

    /// Filter items by a predicate
    fn filter_items<P>(&self, predicate: P) -> Vec<&T>
    where
        P: Fn(&T) -> bool,
    {
        self.items().iter().filter(|item| predicate(item)).collect()
    }
}

/// Generic macro to implement FilterableListResponse
#[macro_export]
macro_rules! impl_filterable_list_response {
    ($response_type:ty, $item_type:ty, $data_field:ident) => {
        impl $crate::models::shared_traits::FilterableListResponse<$item_type> for $response_type {
            fn items(&self) -> &[$item_type] {
                &self.$data_field
            }
        }

        impl $response_type {
            /// Create an empty response
            pub fn empty() -> Self {
                Self {
                    object: "list".to_string(),
                    $data_field: Vec::new(),
                    first_id: None,
                    last_id: None,
                    has_more: false,
                }
            }
        }
    };
}

/// Trait for request types with file ID support
pub trait FileSupported<T> {
    /// Add a file ID to the request
    fn add_file_id<S: Into<String>>(self, file_id: S) -> Self;

    /// Set file IDs for the request
    fn with_file_ids(self, file_ids: Vec<String>) -> Self;

    /// Add a file ID (alias)
    fn file_id<S: Into<String>>(self, file_id: S) -> Self
    where
        Self: Sized,
    {
        self.add_file_id(file_id)
    }
}

/// Generic macro to implement FileSupported
#[macro_export]
macro_rules! impl_file_supported {
    ($builder_type:ty, $field_name:ident) => {
        impl $crate::models::shared_traits::FileSupported<$builder_type> for $builder_type {
            fn add_file_id<S: Into<String>>(mut self, file_id: S) -> Self {
                if self.$field_name.is_none() {
                    self.$field_name = Some(Vec::new());
                }
                self.$field_name.as_mut().unwrap().push(file_id.into());
                self
            }

            fn with_file_ids(mut self, file_ids: Vec<String>) -> Self {
                self.$field_name = Some(file_ids);
                self
            }
        }
    };
}

/// Trait for types that can be measured in bytes
pub trait BytesUsage {
    /// Get usage in bytes
    fn usage_bytes(&self) -> u64;

    /// Get human-readable usage string
    fn usage_human_readable(&self) -> String {
        let bytes = self.usage_bytes();
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

/// Generic macro to implement BytesUsage
#[macro_export]
macro_rules! impl_bytes_usage {
    ($type:ty, $field_name:ident) => {
        impl $crate::models::shared_traits::BytesUsage for $type {
            fn usage_bytes(&self) -> u64 {
                self.$field_name
            }
        }
    };
}

/// Trait for delete responses
pub trait DeleteResponse {
    /// Check if the deletion was successful
    fn is_deleted(&self) -> bool;

    /// Get the ID of the deleted item
    fn deleted_id(&self) -> &str;

    /// Create a successful delete response
    fn success(id: String) -> Self;

    /// Create a failed delete response  
    fn failure(id: String) -> Self;
}

/// Generic macro to implement DeleteResponse
#[macro_export]
macro_rules! impl_delete_response {
    ($response_type:ty, $object_type:expr) => {
        impl $crate::models::shared_traits::DeleteResponse for $response_type {
            fn is_deleted(&self) -> bool {
                self.deleted
            }

            fn deleted_id(&self) -> &str {
                &self.id
            }

            fn success(id: String) -> Self {
                Self {
                    id,
                    object: $object_type.to_string(),
                    deleted: true,
                }
            }

            fn failure(id: String) -> Self {
                Self {
                    id,
                    object: $object_type.to_string(),
                    deleted: false,
                }
            }
        }
    };
}

/// Trait for expiration support
pub trait ExpirationSupported {
    /// Check if the item expires soon (within 24 hours)
    fn expires_soon(&self) -> bool;

    /// Check if the item has expired
    fn has_expired(&self) -> bool;

    /// Get expiration timestamp if available
    fn expires_at(&self) -> Option<i64>;
}

/// Trait for builder pattern validation
pub trait ValidatedBuilder<T, E> {
    /// Build the final object with validation
    fn build(self) -> Result<T, E>;

    /// Build the final object without validation (unsafe)
    fn build_unchecked(self) -> T;
}

/// Generic error type for builder validation
#[derive(Debug, Clone, PartialEq)]
pub struct BuilderError {
    /// The error message
    pub message: String,
    /// The field that caused the error, if applicable
    pub field: Option<String>,
}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(ref field) = self.field {
            write!(f, "Builder error in field '{}': {}", field, self.message)
        } else {
            write!(f, "Builder error: {}", self.message)
        }
    }
}

impl std::error::Error for BuilderError {}

impl BuilderError {
    /// Create a new builder error with a message
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            field: None,
        }
    }

    /// Create a new builder error with a message and field name
    pub fn with_field<S: Into<String>, F: Into<String>>(message: S, field: F) -> Self {
        Self {
            message: message.into(),
            field: Some(field.into()),
        }
    }

    /// Create a builder error for a required field
    pub fn required_field<F: Into<String>>(field: F) -> Self {
        let field_name = field.into();
        Self {
            message: format!("{} is required", field_name),
            field: Some(field_name),
        }
    }
}
