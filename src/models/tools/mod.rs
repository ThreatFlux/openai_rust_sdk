//! # OpenAI Tools Module
//!
//! This module provides comprehensive support for all OpenAI tools including:
//! - Web Search: Include internet data in responses
//! - File Search: Search uploaded files for context
//! - Function Calling: Call custom functions
//! - Remote MCP: Access Model Context Protocol servers
//! - Image Generation: Generate or edit images
//! - Code Interpreter: Execute code in secure containers
//! - Computer Use: Agentic computer interface control

// Builder modules
pub mod code_interpreter_builder;
pub mod computer_use_builder;
pub mod file_search_builder;
pub mod function_builder;
pub mod image_generation_builder;
pub mod mcp_builder;
pub mod tool_builder;
pub mod web_search_builder;

// Other modules
pub mod code_interpreter;
pub mod computer_use;
pub mod core_types;
pub mod file_search;
pub mod function_tools;
pub mod image_generation;
pub mod mcp_tools;
pub mod tool_choice;
pub mod web_search;

// Re-export all builders
pub use code_interpreter::*;
pub use code_interpreter_builder::*;
pub use computer_use::*;
pub use computer_use_builder::*;
pub use core_types::*;
pub use file_search::*;
pub use file_search_builder::*;
pub use function_builder::*;
pub use function_tools::*;
pub use image_generation::*;
pub use image_generation_builder::*;
pub use mcp_builder::*;
pub use mcp_tools::*;
pub use tool_builder::*;
pub use tool_choice::*;
pub use web_search::*;
pub use web_search_builder::*;
