//! # OpenAI Threads & Messages API Models
//!
//! This module provides data structures for OpenAI's Threads API, which allows you to
//! create conversation threads and manage messages within those threads.
//!
//! ## Overview
//!
//! The Threads API supports:
//! - **Thread Management**: Create, retrieve, modify, and delete conversation threads
//! - **Message Management**: Add, retrieve, and modify messages within threads
//! - **File Attachments**: Attach files to messages for analysis and processing
//! - **Content Types**: Support for text and image content in messages
//! - **Annotations**: File citations and file paths within message content
//! - **Metadata**: Store custom metadata for threads and messages
//!
//! ## Message Roles
//!
//! Messages can have different roles:
//! - `user`: Messages from the user
//! - `assistant`: Messages from the AI assistant
//!
//! ## Content Types
//!
//! Messages can contain different types of content:
//! - **Text**: Plain text with optional annotations (file citations, file paths)
//! - **Image**: Image files for visual analysis
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::threads::{ThreadRequest, MessageRequest, MessageRole};
//! use std::collections::HashMap;
//!
//! // Create a thread request
//! let thread_request = ThreadRequest::builder()
//!     .metadata_pair("purpose", "customer_support")
//!     .build();
//!
//! // Create a message request
//! let message_request = MessageRequest::builder()
//!     .role(MessageRole::User)
//!     .content("Hello, I need help with my account.")
//!     .build()
//!     .unwrap();
//! ```

use crate::api::base::Validate;
use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// Macro to generate default object functions
macro_rules! default_object_fn {
    ($fn_name:ident, $object_type:literal) => {
        fn $fn_name() -> String {
            $object_type.to_string()
        }
    };
}

/// Macro to generate list response structures
macro_rules! impl_list_response {
    ($struct_name:ident, $item_type:ty, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Ser, De)]
        pub struct $struct_name {
            /// The object type, which is always "list"
            #[serde(default = "default_list_object")]
            pub object: String,
            /// List of items
            pub data: Vec<$item_type>,
            /// ID of the first item in the list
            pub first_id: Option<String>,
            /// ID of the last item in the list
            pub last_id: Option<String>,
            /// Whether there are more items available
            pub has_more: bool,
        }
    };
}

/// Macro to generate metadata builder methods
macro_rules! impl_metadata_methods {
    () => {
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
    };
}

/// Macro to generate builder setter methods for optional string fields
macro_rules! impl_string_option_setters {
    ($(($method:ident, $field:ident, $doc:literal)),+ $(,)?) => {
        $(
            #[doc = $doc]
            pub fn $method(mut self, $method: impl Into<String>) -> Self {
                self.$field = Some($method.into());
                self
            }
        )+
    };
}

/// Macro to implement ListQueryParams trait for pagination structures
macro_rules! impl_list_query_params {
    ($struct_name:ident, $order_type:ty) => {
        impl crate::api::common::ListQueryParams for $struct_name {
            fn limit(&self) -> Option<u32> {
                self.limit
            }

            fn order_str(&self) -> Option<&str> {
                self.order.as_ref().map(|o| match o {
                    SortOrder::Asc => "asc",
                    SortOrder::Desc => "desc",
                })
            }

            fn after(&self) -> Option<&String> {
                self.after.as_ref()
            }

            fn before(&self) -> Option<&String> {
                self.before.as_ref()
            }
        }
    };
}

/// Macro to generate validation implementations using common validate_metadata
macro_rules! impl_validation {
    ($struct_name:ident, metadata) => {
        impl Validate for $struct_name {
            fn validate(&self) -> Result<(), String> {
                self.validate()
            }
        }
    };
}

/// Common validation functions for threads and messages
mod validation {
    use std::collections::HashMap;

    /// Validate metadata constraints
    pub fn validate_metadata(metadata: &HashMap<String, String>) -> Result<(), String> {
        // Validate metadata count
        if metadata.len() > 16 {
            return Err("Cannot have more than 16 metadata pairs".to_string());
        }

        // Validate metadata key/value lengths
        for (key, value) in metadata {
            if key.len() > 64 {
                return Err("Metadata key cannot exceed 64 characters".to_string());
            }
            if value.len() > 512 {
                return Err("Metadata value cannot exceed 512 characters".to_string());
            }
        }

        Ok(())
    }
}

use validation::validate_metadata;

/// Common trait for builders that support metadata
trait MetadataBuilder {
    /// Get mutable reference to the metadata HashMap
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String>;

    /// Add a metadata key-value pair
    fn add_metadata_pair(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.get_metadata_mut().insert(key.into(), value.into());
    }

    /// Set all metadata
    fn set_metadata(&mut self, metadata: HashMap<String, String>) {
        *self.get_metadata_mut() = metadata;
    }
}

/// A conversation thread that can contain multiple messages
#[derive(Debug, Clone, Ser, De)]
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

default_object_fn!(default_thread_object, "thread");

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

impl_validation!(ThreadRequest, metadata);

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

    impl_metadata_methods!();

    /// Build the thread request
    #[must_use]
    pub fn build(self) -> ThreadRequest {
        ThreadRequest {
            messages: self.messages,
            metadata: self.metadata,
        }
    }
}

/// The role of the message author
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// Message from a user
    User,
    /// Message from an assistant
    Assistant,
}

impl MessageRole {
    /// Check if the role is user
    #[must_use]
    pub fn is_user(&self) -> bool {
        matches!(self, Self::User)
    }

    /// Check if the role is assistant
    #[must_use]
    pub fn is_assistant(&self) -> bool {
        matches!(self, Self::Assistant)
    }
}

/// Content within a message
#[derive(Debug, Clone, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContent {
    /// Text content with optional annotations
    Text {
        /// The text content
        text: TextContent,
    },
    /// Image file content
    ImageFile {
        /// The image file details
        image_file: ImageFile,
    },
}

impl MessageContent {
    /// Create text content
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text {
            text: TextContent {
                value: value.into(),
                annotations: Vec::new(),
            },
        }
    }

    /// Create text content with annotations
    pub fn text_with_annotations(value: impl Into<String>, annotations: Vec<Annotation>) -> Self {
        Self::Text {
            text: TextContent {
                value: value.into(),
                annotations,
            },
        }
    }

    /// Create image file content
    pub fn image_file(file_id: impl Into<String>) -> Self {
        Self::ImageFile {
            image_file: ImageFile {
                file_id: file_id.into(),
            },
        }
    }
}

/// Text content with annotations
#[derive(Debug, Clone, Ser, De)]
pub struct TextContent {
    /// The actual text content
    pub value: String,
    /// Annotations for the text content
    #[serde(default)]
    pub annotations: Vec<Annotation>,
}

/// Image file content
#[derive(Debug, Clone, Ser, De)]
pub struct ImageFile {
    /// The file ID of the image
    pub file_id: String,
}

/// Annotations that can be applied to text content
#[derive(Debug, Clone, Ser, De)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
    /// A citation of a specific quote from a file
    FileCitation {
        /// The text that was annotated
        text: String,
        /// The start index of the annotation
        start_index: u32,
        /// The end index of the annotation
        end_index: u32,
        /// The file citation details
        file_citation: FileCitation,
    },
    /// A file path annotation
    FilePath {
        /// The text that was annotated
        text: String,
        /// The start index of the annotation
        start_index: u32,
        /// The end index of the annotation
        end_index: u32,
        /// The file path details
        file_path: FilePathInfo,
    },
}

/// A citation of a specific quote from a file
#[derive(Debug, Clone, Ser, De)]
pub struct FileCitation {
    /// The ID of the file that was cited
    pub file_id: String,
    /// The specific quote from the file
    pub quote: Option<String>,
}

/// File path information
#[derive(Debug, Clone, Ser, De)]
pub struct FilePathInfo {
    /// The ID of the file
    pub file_id: String,
}

/// A message within a thread
#[derive(Debug, Clone, Ser, De)]
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

default_object_fn!(default_message_object, "thread.message");

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
        if self.content.len() > 32768 {
            return Err("Message content cannot exceed 32,768 characters".to_string());
        }

        // Validate file IDs count
        if self.file_ids.len() > 10 {
            return Err("Message cannot have more than 10 file IDs".to_string());
        }

        // Validate metadata using common function
        validate_metadata(&self.metadata).map_err(|e| format!("Message {}", e))?;

        Ok(())
    }
}

impl_validation!(MessageRequest, metadata);

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

    impl_metadata_methods!();
}

// Generate the build method for MessageRequestBuilder
crate::impl_builder_build! {
    MessageRequestBuilder => MessageRequest {
        required: [role: "Role is required", content: "Content is required"],
        optional: [file_ids, metadata],
        validate: true
    }
}

impl Default for MessageRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl_list_response!(ListThreadsResponse, Thread, "Response from listing threads");

impl_list_response!(
    ListMessagesResponse,
    Message,
    "Response from listing messages"
);

default_object_fn!(default_list_object, "list");

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

    impl_string_option_setters!(
        (after, after, "Set the after cursor for pagination"),
        (
            before,
            before,
            "Set the before cursor for reverse pagination"
        )
    );

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

impl_list_query_params!(ListMessagesParams, SortOrder);

/// Sort order for listing results
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum SortOrder {
    /// Ascending order (oldest first)
    Asc,
    /// Descending order (newest first)
    #[default]
    Desc,
}

/// Response from deleting a thread
#[derive(Debug, Clone, Ser, De)]
pub struct DeletionStatus {
    /// The ID of the deleted object
    pub id: String,
    /// The object type, which is always "thread.deleted"
    #[serde(default = "default_thread_deletion_object")]
    pub object: String,
    /// Whether the deletion was successful
    pub deleted: bool,
}

default_object_fn!(default_thread_deletion_object, "thread.deleted");

/// Message file object representing a file attached to a message
#[derive(Debug, Clone, Ser, De)]
pub struct MessageFile {
    /// The identifier of the message file
    pub id: String,
    /// The object type, which is always "thread.message.file"
    #[serde(default = "default_message_file_object")]
    pub object: String,
    /// The Unix timestamp (in seconds) when the message file was created
    pub created_at: i64,
    /// The ID of the message that the file is attached to
    pub message_id: String,
}

default_object_fn!(default_message_file_object, "thread.message.file");

impl_list_response!(
    ListMessageFilesResponse,
    MessageFile,
    "Response from listing message files"
);

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_message_role() {
        let user_role = MessageRole::User;
        assert!(user_role.is_user());
        assert!(!user_role.is_assistant());

        let assistant_role = MessageRole::Assistant;
        assert!(assistant_role.is_assistant());
        assert!(!assistant_role.is_user());
    }

    #[test]
    fn test_message_content() {
        let text_content = MessageContent::text("Hello, world!");
        match text_content {
            MessageContent::Text { text } => {
                assert_eq!(text.value, "Hello, world!");
                assert!(text.annotations.is_empty());
            }
            _ => panic!("Expected text content"),
        }

        let image_content = MessageContent::image_file("file-123");
        match image_content {
            MessageContent::ImageFile { image_file } => {
                assert_eq!(image_file.file_id, "file-123");
            }
            _ => panic!("Expected image file content"),
        }
    }

    #[test]
    fn test_list_params_limit_clamping() {
        let params = ListMessagesParams::new().limit(150);
        assert_eq!(params.limit, Some(100));

        let params = ListMessagesParams::new().limit(0);
        assert_eq!(params.limit, Some(1));
    }

    #[test]
    fn test_sort_order_default() {
        let order = SortOrder::default();
        assert_eq!(order, SortOrder::Desc);
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
    fn test_annotation_types() {
        let file_citation = Annotation::FileCitation {
            text: "cited text".to_string(),
            start_index: 0,
            end_index: 10,
            file_citation: FileCitation {
                file_id: "file-123".to_string(),
                quote: Some("original quote".to_string()),
            },
        };

        match file_citation {
            Annotation::FileCitation {
                text,
                file_citation,
                ..
            } => {
                assert_eq!(text, "cited text");
                assert_eq!(file_citation.file_id, "file-123");
                assert_eq!(file_citation.quote, Some("original quote".to_string()));
            }
            _ => panic!("Expected file citation annotation"),
        }
    }
}
