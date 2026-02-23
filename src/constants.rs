//! Constants used throughout the OpenAI API SDK
//!
//! This module centralizes all constant values used across the SDK to reduce duplication
//! and make maintenance easier.

/// Default `OpenAI` API base URL
pub const OPENAI_API_BASE_URL: &str = "https://api.openai.com";

/// API version path prefix
pub const API_VERSION_PREFIX: &str = "/v1";

/// Content type for JSON requests
pub const CONTENT_TYPE_JSON: &str = "application/json";

/// `OpenAI` Beta header value for assistants API
pub const OPENAI_BETA_ASSISTANTS: &str = "assistants=v1";

/// Common HTTP header names
pub mod headers {
    /// Authorization header name
    pub const AUTHORIZATION: &str = "Authorization";

    /// Content-Type header name
    pub const CONTENT_TYPE: &str = "Content-Type";

    /// OpenAI-Beta header name
    pub const OPENAI_BETA: &str = "OpenAI-Beta";

    /// User-Agent header name
    pub const USER_AGENT: &str = "User-Agent";
}

/// Common API endpoints
pub mod endpoints {
    /// Assistants API endpoints
    pub mod assistants {
        /// Base assistants endpoint
        pub const BASE: &str = "/v1/assistants";

        /// Get specific assistant by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }
    }

    /// Vector stores API endpoints
    pub mod vector_stores {
        /// Base vector stores endpoint
        pub const BASE: &str = "/v1/vector_stores";

        /// Get specific vector store by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Vector store files endpoint
        #[must_use]
        pub fn files(vector_store_id: &str) -> String {
            format!("{BASE}/{vector_store_id}/files")
        }

        /// Specific vector store file by ID
        #[must_use]
        pub fn file_by_id(vector_store_id: &str, file_id: &str) -> String {
            format!("{BASE}/{vector_store_id}/files/{file_id}")
        }

        /// Vector store file batches endpoint
        #[must_use]
        pub fn file_batches(vector_store_id: &str) -> String {
            format!("{BASE}/{vector_store_id}/file_batches")
        }

        /// Specific vector store file batch by ID
        #[must_use]
        pub fn file_batch_by_id(vector_store_id: &str, batch_id: &str) -> String {
            format!("{BASE}/{vector_store_id}/file_batches/{batch_id}")
        }

        /// Cancel vector store file batch
        #[must_use]
        pub fn cancel_file_batch(vector_store_id: &str, batch_id: &str) -> String {
            format!("{BASE}/{vector_store_id}/file_batches/{batch_id}/cancel")
        }

        /// Vector store file batch files endpoint
        #[must_use]
        pub fn file_batch_files(vector_store_id: &str, batch_id: &str) -> String {
            format!("{BASE}/{vector_store_id}/file_batches/{batch_id}/files")
        }
    }

    /// Threads API endpoints
    pub mod threads {
        /// Base threads endpoint
        pub const BASE: &str = "/v1/threads";

        /// Get specific thread by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Thread messages endpoint
        #[must_use]
        pub fn messages(thread_id: &str) -> String {
            format!("{BASE}/{thread_id}/messages")
        }

        /// Specific message in thread
        #[must_use]
        pub fn message_by_id(thread_id: &str, message_id: &str) -> String {
            format!("{BASE}/{thread_id}/messages/{message_id}")
        }

        /// Thread message files endpoint
        #[must_use]
        pub fn message_files(thread_id: &str, message_id: &str) -> String {
            format!("{BASE}/{thread_id}/messages/{message_id}/files")
        }

        /// Specific message file by ID
        #[must_use]
        pub fn message_file_by_id(thread_id: &str, message_id: &str, file_id: &str) -> String {
            format!("{BASE}/{thread_id}/messages/{message_id}/files/{file_id}")
        }

        /// Thread runs endpoint
        #[must_use]
        pub fn runs(thread_id: &str) -> String {
            format!("{BASE}/{thread_id}/runs")
        }

        /// Specific run in thread
        #[must_use]
        pub fn run_by_id(thread_id: &str, run_id: &str) -> String {
            format!("{BASE}/{thread_id}/runs/{run_id}")
        }

        /// Submit tool outputs to run
        #[must_use]
        pub fn submit_tool_outputs(thread_id: &str, run_id: &str) -> String {
            format!("{BASE}/{thread_id}/runs/{run_id}/submit_tool_outputs")
        }

        /// Cancel run
        #[must_use]
        pub fn cancel_run(thread_id: &str, run_id: &str) -> String {
            format!("{BASE}/{thread_id}/runs/{run_id}/cancel")
        }

        /// Run steps endpoint
        #[must_use]
        pub fn run_steps(thread_id: &str, run_id: &str) -> String {
            format!("{BASE}/{thread_id}/runs/{run_id}/steps")
        }

        /// Specific run step by ID
        #[must_use]
        pub fn run_step_by_id(thread_id: &str, run_id: &str, step_id: &str) -> String {
            format!("{BASE}/{thread_id}/runs/{run_id}/steps/{step_id}")
        }
    }

    /// Runs API endpoints (for cross-thread operations)
    pub mod runs {
        /// Base runs endpoint for creating runs across threads
        pub const BASE: &str = "/v1/threads/runs";
    }

    /// Files API endpoints
    pub mod files {
        /// Base files endpoint
        pub const BASE: &str = "/v1/files";

        /// Get specific file by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Download file content
        #[must_use]
        pub fn content(id: &str) -> String {
            format!("{BASE}/{id}/content")
        }
    }

    /// Models API endpoints
    pub mod models {
        /// Base models endpoint
        pub const BASE: &str = "/v1/models";

        /// Get specific model by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }
    }

    /// Chat completions endpoint
    pub const CHAT_COMPLETIONS: &str = "/v1/chat/completions";

    /// Completions endpoint
    pub const COMPLETIONS: &str = "/v1/completions";

    /// Embeddings endpoint
    pub const EMBEDDINGS: &str = "/v1/embeddings";

    /// Fine-tuning jobs endpoint
    pub const FINE_TUNING_JOBS: &str = "/v1/fine_tuning/jobs";

    /// Images generations endpoint
    pub const IMAGES_GENERATIONS: &str = "/v1/images/generations";

    /// Images edits endpoint
    pub const IMAGES_EDITS: &str = "/v1/images/edits";

    /// Images variations endpoint
    pub const IMAGES_VARIATIONS: &str = "/v1/images/variations";

    /// Audio speech endpoint
    pub const AUDIO_SPEECH: &str = "/v1/audio/speech";

    /// Audio transcriptions endpoint
    pub const AUDIO_TRANSCRIPTIONS: &str = "/v1/audio/transcriptions";

    /// Audio translations endpoint
    pub const AUDIO_TRANSLATIONS: &str = "/v1/audio/translations";

    /// Moderations endpoint
    pub const MODERATIONS: &str = "/v1/moderations";

    /// Batch jobs endpoint
    pub const BATCHES: &str = "/v1/batches";

    /// Containers API endpoints
    pub mod containers {
        /// Base containers endpoint
        pub const BASE: &str = "/v1/containers";

        /// Get specific container by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Container files endpoint
        #[must_use]
        pub fn files(container_id: &str) -> String {
            format!("{BASE}/{container_id}/files")
        }

        /// Specific container file by ID
        #[must_use]
        pub fn file_by_id(container_id: &str, file_id: &str) -> String {
            format!("{BASE}/{container_id}/files/{file_id}")
        }

        /// Container file content endpoint
        #[must_use]
        pub fn file_content(container_id: &str, file_id: &str) -> String {
            format!("{BASE}/{container_id}/files/{file_id}/content")
        }

        /// Container execute endpoint
        #[must_use]
        pub fn execute(container_id: &str) -> String {
            format!("{BASE}/{container_id}/execute")
        }

        /// Container keep-alive endpoint
        #[must_use]
        pub fn keep_alive(container_id: &str) -> String {
            format!("{BASE}/{container_id}/keep-alive")
        }
    }

    /// Fine-tuning API endpoints
    pub mod fine_tuning {
        /// Base fine-tuning jobs endpoint
        pub const BASE: &str = "/v1/fine_tuning/jobs";

        /// Get specific fine-tuning job by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Cancel fine-tuning job
        #[must_use]
        pub fn cancel(id: &str) -> String {
            format!("{BASE}/{id}/cancel")
        }

        /// Fine-tuning job events endpoint
        #[must_use]
        pub fn events(id: &str) -> String {
            format!("{BASE}/{id}/events")
        }

        /// Fine-tuning job checkpoints endpoint
        #[must_use]
        pub fn checkpoints(id: &str) -> String {
            format!("{BASE}/{id}/checkpoints")
        }
    }

    /// Uploads API endpoints
    pub mod uploads {
        /// Base uploads endpoint
        pub const BASE: &str = "/v1/uploads";

        /// Get specific upload by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Upload parts endpoint
        #[must_use]
        pub fn parts(upload_id: &str) -> String {
            format!("{BASE}/{upload_id}/parts")
        }

        /// Complete upload endpoint
        #[must_use]
        pub fn complete(upload_id: &str) -> String {
            format!("{BASE}/{upload_id}/complete")
        }

        /// Cancel upload endpoint
        #[must_use]
        pub fn cancel(upload_id: &str) -> String {
            format!("{BASE}/{upload_id}/cancel")
        }
    }

    /// Evals API endpoints
    pub mod evals {
        /// Base evals endpoint
        pub const BASE: &str = "/v1/evals";

        /// Get specific eval by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Eval runs endpoint
        #[must_use]
        pub fn runs(eval_id: &str) -> String {
            format!("{BASE}/{eval_id}/runs")
        }

        /// Specific eval run by ID
        #[must_use]
        pub fn run_by_id(eval_id: &str, run_id: &str) -> String {
            format!("{BASE}/{eval_id}/runs/{run_id}")
        }

        /// Cancel eval run
        #[must_use]
        pub fn cancel_run(eval_id: &str, run_id: &str) -> String {
            format!("{BASE}/{eval_id}/runs/{run_id}/cancel")
        }

        /// Eval run output items endpoint
        #[must_use]
        pub fn output_items(eval_id: &str, run_id: &str) -> String {
            format!("{BASE}/{eval_id}/runs/{run_id}/output_items")
        }

        /// Specific eval run output item by ID
        #[must_use]
        pub fn output_item_by_id(eval_id: &str, run_id: &str, item_id: &str) -> String {
            format!("{BASE}/{eval_id}/runs/{run_id}/output_items/{item_id}")
        }
    }

    /// Videos (Sora) API endpoints
    pub mod videos {
        /// Base videos endpoint
        pub const BASE: &str = "/v1/videos";

        /// Video generations endpoint
        pub const GENERATIONS: &str = "/v1/videos/generations";

        /// Get specific video by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }
    }

    /// Conversations API endpoints
    pub mod conversations {
        /// Base conversations endpoint
        pub const BASE: &str = "/v1/conversations";

        /// Get specific conversation by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Conversation items endpoint
        #[must_use]
        pub fn items(conversation_id: &str) -> String {
            format!("{BASE}/{conversation_id}/items")
        }

        /// Specific conversation item by ID
        #[must_use]
        pub fn item_by_id(conversation_id: &str, item_id: &str) -> String {
            format!("{BASE}/{conversation_id}/items/{item_id}")
        }
    }

    /// Skills API endpoints
    pub mod skills {
        /// Base skills endpoint
        pub const BASE: &str = "/v1/skills";

        /// Get specific skill by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Skill content endpoint
        #[must_use]
        pub fn content(skill_id: &str) -> String {
            format!("{BASE}/{skill_id}/content")
        }

        /// Skill versions endpoint
        #[must_use]
        pub fn versions(skill_id: &str) -> String {
            format!("{BASE}/{skill_id}/versions")
        }

        /// Specific skill version by ID
        #[must_use]
        pub fn version_by_id(skill_id: &str, version_id: &str) -> String {
            format!("{BASE}/{skill_id}/versions/{version_id}")
        }

        /// Skill version content endpoint
        #[must_use]
        pub fn version_content(skill_id: &str, version_id: &str) -> String {
            format!("{BASE}/{skill_id}/versions/{version_id}/content")
        }
    }

    /// Administration API endpoints
    pub mod admin {
        /// Audit logs endpoint
        pub const AUDIT_LOGS: &str = "/v1/organization/audit_logs";

        /// Invites endpoint
        pub const INVITES: &str = "/v1/organization/invites";

        /// Users endpoint
        pub const USERS: &str = "/v1/organization/users";

        /// Projects endpoint
        pub const PROJECTS: &str = "/v1/organization/projects";

        /// Costs endpoint
        pub const COSTS: &str = "/v1/organization/costs";

        /// Get specific invite by ID
        #[must_use]
        pub fn invite_by_id(id: &str) -> String {
            format!("{INVITES}/{id}")
        }

        /// Get specific user by ID
        #[must_use]
        pub fn user_by_id(id: &str) -> String {
            format!("{USERS}/{id}")
        }

        /// Get specific project by ID
        #[must_use]
        pub fn project_by_id(id: &str) -> String {
            format!("{PROJECTS}/{id}")
        }

        /// Archive project endpoint
        #[must_use]
        pub fn archive_project(id: &str) -> String {
            format!("{PROJECTS}/{id}/archive")
        }

        /// Project users endpoint
        #[must_use]
        pub fn project_users(project_id: &str) -> String {
            format!("{PROJECTS}/{project_id}/users")
        }

        /// Specific project user by ID
        #[must_use]
        pub fn project_user_by_id(project_id: &str, user_id: &str) -> String {
            format!("{PROJECTS}/{project_id}/users/{user_id}")
        }

        /// Project service accounts endpoint
        #[must_use]
        pub fn project_service_accounts(project_id: &str) -> String {
            format!("{PROJECTS}/{project_id}/service_accounts")
        }

        /// Specific project service account by ID
        #[must_use]
        pub fn project_service_account_by_id(project_id: &str, service_account_id: &str) -> String {
            format!("{PROJECTS}/{project_id}/service_accounts/{service_account_id}")
        }

        /// Project API keys endpoint
        #[must_use]
        pub fn project_api_keys(project_id: &str) -> String {
            format!("{PROJECTS}/{project_id}/api_keys")
        }

        /// Specific project API key by ID
        #[must_use]
        pub fn project_api_key_by_id(project_id: &str, key_id: &str) -> String {
            format!("{PROJECTS}/{project_id}/api_keys/{key_id}")
        }

        /// Project rate limits endpoint
        #[must_use]
        pub fn project_rate_limits(project_id: &str) -> String {
            format!("{PROJECTS}/{project_id}/rate_limits")
        }

        /// Specific project rate limit by ID
        #[must_use]
        pub fn project_rate_limit_by_id(project_id: &str, rate_limit_id: &str) -> String {
            format!("{PROJECTS}/{project_id}/rate_limits/{rate_limit_id}")
        }

        /// Usage endpoint for a specific category
        #[must_use]
        pub fn usage(category: &str) -> String {
            format!("/v1/organization/usage/{category}")
        }
    }

    /// Batches API endpoints
    pub mod batches {
        /// Base batches endpoint
        pub const BASE: &str = "/v1/batches";

        /// Get specific batch by ID
        #[must_use]
        pub fn by_id(id: &str) -> String {
            format!("{BASE}/{id}")
        }

        /// Cancel batch
        #[must_use]
        pub fn cancel(id: &str) -> String {
            format!("{BASE}/{id}/cancel")
        }
    }
}

/// Common error messages
pub mod error_messages {
    /// Invalid API key format
    pub const INVALID_API_KEY: &str = "Invalid API key format";

    /// Request failed
    pub const REQUEST_FAILED: &str = "Request failed";

    /// Parse error
    pub const PARSE_ERROR: &str = "Failed to parse response";

    /// Timeout error
    pub const TIMEOUT: &str = "Request timed out";

    /// Authentication failed
    pub const AUTH_FAILED: &str = "Authentication failed";
}

/// Common query parameter names
pub mod query_params {
    /// Limit parameter for pagination
    pub const LIMIT: &str = "limit";

    /// Order parameter for sorting
    pub const ORDER: &str = "order";

    /// After parameter for cursor pagination
    pub const AFTER: &str = "after";

    /// Before parameter for cursor pagination
    pub const BEFORE: &str = "before";

    /// Filter parameter
    pub const FILTER: &str = "filter";
}

/// SDK information
pub mod sdk {
    /// SDK name
    pub const NAME: &str = "openai_rust_sdk";

    /// SDK version
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    /// User agent string for HTTP requests
    pub const USER_AGENT: &str =
        const_format::formatcp!("openai_rust_sdk/{}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_are_valid() {
        assert_eq!(OPENAI_API_BASE_URL, "https://api.openai.com");
        assert_eq!(API_VERSION_PREFIX, "/v1");
        assert_eq!(CONTENT_TYPE_JSON, "application/json");
        assert_eq!(OPENAI_BETA_ASSISTANTS, "assistants=v1");
    }

    #[test]
    fn test_endpoint_construction() {
        test_assistants_endpoints();
        test_vector_stores_endpoints();
        test_threads_endpoints();
        test_files_endpoints();
        test_models_endpoints();
    }

    fn test_assistants_endpoints() {
        assert_eq!(endpoints::assistants::BASE, "/v1/assistants");
        assert_eq!(endpoints::assistants::by_id("test"), "/v1/assistants/test");
    }

    fn test_vector_stores_endpoints() {
        assert_eq!(endpoints::vector_stores::BASE, "/v1/vector_stores");
        assert_eq!(
            endpoints::vector_stores::by_id("vs-123"),
            "/v1/vector_stores/vs-123"
        );
        assert_eq!(
            endpoints::vector_stores::files("vs-123"),
            "/v1/vector_stores/vs-123/files"
        );
        assert_eq!(
            endpoints::vector_stores::file_by_id("vs-123", "file-456"),
            "/v1/vector_stores/vs-123/files/file-456"
        );
        assert_eq!(
            endpoints::vector_stores::file_batches("vs-123"),
            "/v1/vector_stores/vs-123/file_batches"
        );
        assert_eq!(
            endpoints::vector_stores::file_batch_by_id("vs-123", "batch-789"),
            "/v1/vector_stores/vs-123/file_batches/batch-789"
        );
        assert_eq!(
            endpoints::vector_stores::cancel_file_batch("vs-123", "batch-789"),
            "/v1/vector_stores/vs-123/file_batches/batch-789/cancel"
        );
        assert_eq!(
            endpoints::vector_stores::file_batch_files("vs-123", "batch-789"),
            "/v1/vector_stores/vs-123/file_batches/batch-789/files"
        );
    }

    fn test_threads_endpoints() {
        assert_eq!(endpoints::threads::BASE, "/v1/threads");
        assert_eq!(
            endpoints::threads::by_id("thread-123"),
            "/v1/threads/thread-123"
        );
        assert_eq!(
            endpoints::threads::messages("thread-123"),
            "/v1/threads/thread-123/messages"
        );
        assert_eq!(
            endpoints::threads::message_by_id("thread-123", "msg-456"),
            "/v1/threads/thread-123/messages/msg-456"
        );
        assert_eq!(
            endpoints::threads::message_files("thread-123", "msg-456"),
            "/v1/threads/thread-123/messages/msg-456/files"
        );
        assert_eq!(
            endpoints::threads::message_file_by_id("thread-123", "msg-456", "file-789"),
            "/v1/threads/thread-123/messages/msg-456/files/file-789"
        );
        assert_eq!(
            endpoints::threads::runs("thread-123"),
            "/v1/threads/thread-123/runs"
        );
        assert_eq!(
            endpoints::threads::run_by_id("thread-123", "run-456"),
            "/v1/threads/thread-123/runs/run-456"
        );
        assert_eq!(
            endpoints::threads::submit_tool_outputs("thread-123", "run-456"),
            "/v1/threads/thread-123/runs/run-456/submit_tool_outputs"
        );
        assert_eq!(
            endpoints::threads::cancel_run("thread-123", "run-456"),
            "/v1/threads/thread-123/runs/run-456/cancel"
        );
        assert_eq!(
            endpoints::threads::run_steps("thread-123", "run-456"),
            "/v1/threads/thread-123/runs/run-456/steps"
        );
        assert_eq!(
            endpoints::threads::run_step_by_id("thread-123", "run-456", "step-789"),
            "/v1/threads/thread-123/runs/run-456/steps/step-789"
        );
    }

    fn test_files_endpoints() {
        assert_eq!(endpoints::files::BASE, "/v1/files");
        assert_eq!(endpoints::files::by_id("file-123"), "/v1/files/file-123");
        assert_eq!(
            endpoints::files::content("file-123"),
            "/v1/files/file-123/content"
        );
    }

    fn test_models_endpoints() {
        assert_eq!(endpoints::models::BASE, "/v1/models");
        assert_eq!(endpoints::models::by_id("gpt-4"), "/v1/models/gpt-4");
    }

    #[test]
    fn test_new_endpoint_constructions() {
        test_containers_endpoints();
        test_fine_tuning_endpoints();
        test_batches_endpoints();
        test_runs_endpoints();
    }

    fn test_containers_endpoints() {
        let container_id = "container-123";
        let file_id = "file-456";

        assert_eq!(endpoints::containers::BASE, "/v1/containers");
        assert_eq!(
            endpoints::containers::by_id(container_id),
            "/v1/containers/container-123"
        );
        assert_eq!(
            endpoints::containers::files(container_id),
            "/v1/containers/container-123/files"
        );
        assert_eq!(
            endpoints::containers::file_by_id(container_id, file_id),
            "/v1/containers/container-123/files/file-456"
        );
        assert_eq!(
            endpoints::containers::file_content(container_id, file_id),
            "/v1/containers/container-123/files/file-456/content"
        );
        assert_eq!(
            endpoints::containers::execute(container_id),
            "/v1/containers/container-123/execute"
        );
        assert_eq!(
            endpoints::containers::keep_alive(container_id),
            "/v1/containers/container-123/keep-alive"
        );
    }

    fn test_fine_tuning_endpoints() {
        let job_id = "ft-123";

        assert_eq!(endpoints::fine_tuning::BASE, "/v1/fine_tuning/jobs");
        assert_eq!(
            endpoints::fine_tuning::by_id(job_id),
            "/v1/fine_tuning/jobs/ft-123"
        );
        assert_eq!(
            endpoints::fine_tuning::cancel(job_id),
            "/v1/fine_tuning/jobs/ft-123/cancel"
        );
        assert_eq!(
            endpoints::fine_tuning::events(job_id),
            "/v1/fine_tuning/jobs/ft-123/events"
        );
        assert_eq!(
            endpoints::fine_tuning::checkpoints(job_id),
            "/v1/fine_tuning/jobs/ft-123/checkpoints"
        );
    }

    fn test_batches_endpoints() {
        let batch_id = "batch-123";

        assert_eq!(endpoints::batches::BASE, "/v1/batches");
        assert_eq!(endpoints::batches::by_id(batch_id), "/v1/batches/batch-123");
        assert_eq!(
            endpoints::batches::cancel(batch_id),
            "/v1/batches/batch-123/cancel"
        );
    }

    fn test_runs_endpoints() {
        assert_eq!(endpoints::runs::BASE, "/v1/threads/runs");
    }

    #[test]
    fn test_new_api_endpoint_constructions() {
        test_uploads_endpoints();
        test_evals_endpoints();
        test_videos_endpoints();
        test_conversations_endpoints();
        test_skills_endpoints();
        test_admin_endpoints();
    }

    fn test_uploads_endpoints() {
        assert_eq!(endpoints::uploads::BASE, "/v1/uploads");
        assert_eq!(endpoints::uploads::by_id("up-123"), "/v1/uploads/up-123");
        assert_eq!(
            endpoints::uploads::parts("up-123"),
            "/v1/uploads/up-123/parts"
        );
        assert_eq!(
            endpoints::uploads::complete("up-123"),
            "/v1/uploads/up-123/complete"
        );
        assert_eq!(
            endpoints::uploads::cancel("up-123"),
            "/v1/uploads/up-123/cancel"
        );
    }

    fn test_evals_endpoints() {
        assert_eq!(endpoints::evals::BASE, "/v1/evals");
        assert_eq!(endpoints::evals::by_id("eval-1"), "/v1/evals/eval-1");
        assert_eq!(endpoints::evals::runs("eval-1"), "/v1/evals/eval-1/runs");
        assert_eq!(
            endpoints::evals::run_by_id("eval-1", "run-2"),
            "/v1/evals/eval-1/runs/run-2"
        );
        assert_eq!(
            endpoints::evals::cancel_run("eval-1", "run-2"),
            "/v1/evals/eval-1/runs/run-2/cancel"
        );
        assert_eq!(
            endpoints::evals::output_items("eval-1", "run-2"),
            "/v1/evals/eval-1/runs/run-2/output_items"
        );
        assert_eq!(
            endpoints::evals::output_item_by_id("eval-1", "run-2", "item-3"),
            "/v1/evals/eval-1/runs/run-2/output_items/item-3"
        );
    }

    fn test_videos_endpoints() {
        assert_eq!(endpoints::videos::BASE, "/v1/videos");
        assert_eq!(endpoints::videos::GENERATIONS, "/v1/videos/generations");
        assert_eq!(endpoints::videos::by_id("vid-1"), "/v1/videos/vid-1");
    }

    fn test_conversations_endpoints() {
        assert_eq!(endpoints::conversations::BASE, "/v1/conversations");
        assert_eq!(
            endpoints::conversations::by_id("conv-1"),
            "/v1/conversations/conv-1"
        );
        assert_eq!(
            endpoints::conversations::items("conv-1"),
            "/v1/conversations/conv-1/items"
        );
        assert_eq!(
            endpoints::conversations::item_by_id("conv-1", "item-2"),
            "/v1/conversations/conv-1/items/item-2"
        );
    }

    fn test_skills_endpoints() {
        assert_eq!(endpoints::skills::BASE, "/v1/skills");
        assert_eq!(endpoints::skills::by_id("sk-1"), "/v1/skills/sk-1");
        assert_eq!(
            endpoints::skills::content("sk-1"),
            "/v1/skills/sk-1/content"
        );
        assert_eq!(
            endpoints::skills::versions("sk-1"),
            "/v1/skills/sk-1/versions"
        );
        assert_eq!(
            endpoints::skills::version_by_id("sk-1", "ver-2"),
            "/v1/skills/sk-1/versions/ver-2"
        );
        assert_eq!(
            endpoints::skills::version_content("sk-1", "ver-2"),
            "/v1/skills/sk-1/versions/ver-2/content"
        );
    }

    fn test_admin_endpoints() {
        assert_eq!(endpoints::admin::AUDIT_LOGS, "/v1/organization/audit_logs");
        assert_eq!(endpoints::admin::INVITES, "/v1/organization/invites");
        assert_eq!(endpoints::admin::USERS, "/v1/organization/users");
        assert_eq!(endpoints::admin::PROJECTS, "/v1/organization/projects");
        assert_eq!(endpoints::admin::COSTS, "/v1/organization/costs");
        assert_eq!(
            endpoints::admin::invite_by_id("inv-1"),
            "/v1/organization/invites/inv-1"
        );
        assert_eq!(
            endpoints::admin::user_by_id("u-1"),
            "/v1/organization/users/u-1"
        );
        assert_eq!(
            endpoints::admin::project_by_id("p-1"),
            "/v1/organization/projects/p-1"
        );
        assert_eq!(
            endpoints::admin::archive_project("p-1"),
            "/v1/organization/projects/p-1/archive"
        );
        assert_eq!(
            endpoints::admin::project_users("p-1"),
            "/v1/organization/projects/p-1/users"
        );
        assert_eq!(
            endpoints::admin::project_user_by_id("p-1", "u-1"),
            "/v1/organization/projects/p-1/users/u-1"
        );
        assert_eq!(
            endpoints::admin::project_service_accounts("p-1"),
            "/v1/organization/projects/p-1/service_accounts"
        );
        assert_eq!(
            endpoints::admin::project_service_account_by_id("p-1", "sa-1"),
            "/v1/organization/projects/p-1/service_accounts/sa-1"
        );
        assert_eq!(
            endpoints::admin::project_api_keys("p-1"),
            "/v1/organization/projects/p-1/api_keys"
        );
        assert_eq!(
            endpoints::admin::project_api_key_by_id("p-1", "key-1"),
            "/v1/organization/projects/p-1/api_keys/key-1"
        );
        assert_eq!(
            endpoints::admin::project_rate_limits("p-1"),
            "/v1/organization/projects/p-1/rate_limits"
        );
        assert_eq!(
            endpoints::admin::project_rate_limit_by_id("p-1", "rl-1"),
            "/v1/organization/projects/p-1/rate_limits/rl-1"
        );
        assert_eq!(
            endpoints::admin::usage("completions"),
            "/v1/organization/usage/completions"
        );
    }

    #[test]
    fn test_sdk_constants() {
        assert_eq!(sdk::NAME, "openai_rust_sdk");
        // VERSION is a compile-time constant from CARGO_PKG_VERSION, verify it follows semver pattern
        assert!(sdk::VERSION.chars().any(|c| c.is_ascii_digit()));
        assert!(sdk::USER_AGENT.starts_with("openai_rust_sdk/"));
    }
}
