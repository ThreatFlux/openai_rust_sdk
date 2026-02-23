//! # Conversations API
//!
//! Client for the OpenAI Conversations API, which manages conversations
//! and their items (messages).

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::Result;
use crate::models::conversations::{
    Conversation, ConversationDeleteResponse, ConversationItem, ConversationItemList,
    CreateConversationItemRequest, CreateConversationRequest, ListConversationItemsParams,
    UpdateConversationRequest,
};

/// Conversations API client for managing conversations and items
pub struct ConversationsApi {
    /// Shared HTTP client for making requests
    client: HttpClient,
}

impl ApiClientConstructors for ConversationsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self {
            client: http_client,
        }
    }
}

impl ConversationsApi {
    /// Create a new conversation
    pub async fn create_conversation(
        &self,
        request: &CreateConversationRequest,
    ) -> Result<Conversation> {
        self.client.post("/v1/conversations", request).await
    }

    /// Retrieve a conversation by ID
    pub async fn retrieve_conversation(
        &self,
        conversation_id: impl AsRef<str>,
    ) -> Result<Conversation> {
        let path = format!("/v1/conversations/{}", conversation_id.as_ref());
        self.client.get(&path).await
    }

    /// Update a conversation by ID
    pub async fn update_conversation(
        &self,
        conversation_id: impl AsRef<str>,
        request: &UpdateConversationRequest,
    ) -> Result<Conversation> {
        let path = format!("/v1/conversations/{}", conversation_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Delete a conversation by ID
    pub async fn delete_conversation(
        &self,
        conversation_id: impl AsRef<str>,
    ) -> Result<ConversationDeleteResponse> {
        let path = format!("/v1/conversations/{}", conversation_id.as_ref());
        self.client.delete(&path).await
    }

    /// Create an item in a conversation
    pub async fn create_conversation_item(
        &self,
        conversation_id: impl AsRef<str>,
        request: &CreateConversationItemRequest,
    ) -> Result<ConversationItem> {
        let path = format!("/v1/conversations/{}/items", conversation_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Retrieve a specific item from a conversation
    pub async fn retrieve_conversation_item(
        &self,
        conversation_id: impl AsRef<str>,
        item_id: impl AsRef<str>,
    ) -> Result<ConversationItem> {
        let path = format!(
            "/v1/conversations/{}/items/{}",
            conversation_id.as_ref(),
            item_id.as_ref()
        );
        self.client.get(&path).await
    }

    /// Delete a specific item from a conversation
    pub async fn delete_conversation_item(
        &self,
        conversation_id: impl AsRef<str>,
        item_id: impl AsRef<str>,
    ) -> Result<ConversationDeleteResponse> {
        let path = format!(
            "/v1/conversations/{}/items/{}",
            conversation_id.as_ref(),
            item_id.as_ref()
        );
        self.client.delete(&path).await
    }

    /// List items in a conversation with optional pagination
    pub async fn list_conversation_items(
        &self,
        conversation_id: impl AsRef<str>,
        params: Option<&ListConversationItemsParams>,
    ) -> Result<ConversationItemList> {
        let path = format!("/v1/conversations/{}/items", conversation_id.as_ref());
        match params {
            Some(p) => {
                let mut query = Vec::new();
                if let Some(l) = p.limit {
                    query.push(("limit".to_string(), l.to_string()));
                }
                if let Some(ref o) = p.order {
                    query.push(("order".to_string(), o.clone()));
                }
                if let Some(ref a) = p.after {
                    query.push(("after".to_string(), a.clone()));
                }
                if let Some(ref b) = p.before {
                    query.push(("before".to_string(), b.clone()));
                }
                self.client.get_with_query(&path, &query).await
            }
            None => self.client.get(&path).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_conversations_api_creation() {
        let api = ConversationsApi::new("test-key").unwrap();
        assert_eq!(api.client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_conversations_api_creation_with_base_url() {
        let api =
            ConversationsApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_conversations_api_empty_key_fails() {
        let result = ConversationsApi::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_conversation_request_serialization() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());
        let req = CreateConversationRequest {
            metadata: Some(metadata),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["metadata"]["key"], "value");
    }

    #[test]
    fn test_create_conversation_request_empty() {
        let req = CreateConversationRequest { metadata: None };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("metadata").is_none());
    }

    #[test]
    fn test_update_conversation_request_serialization() {
        let req = UpdateConversationRequest { metadata: None };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json.get("metadata").is_none());
    }

    #[test]
    fn test_conversation_deserialization() {
        let json = r#"{
            "id": "conv-123",
            "object": "conversation",
            "created_at": 1700000000,
            "metadata": {"project": "test"}
        }"#;
        let conv: Conversation = serde_json::from_str(json).unwrap();
        assert_eq!(conv.id, "conv-123");
        assert_eq!(conv.object, "conversation");
        let meta = conv.metadata.unwrap();
        assert_eq!(meta.get("project"), Some(&"test".to_string()));
    }

    #[test]
    fn test_conversation_delete_response() {
        let json = r#"{"id": "conv-123", "object": "conversation", "deleted": true}"#;
        let resp: ConversationDeleteResponse = serde_json::from_str(json).unwrap();
        assert!(resp.deleted);
        assert_eq!(resp.id, "conv-123");
    }

    #[test]
    fn test_conversation_item_deserialization() {
        let json = r#"{
            "id": "item-1",
            "object": "conversation.item",
            "created_at": 1700000000,
            "type": "message",
            "role": "user",
            "content": [{"type": "text", "text": "Hello"}],
            "status": "completed"
        }"#;
        let item: ConversationItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "item-1");
        assert_eq!(item.type_field, "message");
        assert_eq!(item.role, Some("user".to_string()));
        assert_eq!(item.status, Some("completed".to_string()));
    }

    #[test]
    fn test_create_conversation_item_request() {
        let req = CreateConversationItemRequest {
            type_field: "message".to_string(),
            role: Some("user".to_string()),
            content: Some(vec![serde_json::json!({"type": "text", "text": "Hi"})]),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["type"], "message");
        assert_eq!(json["role"], "user");
    }

    #[test]
    fn test_conversation_item_list_deserialization() {
        let json = r#"{"object": "list", "data": [], "has_more": false}"#;
        let list: ConversationItemList = serde_json::from_str(json).unwrap();
        assert!(list.data.is_empty());
        assert!(!list.has_more);
    }

    #[test]
    fn test_list_conversation_items_params() {
        let params = ListConversationItemsParams {
            limit: Some(10),
            order: Some("asc".to_string()),
            after: Some("item-0".to_string()),
            before: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["limit"], 10);
        assert_eq!(json["order"], "asc");
        assert!(json.get("before").is_none());
    }
}
