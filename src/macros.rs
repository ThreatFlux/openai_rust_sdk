//! Common macros for reducing code duplication across the codebase

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
