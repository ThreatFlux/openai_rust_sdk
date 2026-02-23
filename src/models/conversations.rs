//! # Conversations API Models
//!
//! Data structures for the OpenAI Conversations API, which manages
//! conversations and their items.
//!
//! ## Endpoints
//!
//! - `POST /v1/conversations` - Create a conversation
//! - `GET /v1/conversations/{id}` - Retrieve a conversation
//! - `POST /v1/conversations/{id}` - Update a conversation
//! - `DELETE /v1/conversations/{id}` - Delete a conversation
//! - `POST /v1/conversations/{id}/items` - Create an item
//! - `GET /v1/conversations/{id}/items/{item_id}` - Retrieve an item
//! - `DELETE /v1/conversations/{id}/items/{item_id}` - Delete an item
//! - `GET /v1/conversations/{id}/items` - List items

use crate::{De, Ser};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// A conversation object returned by the Conversations API.
#[derive(Debug, Clone, Ser, De)]
pub struct Conversation {
    /// The unique identifier for the conversation.
    pub id: String,

    /// The object type, always "conversation".
    pub object: String,

    /// Unix timestamp (in seconds) of when the conversation was created.
    pub created_at: u64,

    /// Optional metadata associated with the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Request body for creating a new conversation.
#[derive(Debug, Clone, Ser, De)]
pub struct CreateConversationRequest {
    /// Optional metadata to attach to the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Request body for updating an existing conversation.
#[derive(Debug, Clone, Ser, De)]
pub struct UpdateConversationRequest {
    /// Optional metadata to update on the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Response returned when a conversation is deleted.
#[derive(Debug, Clone, Ser, De)]
pub struct ConversationDeleteResponse {
    /// The ID of the deleted conversation.
    pub id: String,

    /// The object type.
    pub object: String,

    /// Whether the conversation was successfully deleted.
    pub deleted: bool,
}

/// An item within a conversation.
#[derive(Debug, Clone, Ser, De)]
pub struct ConversationItem {
    /// The unique identifier for the item.
    pub id: String,

    /// The object type.
    pub object: String,

    /// The ID of the conversation this item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    /// Unix timestamp (in seconds) of when the item was created.
    pub created_at: u64,

    /// The type of the item (e.g., "message").
    #[serde(rename = "type")]
    pub type_field: String,

    /// The role of the item author (e.g., "user", "assistant").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// The content of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<serde_json::Value>>,

    /// The status of the item (e.g., "completed", "in_progress").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Request body for creating a new item in a conversation.
#[derive(Debug, Clone, Ser, De)]
pub struct CreateConversationItemRequest {
    /// The type of the item (e.g., "message").
    #[serde(rename = "type")]
    pub type_field: String,

    /// The role of the item author (e.g., "user", "assistant").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// The content of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<serde_json::Value>>,
}

/// A paginated list of conversation items.
#[derive(Debug, Clone, Ser, De)]
pub struct ConversationItemList {
    /// The object type, typically "list".
    pub object: String,

    /// The list of conversation items.
    pub data: Vec<ConversationItem>,

    /// The ID of the first item in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,

    /// The ID of the last item in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Whether there are more items available beyond this page.
    pub has_more: bool,
}

/// Query parameters for listing items in a conversation.
#[derive(Debug, Clone, Ser, De)]
pub struct ListConversationItemsParams {
    /// Maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort order by created_at timestamp ("asc" or "desc").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Cursor for forward pagination; returns items after this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Cursor for backward pagination; returns items before this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}
