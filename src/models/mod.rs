/// Common utilities for models
pub mod common;
/// Common builder patterns to reduce duplication
pub mod common_builder;
/// Shared traits and implementations to eliminate model duplication
pub mod shared_traits;

/// Assistants models for AI assistant creation and management  
pub mod assistants;
/// Audio models for text-to-speech, transcription, and translation
pub mod audio;
/// Container management models for Code Interpreter
pub mod containers;
/// Embeddings models for vector representations
pub mod embeddings;
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
/// Runs models for assistant execution and run steps management
pub mod runs;
/// Threads models for conversation thread and message management
pub mod threads;
/// Comprehensive tools support (web search, file search, MCP, etc.)
pub mod tools;
/// Vector stores models for RAG and knowledge management
pub mod vector_stores;

// Re-export commonly used types while avoiding conflicts
// For conflicting types, users should import from specific modules

// Assistants API
pub use assistants::{
    Assistant, AssistantRequest, AssistantTool, ListAssistantsParams, ListAssistantsResponse,
};

// Audio API
pub use audio::*;

// Containers API
pub use containers::*;

// Embeddings API
pub use embeddings::*;

// Files API
pub use files::{
    File, FileDeleteResponse, FilePurpose, FileStatus, FileUploadRequest, ListFilesParams,
    ListFilesResponse, SortOrder as FileSortOrder,
};

// Fine-tuning API
pub use fine_tuning::*;

// Functions API
pub use functions::{
    CustomTool, FunctionCall as FunctionCallType, FunctionCallOutput, FunctionSelector,
    FunctionTool as FunctionToolType, Grammar, Tool, ToolChoice as FunctionToolChoice,
};

// GPT-5 API
pub use gpt5::*;

// Images API
pub use images::*;

// Models API
pub use models::*;

// Moderations API
pub use moderations::*;

// Real-time Audio API
pub use realtime_audio::*;

// Responses API
pub use responses::{
    ImageContent, ImageDetail, ImageInput, ImageUrl, JsonSchemaSpec, MessageContent,
    MessageContentInput, MessageRole, PromptTemplate, PromptVariable, ResponseFormat,
    ResponseInput, ResponseOutput, ResponseRequest, SchemaValidationResult, TextContent, Usage,
};

// Runs API
pub use runs::{
    CreateThreadAndRunRequest, FunctionCall as RunFunctionCall, ListRunStepsParams,
    ListRunStepsResponse, ListRunsParams, ListRunsResponse, RequiredAction, Run, RunError,
    RunRequest, RunRequestBuilder, RunStatus, RunStep, RunStepStatus, StepDetails,
    SubmitToolOutputsRequest, ToolOutput, Usage as RunUsage,
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

// Vector Stores API
pub use vector_stores::{
    ChunkingStrategy, ExpirationPolicy, FileCounts, ListVectorStoreFilesParams,
    ListVectorStoreFilesResponse, ListVectorStoresParams, ListVectorStoresResponse, VectorStore,
    VectorStoreDeleteResponse, VectorStoreFile, VectorStoreFileBatch, VectorStoreFileBatchRequest,
    VectorStoreFileBatchStatus, VectorStoreFileDeleteResponse, VectorStoreFileError,
    VectorStoreFileRequest, VectorStoreFileStatus, VectorStoreRequest, VectorStoreRequestBuilder,
    VectorStoreStatus,
};
