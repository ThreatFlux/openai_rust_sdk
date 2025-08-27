//! Advanced HTTP request methods for the client

use crate::api::base::client::HttpClient;
use crate::error::Result;
use reqwest::multipart::Form;
use serde::de::DeserializeOwned;

impl HttpClient {
    /// Internal GET request with query parameters and configurable headers
    async fn get_with_query_internal<T>(
        &self,
        path: &str,
        query_params: &[(String, String)],
        use_beta: bool,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(path, query_params);
        let headers = if use_beta {
            self.build_headers_with_beta()?
        } else {
            self.build_headers()?
        };
        self.execute_get_request(&url, headers).await
    }

    /// Make a GET request with query parameters
    pub async fn get_with_query<T>(
        &self,
        path: &str,
        query_params: &[(String, String)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query_internal(path, query_params, false)
            .await
    }

    /// Make a GET request with query parameters and beta headers
    pub async fn get_with_query_and_beta<T>(
        &self,
        path: &str,
        query_params: &[(String, String)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query_internal(path, query_params, true).await
    }

    /// Make a POST request with multipart form data
    pub async fn post_multipart<T>(&self, path: &str, form: Form) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_simple_url(path);
        let headers = self.build_auth_headers()?; // Don't set Content-Type for multipart

        let response = self
            .client()
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a POST request with streaming response
    pub async fn post_stream<B>(&self, path: &str, body: &B) -> Result<reqwest::Response>
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
            Ok(response)
        } else {
            self.handle_error_response(response, status).await
        }
    }
}
