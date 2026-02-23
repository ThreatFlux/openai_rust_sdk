/// Common utilities for models
pub mod common;
/// Common builder patterns to reduce duplication
pub mod common_builder;
/// Shared traits and implementations to eliminate model duplication
pub mod shared_traits;

/// Administration API models for managing users, invites, projects, and more
pub mod admin;
/// Assistants models for AI assistant creation and management
pub mod assistants;
/// Audio models for text-to-speech, transcription, and translation
pub mod audio;
/// Container management models for Code Interpreter
pub mod containers;
/// Conversations API models for conversation and item management
pub mod conversations;
/// Embeddings models for vector representations
pub mod embeddings;
/// Evals models for evaluation management and runs
pub mod evals;
/// Files models for file upload, management, and retrieval
pub mod files;
/// Fine-tuning models for custom model training and management
pub mod fine_tuning;
/// Function calling models and data structures
pub mod functions;
/// GPT-5 specific models and configuration
pub mod gpt5;
/// Images models for DALL-E image generation, editing, and variations
pub mod images;
/// Models API for listing and retrieving model information
#[allow(clippy::module_inception)]
pub mod models;
/// Moderations models for content policy classification
pub mod moderations;
/// Modular moderations models for content policy classification
pub mod moderations_modular;
/// Real-time audio models for WebRTC streaming
pub mod realtime_audio;
/// Response models and data structures
pub mod responses;
/// Modern Responses API data structures
pub mod responses_v2;
/// Runs models for assistant execution and run steps management
pub mod runs;
/// Skills API models for reusable skill management and versioning
pub mod skills;
/// Threads models for conversation thread and message management
pub mod threads;
/// Comprehensive tools support (web search, file search, MCP, etc.)
pub mod tools;
/// Uploads models for multipart large file uploads
pub mod uploads;
/// Vector stores models for RAG and knowledge management
pub mod vector_stores;
/// Videos (Sora) models for AI video generation
pub mod videos;

// Re-export commonly used types while avoiding conflicts
// For conflicting types, users should import from specific modules

// Admin API
pub use admin::{
    AuditLog, AuditLogList, CreateInviteRequest, CreateProjectRequest,
    CreateProjectServiceAccountRequest, CreateProjectUserRequest, Invite, InviteDeleteResponse,
    InviteList, ListAdminParams, ListAuditLogsParams, Project, ProjectApiKey,
    ProjectApiKeyDeleteResponse, ProjectApiKeyList, ProjectList, ProjectRateLimit,
    ProjectRateLimitList, ProjectServiceAccount, ProjectServiceAccountCreateResponse,
    ProjectServiceAccountDeleteResponse, ProjectServiceAccountList, ProjectUser,
    ProjectUserDeleteResponse, ProjectUserList, UpdateProjectRateLimitRequest,
    UpdateProjectRequest, UpdateProjectUserRequest, UpdateUserRequest, UsageBucket, UsageResponse,
    User as AdminUser, UserDeleteResponse, UserList,
};

// Assistants API
pub use assistants::{
    Assistant, AssistantRequest, AssistantTool, ListAssistantsParams, ListAssistantsResponse,
};

// Audio API - explicit exports to avoid conflicts
pub use audio::builders as audio_builders;
pub use audio::requests as audio_requests;
pub use audio::types as audio_types;
pub use audio::{
    AudioFormat, AudioModels, AudioSpeechRequest, AudioSpeechResponse, AudioTranscriptionRequest,
    AudioTranscriptionResponse, AudioTranslationRequest, AudioTranslationResponse,
    TimestampGranularity, TranscriptionFormat, Voice,
};

// Containers API
pub use containers::*;

// Conversations API
pub use conversations::{
    Conversation, ConversationDeleteResponse, ConversationItem, ConversationItemList,
    CreateConversationItemRequest, CreateConversationRequest, ListConversationItemsParams,
    UpdateConversationRequest,
};

// Embeddings API
pub use embeddings::*;

// Evals API
pub use evals::{
    CreateEvalRequest, CreateEvalRunRequest, Eval, EvalDeleteResponse, EvalList, EvalRun,
    EvalRunList, EvalRunOutputItem, EvalRunOutputItemList, ListEvalRunOutputItemsParams,
    ListEvalRunsParams, ListEvalsParams, UpdateEvalRequest,
};

// Files API
pub use files::{
    File, FileDeleteResponse, FilePurpose, FileStatus, FileUploadRequest, ListFilesParams,
    ListFilesResponse, SortOrder as FileSortOrder,
};

// Fine-tuning API
pub use fine_tuning::*;

// Functions API
pub use functions::{
    AllowedToolSelection, CustomTool, FunctionCall as FunctionCallType, FunctionCallOutput,
    FunctionTool as FunctionToolType, FunctionToolSelection, Grammar, Tool,
    ToolChoice as FunctionToolChoice,
};

// GPT-5 API
pub use gpt5::*;

// Images API - explicit exports to avoid conflicts
pub use images::builders as image_builders;
pub use images::requests as image_requests;
pub use images::types as image_types;
pub use images::{
    ImageData, ImageEditRequest, ImageGenerationRequest, ImageModels, ImageQuality, ImageResponse,
    ImageResponseFormat, ImageSize, ImageStyle, ImageVariationRequest,
};

// Models API - explicit exports to avoid conflicts
pub use models::implementations as model_implementations;
pub use models::{
    ListModelsResponse, Model, ModelCapabilities, ModelPermission, ModelRequirements,
};

// Moderations API - explicit exports to avoid conflicts
pub use moderations::builders as moderation_builders;
pub use moderations::types as moderation_types;
pub use moderations::{
    CategoryScores, ModerationCategories, ModerationModels, ModerationRequest, ModerationResponse,
    ModerationResult,
};

// Real-time Audio API
pub use realtime_audio::*;

// Responses API
pub use responses::{
    ImageContent, ImageDetail, ImageInput, ImageUrl, JsonSchemaSpec, MessageContent,
    MessageContentInput, MessageRole, PromptTemplate, PromptVariable, ResponseFormat,
    ResponseInput, ResponseOutput, ResponseRequest, SchemaValidationResult, TextContent, Usage,
};
pub use responses_v2::{
    Annotation as ResponsesApiAnnotation,
    CompletionTokenDetails as ResponsesApiCompletionTokenDetails,
    ContentPart as ResponsesApiContentPart, ConversationObject as ResponsesApiConversationObject,
    ConversationReference as ResponsesApiConversationReference, CreateResponseRequest,
    Instructions as ResponsesApiInstructions, PromptTokenDetails as ResponsesApiPromptTokenDetails,
    ResponseError as ResponsesApiError, ResponseInput as ResponsesApiInput,
    ResponseItem as ResponsesApiItem, ResponseObject, ResponseStatus, ResponseStreamEvent,
    ResponseUsage as ResponsesApiUsage, ServiceTier as ResponsesApiServiceTier,
    StreamOptions as ResponsesApiStreamOptions,
};

// Runs API
pub use runs::{
    CreateThreadAndRunRequest, FunctionCall as RunFunctionCall, ListRunStepsParams,
    ListRunStepsResponse, ListRunsParams, ListRunsResponse, RequiredAction, Run, RunError,
    RunRequest, RunRequestBuilder, RunStatus, RunStep, RunStepStatus, StepDetails,
    SubmitToolOutputsRequest, ToolOutput, Usage as RunUsage,
};

// Skills API
pub use skills::{
    CreateSkillRequest, CreateSkillVersionRequest, ListSkillVersionsParams, ListSkillsParams,
    Skill, SkillContent, SkillDeleteResponse, SkillList, SkillVersion, SkillVersionContent,
    SkillVersionList, UpdateSkillRequest,
};

// Threads API
pub use threads::{
    Annotation, DeletionStatus, FileCitation, FilePathInfo, ImageFile, ListMessageFilesResponse,
    ListMessagesParams, ListMessagesResponse, ListThreadsResponse, Message, MessageFile,
    MessageRequest, MessageRequestBuilder, MessageRole as ThreadMessageRole,
    SortOrder as ThreadSortOrder, Thread, ThreadRequest, ThreadRequestBuilder,
};

// Tools API
pub use tools::{
    CodeInterpreterBuilder, CodeInterpreterConfig, ComputerUseBuilder, ComputerUseConfig,
    EnhancedTool, EnhancedToolChoice, FileSearchBuilder, FileSearchConfig, FunctionBuilder,
    FunctionTool, ImageGenerationConfig, ImageGenerationToolBuilder, McpApproval, McpBuilder,
    McpTool, SearchFilters, SpecificToolChoice, ToolBuilder, WebSearchBuilder, WebSearchConfig,
};

// Uploads API
pub use uploads::{CompleteUploadRequest, CreateUploadRequest, Upload, UploadPart, UploadStatus};

// Videos (Sora) API
pub use videos::{
    CreateVideoRequest, ListVideosParams, Video, VideoDeleteResponse, VideoError, VideoList,
    VideoStatus,
};

// Vector Stores API
pub use vector_stores::{
    ChunkingStrategy, ExpirationPolicy, FileCounts, ListVectorStoreFilesParams,
    ListVectorStoreFilesResponse, ListVectorStoresParams, ListVectorStoresResponse, VectorStore,
    VectorStoreDeleteResponse, VectorStoreFile, VectorStoreFileBatch, VectorStoreFileBatchRequest,
    VectorStoreFileBatchStatus, VectorStoreFileDeleteResponse, VectorStoreFileError,
    VectorStoreFileRequest, VectorStoreFileStatus, VectorStoreRequest, VectorStoreRequestBuilder,
    VectorStoreStatus,
};
