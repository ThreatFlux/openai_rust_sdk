//! Thread-related models and builders

use crate::api::base::Validate;
use crate::{De, Ser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::builders::MetadataBuilder;
use super::message::MessageRequest;
use super::types::{default_thread_object, SortOrder};
use super::validation::common::validate_metadata;

/// A conversation thread that can contain multiple messages
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct Thread {
    /// The identifier of the thread
    pub id: String,
    /// The object type, which is always "thread"
    #[serde(default = "default_thread_object")]
    pub object: String,
    /// The Unix timestamp (in seconds) when the thread was created
    pub created_at: i64,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Request to create or modify a thread
#[derive(Debug, Clone, Ser, De)]
pub struct ThreadRequest {
    /// A list of messages to start the thread with
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<MessageRequest>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

impl ThreadRequest {
    /// Create a new thread request builder
    #[must_use]
    pub fn builder() -> ThreadRequestBuilder {
        ThreadRequestBuilder::new()
    }

    /// Create a new empty thread request
    #[must_use]
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Validate the thread request
    pub fn validate(&self) -> Result<(), String> {
        // Validate metadata using common function
        validate_metadata(&self.metadata).map_err(|e| format!("Thread {}", e))?;

        // Validate messages
        for message in &self.messages {
            message.validate()?;
        }

        Ok(())
    }
}

impl Validate for ThreadRequest {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

impl Default for ThreadRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating thread requests
#[derive(Debug, Clone, Default)]
pub struct ThreadRequestBuilder {
    /// The messages to include in the thread
    messages: Vec<MessageRequest>,
    /// Set of key-value pairs to attach to this thread
    metadata: HashMap<String, String>,
}

impl MetadataBuilder for ThreadRequestBuilder {
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl ThreadRequestBuilder {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a message to the thread
    #[must_use]
    pub fn message(mut self, message: MessageRequest) -> Self {
        self.messages.push(message);
        self
    }

    /// Add multiple messages to the thread
    #[must_use]
    pub fn messages(mut self, messages: Vec<MessageRequest>) -> Self {
        self.messages.extend(messages);
        self
    }

    /// Add metadata
    pub fn metadata_pair(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.add_metadata_pair(key, value);
        self
    }

    /// Set all metadata
    #[must_use]
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.set_metadata(metadata);
        self
    }

    /// Build the thread request
    #[must_use]
    pub fn build(self) -> ThreadRequest {
        ThreadRequest {
            messages: self.messages,
            metadata: self.metadata,
        }
    }
}

// Generate list response for threads
crate::impl_list_response!(ListThreadsResponse, Thread, "Response from listing threads");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::threads::types::MessageRole;

    #[test]
    fn test_thread_request_builder() {
        let request = ThreadRequest::builder()
            .metadata_pair("purpose", "testing")
            .build();

        assert_eq!(request.metadata.len(), 1);
        assert_eq!(
            request.metadata.get("purpose"),
            Some(&"testing".to_string())
        );
        assert!(request.messages.is_empty());
    }

    #[test]
    fn test_thread_request_validation() {
        // Test valid request
        let request = ThreadRequest::builder()
            .metadata_pair("key", "value")
            .build();
        assert!(request.validate().is_ok());

        // Test metadata count validation
        let mut request = ThreadRequest::new();
        for i in 0..17 {
            request
                .metadata
                .insert(format!("key{}", i), "value".to_string());
        }
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_thread_request_with_messages() {
        let message = MessageRequest::new(MessageRole::User, "Hello");
        let request = ThreadRequest::builder().message(message).build();

        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.messages[0].content, "Hello");
    }
}
