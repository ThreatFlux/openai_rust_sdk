//! # Videos (Sora) API Models
//!
//! Data structures for the OpenAI Videos API (Sora), which allows you to
//! generate, retrieve, list, and delete AI-generated videos.
//!
//! ## Overview
//!
//! The Videos API supports:
//! - **Generation**: Create videos from text prompts
//! - **Retrieval**: Get details about a specific video
//! - **Listing**: List all generated videos with pagination
//! - **Deletion**: Delete a video by ID
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::videos::CreateVideoRequest;
//!
//! let request = CreateVideoRequest::new("sora-2", "A cat walking on the moon")
//!     .with_duration(10.0)
//!     .with_width(1920)
//!     .with_height(1080);
//! ```

use crate::{De, Ser};

/// Status of a video generation job
#[derive(Debug, Clone, PartialEq, Eq, Ser, De)]
#[serde(rename_all = "snake_case")]
pub enum VideoStatus {
    /// Video generation is queued by the service.
    Queued,
    /// Video generation is currently running.
    InProgress,
    /// Video generation is queued
    Pending,
    /// Video generation is in progress
    Running,
    /// Video generation completed successfully
    Completed,
    /// Video generation failed
    Failed,
}

/// Error details for a failed video generation
#[derive(Debug, Clone, Ser, De)]
pub struct VideoError {
    /// Human-readable error message
    pub message: String,

    /// Machine-readable error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// Represents a video object in OpenAI's system
#[derive(Debug, Clone, Ser, De)]
pub struct Video {
    /// Unique identifier for the video
    pub id: String,

    /// The object type, which is always "video"
    pub object: String,

    /// The Unix timestamp (in seconds) for when the video was created
    pub created_at: u64,

    /// Current status of the video generation
    pub status: VideoStatus,

    /// Approximate completion percentage for the generation task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u32>,

    /// The model used to generate the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The text prompt used to generate the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// Duration of the video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Width of the video in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Height of the video in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    /// Current API resolution string, such as `1280x720`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// Current API duration string, such as `8`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<String>,

    /// Unix timestamp at which rendering completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<u64>,

    /// Unix timestamp at which downloadable assets expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,

    /// Source video ID when this job is a remix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remixed_from_video_id: Option<String>,

    /// URL to download the generated video (available when status is completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,

    /// Error details if video generation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<VideoError>,
}

/// Request structure for generating a video
#[derive(Debug, Clone, Ser, De)]
pub struct CreateVideoRequest {
    /// ID of the model to use (e.g., "sora-2")
    pub model: String,

    /// Text prompt describing the desired video
    pub prompt: String,

    /// Current API clip duration (`4`, `8`, or `12`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<String>,

    /// Current API resolution, for example `720x1280`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// Optional image URL or uploaded file reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_reference: Option<VideoImageReference>,

    /// Desired duration of the video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Desired width of the video in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Desired height of the video in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    /// Number of videos to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
}

impl CreateVideoRequest {
    /// Create a new video generation request
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            seconds: None,
            size: None,
            input_reference: None,
            duration: None,
            width: None,
            height: None,
            n: None,
        }
    }

    /// Set the desired video duration in seconds
    #[must_use]
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set the current API clip duration.
    #[must_use]
    pub fn with_seconds(mut self, seconds: impl Into<String>) -> Self {
        self.seconds = Some(seconds.into());
        self
    }

    /// Set the current API output resolution.
    #[must_use]
    pub fn with_size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set an image URL or uploaded file as a generation reference.
    #[must_use]
    pub fn with_input_reference(mut self, input_reference: VideoImageReference) -> Self {
        self.input_reference = Some(input_reference);
        self
    }

    /// Set the desired video width in pixels
    #[must_use]
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the desired video height in pixels
    #[must_use]
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the number of videos to generate
    #[must_use]
    pub fn with_n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }
}

/// Image URL or uploaded file reference for video generation.
#[derive(Debug, Clone, Ser, De, Default)]
pub struct VideoImageReference {
    /// Fully qualified URL or base64 data URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Uploaded file identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
}

impl VideoImageReference {
    /// Reference an image URL.
    #[must_use]
    pub fn image_url(url: impl Into<String>) -> Self {
        Self {
            image_url: Some(url.into()),
            file_id: None,
        }
    }

    /// Reference an uploaded file.
    #[must_use]
    pub fn file_id(file_id: impl Into<String>) -> Self {
        Self {
            image_url: None,
            file_id: Some(file_id.into()),
        }
    }
}

/// Reference to a completed video for edit and extend operations.
#[derive(Debug, Clone, Ser, De)]
pub struct VideoReference {
    /// Completed video identifier.
    pub id: String,
}

impl VideoReference {
    /// Create a video reference.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

/// JSON request for editing an existing video.
#[derive(Debug, Clone, Ser, De)]
pub struct EditVideoRequest {
    /// Completed source video.
    pub video: VideoReference,
    /// Prompt describing the edit.
    pub prompt: String,
}

/// JSON request for extending an existing video.
#[derive(Debug, Clone, Ser, De)]
pub struct ExtendVideoRequest {
    /// Completed source video.
    pub video: VideoReference,
    /// Prompt describing the extension.
    pub prompt: String,
    /// Extension duration in seconds.
    pub seconds: String,
}

/// JSON request for remixing an existing video.
#[derive(Debug, Clone, Ser, De)]
pub struct RemixVideoRequest {
    /// Prompt describing the remix.
    pub prompt: String,
}

impl EditVideoRequest {
    /// Create an edit request.
    #[must_use]
    pub fn new(video_id: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            video: VideoReference::new(video_id),
            prompt: prompt.into(),
        }
    }
}

impl ExtendVideoRequest {
    /// Create an extension request.
    #[must_use]
    pub fn new(
        video_id: impl Into<String>,
        prompt: impl Into<String>,
        seconds: impl Into<String>,
    ) -> Self {
        Self {
            video: VideoReference::new(video_id),
            prompt: prompt.into(),
            seconds: seconds.into(),
        }
    }
}

impl RemixVideoRequest {
    /// Create a remix request.
    #[must_use]
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
        }
    }
}

/// Response from the list videos API endpoint
#[derive(Debug, Clone, Ser, De)]
pub struct VideoList {
    /// The object type, which is always "list"
    pub object: String,

    /// The list of videos
    pub data: Vec<Video>,

    /// Cursor for the first item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,

    /// Cursor for the last item in the list (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Whether there are more results available
    pub has_more: bool,
}

/// Response from deleting a video
#[derive(Debug, Clone, Ser, De)]
pub struct VideoDeleteResponse {
    /// Unique identifier for the video that was deleted
    pub id: String,

    /// The object type, which is always "video"
    pub object: String,

    /// Whether the video was successfully deleted
    pub deleted: bool,
}

/// Parameters for listing videos
#[derive(Debug, Clone, Default, Ser, De)]
pub struct ListVideosParams {
    /// Maximum number of videos to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort order for the results ("asc" or "desc")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Pagination cursor - list videos after this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination cursor - list videos before this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl ListVideosParams {
    /// Create new parameters with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of videos to return
    #[must_use]
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the sort order
    pub fn with_order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Set the after cursor for pagination
    pub fn with_after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the before cursor for pagination
    pub fn with_before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_video_request() {
        let req = CreateVideoRequest::new("sora-2", "A cat walking on the moon");
        assert_eq!(req.model, "sora-2");
        assert_eq!(req.prompt, "A cat walking on the moon");
        assert!(req.duration.is_none());
        assert!(req.width.is_none());
        assert!(req.height.is_none());
        assert!(req.n.is_none());
    }

    #[test]
    fn test_create_video_request_builder_methods() {
        let req = CreateVideoRequest::new("sora-2", "A sunset over the ocean")
            .with_duration(10.0)
            .with_width(1920)
            .with_height(1080)
            .with_n(2);

        assert_eq!(req.model, "sora-2");
        assert_eq!(req.prompt, "A sunset over the ocean");
        assert_eq!(req.duration, Some(10.0));
        assert_eq!(req.width, Some(1920));
        assert_eq!(req.height, Some(1080));
        assert_eq!(req.n, Some(2));
    }

    #[test]
    fn test_list_videos_params() {
        let params = ListVideosParams::new()
            .with_limit(10)
            .with_order("desc")
            .with_after("video-abc")
            .with_before("video-xyz");

        assert_eq!(params.limit, Some(10));
        assert_eq!(params.order, Some("desc".to_string()));
        assert_eq!(params.after, Some("video-abc".to_string()));
        assert_eq!(params.before, Some("video-xyz".to_string()));
    }

    #[test]
    fn test_video_status_serialization() {
        let pending = serde_json::to_string(&VideoStatus::Pending).unwrap();
        assert_eq!(pending, "\"pending\"");

        let running = serde_json::to_string(&VideoStatus::Running).unwrap();
        assert_eq!(running, "\"running\"");

        let completed = serde_json::to_string(&VideoStatus::Completed).unwrap();
        assert_eq!(completed, "\"completed\"");

        let failed = serde_json::to_string(&VideoStatus::Failed).unwrap();
        assert_eq!(failed, "\"failed\"");
    }

    #[test]
    fn test_video_deserialization() {
        let json = r#"{
            "id": "video-123",
            "object": "video",
            "created_at": 1700000000,
            "status": "completed",
            "model": "sora-2",
            "prompt": "A cat walking on the moon",
            "duration": 10.0,
            "width": 1920,
            "height": 1080,
            "download_url": "https://example.com/video.mp4"
        }"#;

        let video: Video = serde_json::from_str(json).unwrap();
        assert_eq!(video.id, "video-123");
        assert_eq!(video.object, "video");
        assert_eq!(video.created_at, 1_700_000_000);
        assert_eq!(video.status, VideoStatus::Completed);
        assert_eq!(video.model, Some("sora-2".to_string()));
        assert_eq!(video.prompt, Some("A cat walking on the moon".to_string()));
        assert_eq!(video.duration, Some(10.0));
        assert_eq!(video.width, Some(1920));
        assert_eq!(video.height, Some(1080));
        assert_eq!(
            video.download_url,
            Some("https://example.com/video.mp4".to_string())
        );
        assert!(video.error.is_none());
    }

    #[test]
    fn test_video_with_error_deserialization() {
        let json = r#"{
            "id": "video-456",
            "object": "video",
            "created_at": 1700000000,
            "status": "failed",
            "error": {
                "message": "Content policy violation",
                "code": "content_policy"
            }
        }"#;

        let video: Video = serde_json::from_str(json).unwrap();
        assert_eq!(video.status, VideoStatus::Failed);
        let error = video.error.unwrap();
        assert_eq!(error.message, "Content policy violation");
        assert_eq!(error.code, Some("content_policy".to_string()));
    }

    #[test]
    fn test_video_delete_response() {
        let json = r#"{
            "id": "video-123",
            "object": "video",
            "deleted": true
        }"#;

        let response: VideoDeleteResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "video-123");
        assert_eq!(response.object, "video");
        assert!(response.deleted);
    }

    #[test]
    fn test_video_list_deserialization() {
        let json = r#"{
            "object": "list",
            "data": [],
            "first_id": null,
            "last_id": null,
            "has_more": false
        }"#;

        let list: VideoList = serde_json::from_str(json).unwrap();
        assert_eq!(list.object, "list");
        assert!(list.data.is_empty());
        assert!(!list.has_more);
    }

    #[test]
    fn current_video_request_and_operation_models() {
        let request = CreateVideoRequest::new("sora-2", "A fox in snow")
            .with_seconds("8")
            .with_size("1280x720")
            .with_input_reference(VideoImageReference::image_url(
                "https://example.test/fox.png",
            ));
        let request_json = serde_json::to_value(&request).expect("request JSON");
        assert_eq!(request_json["seconds"], "8");
        assert_eq!(request_json["size"], "1280x720");
        assert_eq!(
            request_json["input_reference"]["image_url"],
            "https://example.test/fox.png"
        );

        let file_reference = VideoImageReference::file_id("file_123");
        assert_eq!(file_reference.file_id.as_deref(), Some("file_123"));
        assert_eq!(VideoReference::new("video_123").id, "video_123");

        let edit = EditVideoRequest::new("video_123", "Make it brighter");
        assert_eq!(edit.video.id, "video_123");
        assert_eq!(edit.prompt, "Make it brighter");

        let extend = ExtendVideoRequest::new("video_123", "Continue the scene", "4");
        assert_eq!(extend.seconds, "4");

        let remix = RemixVideoRequest::new("Use a watercolor style");
        assert_eq!(remix.prompt, "Use a watercolor style");
    }

    #[test]
    fn current_video_response_fields_round_trip() {
        let video: Video = serde_json::from_value(serde_json::json!({
            "id": "video_123",
            "object": "video",
            "created_at": 1_800_000_000,
            "status": "in_progress",
            "progress": 42,
            "seconds": "8",
            "size": "1280x720",
            "completed_at": null,
            "expires_at": 1_800_100_000,
            "remixed_from_video_id": "video_original"
        }))
        .expect("video JSON");
        assert_eq!(video.status, VideoStatus::InProgress);
        assert_eq!(video.progress, Some(42));
        assert_eq!(video.seconds.as_deref(), Some("8"));
        assert_eq!(video.size.as_deref(), Some("1280x720"));
        assert_eq!(
            video.remixed_from_video_id.as_deref(),
            Some("video_original")
        );
    }
}
