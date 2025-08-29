//! Core types and enums for the OpenAI Threads API

use crate::{De, Ser};
use serde::{Deserialize, Serialize};

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

/// Message file object representing a file attached to a message
#[derive(Debug, Clone, PartialEq, Ser, De)]
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

// Default object functions
fn default_thread_deletion_object() -> String {
    "thread.deleted".to_string()
}

/// Default object type for message files
fn default_message_file_object() -> String {
    "thread.message.file".to_string()
}

/// Default object type for list responses
pub fn default_list_object() -> String {
    "list".to_string()
}

/// Default object type for thread objects  
pub fn default_thread_object() -> String {
    "thread".to_string()
}

/// Default object type for message objects
pub fn default_message_object() -> String {
    "thread.message".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_sort_order_default() {
        let order = SortOrder::default();
        assert_eq!(order, SortOrder::Desc);
    }

    #[test]
    fn test_default_objects() {
        assert_eq!(default_thread_object(), "thread");
        assert_eq!(default_message_object(), "thread.message");
        assert_eq!(default_list_object(), "list");
        assert_eq!(default_thread_deletion_object(), "thread.deleted");
        assert_eq!(default_message_file_object(), "thread.message.file");
    }
}
