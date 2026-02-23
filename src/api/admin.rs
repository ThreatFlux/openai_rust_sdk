//! # Administration API
//!
//! Client for the OpenAI Administration APIs, which allow organization owners
//! and administrators to manage users, invites, projects, service accounts,
//! API keys, rate limits, audit logs, and usage.

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::Result;
use crate::models::admin::{
    AuditLogList, CreateInviteRequest, CreateProjectRequest, CreateProjectServiceAccountRequest,
    CreateProjectUserRequest, Invite, InviteDeleteResponse, InviteList, ListAdminParams,
    ListAuditLogsParams, Project, ProjectApiKey, ProjectApiKeyDeleteResponse, ProjectApiKeyList,
    ProjectList, ProjectRateLimit, ProjectRateLimitList, ProjectServiceAccount,
    ProjectServiceAccountCreateResponse, ProjectServiceAccountDeleteResponse,
    ProjectServiceAccountList, ProjectUser, ProjectUserDeleteResponse, ProjectUserList,
    UpdateProjectRateLimitRequest, UpdateProjectRequest, UpdateProjectUserRequest,
    UpdateUserRequest, UsageResponse, User, UserDeleteResponse, UserList,
};
use serde_json::Value;

/// Administration API client for managing organization resources
pub struct AdminApi {
    /// Shared HTTP client for making requests
    client: HttpClient,
}

impl ApiClientConstructors for AdminApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self {
            client: http_client,
        }
    }
}

/// Build query parameters from optional admin list params
fn build_admin_list_params(params: &ListAdminParams) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if let Some(l) = params.limit {
        query.push(("limit".to_string(), l.to_string()));
    }
    if let Some(ref a) = params.after {
        query.push(("after".to_string(), a.clone()));
    }
    if let Some(ref b) = params.before {
        query.push(("before".to_string(), b.clone()));
    }
    query
}

impl AdminApi {
    // ─── Audit Logs ──────────────────────────────────────────────────────

    /// List audit log events for the organization
    pub async fn list_audit_logs(
        &self,
        params: Option<&ListAuditLogsParams>,
    ) -> Result<AuditLogList> {
        match params {
            Some(p) => {
                let mut query = Vec::new();
                if let Some(l) = p.limit {
                    query.push(("limit".to_string(), l.to_string()));
                }
                if let Some(ref a) = p.after {
                    query.push(("after".to_string(), a.clone()));
                }
                if let Some(ref b) = p.before {
                    query.push(("before".to_string(), b.clone()));
                }
                self.client
                    .get_with_query("/v1/organization/audit_logs", &query)
                    .await
            }
            None => self.client.get("/v1/organization/audit_logs").await,
        }
    }

    // ─── Invites ─────────────────────────────────────────────────────────

    /// List pending invitations to the organization
    pub async fn list_invites(&self, params: Option<&ListAdminParams>) -> Result<InviteList> {
        match params {
            Some(p) => {
                let query = build_admin_list_params(p);
                self.client
                    .get_with_query("/v1/organization/invites", &query)
                    .await
            }
            None => self.client.get("/v1/organization/invites").await,
        }
    }

    /// Create a new invite
    pub async fn create_invite(&self, request: &CreateInviteRequest) -> Result<Invite> {
        self.client.post("/v1/organization/invites", request).await
    }

    /// Retrieve an invite by ID
    pub async fn retrieve_invite(&self, invite_id: impl AsRef<str>) -> Result<Invite> {
        let path = format!("/v1/organization/invites/{}", invite_id.as_ref());
        self.client.get(&path).await
    }

    /// Delete an invite by ID
    pub async fn delete_invite(&self, invite_id: impl AsRef<str>) -> Result<InviteDeleteResponse> {
        let path = format!("/v1/organization/invites/{}", invite_id.as_ref());
        self.client.delete(&path).await
    }

    // ─── Users ───────────────────────────────────────────────────────────

    /// List organization members
    pub async fn list_users(&self, params: Option<&ListAdminParams>) -> Result<UserList> {
        match params {
            Some(p) => {
                let query = build_admin_list_params(p);
                self.client
                    .get_with_query("/v1/organization/users", &query)
                    .await
            }
            None => self.client.get("/v1/organization/users").await,
        }
    }

    /// Retrieve a user by ID
    pub async fn retrieve_user(&self, user_id: impl AsRef<str>) -> Result<User> {
        let path = format!("/v1/organization/users/{}", user_id.as_ref());
        self.client.get(&path).await
    }

    /// Update a user's role
    pub async fn update_user(
        &self,
        user_id: impl AsRef<str>,
        request: &UpdateUserRequest,
    ) -> Result<User> {
        let path = format!("/v1/organization/users/{}", user_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Remove a user from the organization
    pub async fn delete_user(&self, user_id: impl AsRef<str>) -> Result<UserDeleteResponse> {
        let path = format!("/v1/organization/users/{}", user_id.as_ref());
        self.client.delete(&path).await
    }

    // ─── Projects ────────────────────────────────────────────────────────

    /// List organization projects
    pub async fn list_projects(&self, params: Option<&ListAdminParams>) -> Result<ProjectList> {
        match params {
            Some(p) => {
                let query = build_admin_list_params(p);
                self.client
                    .get_with_query("/v1/organization/projects", &query)
                    .await
            }
            None => self.client.get("/v1/organization/projects").await,
        }
    }

    /// Create a new project
    pub async fn create_project(&self, request: &CreateProjectRequest) -> Result<Project> {
        self.client.post("/v1/organization/projects", request).await
    }

    /// Retrieve a project by ID
    pub async fn retrieve_project(&self, project_id: impl AsRef<str>) -> Result<Project> {
        let path = format!("/v1/organization/projects/{}", project_id.as_ref());
        self.client.get(&path).await
    }

    /// Update a project
    pub async fn update_project(
        &self,
        project_id: impl AsRef<str>,
        request: &UpdateProjectRequest,
    ) -> Result<Project> {
        let path = format!("/v1/organization/projects/{}", project_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Archive a project
    pub async fn archive_project(&self, project_id: impl AsRef<str>) -> Result<Project> {
        let path = format!("/v1/organization/projects/{}/archive", project_id.as_ref());
        self.client.post(&path, &()).await
    }

    // ─── Project Users ───────────────────────────────────────────────────

    /// List users in a project
    pub async fn list_project_users(
        &self,
        project_id: impl AsRef<str>,
        params: Option<&ListAdminParams>,
    ) -> Result<ProjectUserList> {
        let path = format!("/v1/organization/projects/{}/users", project_id.as_ref());
        match params {
            Some(p) => {
                let query = build_admin_list_params(p);
                self.client.get_with_query(&path, &query).await
            }
            None => self.client.get(&path).await,
        }
    }

    /// Add a user to a project
    pub async fn create_project_user(
        &self,
        project_id: impl AsRef<str>,
        request: &CreateProjectUserRequest,
    ) -> Result<ProjectUser> {
        let path = format!("/v1/organization/projects/{}/users", project_id.as_ref());
        self.client.post(&path, request).await
    }

    /// Retrieve a user in a project
    pub async fn retrieve_project_user(
        &self,
        project_id: impl AsRef<str>,
        user_id: impl AsRef<str>,
    ) -> Result<ProjectUser> {
        let path = format!(
            "/v1/organization/projects/{}/users/{}",
            project_id.as_ref(),
            user_id.as_ref()
        );
        self.client.get(&path).await
    }

    /// Update a user's role in a project
    pub async fn update_project_user(
        &self,
        project_id: impl AsRef<str>,
        user_id: impl AsRef<str>,
        request: &UpdateProjectUserRequest,
    ) -> Result<ProjectUser> {
        let path = format!(
            "/v1/organization/projects/{}/users/{}",
            project_id.as_ref(),
            user_id.as_ref()
        );
        self.client.post(&path, request).await
    }

    /// Remove a user from a project
    pub async fn delete_project_user(
        &self,
        project_id: impl AsRef<str>,
        user_id: impl AsRef<str>,
    ) -> Result<ProjectUserDeleteResponse> {
        let path = format!(
            "/v1/organization/projects/{}/users/{}",
            project_id.as_ref(),
            user_id.as_ref()
        );
        self.client.delete(&path).await
    }

    // ─── Project Service Accounts ────────────────────────────────────────

    /// List service accounts in a project
    pub async fn list_project_service_accounts(
        &self,
        project_id: impl AsRef<str>,
        params: Option<&ListAdminParams>,
    ) -> Result<ProjectServiceAccountList> {
        let path = format!(
            "/v1/organization/projects/{}/service_accounts",
            project_id.as_ref()
        );
        match params {
            Some(p) => {
                let query = build_admin_list_params(p);
                self.client.get_with_query(&path, &query).await
            }
            None => self.client.get(&path).await,
        }
    }

    /// Create a service account in a project
    pub async fn create_project_service_account(
        &self,
        project_id: impl AsRef<str>,
        request: &CreateProjectServiceAccountRequest,
    ) -> Result<ProjectServiceAccountCreateResponse> {
        let path = format!(
            "/v1/organization/projects/{}/service_accounts",
            project_id.as_ref()
        );
        self.client.post(&path, request).await
    }

    /// Retrieve a service account in a project
    pub async fn retrieve_project_service_account(
        &self,
        project_id: impl AsRef<str>,
        service_account_id: impl AsRef<str>,
    ) -> Result<ProjectServiceAccount> {
        let path = format!(
            "/v1/organization/projects/{}/service_accounts/{}",
            project_id.as_ref(),
            service_account_id.as_ref()
        );
        self.client.get(&path).await
    }

    /// Delete a service account from a project
    pub async fn delete_project_service_account(
        &self,
        project_id: impl AsRef<str>,
        service_account_id: impl AsRef<str>,
    ) -> Result<ProjectServiceAccountDeleteResponse> {
        let path = format!(
            "/v1/organization/projects/{}/service_accounts/{}",
            project_id.as_ref(),
            service_account_id.as_ref()
        );
        self.client.delete(&path).await
    }

    // ─── Project API Keys ────────────────────────────────────────────────

    /// List API keys in a project
    pub async fn list_project_api_keys(
        &self,
        project_id: impl AsRef<str>,
        params: Option<&ListAdminParams>,
    ) -> Result<ProjectApiKeyList> {
        let path = format!("/v1/organization/projects/{}/api_keys", project_id.as_ref());
        match params {
            Some(p) => {
                let query = build_admin_list_params(p);
                self.client.get_with_query(&path, &query).await
            }
            None => self.client.get(&path).await,
        }
    }

    /// Retrieve an API key in a project
    pub async fn retrieve_project_api_key(
        &self,
        project_id: impl AsRef<str>,
        api_key_id: impl AsRef<str>,
    ) -> Result<ProjectApiKey> {
        let path = format!(
            "/v1/organization/projects/{}/api_keys/{}",
            project_id.as_ref(),
            api_key_id.as_ref()
        );
        self.client.get(&path).await
    }

    /// Delete an API key from a project
    pub async fn delete_project_api_key(
        &self,
        project_id: impl AsRef<str>,
        api_key_id: impl AsRef<str>,
    ) -> Result<ProjectApiKeyDeleteResponse> {
        let path = format!(
            "/v1/organization/projects/{}/api_keys/{}",
            project_id.as_ref(),
            api_key_id.as_ref()
        );
        self.client.delete(&path).await
    }

    // ─── Project Rate Limits ─────────────────────────────────────────────

    /// List rate limits for a project
    pub async fn list_project_rate_limits(
        &self,
        project_id: impl AsRef<str>,
        params: Option<&ListAdminParams>,
    ) -> Result<ProjectRateLimitList> {
        let path = format!(
            "/v1/organization/projects/{}/rate_limits",
            project_id.as_ref()
        );
        match params {
            Some(p) => {
                let query = build_admin_list_params(p);
                self.client.get_with_query(&path, &query).await
            }
            None => self.client.get(&path).await,
        }
    }

    /// Update a rate limit for a project
    pub async fn update_project_rate_limit(
        &self,
        project_id: impl AsRef<str>,
        rate_limit_id: impl AsRef<str>,
        request: &UpdateProjectRateLimitRequest,
    ) -> Result<ProjectRateLimit> {
        let path = format!(
            "/v1/organization/projects/{}/rate_limits/{}",
            project_id.as_ref(),
            rate_limit_id.as_ref()
        );
        self.client.post(&path, request).await
    }

    // ─── Usage ───────────────────────────────────────────────────────────

    /// Query completions usage data
    pub async fn get_completions_usage(
        &self,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<UsageResponse> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        self.client
            .get_with_query("/v1/organization/usage/completions", &query)
            .await
    }

    /// Query embeddings usage data
    pub async fn get_embeddings_usage(
        &self,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<UsageResponse> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        self.client
            .get_with_query("/v1/organization/usage/embeddings", &query)
            .await
    }

    /// Query moderations usage data
    pub async fn get_moderations_usage(
        &self,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<UsageResponse> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        self.client
            .get_with_query("/v1/organization/usage/moderations", &query)
            .await
    }

    /// Query images usage data
    pub async fn get_images_usage(
        &self,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<UsageResponse> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        self.client
            .get_with_query("/v1/organization/usage/images", &query)
            .await
    }

    /// Query audio speeches usage data
    pub async fn get_audio_speeches_usage(
        &self,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<UsageResponse> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        self.client
            .get_with_query("/v1/organization/usage/audio_speeches", &query)
            .await
    }

    /// Query audio transcriptions usage data
    pub async fn get_audio_transcriptions_usage(
        &self,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<UsageResponse> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        self.client
            .get_with_query("/v1/organization/usage/audio_transcriptions", &query)
            .await
    }

    /// Query vector stores usage data
    pub async fn get_vector_stores_usage(
        &self,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<UsageResponse> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        self.client
            .get_with_query("/v1/organization/usage/vector_stores", &query)
            .await
    }

    /// Query code interpreter sessions usage data
    pub async fn get_code_interpreter_usage(
        &self,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<UsageResponse> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        self.client
            .get_with_query("/v1/organization/usage/code_interpreter_sessions", &query)
            .await
    }

    /// Query generic usage data by category
    pub async fn get_usage(
        &self,
        category: impl AsRef<str>,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<UsageResponse> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        let path = format!("/v1/organization/usage/{}", category.as_ref());
        self.client.get_with_query(&path, &query).await
    }

    /// Query costs data
    pub async fn get_costs(
        &self,
        start_time: u64,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Value> {
        let mut query = vec![("start_time".to_string(), start_time.to_string())];
        if let Some(extra) = params {
            for (k, v) in extra {
                query.push(((*k).to_string(), (*v).to_string()));
            }
        }
        self.client
            .get_with_query("/v1/organization/costs", &query)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_api_creation() {
        let api = AdminApi::new("test-key").unwrap();
        assert_eq!(api.client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_admin_api_creation_with_base_url() {
        let api = AdminApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_admin_api_empty_key_fails() {
        let result = AdminApi::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_admin_list_params_all() {
        let params = ListAdminParams {
            limit: Some(50),
            after: Some("cursor-a".to_string()),
            before: Some("cursor-b".to_string()),
        };
        let query = build_admin_list_params(&params);
        assert_eq!(query.len(), 3);
        assert_eq!(query[0], ("limit".to_string(), "50".to_string()));
        assert_eq!(query[1], ("after".to_string(), "cursor-a".to_string()));
        assert_eq!(query[2], ("before".to_string(), "cursor-b".to_string()));
    }

    #[test]
    fn test_build_admin_list_params_empty() {
        let params = ListAdminParams::default();
        let query = build_admin_list_params(&params);
        assert!(query.is_empty());
    }

    #[test]
    fn test_build_admin_list_params_partial() {
        let params = ListAdminParams {
            limit: Some(10),
            after: None,
            before: None,
        };
        let query = build_admin_list_params(&params);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0].1, "10");
    }

    #[test]
    fn test_create_invite_request_serialization() {
        let req = CreateInviteRequest {
            email: "test@example.com".to_string(),
            role: "reader".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["email"], "test@example.com");
        assert_eq!(json["role"], "reader");
    }

    #[test]
    fn test_invite_deserialization() {
        let json = r#"{
            "object": "organization.invite",
            "id": "invite-1",
            "email": "user@test.com",
            "role": "owner",
            "status": "pending",
            "invited_at": 1700000000,
            "expires_at": 1700086400
        }"#;
        let invite: Invite = serde_json::from_str(json).unwrap();
        assert_eq!(invite.id, "invite-1");
        assert_eq!(invite.email, "user@test.com");
        assert_eq!(invite.status, "pending");
        assert!(invite.accepted_at.is_none());
    }

    #[test]
    fn test_user_deserialization() {
        let json = r#"{
            "object": "organization.user",
            "id": "user-1",
            "name": "Alice",
            "email": "alice@test.com",
            "role": "owner",
            "added_at": 1700000000
        }"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.role, "owner");
    }

    #[test]
    fn test_project_deserialization() {
        let json = r#"{
            "object": "organization.project",
            "id": "proj-1",
            "name": "My Project",
            "created_at": 1700000000,
            "status": "active"
        }"#;
        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.name, "My Project");
        assert_eq!(project.status, "active");
        assert!(project.archived_at.is_none());
    }

    #[test]
    fn test_update_user_request() {
        let req = UpdateUserRequest {
            role: "reader".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["role"], "reader");
    }

    #[test]
    fn test_create_project_request() {
        let req = CreateProjectRequest {
            name: "New Project".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["name"], "New Project");
    }

    #[test]
    fn test_update_project_request() {
        let req = UpdateProjectRequest {
            name: "Renamed".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["name"], "Renamed");
    }

    #[test]
    fn test_project_user_deserialization() {
        let json = r#"{
            "object": "organization.project.user",
            "id": "user-1",
            "name": "Bob",
            "email": "bob@test.com",
            "role": "member",
            "added_at": 1700000000
        }"#;
        let pu: ProjectUser = serde_json::from_str(json).unwrap();
        assert_eq!(pu.name, "Bob");
        assert_eq!(pu.role, "member");
    }

    #[test]
    fn test_create_project_user_request() {
        let req = CreateProjectUserRequest {
            user_id: "user-1".to_string(),
            role: "member".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["user_id"], "user-1");
        assert_eq!(json["role"], "member");
    }

    #[test]
    fn test_create_project_service_account_request() {
        let req = CreateProjectServiceAccountRequest {
            name: "my-sa".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["name"], "my-sa");
    }

    #[test]
    fn test_update_project_rate_limit_request_partial() {
        let req = UpdateProjectRateLimitRequest {
            max_requests_per_1_minute: Some(1000),
            max_tokens_per_1_minute: None,
            max_images_per_1_minute: None,
            max_audio_megabytes_per_1_minute: None,
            max_requests_per_1_day: Some(50000),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["max_requests_per_1_minute"], 1000);
        assert_eq!(json["max_requests_per_1_day"], 50000);
        assert!(json.get("max_tokens_per_1_minute").is_none());
    }

    #[test]
    fn test_project_rate_limit_deserialization() {
        let json = r#"{
            "object": "project.rate_limit",
            "id": "rl-1",
            "model": "gpt-4",
            "max_requests_per_1_minute": 500,
            "max_tokens_per_1_minute": 100000
        }"#;
        let rl: ProjectRateLimit = serde_json::from_str(json).unwrap();
        assert_eq!(rl.model, "gpt-4");
        assert_eq!(rl.max_requests_per_1_minute, 500);
        assert!(rl.max_images_per_1_minute.is_none());
    }

    #[test]
    fn test_usage_response_deserialization() {
        let json = r#"{
            "object": "page",
            "data": [{
                "object": "bucket",
                "start_time": 1700000000,
                "end_time": 1700003600,
                "results": []
            }],
            "has_more": false
        }"#;
        let resp: UsageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.len(), 1);
        assert!(!resp.has_more);
        assert!(resp.next_page.is_none());
    }

    #[test]
    fn test_delete_response_types() {
        let inv_del: InviteDeleteResponse = serde_json::from_str(
            r#"{"object": "organization.invite.deleted", "id": "inv-1", "deleted": true}"#,
        )
        .unwrap();
        assert!(inv_del.deleted);

        let user_del: UserDeleteResponse = serde_json::from_str(
            r#"{"object": "organization.user.deleted", "id": "u-1", "deleted": true}"#,
        )
        .unwrap();
        assert!(user_del.deleted);

        let pu_del: ProjectUserDeleteResponse = serde_json::from_str(
            r#"{"object": "organization.project.user.deleted", "id": "u-2", "deleted": true}"#,
        )
        .unwrap();
        assert!(pu_del.deleted);

        let sa_del: ProjectServiceAccountDeleteResponse = serde_json::from_str(
            r#"{"object": "organization.project.service_account.deleted", "id": "sa-1", "deleted": true}"#,
        )
        .unwrap();
        assert!(sa_del.deleted);

        let key_del: ProjectApiKeyDeleteResponse = serde_json::from_str(
            r#"{"object": "organization.project.api_key.deleted", "id": "key-1", "deleted": true}"#,
        )
        .unwrap();
        assert!(key_del.deleted);
    }

    #[test]
    fn test_list_audit_logs_params_default() {
        let params = ListAuditLogsParams::default();
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn test_project_api_key_deserialization() {
        let json = r#"{
            "object": "organization.project.api_key",
            "redacted_value": "sk-...abc",
            "name": "Test Key",
            "created_at": 1700000000,
            "id": "key-1",
            "owner": {"type": "user"}
        }"#;
        let key: ProjectApiKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.redacted_value, "sk-...abc");
        assert_eq!(key.name, Some("Test Key".to_string()));
    }

    #[test]
    fn test_paginated_list_types() {
        let inv_list: InviteList =
            serde_json::from_str(r#"{"object": "list", "data": [], "has_more": false}"#).unwrap();
        assert!(!inv_list.has_more);

        let user_list: UserList =
            serde_json::from_str(r#"{"object": "list", "data": [], "has_more": true}"#).unwrap();
        assert!(user_list.has_more);

        let proj_list: ProjectList =
            serde_json::from_str(r#"{"object": "list", "data": [], "has_more": false}"#).unwrap();
        assert!(proj_list.data.is_empty());
    }
}
