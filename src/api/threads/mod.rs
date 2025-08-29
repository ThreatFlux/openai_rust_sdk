//! # OpenAI Threads & Messages API Client
//!
//! This module provides a complete implementation of OpenAI's Threads API, which allows you to
//! create conversation threads and manage messages within those threads.
//!
//! ## Features
//!
//! - **Thread Management**: Create, retrieve, modify, and delete conversation threads
//! - **Message Management**: Add, retrieve, modify, and list messages within threads
//! - **File Management**: Attach files to messages and manage message files
//! - **Pagination**: List messages with cursor-based pagination
//! - **Content Types**: Support for text and image content in messages
//! - **Annotations**: Handle file citations and file paths within message content
//! - **Error Handling**: Comprehensive error handling with detailed messages
//!
//! ## Thread Capabilities
//!
//! Threads provide a way to organize conversations and can:
//! - Store multiple messages in chronological order
//! - Maintain conversation context across multiple interactions
//! - Support file attachments for analysis and processing
//! - Store custom metadata for organization and tracking
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::{threads::ThreadsApi, common::ApiClientConstructors};
//! use openai_rust_sdk::models::threads::{ThreadRequest, MessageRequest, MessageRole};
//!
//! # tokio_test::block_on(async {
//! let api = ThreadsApi::new("your-api-key")?;
//!
//! // Create a new thread
//! let thread_request = ThreadRequest::builder()
//!     .metadata_pair("purpose", "customer_support")
//!     .build();
//!
//! let thread = api.create_thread(thread_request).await?;
//! println!("Created thread: {}", thread.id);
//!
//! // Add a message to the thread
//! let message_request = MessageRequest::builder()
//!     .role(MessageRole::User)
//!     .content("Hello, I need help with my account.")
//!     .build()?;
//!
//! let message = api.create_message(&thread.id, message_request).await?;
//! println!("Created message: {}", message.id);
//!
//! // List messages in the thread
//! let messages = api.list_messages(&thread.id, None).await?;
//! println!("Found {} messages", messages.data.len());
//!
//! // Delete the thread
//! let deleted = api.delete_thread(&thread.id).await?;
//! println!("Deleted: {}", deleted.deleted);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```

/// Core client implementation and API struct
pub mod client;
/// Message file management functionality
pub mod files;
/// Message management functionality  
pub mod messages;
/// Thread operations functionality
pub mod operations;
/// Type definitions and re-exports
pub mod types;

// Re-export the main API client
pub use client::ThreadsApi;

// Re-export all types for convenience
pub use types::*;
