//! # Prompt Engineering Module
//!
//! This module provides utilities and builders for constructing effective prompts
//! using best practices like structured formatting, few-shot learning, and XML/Markdown.

use crate::models::responses::{Message, MessageRole, PromptTemplate, PromptVariable};
use std::collections::HashMap;
use std::fmt::Write;

/// Builder for constructing well-structured prompts
pub struct PromptBuilder {
    sections: Vec<PromptSection>,
}

/// A section of a structured prompt
#[derive(Debug, Clone)]
pub struct PromptSection {
    /// Section type
    pub section_type: SectionType,
    /// Section content
    pub content: String,
}

/// Types of prompt sections
#[derive(Debug, Clone, PartialEq)]
pub enum SectionType {
    /// Identity/role definition
    Identity,
    /// Instructions for the model
    Instructions,
    /// Few-shot examples
    Examples,
    /// Additional context
    Context,
    /// Custom section with a name
    Custom(String),
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptBuilder {
    /// Create a new prompt builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    /// Add an identity section describing the assistant's role
    pub fn with_identity(mut self, identity: impl Into<String>) -> Self {
        self.sections.push(PromptSection {
            section_type: SectionType::Identity,
            content: identity.into(),
        });
        self
    }

    /// Add instructions for the model
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.sections.push(PromptSection {
            section_type: SectionType::Instructions,
            content: instructions.into(),
        });
        self
    }

    /// Add a list of instruction items
    #[must_use]
    pub fn with_instruction_list(mut self, items: Vec<String>) -> Self {
        let mut content = String::new();
        for item in items {
            writeln!(&mut content, "* {item}").unwrap();
        }
        self.sections.push(PromptSection {
            section_type: SectionType::Instructions,
            content,
        });
        self
    }

    /// Add few-shot examples
    #[must_use]
    pub fn with_examples(mut self, examples: Vec<Example>) -> Self {
        let mut content = String::new();
        for example in examples {
            writeln!(&mut content, "{}", example.to_xml()).unwrap();
        }
        self.sections.push(PromptSection {
            section_type: SectionType::Examples,
            content,
        });
        self
    }

    /// Add context information
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.sections.push(PromptSection {
            section_type: SectionType::Context,
            content: context.into(),
        });
        self
    }

    /// Add a custom section
    pub fn with_section(mut self, name: impl Into<String>, content: impl Into<String>) -> Self {
        self.sections.push(PromptSection {
            section_type: SectionType::Custom(name.into()),
            content: content.into(),
        });
        self
    }

    /// Build the prompt as a formatted string
    #[must_use]
    pub fn build(self) -> String {
        let mut prompt = String::new();

        for (i, section) in self.sections.iter().enumerate() {
            if i > 0 {
                prompt.push_str("\n\n");
            }

            match &section.section_type {
                SectionType::Identity => {
                    writeln!(&mut prompt, "# Identity\n").unwrap();
                    prompt.push_str(&section.content);
                }
                SectionType::Instructions => {
                    writeln!(&mut prompt, "# Instructions\n").unwrap();
                    prompt.push_str(&section.content);
                }
                SectionType::Examples => {
                    writeln!(&mut prompt, "# Examples\n").unwrap();
                    prompt.push_str(&section.content);
                }
                SectionType::Context => {
                    writeln!(&mut prompt, "# Context\n").unwrap();
                    prompt.push_str(&section.content);
                }
                SectionType::Custom(name) => {
                    writeln!(&mut prompt, "# {name}\n").unwrap();
                    prompt.push_str(&section.content);
                }
            }
        }

        prompt
    }

    /// Build as a Message with the specified role
    #[must_use]
    pub fn build_message(self, role: MessageRole) -> Message {
        Message {
            role,
            content: crate::models::responses::MessageContentInput::Text(self.build()),
        }
    }

    /// Build as a developer message
    #[must_use]
    pub fn build_developer_message(self) -> Message {
        self.build_message(MessageRole::Developer)
    }
}

/// Represents a few-shot learning example
#[derive(Debug, Clone)]
pub struct Example {
    /// Input for the example
    pub input: String,
    /// Expected output for the example
    pub output: String,
    /// Optional ID for the example
    pub id: Option<String>,
    /// Optional attributes for the example
    pub attributes: HashMap<String, String>,
}

impl Example {
    /// Create a new example
    pub fn new(input: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            output: output.into(),
            id: None,
            attributes: HashMap::new(),
        }
    }

    /// Set the ID for the example
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add an attribute to the example
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Convert the example to XML format
    #[must_use]
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();

        // Input
        write!(&mut xml, "<user_query").unwrap();
        if let Some(id) = &self.id {
            write!(&mut xml, " id=\"{id}\"").unwrap();
        }
        for (key, value) in &self.attributes {
            write!(&mut xml, " {key}=\"{value}\"").unwrap();
        }
        writeln!(&mut xml, ">").unwrap();
        writeln!(&mut xml, "{}", self.input).unwrap();
        writeln!(&mut xml, "</user_query>").unwrap();

        // Output
        write!(&mut xml, "\n<assistant_response").unwrap();
        if let Some(id) = &self.id {
            write!(&mut xml, " id=\"{id}\"").unwrap();
        }
        writeln!(&mut xml, ">").unwrap();
        writeln!(&mut xml, "{}", self.output).unwrap();
        write!(&mut xml, "</assistant_response>").unwrap();

        xml
    }
}

/// Builder for XML-structured content
pub struct XmlContentBuilder {
    tag: String,
    attributes: HashMap<String, String>,
    content: String,
}

impl XmlContentBuilder {
    /// Create a new XML content builder
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            attributes: HashMap::new(),
            content: String::new(),
        }
    }

    /// Add an attribute
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Set the content
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Build the XML string
    #[must_use]
    pub fn build(self) -> String {
        let mut xml = String::new();

        write!(&mut xml, "<{}", self.tag).unwrap();
        for (key, value) in &self.attributes {
            write!(&mut xml, " {key}=\"{value}\"").unwrap();
        }

        if self.content.is_empty() {
            write!(&mut xml, " />").unwrap();
        } else {
            writeln!(&mut xml, ">").unwrap();
            write!(&mut xml, "{}", self.content).unwrap();
            write!(&mut xml, "\n</{}>", self.tag).unwrap();
        }

        xml
    }
}

/// Helper for creating prompt templates with variables
pub struct PromptTemplateBuilder {
    id: String,
    version: Option<String>,
    variables: HashMap<String, PromptVariable>,
}

impl PromptTemplateBuilder {
    /// Create a new prompt template builder
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: None,
            variables: HashMap::new(),
        }
    }

    /// Set the version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Add a string variable
    pub fn with_string_variable(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.variables
            .insert(name.into(), PromptVariable::String(value.into()));
        self
    }

    /// Add an image variable
    pub fn with_image_variable(
        mut self,
        name: impl Into<String>,
        url: impl Into<String>,
        detail: Option<String>,
    ) -> Self {
        self.variables.insert(
            name.into(),
            PromptVariable::Image(crate::models::responses::ImageInput {
                input_type: "input_image".to_string(),
                url: url.into(),
                detail,
            }),
        );
        self
    }

    /// Add a file variable
    pub fn with_file_variable(
        mut self,
        name: impl Into<String>,
        file_id: impl Into<String>,
    ) -> Self {
        self.variables.insert(
            name.into(),
            PromptVariable::File(crate::models::responses::FileInput {
                input_type: "input_file".to_string(),
                file_id: file_id.into(),
            }),
        );
        self
    }

    /// Add a JSON variable
    pub fn with_json_variable(mut self, name: impl Into<String>, value: serde_json::Value) -> Self {
        self.variables
            .insert(name.into(), PromptVariable::Json(value));
        self
    }

    /// Build the prompt template
    #[must_use]
    pub fn build(self) -> PromptTemplate {
        PromptTemplate {
            id: self.id,
            version: self.version,
            variables: if self.variables.is_empty() {
                None
            } else {
                Some(self.variables)
            },
        }
    }
}

/// Common prompt patterns and templates
pub struct PromptPatterns;

impl PromptPatterns {
    /// Create a classification prompt
    #[must_use]
    pub fn classification(categories: Vec<String>, instructions: Option<String>) -> PromptBuilder {
        let builder = PromptBuilder::new()
            .with_identity("You are a classification assistant that categorizes inputs into predefined categories.");

        let mut instruction_list = vec![
            format!(
                "Classify the input into one of these categories: {}",
                categories.join(", ")
            ),
            "Output only the category name with no additional text or formatting".to_string(),
            "Be consistent and objective in your classification".to_string(),
        ];

        if let Some(custom) = instructions {
            instruction_list.push(custom);
        }

        builder.with_instruction_list(instruction_list)
    }

    /// Create a data extraction prompt
    #[must_use]
    pub fn extraction(fields: Vec<String>, output_format: &str) -> PromptBuilder {
        PromptBuilder::new()
            .with_identity("You are a data extraction assistant that extracts structured information from unstructured text.")
            .with_instruction_list(vec![
                format!("Extract the following fields: {}", fields.join(", ")),
                format!("Output the data in {} format", output_format),
                "If a field is not present in the input, use null or an appropriate default".to_string(),
                "Maintain data types consistently (numbers as numbers, dates in ISO format, etc.)".to_string(),
            ])
    }

    /// Create a summarization prompt
    #[must_use]
    pub fn summarization(max_length: Option<usize>, style: Option<&str>) -> PromptBuilder {
        let builder = PromptBuilder::new().with_identity(
            "You are a summarization assistant that creates concise, accurate summaries.",
        );

        let mut instructions = vec![
            "Create a clear and concise summary of the provided content".to_string(),
            "Preserve key information and main points".to_string(),
            "Maintain factual accuracy without adding interpretations".to_string(),
        ];

        if let Some(length) = max_length {
            instructions.push(format!("Keep the summary under {length} words"));
        }

        if let Some(s) = style {
            instructions.push(format!("Use a {s} writing style"));
        }

        builder.with_instruction_list(instructions)
    }

    /// Create a code generation prompt
    #[must_use]
    pub fn code_generation(language: &str, requirements: Vec<String>) -> PromptBuilder {
        PromptBuilder::new()
            .with_identity(format!("You are a {language} programming assistant that generates clean, efficient, and well-documented code."))
            .with_instruction_list(vec![
                format!("Write {} code that meets the specified requirements", language),
                "Include appropriate error handling".to_string(),
                "Add clear comments explaining complex logic".to_string(),
                "Follow language-specific best practices and conventions".to_string(),
                "Consider edge cases and input validation".to_string(),
            ])
            .with_section("Requirements", requirements.join("\n"))
    }

    /// Create a Q&A prompt with context
    #[must_use]
    pub fn qa_with_context(context: &str, instructions: Option<Vec<String>>) -> PromptBuilder {
        let builder = PromptBuilder::new().with_identity(
            "You are a helpful assistant that answers questions based on provided context.",
        );

        let mut default_instructions = vec![
            "Answer questions based only on the provided context".to_string(),
            "If the answer is not in the context, say so clearly".to_string(),
            "Quote relevant parts of the context when appropriate".to_string(),
            "Be concise but complete in your answers".to_string(),
        ];

        if let Some(custom) = instructions {
            default_instructions.extend(custom);
        }

        builder
            .with_instruction_list(default_instructions)
            .with_context(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder() {
        let prompt = PromptBuilder::new()
            .with_identity("You are a helpful assistant.")
            .with_instructions("Be concise and accurate.")
            .build();

        assert!(prompt.contains("# Identity"));
        assert!(prompt.contains("# Instructions"));
        assert!(prompt.contains("helpful assistant"));
    }

    #[test]
    fn test_example_xml() {
        let example = Example::new("What is 2+2?", "4").with_id("math-1");

        let xml = example.to_xml();
        assert!(xml.contains("<user_query id=\"math-1\">"));
        assert!(xml.contains("<assistant_response id=\"math-1\">"));
        assert!(xml.contains("What is 2+2?"));
        assert!(xml.contains("4"));
    }

    #[test]
    fn test_xml_builder() {
        let xml = XmlContentBuilder::new("document")
            .with_attribute("version", "1.0")
            .with_content("Test content")
            .build();

        assert!(xml.contains("<document version=\"1.0\">"));
        assert!(xml.contains("Test content"));
        assert!(xml.contains("</document>"));
    }

    #[test]
    fn test_prompt_template_builder() {
        let template = PromptTemplateBuilder::new("pmpt_test")
            .with_version("1.0")
            .with_string_variable("name", "John")
            .build();

        assert_eq!(template.id, "pmpt_test");
        assert_eq!(template.version, Some("1.0".to_string()));
        assert!(template.variables.is_some());
    }
}
