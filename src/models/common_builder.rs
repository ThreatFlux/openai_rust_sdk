//! # Common Builder Patterns
//!
//! Shared builder traits and macros to eliminate duplication across different
//! API request builders while maintaining type safety and ergonomics.

/// A generic builder trait that provides common builder functionality
pub trait Builder<T> {
    /// Build the final request type
    fn build(self) -> T;
}

/// Trait for builders that support setting the number of items (n)
pub trait WithN<T>: Builder<T> {
    /// Set the number of items with validation
    fn n(self, n: u32) -> Self;
}

/// Trait for builders that support setting response format
pub trait WithFormat<T, F>: Builder<T> {
    /// Set the response format
    fn format(self, format: F) -> Self;
}

/// Trait for builders that support setting size
pub trait WithSize<T, S>: Builder<T> {
    /// Set the size
    fn size(self, size: S) -> Self;
}

/// Trait for builders that support setting user identifier
pub trait WithUser<T>: Builder<T> {
    /// Set the user identifier
    fn user(self, user: impl Into<String>) -> Self;
}

/// Trait for builders that support setting quality
pub trait WithQuality<T, Q>: Builder<T> {
    /// Set the quality
    fn quality(self, quality: Q) -> Self;
}

/// Trait for builders that support setting temperature
pub trait WithTemperature<T>: Builder<T> {
    /// Set the temperature (0.0 to 1.0)
    fn temperature(self, temperature: f32) -> Self;
}

/// Trait for builders that support setting speed
pub trait WithSpeed<T>: Builder<T> {
    /// Set the speed
    fn speed(self, speed: f32) -> Self;
}

/// Macro to generate common format methods for builders
#[macro_export]
macro_rules! impl_format_methods {
    ($builder:ty, $format_type:ty, $request_field:ident) => {
        impl $builder {
            /// Return URLs
            #[must_use]
            pub fn url_format(mut self) -> Self {
                self.$request_field.response_format = Some(<$format_type>::Url);
                self
            }

            /// Return base64 JSON
            #[must_use]
            pub fn b64_json_format(mut self) -> Self {
                self.$request_field.response_format = Some(<$format_type>::B64Json);
                self
            }
        }
    };
}

/// Macro to generate common size methods for image builders
#[macro_export]
macro_rules! impl_image_size_methods {
    ($builder:ty, $request_field:ident) => {
        impl $builder {
            /// Set size to 256x256
            #[must_use]
            pub fn size_256x256(mut self) -> Self {
                self.$request_field.size =
                    Some($crate::models::images::types::ImageSize::Size256x256);
                self
            }

            /// Set size to 512x512
            #[must_use]
            pub fn size_512x512(mut self) -> Self {
                self.$request_field.size =
                    Some($crate::models::images::types::ImageSize::Size512x512);
                self
            }

            /// Set size to 1024x1024
            #[must_use]
            pub fn size_1024x1024(mut self) -> Self {
                self.$request_field.size =
                    Some($crate::models::images::types::ImageSize::Size1024x1024);
                self
            }

            /// Set size to 1792x1024 (landscape)
            #[must_use]
            pub fn size_1792x1024(mut self) -> Self {
                self.$request_field.size =
                    Some($crate::models::images::types::ImageSize::Size1792x1024);
                self
            }

            /// Set size to 1024x1792 (portrait)
            #[must_use]
            pub fn size_1024x1792(mut self) -> Self {
                self.$request_field.size =
                    Some($crate::models::images::types::ImageSize::Size1024x1792);
                self
            }
        }
    };
}

/// Macro to generate common audio format methods for audio builders
#[macro_export]
macro_rules! impl_audio_format_methods {
    ($builder:ty, $request_field:ident) => {
        impl $builder {
            /// Use MP3 format
            #[must_use]
            pub fn mp3(mut self) -> Self {
                self.$request_field.response_format = Some($crate::models::AudioFormat::Mp3);
                self
            }

            /// Use Opus format
            #[must_use]
            pub fn opus(mut self) -> Self {
                self.$request_field.response_format = Some($crate::models::AudioFormat::Opus);
                self
            }

            /// Use AAC format
            #[must_use]
            pub fn aac(mut self) -> Self {
                self.$request_field.response_format = Some($crate::models::AudioFormat::Aac);
                self
            }

            /// Use FLAC format
            #[must_use]
            pub fn flac(mut self) -> Self {
                self.$request_field.response_format = Some($crate::models::AudioFormat::Flac);
                self
            }
        }
    };
}

/// Macro to generate common transcription format methods
#[macro_export]
macro_rules! impl_transcription_format_methods {
    ($builder:ty, $request_field:ident) => {
        impl $builder {
            /// Use JSON format
            #[must_use]
            pub fn json(mut self) -> Self {
                self.$request_field.response_format =
                    Some($crate::models::TranscriptionFormat::Json);
                self
            }

            /// Use verbose JSON format with timestamps
            #[must_use]
            pub fn verbose_json(mut self) -> Self {
                self.$request_field.response_format =
                    Some($crate::models::TranscriptionFormat::VerboseJson);
                self
            }

            /// Use plain text format
            #[must_use]
            pub fn text(mut self) -> Self {
                self.$request_field.response_format =
                    Some($crate::models::TranscriptionFormat::Text);
                self
            }

            /// Use SRT subtitle format
            #[must_use]
            pub fn srt(mut self) -> Self {
                self.$request_field.response_format =
                    Some($crate::models::TranscriptionFormat::Srt);
                self
            }

            /// Use WebVTT subtitle format
            #[must_use]
            pub fn vtt(mut self) -> Self {
                self.$request_field.response_format =
                    Some($crate::models::TranscriptionFormat::Vtt);
                self
            }
        }
    };
}

/// Macro to implement the basic Builder trait
#[macro_export]
macro_rules! impl_builder {
    ($builder:ty, $request_type:ty, $request_field:ident) => {
        impl $crate::models::common_builder::Builder<$request_type> for $builder {
            fn build(self) -> $request_type {
                self.$request_field
            }
        }
    };
}

/// Macro to implement WithN trait
#[macro_export]
macro_rules! impl_with_n {
    ($builder:ty, $request_type:ty, $request_field:ident, $validation:expr) => {
        impl $crate::models::common_builder::WithN<$request_type> for $builder {
            fn n(mut self, n: u32) -> Self {
                let validated_n = ($validation)(n);
                self.$request_field.n = Some(validated_n);
                self
            }
        }
    };
}

/// Macro to implement WithFormat trait
#[macro_export]
macro_rules! impl_with_format {
    ($builder:ty, $request_type:ty, $request_field:ident, $format_type:ty) => {
        impl $crate::models::common_builder::WithFormat<$request_type, $format_type> for $builder {
            fn format(mut self, format: $format_type) -> Self {
                self.$request_field.response_format = Some(format);
                self
            }
        }
    };
}

/// Macro to implement WithSize trait
#[macro_export]
macro_rules! impl_with_size {
    ($builder:ty, $request_type:ty, $request_field:ident, $size_type:ty) => {
        impl $crate::models::common_builder::WithSize<$request_type, $size_type> for $builder {
            fn size(mut self, size: $size_type) -> Self {
                self.$request_field.size = Some(size);
                self
            }
        }
    };
}

/// Macro to implement WithUser trait
#[macro_export]
macro_rules! impl_with_user {
    ($builder:ty, $request_type:ty, $request_field:ident) => {
        impl $crate::models::common_builder::WithUser<$request_type> for $builder {
            fn user(mut self, user: impl Into<String>) -> Self {
                self.$request_field.user = Some(user.into());
                self
            }
        }
    };
}

/// Macro to implement WithQuality trait
#[macro_export]
macro_rules! impl_with_quality {
    ($builder:ty, $request_type:ty, $request_field:ident, $quality_type:ty) => {
        impl $crate::models::common_builder::WithQuality<$request_type, $quality_type>
            for $builder
        {
            fn quality(mut self, quality: $quality_type) -> Self {
                self.$request_field.quality = Some(quality);
                self
            }
        }
    };
}

/// Macro to implement WithTemperature trait
#[macro_export]
macro_rules! impl_with_temperature {
    ($builder:ty, $request_type:ty, $request_field:ident) => {
        impl $crate::models::common_builder::WithTemperature<$request_type> for $builder {
            fn temperature(mut self, temperature: f32) -> Self {
                self.$request_field.temperature = Some(temperature.clamp(0.0, 1.0));
                self
            }
        }
    };
}

/// Macro to implement WithSpeed trait
#[macro_export]
macro_rules! impl_with_speed {
    ($builder:ty, $request_type:ty, $request_field:ident, $speed_range:expr) => {
        impl $crate::models::common_builder::WithSpeed<$request_type> for $builder {
            fn speed(mut self, speed: f32) -> Self {
                let (min, max) = $speed_range;
                self.$request_field.speed = Some(speed.clamp(min, max));
                self
            }
        }
    };
}
