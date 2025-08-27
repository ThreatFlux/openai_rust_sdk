//! # Real-time Audio API Client
//!
//! Core client functionality for the Real-time Audio API.

use super::{config::RealtimeAudioConfig, session::RealtimeSession};
use crate::api::base::HttpClient;
use crate::error::{OpenAIError, Result};
use crate::models::realtime_audio::{RealtimeSessionRequest, RealtimeSessionResponse};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Real-time Audio API client with WebRTC support
pub struct RealtimeAudioApi {
    /// HTTP client for making API requests
    http_client: HttpClient,
    /// Active real-time sessions mapped by session ID
    sessions: Arc<RwLock<HashMap<String, Arc<RealtimeSession>>>>,
    /// Configuration for real-time audio API
    pub config: RealtimeAudioConfig,
}

impl RealtimeAudioApi {
    /// Create a new Real-time Audio API client
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config: RealtimeAudioConfig::default(),
        })
    }

    /// Create a new client with custom configuration
    pub fn new_with_config<S: Into<String>>(
        api_key: S,
        config: RealtimeAudioConfig,
    ) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(api_key)?,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    /// Create a new client with custom base URL
    pub fn new_with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new_with_base_url(api_key, base_url)?,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config: RealtimeAudioConfig::default(),
        })
    }

    /// Create a new real-time audio session
    pub async fn create_session(
        &self,
        request: &RealtimeSessionRequest,
    ) -> Result<Arc<RealtimeSession>> {
        let url = format!("{}/realtime/sessions", self.http_client.base_url());
        let headers = self.http_client.build_headers()?;

        let response = self
            .http_client
            .client()
            .post(&url)
            .headers(headers)
            .json(request)
            .send()
            .await
            .map_err(crate::request_err!(to_string))?;

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

        let session_response: RealtimeSessionResponse = response
            .json()
            .await
            .map_err(crate::parse_err!(to_string))?;

        let session = self
            .create_webrtc_session(session_response, request.config.clone())
            .await?;

        // Store the session
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session.clone());

        Ok(session)
    }

    /// Get an existing session
    pub async fn get_session(&self, session_id: &str) -> Option<Arc<RealtimeSession>> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// List active sessions
    pub async fn list_sessions(&self) -> Vec<String> {
        let sessions = self.sessions.read().await;
        sessions.keys().cloned().collect()
    }

    /// Close a session
    pub async fn close_session(&self, session_id: &str) -> Result<()> {
        if let Some(session) = {
            let mut sessions = self.sessions.write().await;
            sessions.remove(session_id)
        } {
            session.close().await?;
        }
        Ok(())
    }

    /// Get the HTTP client for internal use
    pub(crate) fn http_client(&self) -> &HttpClient {
        &self.http_client
    }

    /// Get the sessions map for internal use
    pub(crate) fn sessions(&self) -> &Arc<RwLock<HashMap<String, Arc<RealtimeSession>>>> {
        &self.sessions
    }
}
