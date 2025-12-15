//! Tag-related types.

use serde::{Deserialize, Serialize};

/// Query parameters for listing tags.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListTagsParams {
    /// Cursor for pagination - return results after this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Maximum number of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Text to filter/search tags.
    #[serde(skip_serializing_if = "Option::is_none", rename = "queryText")]
    pub query_text: Option<String>,
}

/// Tag schema (used in other entities).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TagSchema {
    /// The tag value.
    pub tag: String,
}

impl From<String> for TagSchema {
    fn from(tag: String) -> Self {
        Self { tag }
    }
}

impl From<&str> for TagSchema {
    fn from(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
        }
    }
}

impl std::fmt::Display for TagSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag)
    }
}
