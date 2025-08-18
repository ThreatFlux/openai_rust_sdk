//! Macros for reducing builder pattern duplication

/// Macro to implement standard builder pattern setter methods
#[macro_export]
macro_rules! impl_builder_setters {
    ($struct_name:ident, {
        $( $field:ident: String $(,)? )*
    }) => {
        impl $struct_name {
            $(
                #[doc = concat!("Set the ", stringify!($field))]
                pub fn $field<S: Into<String>>(mut self, value: S) -> Self {
                    self.$field = Some(value.into());
                    self
                }
            )*
        }
    };

    ($struct_name:ident, {
        strings: { $( $string_field:ident $(,)? )* },
        options: { $( $opt_field:ident: $opt_type:ty $(,)? )* },
        vecs: { $( $vec_field:ident: $vec_type:ty $(,)? )* }
    }) => {
        impl $struct_name {
            $(
                #[doc = concat!("Set the ", stringify!($string_field))]
                pub fn $string_field<S: Into<String>>(mut self, value: S) -> Self {
                    self.$string_field = Some(value.into());
                    self
                }
            )*

            $(
                #[doc = concat!("Set the ", stringify!($opt_field))]
                pub fn $opt_field(mut self, value: $opt_type) -> Self {
                    self.$opt_field = Some(value);
                    self
                }
            )*

            $(
                #[doc = concat!("Add a ", stringify!($vec_field))]
                pub fn $vec_field(mut self, value: $vec_type) -> Self {
                    self.$vec_field.get_or_insert_with(Vec::new).push(value);
                    self
                }
            )*
        }
    };
}

/// Macro to implement common validation methods
#[macro_export]
macro_rules! impl_validation {
    ($struct_name:ident, {
        max_length: { $( $field:ident: $max:expr $(,)? )* }
    }) => {
        impl $struct_name {
            /// Validate the request
            pub fn validate(&self) -> Result<(), String> {
                $(
                    if let Some(ref value) = self.$field {
                        if value.len() > $max {
                            return Err(format!(
                                "{} must be {} characters or less",
                                stringify!($field),
                                $max
                            ));
                        }
                    }
                )*
                Ok(())
            }
        }
    };
}
