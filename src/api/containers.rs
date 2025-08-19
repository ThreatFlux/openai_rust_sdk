//! # Container Management API
//!
//! This module provides container management for the Code Interpreter tool,
//! allowing models to execute Python code in sandboxed environments with
//! file handling capabilities.

use crate::error::OpenAIError;
use crate::models::containers::{
    CodeExecutionRequest, CodeExecutionResult, Container, ContainerConfig, ContainerFile,
    ContainerFileList, ContainerList, ListContainersParams,
};
use reqwest::{multipart, Client};
use serde_json::json;
use std::path::Path;
use tokio::fs;

/// Container Management API client
pub struct ContainersApi {
    /// HTTP client for making requests
    client: Client,
    /// OpenAI API key for authentication
    api_key: String,
    /// Base URL for API requests
    base_url: String,
}

impl ContainersApi {
    /// Create a new Containers API client
    pub fn new(api_key: String) -> Result<Self, OpenAIError> {
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
        })
    }

    /// Create a new container explicitly
    pub async fn create_container(
        &self,
        config: ContainerConfig,
    ) -> Result<Container, OpenAIError> {
        let url = format!("{}/containers", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&config)
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .json::<Container>()
            .await
            .map_err(|e| OpenAIError::ParseError(e.to_string()))
    }

    /// Get container details
    pub async fn get_container(&self, container_id: &str) -> Result<Container, OpenAIError> {
        let url = format!("{}/containers/{}", self.base_url, container_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .json::<Container>()
            .await
            .map_err(|e| OpenAIError::ParseError(e.to_string()))
    }

    /// List all containers
    pub async fn list_containers(
        &self,
        params: Option<ListContainersParams>,
    ) -> Result<ContainerList, OpenAIError> {
        let mut url = format!("{}/containers", self.base_url);

        if let Some(p) = params {
            let query_params =
                serde_json::to_string(&p).map_err(|e| OpenAIError::ParseError(e.to_string()))?;
            url.push('?');
            url.push_str(&query_params);
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .json::<ContainerList>()
            .await
            .map_err(|e| OpenAIError::ParseError(e.to_string()))
    }

    /// Update container metadata
    pub async fn update_container(
        &self,
        container_id: &str,
        metadata: serde_json::Value,
    ) -> Result<Container, OpenAIError> {
        let url = format!("{}/containers/{}", self.base_url, container_id);

        let response = self
            .client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({ "metadata": metadata }))
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .json::<Container>()
            .await
            .map_err(|e| OpenAIError::ParseError(e.to_string()))
    }

    /// Upload a file to a container
    pub async fn upload_file(
        &self,
        container_id: &str,
        file_path: &Path,
    ) -> Result<ContainerFile, OpenAIError> {
        let url = format!("{}/containers/{}/files", self.base_url, container_id);

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

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .json::<ContainerFile>()
            .await
            .map_err(|e| OpenAIError::ParseError(e.to_string()))
    }

    /// Upload file content directly
    pub async fn upload_file_content(
        &self,
        container_id: &str,
        file_name: &str,
        content: Vec<u8>,
    ) -> Result<ContainerFile, OpenAIError> {
        let url = format!("{}/containers/{}/files", self.base_url, container_id);

        // Create multipart form
        let part = multipart::Part::bytes(content).file_name(file_name.to_string());

        let form = multipart::Form::new()
            .part("file", part)
            .text("purpose", "code_interpreter");

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .json::<ContainerFile>()
            .await
            .map_err(|e| OpenAIError::ParseError(e.to_string()))
    }

    /// List files in a container
    pub async fn list_files(&self, container_id: &str) -> Result<ContainerFileList, OpenAIError> {
        let url = format!("{}/containers/{}/files", self.base_url, container_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .json::<ContainerFileList>()
            .await
            .map_err(|e| OpenAIError::ParseError(e.to_string()))
    }

    /// Download a file from a container
    pub async fn download_file(
        &self,
        container_id: &str,
        file_id: &str,
    ) -> Result<Vec<u8>, OpenAIError> {
        let url = format!(
            "{}/containers/{}/files/{}/content",
            self.base_url, container_id, file_id
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| OpenAIError::RequestError(e.to_string()))
    }

    /// Download a file and save it to disk
    pub async fn download_file_to_path(
        &self,
        container_id: &str,
        file_id: &str,
        output_path: &Path,
    ) -> Result<(), OpenAIError> {
        let content = self.download_file(container_id, file_id).await?;

        fs::write(output_path, content)
            .await
            .map_err(|e| OpenAIError::FileError(format!("Failed to write file: {e}")))?;

        Ok(())
    }

    /// Delete a file from a container
    pub async fn delete_file(&self, container_id: &str, file_id: &str) -> Result<(), OpenAIError> {
        let url = format!(
            "{}/containers/{}/files/{}",
            self.base_url, container_id, file_id
        );

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        Ok(())
    }

    /// Execute Python code in a container
    pub async fn execute_code(
        &self,
        container_id: &str,
        code: &str,
    ) -> Result<CodeExecutionResult, OpenAIError> {
        let url = format!("{}/containers/{}/execute", self.base_url, container_id);

        let request = CodeExecutionRequest {
            code: code.to_string(),
            timeout_ms: None,
            include_output: Some(true),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .json::<CodeExecutionResult>()
            .await
            .map_err(|e| OpenAIError::ParseError(e.to_string()))
    }

    /// Execute code with timeout
    pub async fn execute_code_with_timeout(
        &self,
        container_id: &str,
        code: &str,
        timeout_ms: u32,
    ) -> Result<CodeExecutionResult, OpenAIError> {
        let url = format!("{}/containers/{}/execute", self.base_url, container_id);

        let request = CodeExecutionRequest {
            code: code.to_string(),
            timeout_ms: Some(timeout_ms),
            include_output: Some(true),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        response
            .json::<CodeExecutionResult>()
            .await
            .map_err(|e| OpenAIError::ParseError(e.to_string()))
    }

    /// Delete a container
    pub async fn delete_container(&self, container_id: &str) -> Result<(), OpenAIError> {
        let url = format!("{}/containers/{}", self.base_url, container_id);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        Ok(())
    }

    /// Keep a container alive by updating its last activity
    pub async fn keep_alive(&self, container_id: &str) -> Result<(), OpenAIError> {
        let url = format!("{}/containers/{}/keep-alive", self.base_url, container_id);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| OpenAIError::RequestError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OpenAIError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_api_creation() {
        let api = ContainersApi::new("test_key".to_string()).unwrap();
        assert_eq!(api.base_url, "https://api.openai.com/v1");
    }
}
