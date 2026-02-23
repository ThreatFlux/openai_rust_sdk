//! # Skills API
//!
//! Client for the OpenAI Skills API, which allows you to create, manage,
//! and version reusable skills.

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::Result;
use crate::models::skills::{
    CreateSkillRequest, CreateSkillVersionRequest, ListSkillVersionsParams, ListSkillsParams,
    Skill, SkillContent, SkillDeleteResponse, SkillList, SkillVersion, SkillVersionContent,
    SkillVersionList, UpdateSkillRequest,
};

/// Skills API client for managing reusable skills and their versions
pub struct SkillsApi {
    /// Shared HTTP client for making requests
    client: HttpClient,
}

impl ApiClientConstructors for SkillsApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self {
            client: http_client,
        }
    }
}

/// Build query parameters from optional list fields
fn build_list_params(
    limit: Option<u32>,
    order: Option<&str>,
    after: Option<&str>,
    before: Option<&str>,
) -> Vec<(String, String)> {
    let mut params = Vec::new();
    if let Some(l) = limit {
        params.push(("limit".to_string(), l.to_string()));
    }
    if let Some(o) = order {
        params.push(("order".to_string(), o.to_string()));
    }
    if let Some(a) = after {
        params.push(("after".to_string(), a.to_string()));
    }
    if let Some(b) = before {
        params.push(("before".to_string(), b.to_string()));
    }
    params
}

impl SkillsApi {
    /// Create a new skill
    pub async fn create_skill(&self, request: &CreateSkillRequest) -> Result<Skill> {
        self.client.post("/v1/skills", request).await
    }

    /// Retrieve a skill by ID
    pub async fn retrieve_skill(&self, skill_id: impl AsRef<str>) -> Result<Skill> {
        let path = format!("/v1/skills/{}", skill_id.as_ref());
        self.client.get(&path).await
    }

    /// Retrieve the content of a skill
    pub async fn retrieve_skill_content(&self, skill_id: impl AsRef<str>) -> Result<SkillContent> {
        let path = format!("/v1/skills/{}/content", skill_id.as_ref());
        self.client.get(&path).await
    }

    /// Update a skill by ID
    pub async fn update_skill(
        &self,
        skill_id: impl AsRef<str>,
        request: &UpdateSkillRequest,
    ) -> Result<Skill> {
        let path = format!("/v1/skills/{}", skill_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Delete a skill by ID
    pub async fn delete_skill(&self, skill_id: impl AsRef<str>) -> Result<SkillDeleteResponse> {
        let path = format!("/v1/skills/{}", skill_id.as_ref());
        self.client.delete(&path).await
    }

    /// List skills with optional pagination parameters
    pub async fn list_skills(&self, params: Option<&ListSkillsParams>) -> Result<SkillList> {
        match params {
            Some(p) => {
                let query = build_list_params(
                    p.limit,
                    p.order.as_deref(),
                    p.after.as_deref(),
                    p.before.as_deref(),
                );
                self.client.get_with_query("/v1/skills", &query).await
            }
            None => self.client.get("/v1/skills").await,
        }
    }

    /// Create a new version for a skill
    pub async fn create_skill_version(
        &self,
        skill_id: impl AsRef<str>,
        request: &CreateSkillVersionRequest,
    ) -> Result<SkillVersion> {
        let path = format!("/v1/skills/{}/versions", skill_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Retrieve a specific version of a skill
    pub async fn retrieve_skill_version(
        &self,
        skill_id: impl AsRef<str>,
        version_id: impl AsRef<str>,
    ) -> Result<SkillVersion> {
        let path = format!(
            "/v1/skills/{}/versions/{}",
            skill_id.as_ref(),
            version_id.as_ref()
        );
        self.client.get(&path).await
    }

    /// Retrieve the content of a specific skill version
    pub async fn retrieve_skill_version_content(
        &self,
        skill_id: impl AsRef<str>,
        version_id: impl AsRef<str>,
    ) -> Result<SkillVersionContent> {
        let path = format!(
            "/v1/skills/{}/versions/{}/content",
            skill_id.as_ref(),
            version_id.as_ref()
        );
        self.client.get(&path).await
    }

    /// Delete a specific version of a skill
    pub async fn delete_skill_version(
        &self,
        skill_id: impl AsRef<str>,
        version_id: impl AsRef<str>,
    ) -> Result<SkillDeleteResponse> {
        let path = format!(
            "/v1/skills/{}/versions/{}",
            skill_id.as_ref(),
            version_id.as_ref()
        );
        self.client.delete(&path).await
    }

    /// List versions for a skill with optional pagination parameters
    pub async fn list_skill_versions(
        &self,
        skill_id: impl AsRef<str>,
        params: Option<&ListSkillVersionsParams>,
    ) -> Result<SkillVersionList> {
        let path = format!("/v1/skills/{}/versions", skill_id.as_ref());
        match params {
            Some(p) => {
                let query = build_list_params(
                    p.limit,
                    p.order.as_deref(),
                    p.after.as_deref(),
                    p.before.as_deref(),
                );
                self.client.get_with_query(&path, &query).await
            }
            None => self.client.get(&path).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_skills_api_creation() {
        let api = SkillsApi::new("test-key").unwrap();
        assert_eq!(api.client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_skills_api_creation_with_base_url() {
        let api = SkillsApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_skills_api_empty_key_fails() {
        let result = SkillsApi::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_list_params_all() {
        let params = build_list_params(Some(20), Some("asc"), Some("a1"), Some("b2"));
        assert_eq!(params.len(), 4);
        assert_eq!(params[0].1, "20");
        assert_eq!(params[1].1, "asc");
        assert_eq!(params[2].1, "a1");
        assert_eq!(params[3].1, "b2");
    }

    #[test]
    fn test_build_list_params_empty() {
        let params = build_list_params(None, None, None, None);
        assert!(params.is_empty());
    }

    #[test]
    fn test_create_skill_request_serialization() {
        let req = CreateSkillRequest {
            name: "my-skill".to_string(),
            description: Some("A test skill".to_string()),
            metadata: None,
            content: Some(json!({"type": "prompt", "text": "Hello"})),
        };
        let json_val = serde_json::to_value(&req).unwrap();
        assert_eq!(json_val["name"], "my-skill");
        assert_eq!(json_val["description"], "A test skill");
        assert!(json_val.get("metadata").is_none());
        assert!(json_val["content"].is_object());
    }

    #[test]
    fn test_update_skill_request_skip_none() {
        let req = UpdateSkillRequest {
            name: Some("updated".to_string()),
            description: None,
            metadata: None,
            active_version_id: None,
        };
        let json_val = serde_json::to_value(&req).unwrap();
        assert_eq!(json_val["name"], "updated");
        assert!(json_val.get("description").is_none());
        assert!(json_val.get("active_version_id").is_none());
    }

    #[test]
    fn test_skill_deserialization() {
        let json = r#"{
            "id": "skill-123",
            "object": "skill",
            "created_at": 1700000000,
            "name": "my-skill"
        }"#;
        let skill: Skill = serde_json::from_str(json).unwrap();
        assert_eq!(skill.id, "skill-123");
        assert_eq!(skill.name, "my-skill");
        assert!(skill.description.is_none());
        assert!(skill.latest_version_id.is_none());
    }

    #[test]
    fn test_skill_delete_response() {
        let json = r#"{"id": "skill-123", "object": "skill.deleted", "deleted": true}"#;
        let resp: SkillDeleteResponse = serde_json::from_str(json).unwrap();
        assert!(resp.deleted);
        assert_eq!(resp.id, "skill-123");
    }

    #[test]
    fn test_skill_content_deserialization() {
        let json = r#"{"object": "skill.content", "content": {"type": "prompt"}}"#;
        let content: SkillContent = serde_json::from_str(json).unwrap();
        assert_eq!(content.object, "skill.content");
        assert!(content.content.is_object());
    }

    #[test]
    fn test_skill_version_deserialization() {
        let json = r#"{
            "id": "ver-1",
            "object": "skill.version",
            "skill_id": "skill-123",
            "created_at": 1700000000
        }"#;
        let ver: SkillVersion = serde_json::from_str(json).unwrap();
        assert_eq!(ver.id, "ver-1");
        assert_eq!(ver.skill_id, "skill-123");
    }

    #[test]
    fn test_create_skill_version_request() {
        let req = CreateSkillVersionRequest {
            description: Some("v2".to_string()),
            metadata: None,
            content: json!({"text": "updated content"}),
        };
        let json_val = serde_json::to_value(&req).unwrap();
        assert_eq!(json_val["description"], "v2");
        assert!(json_val["content"].is_object());
    }

    #[test]
    fn test_skill_list_deserialization() {
        let json = r#"{"object": "list", "data": [], "has_more": false}"#;
        let list: SkillList = serde_json::from_str(json).unwrap();
        assert!(list.data.is_empty());
        assert!(!list.has_more);
    }

    #[test]
    fn test_skill_version_list_deserialization() {
        let json = r#"{"object": "list", "data": [], "has_more": true, "first_id": "v1", "last_id": "v5"}"#;
        let list: SkillVersionList = serde_json::from_str(json).unwrap();
        assert!(list.has_more);
        assert_eq!(list.first_id, Some("v1".to_string()));
        assert_eq!(list.last_id, Some("v5".to_string()));
    }
}
