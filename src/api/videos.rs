//! # Videos (Sora) API
//!
//! Client for the OpenAI Videos API (Sora), which allows you to generate,
//! retrieve, list, and delete AI-generated videos.

use crate::api::base::HttpClient;
use crate::api::common::ApiClientConstructors;
use crate::error::Result;
use crate::models::videos::{
    CreateVideoRequest, ListVideosParams, Video, VideoDeleteResponse, VideoList,
};

/// Videos API client for Sora video generation
pub struct VideosApi {
    /// Shared HTTP client for making requests
    client: HttpClient,
}

impl ApiClientConstructors for VideosApi {
    fn from_http_client(http_client: HttpClient) -> Self {
        Self {
            client: http_client,
        }
    }
}

impl VideosApi {
    /// Generate a new video from a text prompt
    pub async fn create_video(&self, request: &CreateVideoRequest) -> Result<Video> {
        self.client.post("/v1/videos/generations", request).await
    }

    /// Retrieve a video by ID
    pub async fn retrieve_video(&self, video_id: impl AsRef<str>) -> Result<Video> {
        let path = format!("/v1/videos/{}", video_id.as_ref());
        self.client.get(&path).await
    }

    /// List videos with optional pagination parameters
    pub async fn list_videos(&self, params: Option<&ListVideosParams>) -> Result<VideoList> {
        match params {
            Some(p) => {
                let mut query = Vec::new();
                if let Some(l) = p.limit {
                    query.push(("limit".to_string(), l.to_string()));
                }
                if let Some(ref o) = p.order {
                    query.push(("order".to_string(), o.clone()));
                }
                if let Some(ref a) = p.after {
                    query.push(("after".to_string(), a.clone()));
                }
                if let Some(ref b) = p.before {
                    query.push(("before".to_string(), b.clone()));
                }
                self.client.get_with_query("/v1/videos", &query).await
            }
            None => self.client.get("/v1/videos").await,
        }
    }

    /// Delete a video by ID
    pub async fn delete_video(&self, video_id: impl AsRef<str>) -> Result<VideoDeleteResponse> {
        let path = format!("/v1/videos/{}", video_id.as_ref());
        self.client.delete(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_videos_api_creation() {
        let api = VideosApi::new("test-key").unwrap();
        assert_eq!(api.client.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_videos_api_creation_with_base_url() {
        let api = VideosApi::new_with_base_url("test-key", "https://custom.api.com").unwrap();
        assert_eq!(api.client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_videos_api_empty_key_fails() {
        let result = VideosApi::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_video_request_serialization() {
        let req = CreateVideoRequest::new("sora-2", "A cat in space")
            .with_duration(10.0)
            .with_width(1920)
            .with_height(1080)
            .with_n(2);
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["model"], "sora-2");
        assert_eq!(json["prompt"], "A cat in space");
        assert_eq!(json["duration"], 10.0);
        assert_eq!(json["width"], 1920);
        assert_eq!(json["height"], 1080);
        assert_eq!(json["n"], 2);
    }

    #[test]
    fn test_create_video_request_minimal() {
        let req = CreateVideoRequest::new("sora-2", "Hello");
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["model"], "sora-2");
        assert!(json.get("duration").is_none());
        assert!(json.get("width").is_none());
        assert!(json.get("n").is_none());
    }

    #[test]
    fn test_list_videos_params_builder() {
        let params = ListVideosParams::new()
            .with_limit(5)
            .with_order("desc")
            .with_after("video-1")
            .with_before("video-99");
        assert_eq!(params.limit, Some(5));
        assert_eq!(params.order, Some("desc".to_string()));
        assert_eq!(params.after, Some("video-1".to_string()));
        assert_eq!(params.before, Some("video-99".to_string()));
    }

    #[test]
    fn test_list_videos_params_default() {
        let params = ListVideosParams::default();
        assert!(params.limit.is_none());
        assert!(params.order.is_none());
        assert!(params.after.is_none());
        assert!(params.before.is_none());
    }

    #[test]
    fn test_video_deserialization() {
        let json = r#"{
            "id": "video-123",
            "object": "video",
            "created_at": 1700000000,
            "status": "completed",
            "model": "sora-2",
            "prompt": "A sunset",
            "duration": 5.0,
            "width": 1280,
            "height": 720,
            "download_url": "https://example.com/video.mp4"
        }"#;
        let video: Video = serde_json::from_str(json).unwrap();
        assert_eq!(video.id, "video-123");
        assert_eq!(video.status, crate::models::videos::VideoStatus::Completed);
        assert_eq!(
            video.download_url,
            Some("https://example.com/video.mp4".to_string())
        );
        assert!(video.error.is_none());
    }

    #[test]
    fn test_video_failed_deserialization() {
        let json = r#"{
            "id": "video-456",
            "object": "video",
            "created_at": 1700000000,
            "status": "failed",
            "error": {"message": "Policy violation", "code": "content_policy"}
        }"#;
        let video: Video = serde_json::from_str(json).unwrap();
        assert_eq!(video.status, crate::models::videos::VideoStatus::Failed);
        let err = video.error.unwrap();
        assert_eq!(err.message, "Policy violation");
        assert_eq!(err.code, Some("content_policy".to_string()));
    }

    #[test]
    fn test_video_delete_response() {
        let json = r#"{"id": "video-123", "object": "video", "deleted": true}"#;
        let resp: VideoDeleteResponse = serde_json::from_str(json).unwrap();
        assert!(resp.deleted);
    }

    #[test]
    fn test_video_list_deserialization() {
        let json = r#"{"object": "list", "data": [], "has_more": false}"#;
        let list: VideoList = serde_json::from_str(json).unwrap();
        assert!(list.data.is_empty());
        assert!(!list.has_more);
        assert!(list.first_id.is_none());
    }
}
