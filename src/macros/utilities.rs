//! Utility macros for common patterns and helper implementations

/// Macro to generate default object type functions
#[macro_export]
macro_rules! impl_default_object_type {
    ($fn_name:ident, $object_type:literal) => {
        fn $fn_name() -> String {
            $object_type.to_string()
        }
    };
}

/// Macro to generate Display implementations for enums that serialize as lowercase strings
#[macro_export]
macro_rules! impl_enum_display {
    ($enum_name:ident {
        $($variant:ident => $display:literal),* $(,)?
    }) => {
        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    $(
                        $enum_name::$variant => $display,
                    )*
                };
                write!(f, "{}", s)
            }
        }
    };
}

/// Macro to generate usage bytes methods for list responses
#[macro_export]
macro_rules! impl_usage_methods {
    ($struct_name:ident, $usage_field:ident) => {
        impl $struct_name {
            /// Get total usage bytes of all items
            #[must_use]
            pub fn total_usage_bytes(&self) -> u64 {
                self.data.iter().map(|item| item.$usage_field).sum()
            }

            /// Get human-readable total usage
            #[must_use]
            pub fn total_usage_human_readable(&self) -> String {
                bytes_to_human_readable(self.total_usage_bytes())
            }
        }
    };
}

/// Macro to generate bytes to human readable formatting
#[macro_export]
macro_rules! impl_bytes_to_human_readable {
    () => {
        /// Convert bytes to human-readable format
        #[must_use]
        pub fn bytes_to_human_readable(bytes: u64) -> String {
            let bytes = bytes as f64;
            if bytes < 1024.0 {
                format!("{bytes} B")
            } else if bytes < 1024.0 * 1024.0 {
                format!("{:.1} KB", bytes / 1024.0)
            } else if bytes < 1024.0 * 1024.0 * 1024.0 {
                format!("{:.1} MB", bytes / (1024.0 * 1024.0))
            } else {
                format!("{:.1} GB", bytes / (1024.0 * 1024.0 * 1024.0))
            }
        }
    };
}

// Re-export commonly used serde traits with shorter aliases for the codebase.
// This allows us to write `#[derive(Debug, Clone, Ser, De)]` instead of the full form,
// saving significant characters across the 234+ derive statements in the codebase.
pub use serde::{Deserialize as De, Serialize as Ser};
