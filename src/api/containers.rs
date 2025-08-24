//! # Container Management API
//!
//! This module provides container management for the Code Interpreter tool,
//! allowing models to execute Python code in sandboxed environments with
//! file handling capabilities.

use crate::models::containers::{
    CodeExecutionRequest, CodeExecutionResult, Container, ContainerConfig, ContainerFile,
    ContainerFileList, ContainerList, ListContainersParams,
};
use crate::{
    api::{base::HttpClient, common::ApiClientConstructors},
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
        let path = format!("/v1/containers/{container_id}");
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
        let path = format!("/v1/containers/{container_id}");
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
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        self.client.handle_response(response).await
    }

    /// Upload a file to a container
    pub async fn upload_file(&self, container_id: &str, file_path: &Path) -> Result<ContainerFile> {
        // Read file content
        let file_content = fs::read(file_path)
            .await
            .map_err(|e| OpenAIError::FileError(format!("Failed to read file: {e}")))?;

        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| OpenAIError::FileError("Invalid file name".to_string()))?;

        // Create multipart form
        let part = multipart::Part::bytes(file_content).file_name(file_name.to_string());

        let form = multipart::Form::new()
            .part("file", part)
            .text("purpose", "code_interpreter");

        let path = format!("/v1/containers/{container_id}/files");
        self.client.post_multipart(&path, form).await
    }

    /// Upload file content directly
    pub async fn upload_file_content(
        &self,
        container_id: &str,
        file_name: &str,
        content: Vec<u8>,
    ) -> Result<ContainerFile> {
        // Create multipart form
        let part = multipart::Part::bytes(content).file_name(file_name.to_string());

        let form = multipart::Form::new()
            .part("file", part)
            .text("purpose", "code_interpreter");

        let path = format!("/v1/containers/{container_id}/files");
        self.client.post_multipart(&path, form).await
    }

    /// List files in a container
    pub async fn list_files(&self, container_id: &str) -> Result<ContainerFileList> {
        let path = format!("/v1/containers/{container_id}/files");
        self.client.get(&path).await
    }

    /// Download a file from a container
    pub async fn download_file(&self, container_id: &str, file_id: &str) -> Result<Vec<u8>> {
        let path = format!("/v1/containers/{container_id}/files/{file_id}/content");
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

        fs::write(output_path, content)
            .await
            .map_err(|e| OpenAIError::FileError(format!("Failed to write file: {e}")))?;

        Ok(())
    }

    /// Delete a file from a container
    pub async fn delete_file(&self, container_id: &str, file_id: &str) -> Result<()> {
        let path = format!("/v1/containers/{container_id}/files/{file_id}");

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
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

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
        let path = format!("/v1/containers/{container_id}/execute");

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
        let path = format!("/v1/containers/{container_id}/execute");

        let request = CodeExecutionRequest {
            code: code.to_string(),
            timeout_ms: Some(timeout_ms),
            include_output: Some(true),
        };

        self.client.post(&path, &request).await
    }

    /// Delete a container
    pub async fn delete_container(&self, container_id: &str) -> Result<()> {
        let path = format!("/v1/containers/{container_id}");

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
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            self.client.handle_response::<()>(response).await
        }
    }

    /// Keep a container alive by updating its last activity
    pub async fn keep_alive(&self, container_id: &str) -> Result<()> {
        let path = format!("/v1/containers/{container_id}/keep-alive");

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
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            self.client.handle_response::<()>(response).await
        }
    }
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
