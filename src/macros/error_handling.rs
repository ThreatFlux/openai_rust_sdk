//! Error handling macros for reducing error mapping code duplication

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
