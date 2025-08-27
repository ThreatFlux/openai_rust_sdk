//! # OpenAI Threads & Messages API Models
//!
//! This module provides data structures for OpenAI's Threads API, which allows you to
//! create conversation threads and manage messages within those threads.
//!
//! ## Overview
//!
//! The Threads API supports:
//! - **Thread Management**: Create, retrieve, modify, and delete conversation threads
//! - **Message Management**: Add, retrieve, and modify messages within threads
//! - **File Attachments**: Attach files to messages for analysis and processing
//! - **Content Types**: Support for text and image content in messages
//! - **Annotations**: File citations and file paths within message content
//! - **Metadata**: Store custom metadata for threads and messages
//!
//! ## Message Roles
//!
//! Messages can have different roles:
//! - `user`: Messages from the user
//! - `assistant`: Messages from the AI assistant
//!
//! ## Content Types
//!
//! Messages can contain different types of content:
//! - **Text**: Plain text with optional annotations (file citations, file paths)
//! - **Image**: Image files for visual analysis
//!
//! ## Examples
//!
//! ```rust
//! use openai_rust_sdk::models::threads::{ThreadRequest, MessageRequest, MessageRole};
//! use std::collections::HashMap;
//!
//! // Create a thread request
//! let thread_request = ThreadRequest::builder()
//!     .metadata_pair("purpose", "customer_support")
//!     .build();
//!
//! // Create a message request
//! let message_request = MessageRequest::builder()
//!     .role(MessageRole::User)
//!     .content("Hello, I need help with my account.")
//!     .build()
//!     .unwrap();
//! ```

pub mod builders;
pub mod content;
pub mod message;
pub mod thread;
pub mod types;
pub mod validation;

// Re-export main types for convenience
pub use content::{Annotation, FileCitation, FilePathInfo, ImageFile, MessageContent, TextContent};
pub use message::{
    ListMessageFilesResponse, ListMessagesParams, ListMessagesResponse, Message, MessageRequest,
    MessageRequestBuilder,
};
pub use thread::{ListThreadsResponse, Thread, ThreadRequest, ThreadRequestBuilder};
pub use types::{DeletionStatus, MessageFile, MessageRole, SortOrder};

// Re-export builder traits
pub use builders::MetadataBuilder;
