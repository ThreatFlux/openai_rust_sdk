//! Basic HTTP request methods for the client

use crate::api::base::client::HttpClient;
use crate::error::Result;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

impl HttpClient {
    /// Execute a GET request with the given headers
    pub(crate) async fn execute_get_request<T>(&self, url: &str, headers: HeaderMap) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.client().get(url).headers(headers).send().await?;
        self.handle_response(response).await
    }

    /// Execute a POST request with JSON body and the given headers
    pub(crate) async fn execute_post_request<T, B>(
        &self,
        url: &str,
        headers: HeaderMap,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let response = self
            .client()
            .post(url)
            .headers(headers)
            .json(body)
            .send()
            .await?;
        self.handle_response(response).await
    }

    /// Execute a DELETE request with the given headers
    pub(crate) async fn execute_delete_request<T>(&self, url: &str, headers: HeaderMap) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.client().delete(url).headers(headers).send().await?;
        self.handle_response(response).await
    }

    /// Internal GET request with configurable headers
    async fn get_internal<T>(&self, path: &str, use_beta: bool) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_simple_url(path);
        let headers = if use_beta {
            self.build_headers_with_beta()?
        } else {
            self.build_headers()?
        };
        self.execute_get_request(&url, headers).await
    }

    /// Make a GET request to the specified path
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_internal(path, false).await
    }

    /// Make a GET request with beta headers to the specified path
    pub async fn get_with_beta<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_internal(path, true).await
    }

    /// Internal POST request with configurable headers
    async fn post_internal<T, B>(&self, path: &str, body: &B, use_beta: bool) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        let url = self.build_simple_url(path);
        let headers = if use_beta {
            self.build_headers_with_beta()?
        } else {
            self.build_headers()?
        };
        self.execute_post_request(&url, headers, body).await
    }

    /// Make a POST request with JSON body to the specified path
    #[allow(clippy::future_not_send)]
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        self.post_internal(path, body, false).await
    }

    /// Make a POST request with JSON body and beta headers to the specified path
    #[allow(clippy::future_not_send)]
    pub async fn post_with_beta<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        self.post_internal(path, body, true).await
    }

    /// Internal DELETE request with configurable headers
    async fn delete_internal<T>(&self, path: &str, use_beta: bool) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_simple_url(path);
        let headers = if use_beta {
            self.build_headers_with_beta()?
        } else {
            self.build_headers()?
        };
        self.execute_delete_request(&url, headers).await
    }

    /// Make a DELETE request to the specified path
    pub async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.delete_internal(path, false).await
    }

    /// Make a DELETE request with beta headers to the specified path
    pub async fn delete_with_beta<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.delete_internal(path, true).await
    }
}
