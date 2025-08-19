//! Constants used throughout the OpenAI API SDK
//!
//! This module centralizes all constant values used across the SDK to reduce duplication
//! and make maintenance easier.

/// Default `OpenAI` API base URL
pub const OPENAI_API_BASE_URL: &str = "https://api.openai.com";

/// API version path prefix
pub const API_VERSION_PREFIX: &str = "/v1";

/// Content type for JSON requests
pub const CONTENT_TYPE_JSON: &str = "application/json";

/// `OpenAI` Beta header value for assistants API
pub const OPENAI_BETA_ASSISTANTS: &str = "assistants=v1";

/// Common HTTP header names
pub mod headers {
    /// Authorization header name
    pub const AUTHORIZATION: &str = "Authorization";

    /// Content-Type header name
    pub const CONTENT_TYPE: &str = "Content-Type";

    /// OpenAI-Beta header name
    pub const OPENAI_BETA: &str = "OpenAI-Beta";

    /// User-Agent header name
    pub const USER_AGENT: &str = "User-Agent";
}

/// Common API endpoints
pub mod endpoints {
    /// Assistants API endpoints
    pub mod assistants {
        /// Base assistants endpoint
        pub const BASE: &str = "/v1/assistants";

        /// Get specific assistant by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }
    }

    /// Vector stores API endpoints
    pub mod vector_stores {
        /// Base vector stores endpoint
        pub const BASE: &str = "/v1/vector_stores";

        /// Get specific vector store by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Vector store files endpoint
        #[must_use]
        pub fn files(vector_store_id: &str) -> String {
            format!("{BASE}/{vector_store_id}/files")
        }

        /// Vector store file batches endpoint
        #[must_use]
        pub fn file_batches(vector_store_id: &str) -> String {
            format!("{BASE}/{vector_store_id}/file_batches")
        }
    }

    /// Threads API endpoints
    pub mod threads {
        /// Base threads endpoint
        pub const BASE: &str = "/v1/threads";

        /// Get specific thread by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Thread messages endpoint
        #[must_use]
        pub fn messages(thread_id: &str) -> String {
            format!("{BASE}/{thread_id}/messages")
        }

        /// Specific message in thread
        #[must_use]
        pub fn message_by_id(thread_id: &str, message_id: &str) -> String {
            format!("{BASE}/{thread_id}/messages/{message_id}")
        }
    }

    /// Files API endpoints
    pub mod files {
        /// Base files endpoint
        pub const BASE: &str = "/v1/files";

        /// Get specific file by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Download file content
        #[must_use]
        pub fn content(id: &str) -> String {
            format!("{BASE}/{id}/content")
        }
    }

    /// Models API endpoints
    pub mod models {
        /// Base models endpoint
        pub const BASE: &str = "/v1/models";

        /// Get specific model by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }
    }

    /// Chat completions endpoint
    pub const CHAT_COMPLETIONS: &str = "/v1/chat/completions";

    /// Completions endpoint
    pub const COMPLETIONS: &str = "/v1/completions";

    /// Embeddings endpoint
    pub const EMBEDDINGS: &str = "/v1/embeddings";

    /// Fine-tuning jobs endpoint
    pub const FINE_TUNING_JOBS: &str = "/v1/fine_tuning/jobs";

    /// Images generations endpoint
    pub const IMAGES_GENERATIONS: &str = "/v1/images/generations";

    /// Images edits endpoint
    pub const IMAGES_EDITS: &str = "/v1/images/edits";

    /// Images variations endpoint
    pub const IMAGES_VARIATIONS: &str = "/v1/images/variations";

    /// Audio speech endpoint
    pub const AUDIO_SPEECH: &str = "/v1/audio/speech";

    /// Audio transcriptions endpoint
    pub const AUDIO_TRANSCRIPTIONS: &str = "/v1/audio/transcriptions";

    /// Audio translations endpoint
    pub const AUDIO_TRANSLATIONS: &str = "/v1/audio/translations";

    /// Moderations endpoint
    pub const MODERATIONS: &str = "/v1/moderations";

    /// Batch jobs endpoint
    pub const BATCHES: &str = "/v1/batches";
}

/// Common error messages
pub mod error_messages {
    /// Invalid API key format
    pub const INVALID_API_KEY: &str = "Invalid API key format";

    /// Request failed
    pub const REQUEST_FAILED: &str = "Request failed";

    /// Parse error
    pub const PARSE_ERROR: &str = "Failed to parse response";

    /// Timeout error
    pub const TIMEOUT: &str = "Request timed out";

    /// Authentication failed
    pub const AUTH_FAILED: &str = "Authentication failed";
}

/// Common query parameter names
pub mod query_params {
    /// Limit parameter for pagination
    pub const LIMIT: &str = "limit";

    /// Order parameter for sorting
    pub const ORDER: &str = "order";

    /// After parameter for cursor pagination
    pub const AFTER: &str = "after";

    /// Before parameter for cursor pagination
    pub const BEFORE: &str = "before";

    /// Filter parameter
    pub const FILTER: &str = "filter";
}

/// SDK information
pub mod sdk {
    /// SDK name
    pub const NAME: &str = "openai_rust_sdk";

    /// SDK version
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    /// User agent string for HTTP requests
    pub const USER_AGENT: &str =
        const_format::formatcp!("openai_rust_sdk/{}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_are_valid() {
        assert_eq!(OPENAI_API_BASE_URL, "https://api.openai.com");
        assert_eq!(API_VERSION_PREFIX, "/v1");
        assert_eq!(CONTENT_TYPE_JSON, "application/json");
        assert_eq!(OPENAI_BETA_ASSISTANTS, "assistants=v1");
    }

    #[test]
    fn test_endpoint_construction() {
        assert_eq!(endpoints::assistants::BASE, "/v1/assistants");
        assert_eq!(endpoints::assistants::by_id("test"), "/v1/assistants/test");

        assert_eq!(endpoints::vector_stores::BASE, "/v1/vector_stores");
        assert_eq!(
            endpoints::vector_stores::by_id("vs-123"),
            "/v1/vector_stores/vs-123"
        );
        assert_eq!(
            endpoints::vector_stores::files("vs-123"),
            "/v1/vector_stores/vs-123/files"
        );

        assert_eq!(endpoints::threads::BASE, "/v1/threads");
        assert_eq!(
            endpoints::threads::by_id("thread-123"),
            "/v1/threads/thread-123"
        );
        assert_eq!(
            endpoints::threads::messages("thread-123"),
            "/v1/threads/thread-123/messages"
        );
    }

    #[test]
    fn test_sdk_constants() {
        assert_eq!(sdk::NAME, "openai_rust_sdk");
        // VERSION is a compile-time constant from CARGO_PKG_VERSION, verify it follows semver pattern
        assert!(sdk::VERSION.chars().any(|c| c.is_ascii_digit()));
        assert!(sdk::USER_AGENT.starts_with("openai_rust_sdk/"));
    }
}
