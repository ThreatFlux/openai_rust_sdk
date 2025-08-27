//! Type definitions for response format demos

use serde::{Deserialize, Serialize};

/// Person structure for demonstrating type-safe responses
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub occupation: String,
    pub skills: Vec<String>,
}

/// Task list structure for complex schema demonstrations
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TaskList {
    pub title: String,
    pub tasks: Vec<Task>,
    pub priority: Priority,
    pub estimated_hours: f32,
}

/// Individual task structure
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub completed: bool,
}

/// Priority enumeration for tasks
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_person_struct_serialization() {
        let person = Person {
            name: "Test User".to_string(),
            age: 30,
            occupation: "Developer".to_string(),
            skills: vec!["Rust".to_string(), "Python".to_string()],
        };

        let json_value = serde_json::to_value(&person).unwrap();
        let parsed_person: Person = serde_json::from_value(json_value).unwrap();

        assert_eq!(person.name, parsed_person.name);
        assert_eq!(person.age, parsed_person.age);
        assert_eq!(person.skills, parsed_person.skills);
    }
}
