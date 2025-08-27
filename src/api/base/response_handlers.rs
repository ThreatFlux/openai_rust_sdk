//! Response handling and error conversion functionality

use crate::api::base::client::HttpClient;
use crate::error::{OpenAIError, Result};
use serde::de::DeserializeOwned;

impl HttpClient {
    /// Handle error response by extracting text and parsing as API error
    pub(crate) async fn handle_error_response<T>(
        &self,
        response: reqwest::Response,
        status: reqwest::StatusCode,
    ) -> Result<T> {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        // Try to parse as API error response
        serde_json::from_str::<crate::error::ApiErrorResponse>(&error_text).map_or_else(
            |_| {
                Err(OpenAIError::ApiError {
                    status: status.as_u16(),
                    message: error_text,
                })
            },
            |api_error| Err(OpenAIError::from_api_response(status.as_u16(), api_error)),
        )
    }

    /// Handle API response and convert to the desired type
    pub async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text).map_err(|e| {
                OpenAIError::ParseError(format!("Failed to parse response: {e}. Response: {text}"))
            })
        } else {
            self.handle_error_response(response, status).await
        }
    }

    /// Extract raw content from a successful response
    async fn extract_raw_content<F, R>(
        &self,
        response: reqwest::Response,
        status: reqwest::StatusCode,
        extractor: F,
        error_context: &str,
    ) -> Result<R>
    where
        F: FnOnce(
            reqwest::Response,
        )
            -> futures::future::BoxFuture<'static, std::result::Result<R, reqwest::Error>>,
        R: 'static,
    {
        if status.is_success() {
            extractor(response).await.map_err(crate::map_err!(
                RequestError,
                error_context,
                to_string
            ))
        } else {
            self.handle_error_response(response, status).await
        }
    }

    /// Make a GET request and return raw text content
    pub async fn get_text(&self, path: &str) -> Result<String> {
        let url = self.build_simple_url(path);
        let headers = self.build_headers()?;

        let response = self.client().get(&url).headers(headers).send().await?;
        let status = response.status();

        self.extract_raw_content(
            response,
            status,
            |r| Box::pin(async move { r.text().await }),
            "Failed to read response text",
        )
        .await
    }

    /// Make a GET request and return raw bytes
    pub async fn get_bytes(&self, path: &str) -> Result<Vec<u8>> {
        let url = self.build_simple_url(path);
        let headers = self.build_headers()?;

        let response = self.client().get(&url).headers(headers).send().await?;
        let status = response.status();

        self.extract_raw_content(
            response,
            status,
            |r| Box::pin(async move { r.bytes().await.map(|b| b.to_vec()) }),
            "Failed to read response bytes",
        )
        .await
    }

    /// Make a POST request and return raw bytes with content type
    pub async fn post_bytes_with_content_type<B>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(Vec<u8>, String)>
    where
        B: serde::Serialize,
    {
        let url = self.build_simple_url(path);
        let headers = self.build_headers()?;

        let response = self
            .client()
            .post(&url)
            .headers(headers)
            .json(body)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/octet-stream")
                .to_string();

            let bytes = response.bytes().await.map_err(|e| {
                OpenAIError::RequestError(format!("Failed to read response bytes: {e}"))
            })?;

            Ok((bytes.to_vec(), content_type))
        } else {
            self.handle_error_response(response, status).await
        }
    }
}
