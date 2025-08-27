//! Message-related models and builders

use crate::api::base::Validate;
use crate::{De, Ser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::builders::MetadataBuilder;
use super::content::MessageContent;
use super::types::{default_message_object, MessageFile, MessageRole, SortOrder};
use super::validation::common::{
    validate_content_length, validate_file_ids_count, validate_metadata,
};

/// A message within a thread
#[derive(Debug, Clone, PartialEq, Ser, De)]
pub struct Message {
    /// The identifier of the message
    pub id: String,
    /// The object type, which is always "thread.message"
    #[serde(default = "default_message_object")]
    pub object: String,
    /// The Unix timestamp (in seconds) when the message was created
    pub created_at: i64,
    /// The thread ID that this message belongs to
    pub thread_id: String,
    /// The entity that produced the message
    pub role: MessageRole,
    /// The content of the message
    pub content: Vec<MessageContent>,
    /// If applicable, the ID of the assistant that authored this message
    pub assistant_id: Option<String>,
    /// If applicable, the ID of the run associated with the authoring of this message
    pub run_id: Option<String>,
    /// A list of file IDs that the assistant should use
    #[serde(default)]
    pub file_ids: Vec<String>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Request to create or modify a message
#[derive(Debug, Clone, Ser, De)]
pub struct MessageRequest {
    /// The role of the entity that is creating the message
    pub role: MessageRole,
    /// The content of the message
    pub content: String,
    /// A list of file IDs that the message should use (max 10 files)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_ids: Vec<String>,
    /// Set of 16 key-value pairs that can be attached to an object
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

impl MessageRequest {
    /// Create a new message request builder
    #[must_use]
    pub fn builder() -> MessageRequestBuilder {
        MessageRequestBuilder::new()
    }

    /// Create a new message request
    pub fn new(role: MessageRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            file_ids: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Validate the message request
    pub fn validate(&self) -> Result<(), String> {
        // Validate content length (max 32,768 characters)
        validate_content_length(&self.content)?;

        // Validate file IDs count
        validate_file_ids_count(&self.file_ids)?;

        // Validate metadata using common function
        validate_metadata(&self.metadata).map_err(|e| format!("Message {}", e))?;

        Ok(())
    }
}

impl Validate for MessageRequest {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

/// Builder for creating message requests
#[derive(Debug, Clone)]
pub struct MessageRequestBuilder {
    /// The role of the entity that is creating the message
    role: Option<MessageRole>,
    /// The content of the message
    content: Option<String>,
    /// A list of file IDs to attach to this message
    file_ids: Vec<String>,
    /// Set of key-value pairs to attach to this message
    metadata: HashMap<String, String>,
}

impl MetadataBuilder for MessageRequestBuilder {
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl MessageRequestBuilder {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            role: None,
            content: None,
            file_ids: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set the role
    #[must_use]
    pub fn role(mut self, role: MessageRole) -> Self {
        self.role = Some(role);
        self
    }

    /// Set the content
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Add a file ID
    pub fn file_id(mut self, file_id: impl Into<String>) -> Self {
        self.file_ids.push(file_id.into());
        self
    }

    /// Add multiple file IDs
    #[must_use]
    pub fn file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids.extend(file_ids);
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

    /// Build the message request
    pub fn build(self) -> Result<MessageRequest, String> {
        let role = self.role.ok_or_else(|| "Role is required".to_string())?;
        let content = self
            .content
            .ok_or_else(|| "Content is required".to_string())?;

        let request = MessageRequest {
            role,
            content,
            file_ids: self.file_ids,
            metadata: self.metadata,
        };

        request.validate()?;
        Ok(request)
    }
}

impl Default for MessageRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameters for listing messages
#[derive(Debug, Clone, Default, Ser, De)]
pub struct ListMessagesParams {
    /// Number of messages to retrieve (1-100, default: 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order for the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SortOrder>,
    /// Cursor for pagination (message ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Cursor for reverse pagination (message ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl ListMessagesParams {
    /// Create new list parameters
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit.clamp(1, 100));
        self
    }

    /// Set the sort order
    #[must_use]
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }

    /// Set the after cursor for pagination
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the before cursor for reverse pagination
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Build query parameters for the API request
    #[must_use]
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(order) = &self.order {
            let order_str = match order {
                SortOrder::Asc => "asc",
                SortOrder::Desc => "desc",
            };
            params.push(("order".to_string(), order_str.to_string()));
        }
        if let Some(after) = &self.after {
            params.push(("after".to_string(), after.clone()));
        }
        if let Some(before) = &self.before {
            params.push(("before".to_string(), before.clone()));
        }
        params
    }
}

crate::impl_list_query_params!(ListMessagesParams, SortOrder);

// Generate list responses
crate::impl_list_response!(
    ListMessagesResponse,
    Message,
    "Response from listing messages"
);

crate::impl_list_response!(
    ListMessageFilesResponse,
    MessageFile,
    "Response from listing message files"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_request_builder() {
        let request = MessageRequest::builder()
            .role(MessageRole::User)
            .content("Hello, world!")
            .file_id("file-123")
            .metadata_pair("priority", "high")
            .build()
            .unwrap();

        assert_eq!(request.role, MessageRole::User);
        assert_eq!(request.content, "Hello, world!");
        assert_eq!(request.file_ids.len(), 1);
        assert_eq!(request.metadata.len(), 1);
    }

    #[test]
    fn test_message_request_validation() {
        // Test content length validation
        let long_content = "a".repeat(32769);
        let request = MessageRequest::builder()
            .role(MessageRole::User)
            .content(long_content)
            .build();
        assert!(request.is_err());

        // Test valid request
        let request = MessageRequest::builder()
            .role(MessageRole::User)
            .content("Valid content")
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_message_request_file_ids_validation() {
        let mut builder = MessageRequest::builder()
            .role(MessageRole::User)
            .content("Test content");

        for i in 0..11 {
            builder = builder.file_id(format!("file-{}", i));
        }

        let request = builder.build();
        assert!(request.is_err());
    }

    #[test]
    fn test_list_params_limit_clamping() {
        let params = ListMessagesParams::new().limit(150);
        assert_eq!(params.limit, Some(100));

        let params = ListMessagesParams::new().limit(0);
        assert_eq!(params.limit, Some(1));
    }

    #[test]
    fn test_message_new() {
        let message = MessageRequest::new(MessageRole::User, "Hello");
        assert_eq!(message.role, MessageRole::User);
        assert_eq!(message.content, "Hello");
        assert!(message.file_ids.is_empty());
        assert!(message.metadata.is_empty());
    }
}
