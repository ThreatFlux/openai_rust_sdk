//! # Administration API Models
//!
//! This module provides data structures for OpenAI's Administration APIs,
//! which allow organization owners and administrators to manage users,
//! invites, projects, service accounts, API keys, rate limits, audit logs,
//! and usage across their organization.
//!
//! ## API Areas
//!
//! - **Audit Logs**: Query audit log events for the organization
//! - **Invites**: Manage pending invitations to the organization
//! - **Users**: Manage organization members and their roles
//! - **Projects**: Create and manage projects with users, service accounts,
//!   API keys, and rate limits
//! - **Usage**: Query usage data across the organization

use crate::{De, Ser};
use serde_json::Value;

// ─── Common ──────────────────────────────────────────────────────────────────

/// Common pagination parameters for admin list endpoints.
#[derive(Debug, Clone, Default, Ser, De)]
pub struct ListAdminParams {
    /// Maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination – return items after this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Cursor for pagination – return items before this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

// ─── Audit Logs ──────────────────────────────────────────────────────────────

/// A single audit log entry.
#[derive(Debug, Clone, Ser, De)]
pub struct AuditLog {
    /// Unique identifier for the audit log event.
    pub id: String,
    /// The event type.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The ISO-8601 timestamp at which the event became effective.
    pub effective_at: String,
    /// The project associated with the event, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Value>,
    /// The actor who performed the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<Value>,
    /// The API key associated with the event, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<Value>,
    /// Additional details about the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

/// Paginated list of audit log entries.
#[derive(Debug, Clone, Ser, De)]
pub struct AuditLogList {
    /// The object type, always `"list"`.
    pub object: String,
    /// The list of audit log entries.
    pub data: Vec<AuditLog>,
    /// The ID of the first item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available after this page.
    pub has_more: bool,
}

/// Query parameters for listing audit logs.
#[derive(Debug, Clone, Default, Ser, De)]
pub struct ListAuditLogsParams {
    /// Maximum number of audit log entries to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination – return entries after this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Cursor for pagination – return entries before this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Filter by effective timestamp (e.g., `{"gte": "...", "lte": "..."}`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<Value>,
    /// Filter by project IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
    /// Filter by event types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    /// Filter by actor IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_ids: Option<Vec<String>>,
    /// Filter by actor email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_emails: Option<Vec<String>>,
}

// ─── Invites ─────────────────────────────────────────────────────────────────

/// An organization invite.
#[derive(Debug, Clone, Ser, De)]
pub struct Invite {
    /// The object type, always `"organization.invite"`.
    pub object: String,
    /// Unique identifier for the invite.
    pub id: String,
    /// The email address the invite was sent to.
    pub email: String,
    /// The role assigned to the invited user.
    pub role: String,
    /// The status of the invite (e.g., `"pending"`, `"accepted"`, `"expired"`).
    pub status: String,
    /// Unix timestamp (in seconds) when the invite was created.
    pub invited_at: u64,
    /// Unix timestamp (in seconds) when the invite expires.
    pub expires_at: u64,
    /// Unix timestamp (in seconds) when the invite was accepted, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<u64>,
}

/// Paginated list of invites.
#[derive(Debug, Clone, Ser, De)]
pub struct InviteList {
    /// The object type, always `"list"`.
    pub object: String,
    /// The list of invites.
    pub data: Vec<Invite>,
    /// The ID of the first item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available after this page.
    pub has_more: bool,
}

/// Request body for creating a new invite.
#[derive(Debug, Clone, Ser, De)]
pub struct CreateInviteRequest {
    /// The email address to invite.
    pub email: String,
    /// The role to assign to the invited user (e.g., `"owner"`, `"reader"`).
    pub role: String,
}

/// Response returned when an invite is deleted.
#[derive(Debug, Clone, Ser, De)]
pub struct InviteDeleteResponse {
    /// The object type, always `"organization.invite.deleted"`.
    pub object: String,
    /// The ID of the deleted invite.
    pub id: String,
    /// Whether the invite was successfully deleted.
    pub deleted: bool,
}

// ─── Users ───────────────────────────────────────────────────────────────────

/// An organization user (member).
#[derive(Debug, Clone, Ser, De)]
pub struct User {
    /// The object type, always `"organization.user"`.
    pub object: String,
    /// Unique identifier for the user.
    pub id: String,
    /// The user's display name.
    pub name: String,
    /// The user's email address.
    pub email: String,
    /// The user's role in the organization.
    pub role: String,
    /// Unix timestamp (in seconds) when the user was added.
    pub added_at: u64,
}

/// Paginated list of users.
#[derive(Debug, Clone, Ser, De)]
pub struct UserList {
    /// The object type, always `"list"`.
    pub object: String,
    /// The list of users.
    pub data: Vec<User>,
    /// The ID of the first item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available after this page.
    pub has_more: bool,
}

/// Request body for updating a user's role.
#[derive(Debug, Clone, Ser, De)]
pub struct UpdateUserRequest {
    /// The new role for the user.
    pub role: String,
}

/// Response returned when a user is deleted from the organization.
#[derive(Debug, Clone, Ser, De)]
pub struct UserDeleteResponse {
    /// The object type, always `"organization.user.deleted"`.
    pub object: String,
    /// The ID of the deleted user.
    pub id: String,
    /// Whether the user was successfully deleted.
    pub deleted: bool,
}

// ─── Projects ────────────────────────────────────────────────────────────────

/// An organization project.
#[derive(Debug, Clone, Ser, De)]
pub struct Project {
    /// The object type, always `"organization.project"`.
    pub object: String,
    /// Unique identifier for the project.
    pub id: String,
    /// The project name.
    pub name: String,
    /// Unix timestamp (in seconds) when the project was created.
    pub created_at: u64,
    /// Unix timestamp (in seconds) when the project was archived, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<u64>,
    /// The project status (e.g., `"active"`, `"archived"`).
    pub status: String,
}

/// Paginated list of projects.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectList {
    /// The object type, always `"list"`.
    pub object: String,
    /// The list of projects.
    pub data: Vec<Project>,
    /// The ID of the first item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available after this page.
    pub has_more: bool,
}

/// Request body for creating a new project.
#[derive(Debug, Clone, Ser, De)]
pub struct CreateProjectRequest {
    /// The name of the project to create.
    pub name: String,
}

/// Request body for updating a project.
#[derive(Debug, Clone, Ser, De)]
pub struct UpdateProjectRequest {
    /// The new name for the project.
    pub name: String,
}

// ─── Project Users ───────────────────────────────────────────────────────────

/// A user within a project.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectUser {
    /// The object type, always `"organization.project.user"`.
    pub object: String,
    /// Unique identifier for the user.
    pub id: String,
    /// The user's display name.
    pub name: String,
    /// The user's email address.
    pub email: String,
    /// The user's role in the project.
    pub role: String,
    /// Unix timestamp (in seconds) when the user was added to the project.
    pub added_at: u64,
}

/// Paginated list of project users.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectUserList {
    /// The object type, always `"list"`.
    pub object: String,
    /// The list of project users.
    pub data: Vec<ProjectUser>,
    /// The ID of the first item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available after this page.
    pub has_more: bool,
}

/// Request body for adding a user to a project.
#[derive(Debug, Clone, Ser, De)]
pub struct CreateProjectUserRequest {
    /// The ID of the user to add.
    pub user_id: String,
    /// The role to assign to the user in the project.
    pub role: String,
}

/// Request body for updating a project user's role.
#[derive(Debug, Clone, Ser, De)]
pub struct UpdateProjectUserRequest {
    /// The new role for the user in the project.
    pub role: String,
}

/// Response returned when a user is removed from a project.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectUserDeleteResponse {
    /// The object type, always `"organization.project.user.deleted"`.
    pub object: String,
    /// The ID of the removed user.
    pub id: String,
    /// Whether the user was successfully removed.
    pub deleted: bool,
}

// ─── Project Service Accounts ────────────────────────────────────────────────

/// A service account within a project.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectServiceAccount {
    /// The object type, always `"organization.project.service_account"`.
    pub object: String,
    /// Unique identifier for the service account.
    pub id: String,
    /// The service account name.
    pub name: String,
    /// The service account's role in the project.
    pub role: String,
    /// Unix timestamp (in seconds) when the service account was created.
    pub created_at: u64,
}

/// Paginated list of project service accounts.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectServiceAccountList {
    /// The object type, always `"list"`.
    pub object: String,
    /// The list of project service accounts.
    pub data: Vec<ProjectServiceAccount>,
    /// The ID of the first item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available after this page.
    pub has_more: bool,
}

/// Request body for creating a new service account in a project.
#[derive(Debug, Clone, Ser, De)]
pub struct CreateProjectServiceAccountRequest {
    /// The name of the service account to create.
    pub name: String,
}

/// Response returned when a service account is created, including the API key.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectServiceAccountCreateResponse {
    /// The object type, always `"organization.project.service_account"`.
    pub object: String,
    /// Unique identifier for the newly created service account.
    pub id: String,
    /// The service account name.
    pub name: String,
    /// The role assigned to the service account.
    pub role: String,
    /// Unix timestamp (in seconds) when the service account was created.
    pub created_at: u64,
    /// The API key created for the service account (only returned at creation time).
    pub api_key: Value,
}

/// Response returned when a service account is deleted from a project.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectServiceAccountDeleteResponse {
    /// The object type, always `"organization.project.service_account.deleted"`.
    pub object: String,
    /// The ID of the deleted service account.
    pub id: String,
    /// Whether the service account was successfully deleted.
    pub deleted: bool,
}

// ─── Project API Keys ────────────────────────────────────────────────────────

/// An API key within a project.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectApiKey {
    /// The object type, always `"organization.project.api_key"`.
    pub object: String,
    /// The redacted value of the API key.
    pub redacted_value: String,
    /// The name of the API key, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Unix timestamp (in seconds) when the API key was created.
    pub created_at: u64,
    /// Unique identifier for the API key.
    pub id: String,
    /// The owner of the API key (user or service account).
    pub owner: Value,
}

/// Paginated list of project API keys.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectApiKeyList {
    /// The object type, always `"list"`.
    pub object: String,
    /// The list of project API keys.
    pub data: Vec<ProjectApiKey>,
    /// The ID of the first item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available after this page.
    pub has_more: bool,
}

/// Response returned when an API key is deleted.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectApiKeyDeleteResponse {
    /// The object type, always `"organization.project.api_key.deleted"`.
    pub object: String,
    /// The ID of the deleted API key.
    pub id: String,
    /// Whether the API key was successfully deleted.
    pub deleted: bool,
}

// ─── Project Rate Limits ─────────────────────────────────────────────────────

/// A rate limit configuration for a model within a project.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectRateLimit {
    /// The object type, always `"project.rate_limit"`.
    pub object: String,
    /// Unique identifier for the rate limit.
    pub id: String,
    /// The model this rate limit applies to.
    pub model: String,
    /// Maximum number of requests per minute.
    pub max_requests_per_1_minute: u64,
    /// Maximum number of tokens per minute.
    pub max_tokens_per_1_minute: u64,
    /// Maximum number of images per minute, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images_per_1_minute: Option<u64>,
    /// Maximum audio megabytes per minute, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_megabytes_per_1_minute: Option<f64>,
    /// Maximum number of requests per day, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests_per_1_day: Option<u64>,
    /// Batch-specific rate limit configuration, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch: Option<Value>,
}

/// Paginated list of project rate limits.
#[derive(Debug, Clone, Ser, De)]
pub struct ProjectRateLimitList {
    /// The object type, always `"list"`.
    pub object: String,
    /// The list of rate limits.
    pub data: Vec<ProjectRateLimit>,
    /// The ID of the first item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last item in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether there are more results available after this page.
    pub has_more: bool,
}

/// Request body for updating a project rate limit.
#[derive(Debug, Clone, Ser, De)]
pub struct UpdateProjectRateLimitRequest {
    /// New maximum number of requests per minute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests_per_1_minute: Option<u64>,
    /// New maximum number of tokens per minute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens_per_1_minute: Option<u64>,
    /// New maximum number of images per minute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images_per_1_minute: Option<u64>,
    /// New maximum audio megabytes per minute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_megabytes_per_1_minute: Option<f64>,
    /// New maximum number of requests per day.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests_per_1_day: Option<u64>,
}

// ─── Usage ───────────────────────────────────────────────────────────────────

/// A single time bucket of usage data.
#[derive(Debug, Clone, Ser, De)]
pub struct UsageBucket {
    /// The object type, always `"bucket"`.
    pub object: String,
    /// Unix timestamp (in seconds) for the start of the bucket.
    pub start_time: u64,
    /// Unix timestamp (in seconds) for the end of the bucket.
    pub end_time: u64,
    /// The usage results within this time bucket.
    pub results: Vec<Value>,
}

/// Response from a usage query.
#[derive(Debug, Clone, Ser, De)]
pub struct UsageResponse {
    /// The object type, always `"page"`.
    pub object: String,
    /// The list of usage buckets.
    pub data: Vec<UsageBucket>,
    /// Whether there are more results available after this page.
    pub has_more: bool,
    /// The cursor for the next page, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_deserialize() {
        let json = r#"{
            "id": "audit-log-123",
            "type": "api_key.created",
            "effective_at": "2024-01-01T00:00:00Z",
            "project": null,
            "actor": {"type": "user", "id": "user-1"},
            "api_key": null,
            "details": null
        }"#;
        let log: AuditLog = serde_json::from_str(json).unwrap();
        assert_eq!(log.id, "audit-log-123");
        assert_eq!(log.type_field, "api_key.created");
        assert_eq!(log.effective_at, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_audit_log_list_deserialize() {
        let json = r#"{
            "object": "list",
            "data": [],
            "first_id": null,
            "last_id": null,
            "has_more": false
        }"#;
        let list: AuditLogList = serde_json::from_str(json).unwrap();
        assert_eq!(list.object, "list");
        assert!(list.data.is_empty());
        assert!(!list.has_more);
    }

    #[test]
    fn test_invite_roundtrip() {
        let invite = Invite {
            object: "organization.invite".to_string(),
            id: "invite-123".to_string(),
            email: "user@example.com".to_string(),
            role: "reader".to_string(),
            status: "pending".to_string(),
            invited_at: 1_700_000_000,
            expires_at: 1_700_086_400,
            accepted_at: None,
        };
        let json = serde_json::to_string(&invite).unwrap();
        let deserialized: Invite = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "invite-123");
        assert_eq!(deserialized.email, "user@example.com");
        assert!(deserialized.accepted_at.is_none());
    }

    #[test]
    fn test_create_invite_request_serialize() {
        let req = CreateInviteRequest {
            email: "new@example.com".to_string(),
            role: "owner".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["email"], "new@example.com");
        assert_eq!(json["role"], "owner");
    }

    #[test]
    fn test_user_deserialize() {
        let json = r#"{
            "object": "organization.user",
            "id": "user-abc",
            "name": "Test User",
            "email": "test@example.com",
            "role": "owner",
            "added_at": 1700000000
        }"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, "user-abc");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.role, "owner");
    }

    #[test]
    fn test_project_roundtrip() {
        let project = Project {
            object: "organization.project".to_string(),
            id: "proj-123".to_string(),
            name: "My Project".to_string(),
            created_at: 1_700_000_000,
            archived_at: None,
            status: "active".to_string(),
        };
        let json = serde_json::to_string(&project).unwrap();
        assert!(!json.contains("archived_at"));
        let deserialized: Project = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "My Project");
        assert_eq!(deserialized.status, "active");
    }

    #[test]
    fn test_project_rate_limit_optional_fields() {
        let json = r#"{
            "object": "project.rate_limit",
            "id": "rl-123",
            "model": "gpt-4",
            "max_requests_per_1_minute": 500,
            "max_tokens_per_1_minute": 100000,
            "max_images_per_1_minute": null,
            "max_audio_megabytes_per_1_minute": null,
            "max_requests_per_1_day": null,
            "batch": null
        }"#;
        let rl: ProjectRateLimit = serde_json::from_str(json).unwrap();
        assert_eq!(rl.model, "gpt-4");
        assert_eq!(rl.max_requests_per_1_minute, 500);
        assert!(rl.max_images_per_1_minute.is_none());
    }

    #[test]
    fn test_usage_response_deserialize() {
        let json = r#"{
            "object": "page",
            "data": [
                {
                    "object": "bucket",
                    "start_time": 1700000000,
                    "end_time": 1700003600,
                    "results": [{"input_tokens": 100, "output_tokens": 50}]
                }
            ],
            "has_more": false,
            "next_page": null
        }"#;
        let resp: UsageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].start_time, 1_700_000_000);
        assert!(!resp.has_more);
    }

    #[test]
    fn test_list_admin_params_default() {
        let params = ListAdminParams::default();
        assert!(params.limit.is_none());
        assert!(params.after.is_none());
        assert!(params.before.is_none());
    }

    #[test]
    fn test_list_audit_logs_params_skip_none() {
        let params = ListAuditLogsParams::default();
        let json = serde_json::to_value(&params).unwrap();
        // All fields are None by default, so the serialized JSON should be an empty object.
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn test_update_project_rate_limit_partial() {
        let req = UpdateProjectRateLimitRequest {
            max_requests_per_1_minute: Some(1000),
            max_tokens_per_1_minute: None,
            max_images_per_1_minute: None,
            max_audio_megabytes_per_1_minute: Some(25.5),
            max_requests_per_1_day: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["max_requests_per_1_minute"], 1000);
        assert_eq!(json["max_audio_megabytes_per_1_minute"], 25.5);
        assert!(json.get("max_tokens_per_1_minute").is_none());
    }

    #[test]
    fn test_project_service_account_create_response() {
        let json = r#"{
            "object": "organization.project.service_account",
            "id": "sa-123",
            "name": "my-service-account",
            "role": "member",
            "created_at": 1700000000,
            "api_key": {"value": "sk-proj-xxx", "name": "Secret Key", "id": "key-1"}
        }"#;
        let resp: ProjectServiceAccountCreateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, "sa-123");
        assert_eq!(resp.name, "my-service-account");
        assert!(resp.api_key.is_object());
    }

    #[test]
    fn test_project_api_key_deserialize() {
        let json = r#"{
            "object": "organization.project.api_key",
            "redacted_value": "sk-proj-...abc",
            "name": "My Key",
            "created_at": 1700000000,
            "id": "key-123",
            "owner": {"type": "user", "user": {"id": "user-1", "name": "Alice"}}
        }"#;
        let key: ProjectApiKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.redacted_value, "sk-proj-...abc");
        assert_eq!(key.name, Some("My Key".to_string()));
        assert!(key.owner.is_object());
    }

    #[test]
    fn test_delete_responses_serialize() {
        let invite_del = InviteDeleteResponse {
            object: "organization.invite.deleted".to_string(),
            id: "invite-1".to_string(),
            deleted: true,
        };
        let user_del = UserDeleteResponse {
            object: "organization.user.deleted".to_string(),
            id: "user-1".to_string(),
            deleted: true,
        };
        let proj_user_del = ProjectUserDeleteResponse {
            object: "organization.project.user.deleted".to_string(),
            id: "user-2".to_string(),
            deleted: true,
        };
        let sa_del = ProjectServiceAccountDeleteResponse {
            object: "organization.project.service_account.deleted".to_string(),
            id: "sa-1".to_string(),
            deleted: true,
        };
        let key_del = ProjectApiKeyDeleteResponse {
            object: "organization.project.api_key.deleted".to_string(),
            id: "key-1".to_string(),
            deleted: true,
        };

        assert!(invite_del.deleted);
        assert!(user_del.deleted);
        assert!(proj_user_del.deleted);
        assert!(sa_del.deleted);
        assert!(key_del.deleted);
    }
}
