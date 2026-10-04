//! # Container Management API
//!
//! This module provides container management for the Code Interpreter tool,
//! allowing models to execute Python code in sandboxed environments with
//! file handling capabilities.

use crate::models::containers::{
    CodeExecutionRequest, CodeExecutionResult, Container, ContainerConfig, ContainerFile,
    ContainerFileList, ContainerFileMetadata, ContainerFileMetadataList, ContainerList,
    ListContainersParams,
};
use crate::{
    api::{
        base::HttpClient,
        common::{ApiClientConstructors, StandardListParams, build_list_query_params},
        shared_utilities::FormBuilder,
    },
    constants::endpoints,
    error::{OpenAIError, Result},
};
use reqwest::multipart;
use serde_json::json;
use std::path::Path;
use tokio::fs;

/// Container Management API client
pub struct ContainersApi {
    /// Shared HTTP client for making requests
    client: HttpClient,
}

impl ApiClientConstructors for ContainersApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self {
            client: http_client,
        }
    }
}

impl ContainersApi {
    /// Create a new container explicitly
    pub async fn create_container(&self, config: ContainerConfig) -> Result<Container> {
        self.client.post("/v1/containers", &config).await
    }

    /// Get container details
    pub async fn get_container(&self, container_id: &str) -> Result<Container> {
        let path = endpoints::containers::by_id(container_id);
        self.client.get(&path).await
    }

    /// List all containers
    pub async fn list_containers(
        &self,
        params: Option<ListContainersParams>,
    ) -> Result<ContainerList> {
        match params {
            Some(p) => {
                // Convert params to query parameters
                let query_params: Vec<(String, String)> = vec![
                    (
                        "limit".to_string(),
                        p.limit.map(|l| l.to_string()).unwrap_or_default(),
                    ),
                    ("order".to_string(), p.order.unwrap_or_default()),
                    ("after".to_string(), p.after.unwrap_or_default()),
                    ("before".to_string(), p.before.unwrap_or_default()),
                ]
                .into_iter()
                .filter(|(_, v)| !v.is_empty())
                .collect();
                self.client
                    .get_with_query("/v1/containers", &query_params)
                    .await
            }
            None => self.client.get("/v1/containers").await,
        }
    }

    /// Update container metadata
    pub async fn update_container(
        &self,
        container_id: &str,
        metadata: serde_json::Value,
    ) -> Result<Container> {
        let path = endpoints::containers::by_id(container_id);
        let body = json!({ "metadata": metadata });

        // Use reqwest client directly for PATCH since HttpClient doesn't have patch method yet
        let url = format!("{}{path}", self.client.base_url());
        let headers = self.client.build_headers()?;

        let response = self
            .client
            .client()
            .patch(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(crate::request_err!(to_string))?;

        self.client.handle_response(response).await
    }

    /// Upload a file to a container
    pub async fn upload_file(&self, container_id: &str, file_path: &Path) -> Result<ContainerFile> {
        // Read file content
        let file_content = crate::helpers::read_bytes(file_path).await?;

        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| OpenAIError::FileError("Invalid file name".to_string()))?;

        // Create multipart form using shared utilities
        let form = FormBuilder::create_container_file_form(file_content, file_name.to_string())?;

        let path = endpoints::containers::files(container_id);
        self.client.post_multipart(&path, form).await
    }

    /// Upload file content directly
    pub async fn upload_file_content(
        &self,
        container_id: &str,
        file_name: &str,
        content: Vec<u8>,
    ) -> Result<ContainerFile> {
        // Create multipart form using shared utilities
        let form = FormBuilder::create_container_file_form(content, file_name.to_string())?;

        let path = endpoints::containers::files(container_id);
        self.client.post_multipart(&path, form).await
    }

    /// List files in a container
    pub async fn list_files(&self, container_id: &str) -> Result<ContainerFileList> {
        let path = endpoints::containers::files(container_id);
        self.client.get(&path).await
    }

    /// Retrieve current metadata from the official container-file endpoint.
    ///
    /// This returns the wire-compatible model instead of the legacy upload helper
    /// model, which expects fields such as `filename` that the endpoint omits.
    pub async fn retrieve_file(
        &self,
        container_id: &str,
        file_id: &str,
    ) -> Result<ContainerFileMetadata> {
        validate_resource_id(container_id)?;
        validate_resource_id(file_id)?;
        self.client
            .get(&endpoints::containers::file_by_id(container_id, file_id))
            .await
    }

    /// List current container-file metadata with cursor pagination.
    ///
    /// The official endpoint supports `after`, `limit`, and `order`; `before`
    /// is rejected rather than being sent as an unsupported query option.
    pub async fn list_files_with_params(
        &self,
        container_id: &str,
        params: &StandardListParams,
    ) -> Result<ContainerFileMetadataList> {
        validate_resource_id(container_id)?;
        validate_file_list_params(params)?;
        self.client
            .get_with_query(
                &endpoints::containers::files(container_id),
                &build_list_query_params(params),
            )
            .await
    }

    /// Download a file from a container
    pub async fn download_file(&self, container_id: &str, file_id: &str) -> Result<Vec<u8>> {
        let path = endpoints::containers::file_content(container_id, file_id);
        self.client.get_bytes(&path).await
    }

    /// Download a file and save it to disk
    pub async fn download_file_to_path(
        &self,
        container_id: &str,
        file_id: &str,
        output_path: &Path,
    ) -> Result<()> {
        let content = self.download_file(container_id, file_id).await?;

        crate::helpers::write_bytes(output_path, &content).await?;

        Ok(())
    }

    /// Delete a file from a container
    pub async fn delete_file(&self, container_id: &str, file_id: &str) -> Result<()> {
        let path = endpoints::containers::file_by_id(container_id, file_id);

        // Use reqwest client directly for DELETE with () response since HttpClient doesn't handle () yet
        let url = format!("{}{path}", self.client.base_url());
        let headers = self.client.build_headers()?;

        let response = self
            .client
            .client()
            .delete(&url)
            .headers(headers)
            .send()
            .await
            .map_err(crate::request_err!(to_string))?;

        if response.status().is_success() {
            Ok(())
        } else {
            self.client.handle_response::<()>(response).await
        }
    }

    /// Execute Python code in a container
    pub async fn execute_code(
        &self,
        container_id: &str,
        code: &str,
    ) -> Result<CodeExecutionResult> {
        let path = endpoints::containers::execute(container_id);

        let request = CodeExecutionRequest {
            code: code.to_string(),
            timeout_ms: None,
            include_output: Some(true),
        };

        self.client.post(&path, &request).await
    }

    /// Execute code with timeout
    pub async fn execute_code_with_timeout(
        &self,
        container_id: &str,
        code: &str,
        timeout_ms: u32,
    ) -> Result<CodeExecutionResult> {
        let path = endpoints::containers::execute(container_id);

        let request = CodeExecutionRequest {
            code: code.to_string(),
            timeout_ms: Some(timeout_ms),
            include_output: Some(true),
        };

        self.client.post(&path, &request).await
    }

    /// Delete a container
    pub async fn delete_container(&self, container_id: &str) -> Result<()> {
        let path = endpoints::containers::by_id(container_id);

        // Use reqwest client directly for DELETE with () response since HttpClient doesn't handle () yet
        let url = format!("{}{path}", self.client.base_url());
        let headers = self.client.build_headers()?;

        let response = self
            .client
            .client()
            .delete(&url)
            .headers(headers)
            .send()
            .await
            .map_err(crate::request_err!(to_string))?;

        if response.status().is_success() {
            Ok(())
        } else {
            self.client.handle_response::<()>(response).await
        }
    }

    /// Keep a container alive by updating its last activity
    pub async fn keep_alive(&self, container_id: &str) -> Result<()> {
        let path = endpoints::containers::keep_alive(container_id);

        // Use reqwest client directly for POST with () response since HttpClient doesn't handle () well yet
        let url = format!("{}{path}", self.client.base_url());
        let headers = self.client.build_headers()?;

        let response = self
            .client
            .client()
            .post(&url)
            .headers(headers)
            .send()
            .await
            .map_err(crate::request_err!(to_string))?;

        if response.status().is_success() {
            Ok(())
        } else {
            self.client.handle_response::<()>(response).await
        }
    }
}

/// Reject identifiers that can change the request path or query.
fn validate_resource_id(value: &str) -> Result<()> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    {
        return Err(OpenAIError::invalid_request(
            "Invalid container or file identifier",
        ));
    }
    Ok(())
}

/// Enforce only the documented container-file pagination controls.
fn validate_file_list_params(params: &StandardListParams) -> Result<()> {
    if params.before.is_some() {
        return Err(OpenAIError::invalid_request(
            "Container file listing does not support before",
        ));
    }
    if params
        .limit
        .is_some_and(|limit| !(1..=100).contains(&limit))
    {
        return Err(OpenAIError::invalid_request(
            "Container file limit must be between 1 and 100",
        ));
    }
    if params
        .order
        .as_deref()
        .is_some_and(|order| !matches!(order, "asc" | "desc"))
    {
        return Err(OpenAIError::invalid_request(
            "Container file order must be asc or desc",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_api_creation() {
        use crate::api::common::ApiClientConstructors;
        let api = ContainersApi::new("test_key").unwrap();
        assert_eq!(api.client.base_url(), "https://api.openai.com");
    }
}
