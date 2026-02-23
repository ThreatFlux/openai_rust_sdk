//! # Skills API Models
//!
//! Data structures for the OpenAI Skills API, which allows you to create,
//! manage, and version reusable skills.
//!
//! ## Endpoints
//!
//! - `POST   /v1/skills`                                  - Create a skill
//! - `GET    /v1/skills/{id}`                              - Retrieve a skill
//! - `GET    /v1/skills/{id}/content`                      - Retrieve skill content
//! - `POST   /v1/skills/{id}`                              - Update a skill
//! - `DELETE /v1/skills/{id}`                               - Delete a skill
//! - `GET    /v1/skills`                                   - List skills
//! - `POST   /v1/skills/{id}/versions`                     - Create a version
//! - `GET    /v1/skills/{id}/versions/{version_id}`        - Retrieve a version
//! - `GET    /v1/skills/{id}/versions/{version_id}/content`- Retrieve version content
//! - `DELETE /v1/skills/{id}/versions/{version_id}`        - Delete a version
//! - `GET    /v1/skills/{id}/versions`                     - List versions

use crate::{De, Ser};
use std::collections::HashMap;

/// A skill object.
#[derive(Debug, Clone, Ser, De)]
pub struct Skill {
    /// Unique identifier for the skill.
    pub id: String,

    /// Object type, always `"skill"`.
    pub object: String,

    /// Unix timestamp (in seconds) of when the skill was created.
    pub created_at: u64,

    /// The name of the skill.
    pub name: String,

    /// An optional description of the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional metadata associated with the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// The ID of the latest version of this skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_id: Option<String>,

    /// The ID of the currently active version of this skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_version_id: Option<String>,
}

/// Request body for creating a new skill.
#[derive(Debug, Clone, Ser, De)]
pub struct CreateSkillRequest {
    /// The name of the skill.
    pub name: String,

    /// An optional description of the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional metadata to attach to the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Optional content for the initial version of the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}

/// Request body for updating an existing skill.
#[derive(Debug, Clone, Ser, De)]
pub struct UpdateSkillRequest {
    /// An optional updated name for the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// An optional updated description for the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional updated metadata for the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Optional version ID to set as the active version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_version_id: Option<String>,
}

/// Response returned when a skill is deleted.
#[derive(Debug, Clone, Ser, De)]
pub struct SkillDeleteResponse {
    /// The ID of the deleted skill.
    pub id: String,

    /// Object type, always `"skill.deleted"`.
    pub object: String,

    /// Whether the skill was successfully deleted.
    pub deleted: bool,
}

/// The content of a skill.
#[derive(Debug, Clone, Ser, De)]
pub struct SkillContent {
    /// Object type, always `"skill.content"`.
    pub object: String,

    /// The content payload of the skill.
    pub content: serde_json::Value,
}

/// A paginated list of skills.
#[derive(Debug, Clone, Ser, De)]
pub struct SkillList {
    /// Object type, always `"list"`.
    pub object: String,

    /// The list of skill objects.
    pub data: Vec<Skill>,

    /// The ID of the first skill in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,

    /// The ID of the last skill in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Whether there are more skills available beyond this page.
    pub has_more: bool,
}

/// A version of a skill.
#[derive(Debug, Clone, Ser, De)]
pub struct SkillVersion {
    /// Unique identifier for the version.
    pub id: String,

    /// Object type, always `"skill.version"`.
    pub object: String,

    /// The ID of the skill this version belongs to.
    pub skill_id: String,

    /// Unix timestamp (in seconds) of when the version was created.
    pub created_at: u64,

    /// An optional description of this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional metadata associated with this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Request body for creating a new skill version.
#[derive(Debug, Clone, Ser, De)]
pub struct CreateSkillVersionRequest {
    /// An optional description of this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional metadata to attach to the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// The content payload for this version.
    pub content: serde_json::Value,
}

/// The content of a skill version.
#[derive(Debug, Clone, Ser, De)]
pub struct SkillVersionContent {
    /// Object type, always `"skill.version.content"`.
    pub object: String,

    /// The content payload of this version.
    pub content: serde_json::Value,
}

/// A paginated list of skill versions.
#[derive(Debug, Clone, Ser, De)]
pub struct SkillVersionList {
    /// Object type, always `"list"`.
    pub object: String,

    /// The list of skill version objects.
    pub data: Vec<SkillVersion>,

    /// The ID of the first version in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,

    /// The ID of the last version in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Whether there are more versions available beyond this page.
    pub has_more: bool,
}

/// Query parameters for listing skills.
#[derive(Debug, Clone, Ser, De)]
pub struct ListSkillsParams {
    /// Maximum number of skills to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort order by creation date (`"asc"` or `"desc"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Cursor for forward pagination; return results after this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Cursor for backward pagination; return results before this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// Query parameters for listing skill versions.
#[derive(Debug, Clone, Ser, De)]
pub struct ListSkillVersionsParams {
    /// Maximum number of versions to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort order by creation date (`"asc"` or `"desc"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Cursor for forward pagination; return results after this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Cursor for backward pagination; return results before this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}
