//! Models Demo Module Structure
//!
//! This module organizes the OpenAI Models API demonstrations into focused components:
//! - `basic_demos`: Basic model operations (list, retrieve, group)
//! - `analysis_demos`: Model analysis and filtering operations
//! - `cost_demos`: Cost comparison and optimization features  
//! - `utility_demos`: Model utilities and advanced filtering
//! - `helpers`: Utility functions and shared data structures

pub mod analysis_demos;
pub mod basic_demos;
pub mod cost_demos;
pub mod helpers;
pub mod utility_demos;

pub use analysis_demos::*;
pub use basic_demos::*;
pub use cost_demos::*;
pub use helpers::*;
pub use utility_demos::*;
