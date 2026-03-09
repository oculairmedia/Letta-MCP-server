//! Memory response optimization utilities
//!
//! Helper functions and types for truncating and summarizing memory responses
//! to reduce token usage while maintaining useful information.

use crate::tools::response_utils::{
    truncate_preview as shared_truncate_preview, truncate_with_indicator,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Truncate a string to a maximum length, adding indicator if truncated
pub fn truncate_string(s: &str, max_len: usize) -> String {
    truncate_with_indicator(s, max_len)
}

/// Truncate a string to show preview only
pub fn truncate_preview(s: &str, max_len: usize) -> String {
    shared_truncate_preview(s, max_len)
}

/// Block summary for list operations
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockSummary {
    pub id: Option<String>,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value_preview: String,
    pub value_length: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl BlockSummary {
    /// Create a BlockSummary from a memory block Value
    pub fn from_block_value(block: &Value) -> Self {
        let id = block
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let label = block
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let description = block
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| truncate_preview(s, 100));

        let value = block.get("value").and_then(|v| v.as_str()).unwrap_or("");

        let value_length = value.len();
        let value_preview = truncate_preview(value, 100);

        let is_template = block.get("is_template").and_then(|v| v.as_bool());

        let created_at = block
            .get("created_at")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        BlockSummary {
            id,
            label,
            description,
            value_preview,
            value_length,
            is_template,
            created_at,
        }
    }
}

/// Passage summary for search/list operations
#[derive(Debug, Serialize, Deserialize)]
pub struct PassageSummary {
    pub id: String,
    pub text_preview: String,
    pub text_length: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl PassageSummary {
    /// Create a PassageSummary from a passage Value
    pub fn from_passage_value(passage: &Value) -> Self {
        let id = passage
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let text = passage.get("text").and_then(|v| v.as_str()).unwrap_or("");

        let text_length = text.len();
        let text_preview = truncate_preview(text, 200);

        let created_at = passage
            .get("created_at")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let source = passage
            .get("source")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        PassageSummary {
            id,
            text_preview,
            text_length,
            created_at,
            source,
        }
    }
}

/// Pagination metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub total: usize,
    pub returned: usize,
    pub limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hints: Vec<String>,
}

impl PaginationMeta {
    pub fn new(total: usize, returned: usize, limit: usize) -> Self {
        PaginationMeta {
            total,
            returned,
            limit,
            offset: None,
            hints: Vec::new(),
        }
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_hint(mut self, hint: String) -> Self {
        self.hints.push(hint);
        self
    }
}

/// Truncate a block's value field in a Value object
pub fn truncate_block_value(block: &mut Value, max_len: usize) -> bool {
    // Clone the value string to avoid borrowing issues
    let value_info = block
        .get("value")
        .and_then(|v| v.as_str())
        .map(|s| (s.to_string(), s.len()));

    if let Some((value_str, value_len)) = value_info
        && value_len > max_len
    {
        let truncated = truncate_string(&value_str, max_len);
        if let Some(obj) = block.as_object_mut() {
            obj.insert("value".to_string(), Value::String(truncated));
            obj.insert("value_length".to_string(), Value::Number(value_len.into()));
            obj.insert("truncated".to_string(), Value::Bool(true));
            return true;
        }
    }
    false
}

/// Truncate a passage's text field in a Value object
pub fn truncate_passage_text(passage: &mut Value, max_len: usize) -> bool {
    // Clone the text string to avoid borrowing issues
    let text_info = passage
        .get("text")
        .and_then(|v| v.as_str())
        .map(|s| (s.to_string(), s.len()));

    if let Some((text_str, text_len)) = text_info
        && text_len > max_len
    {
        let truncated = truncate_string(&text_str, max_len);
        if let Some(obj) = passage.as_object_mut() {
            obj.insert("text".to_string(), Value::String(truncated));
            obj.insert("text_length".to_string(), Value::Number(text_len.into()));
            obj.insert("truncated".to_string(), Value::Bool(true));
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_string() {
        let short = "Hello";
        assert_eq!(truncate_string(short, 10), "Hello");

        let long = "Hello, this is a very long string that needs truncation";
        let result = truncate_string(long, 20);
        assert!(result.contains("Hello, this is a ver"));
        assert!(result.contains("truncated"));
    }

    #[test]
    fn test_truncate_preview() {
        let short = "Hello";
        assert_eq!(truncate_preview(short, 10), "Hello");

        let long = "Hello, this is a very long string";
        let result = truncate_preview(long, 10);
        assert_eq!(result, "Hello, thi...");
    }
}
