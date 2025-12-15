//! Project-related types for the Letta API.

use serde::{Deserialize, Serialize};

/// Project item in list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItem {
    /// Project name.
    pub name: String,
    /// Project slug (used in URLs).
    pub slug: String,
    /// Project ID.
    pub id: String,
}

/// Response from listing projects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsListResponse {
    /// List of projects.
    pub projects: Vec<ProjectItem>,
    /// Whether there are more pages.
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_item_serialization() {
        let item = ProjectItem {
            name: "My Project".to_string(),
            slug: "my-project".to_string(),
            id: "proj-123".to_string(),
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("My Project"));
        assert!(json.contains("my-project"));
        assert!(json.contains("proj-123"));
    }

    #[test]
    fn test_projects_list_response_deserialization() {
        let json = r#"{
            "projects": [
                {
                    "name": "Test",
                    "slug": "test",
                    "id": "123"
                }
            ],
            "hasNextPage": false
        }"#;

        let response: ProjectsListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.projects.len(), 1);
        assert!(!response.has_next_page);
    }
}
