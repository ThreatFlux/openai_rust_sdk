//! Builder pattern macros for reducing boilerplate in builder implementations

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

/// Macro to generate RunConfigurationBuilder trait implementations
#[macro_export]
macro_rules! impl_run_config_builder {
    ($struct_name:ident) => {
        impl RunConfigurationBuilder for $struct_name {
            fn get_model_mut(&mut self) -> &mut Option<String> {
                &mut self.model
            }

            fn get_instructions_mut(&mut self) -> &mut Option<String> {
                &mut self.instructions
            }

            fn get_tools_mut(
                &mut self,
            ) -> &mut Option<Vec<$crate::models::assistants::AssistantTool>> {
                &mut self.tools
            }

            fn get_file_ids_mut(&mut self) -> &mut Option<Vec<String>> {
                &mut self.file_ids
            }

            fn get_metadata_mut(
                &mut self,
            ) -> &mut Option<std::collections::HashMap<String, String>> {
                &mut self.metadata
            }
        }
    };
}

/// Macro to generate run builder methods that delegate to RunConfigurationBuilder trait
#[macro_export]
macro_rules! impl_run_builder_methods {
    () => {
        /// Set the model
        pub fn model<S: Into<String>>(self, model: S) -> Self {
            RunConfigurationBuilder::model(self, model)
        }

        /// Set the instructions
        pub fn instructions<S: Into<String>>(self, instructions: S) -> Self {
            RunConfigurationBuilder::instructions(self, instructions)
        }

        /// Add a tool
        pub fn tool(self, tool: $crate::models::assistants::AssistantTool) -> Self {
            RunConfigurationBuilder::tool(self, tool)
        }

        /// Set tools
        #[must_use]
        pub fn tools(self, tools: Vec<$crate::models::assistants::AssistantTool>) -> Self {
            RunConfigurationBuilder::tools(self, tools)
        }

        /// Add a file ID
        pub fn file_id<S: Into<String>>(self, file_id: S) -> Self {
            RunConfigurationBuilder::file_id(self, file_id)
        }

        /// Set file IDs
        #[must_use]
        pub fn file_ids(self, file_ids: Vec<String>) -> Self {
            RunConfigurationBuilder::file_ids(self, file_ids)
        }

        /// Add metadata key-value pair
        pub fn metadata_pair<K: Into<String>, V: Into<String>>(self, key: K, value: V) -> Self {
            RunConfigurationBuilder::metadata_pair(self, key, value)
        }

        /// Set metadata
        #[must_use]
        pub fn metadata(self, metadata: std::collections::HashMap<String, String>) -> Self {
            RunConfigurationBuilder::metadata(self, metadata)
        }
    };
}

/// Macro to generate fluent setter methods for builders
#[macro_export]
macro_rules! impl_fluent_setters {
    (
        $self:ident;
        string: [$($string_field:ident),* $(,)?];
        option: [$($option_field:ident : $option_type:ty),* $(,)?];
        vec: [$($vec_field:ident : $vec_type:ty),* $(,)?];
        map: [$($map_field:ident),* $(,)?];
    ) => {
        $(
            /// Set the field (string)
            pub fn $string_field(mut $self, value: impl Into<String>) -> Self {
                $self.$string_field = Some(value.into());
                $self
            }
        )*

        $(
            /// Set the field (option)
            #[must_use]
            pub fn $option_field(mut $self, value: $option_type) -> Self {
                $self.$option_field = Some(value);
                $self
            }
        )*

        $(
            /// Add to the vector field
            pub fn $vec_field(mut $self, value: $vec_type) -> Self {
                if $self.$vec_field.is_none() {
                    $self.$vec_field = Some(Vec::new());
                }
                $self.$vec_field.as_mut().unwrap().push(value);
                $self
            }

            /// Set all values for the vector field
            #[must_use]
            pub fn set_$vec_field(mut $self, values: Vec<$vec_type>) -> Self {
                $self.$vec_field = Some(values);
                $self
            }
        )*

        $(
            /// Set the map field
            #[must_use]
            pub fn $map_field(mut $self, value: std::collections::HashMap<String, String>) -> Self {
                $self.$map_field = Some(value);
                $self
            }

            /// Add a key-value pair to the map field
            pub fn add_$map_field(mut $self, key: impl Into<String>, value: impl Into<String>) -> Self {
                if $self.$map_field.is_none() {
                    $self.$map_field = Some(std::collections::HashMap::new());
                }
                $self.$map_field
                    .as_mut()
                    .unwrap()
                    .insert(key.into(), value.into());
                $self
            }
        )*
    };
}
