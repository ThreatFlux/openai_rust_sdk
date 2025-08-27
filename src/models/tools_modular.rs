//! # OpenAI Tools Module - Modular Version
//!
//! This module provides comprehensive support for all OpenAI tools including:
//! - Web Search: Include internet data in responses
//! - File Search: Search uploaded files for context
//! - Function Calling: Call custom functions
//! - Remote MCP: Access Model Context Protocol servers
//! - Image Generation: Generate or edit images
//! - Code Interpreter: Execute code in secure containers
//! - Computer Use: Agentic computer interface control

pub mod tools;

pub use tools::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_search_tool() {
        let tool = ToolBuilder::web_search();
        match tool {
            EnhancedTool::WebSearchPreview => {}
            _ => panic!("Expected WebSearchPreview"),
        }
    }

    #[test]
    fn test_file_search_builder() {
        let tool = ToolBuilder::file_search(vec!["store_123".to_string()])
            .max_chunks(10)
            .file_types(vec!["pdf".to_string(), "txt".to_string()])
            .build();

        match tool {
            EnhancedTool::FileSearch(config) => {
                assert_eq!(config.vector_store_ids, vec!["store_123"]);
                assert_eq!(config.max_chunks, Some(10));
                assert_eq!(
                    config.file_types,
                    Some(vec!["pdf".to_string(), "txt".to_string()])
                );
            }
            _ => panic!("Expected FileSearch"),
        }
    }

    #[test]
    fn test_mcp_builder() {
        let tool = ToolBuilder::mcp("deepwiki", "https://mcp.deepwiki.com/mcp")
            .require_approval(McpApproval::Never)
            .header("Authorization", "Bearer token")
            .timeout_ms(5000)
            .build();

        match tool {
            EnhancedTool::Mcp(config) => {
                assert_eq!(config.server_label, "deepwiki");
                assert_eq!(config.server_url, "https://mcp.deepwiki.com/mcp");
                assert!(matches!(config.require_approval, McpApproval::Never));
                assert!(config.headers.is_some());
                assert_eq!(config.timeout_ms, Some(5000));
            }
            _ => panic!("Expected Mcp"),
        }
    }

    #[test]
    fn test_image_generation_builder() {
        let tool = ToolBuilder::image_generation()
            .size("1024x1024")
            .quality("hd")
            .style("vivid")
            .count(2)
            .build();

        match tool {
            EnhancedTool::ImageGeneration(config) => {
                assert_eq!(config.size, Some("1024x1024".to_string()));
                assert_eq!(config.quality, Some("hd".to_string()));
                assert_eq!(config.style, Some("vivid".to_string()));
                assert_eq!(config.n, Some(2));
            }
            _ => panic!("Expected ImageGeneration"),
        }
    }

    #[test]
    fn test_code_interpreter_builder() {
        let tool = ToolBuilder::code_interpreter()
            .language("python")
            .max_execution_time_ms(30000)
            .libraries(vec!["numpy".to_string(), "pandas".to_string()])
            .persist_container(true)
            .include_citations(false)
            .build();

        match tool {
            EnhancedTool::CodeInterpreter(config) => {
                assert_eq!(config.language, Some("python".to_string()));
                assert_eq!(config.max_execution_time_ms, Some(30000));
                assert_eq!(
                    config.libraries,
                    Some(vec!["numpy".to_string(), "pandas".to_string()])
                );
                assert_eq!(config.persist_container, Some(true));
                assert_eq!(config.include_citations, Some(false));
            }
            _ => panic!("Expected CodeInterpreter"),
        }
    }

    #[test]
    fn test_computer_use_builder() {
        let tool = ToolBuilder::computer_use()
            .resolution("1920x1080")
            .os_type("linux")
            .applications(vec!["firefox".to_string(), "terminal".to_string()])
            .max_actions(50)
            .build();

        match tool {
            EnhancedTool::ComputerUse(config) => {
                assert_eq!(config.resolution, Some("1920x1080".to_string()));
                assert_eq!(config.os_type, Some("linux".to_string()));
                assert_eq!(
                    config.applications,
                    Some(vec!["firefox".to_string(), "terminal".to_string()])
                );
                assert_eq!(config.max_actions, Some(50));
            }
            _ => panic!("Expected ComputerUse"),
        }
    }

    #[test]
    fn test_web_search_advanced_builder() {
        let tool = ToolBuilder::web_search_advanced()
            .max_results(5)
            .include_domains(vec!["example.com".to_string()])
            .exclude_domains(vec!["spam.com".to_string()])
            .time_range("past_week")
            .build();

        match tool {
            EnhancedTool::WebSearch(config) => {
                assert_eq!(config.max_results, Some(5));
                assert_eq!(config.time_range, Some("past_week".to_string()));
                if let Some(filters) = &config.filters {
                    assert_eq!(
                        filters.include_domains,
                        Some(vec!["example.com".to_string()])
                    );
                    assert_eq!(
                        filters.exclude_domains,
                        Some(vec!["spam.com".to_string()])
                    );
                }
            }
            _ => panic!("Expected WebSearch"),
        }
    }
}