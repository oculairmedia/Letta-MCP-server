//! Response optimization utilities
//!
//! Shared utilities for response truncation, pagination, and formatting
//! across all Letta MCP tools. These help reduce token usage while
//! maintaining useful information.

use serde::{Deserialize, Serialize};

// ===================================================
// Response Size Limits (configurable defaults)
// ===================================================

/// Default limits for response truncation
pub mod limits {
    /// Maximum characters for description previews
    pub const DESCRIPTION_PREVIEW: usize = 100;
    /// Maximum characters for short descriptions (e.g., tool descriptions)
    pub const SHORT_DESCRIPTION: usize = 80;
    /// Maximum characters for content previews
    pub const CONTENT_PREVIEW: usize = 200;
    /// Maximum characters for system prompts in responses
    pub const SYSTEM_PROMPT: usize = 500;
    /// Maximum characters for message content
    pub const MESSAGE_CONTENT: usize = 1000;
    /// Default items per page for list operations
    pub const DEFAULT_PAGE_SIZE: usize = 15;
    /// Maximum items per page for list operations
    pub const MAX_PAGE_SIZE: usize = 50;
    /// Maximum characters for value previews
    pub const VALUE_PREVIEW: usize = 100;
    /// Maximum characters for text previews in passages
    pub const PASSAGE_TEXT_PREVIEW: usize = 200;
}

// ===================================================
// String Truncation Functions
// ===================================================

/// Truncate text with indicator showing how many chars were truncated.
/// Use this for content that the user might want to know the full length of.
///
/// # Example
/// ```
/// use letta_server::tools::response_utils::truncate_with_indicator;
/// let result = truncate_with_indicator("Hello, World!", 5);
/// assert_eq!(result, "Hello...[truncated, 8 more chars]");
/// ```
pub fn truncate_with_indicator(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        text.to_string()
    } else {
        let remaining = text.len() - max_chars;
        format!("{}...[truncated, {} more chars]", &text[..max_chars], remaining)
    }
}

/// Truncate text with simple ellipsis.
/// Use this for previews where exact length isn't important.
///
/// # Example
/// ```
/// use letta_server::tools::response_utils::truncate_preview;
/// let result = truncate_preview("Hello, World!", 5);
/// assert_eq!(result, "Hello...");
/// ```
pub fn truncate_preview(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        text.to_string()
    } else {
        format!("{}...", &text[..max_chars])
    }
}

/// Truncate text without any indicator (for internal use).
/// Returns the original if shorter than max_chars.
pub fn truncate_silent(text: &str, max_chars: usize) -> &str {
    if text.len() <= max_chars {
        text
    } else {
        &text[..max_chars]
    }
}

// ===================================================
// Pagination Helpers
// ===================================================

/// Standard pagination metadata included in list responses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationMeta {
    /// Total number of items available
    pub total: usize,
    /// Number of items returned in this response
    pub returned: usize,
    /// Current offset (starting position)
    pub offset: usize,
    /// Maximum items per page
    pub limit: usize,
    /// Whether more items are available
    pub has_more: bool,
    /// Helpful hints for the user
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hints: Vec<String>,
}

impl PaginationMeta {
    /// Create new pagination metadata
    pub fn new(total: usize, returned: usize, offset: usize, limit: usize) -> Self {
        let has_more = total > offset + returned;
        Self {
            total,
            returned,
            offset,
            limit,
            has_more,
            hints: Vec::new(),
        }
    }

    /// Add a hint to the metadata
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }

    /// Add standard pagination hints
    pub fn with_standard_hints(mut self, detail_op: &str) -> Self {
        self.hints
            .push(format!("Use '{}' with id for full details", detail_op));
        if self.has_more {
            self.hints
                .push(format!("Use offset={} for next page", self.offset + self.returned));
        }
        self
    }
}

/// Apply pagination defaults and caps
pub fn apply_pagination_defaults(
    limit: Option<usize>,
    offset: Option<usize>,
) -> (usize, usize) {
    let limit = limit
        .map(|l| l.min(limits::MAX_PAGE_SIZE))
        .unwrap_or(limits::DEFAULT_PAGE_SIZE);
    let offset = offset.unwrap_or(0);
    (limit, offset)
}

// ===================================================
// Item Summary Trait
// ===================================================

/// Trait for creating summaries of items for list operations
pub trait Summarize {
    type Summary: Serialize;

    /// Create a summary representation suitable for list responses
    fn summarize(&self) -> Self::Summary;
}

// ===================================================
// Response Hints
// ===================================================

/// Common hint messages
pub mod hints {
    pub const USE_GET_FOR_DETAILS: &str = "Use 'get' operation with id for full details";
    pub const USE_PAGINATION: &str = "Use limit/offset parameters for pagination";
    pub const RESPONSE_TRUNCATED: &str = "Some fields truncated to reduce response size";
}

// ===================================================
// Tests
// ===================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_with_indicator() {
        // Short text - no truncation
        assert_eq!(truncate_with_indicator("Hello", 10), "Hello");

        // Exact length - no truncation
        assert_eq!(truncate_with_indicator("Hello", 5), "Hello");

        // Long text - truncated with indicator
        let result = truncate_with_indicator("Hello, World!", 5);
        assert!(result.starts_with("Hello"));
        assert!(result.contains("truncated"));
        assert!(result.contains("8 more chars"));
    }

    #[test]
    fn test_truncate_preview() {
        // Short text - no truncation
        assert_eq!(truncate_preview("Hello", 10), "Hello");

        // Long text - truncated with ellipsis
        assert_eq!(truncate_preview("Hello, World!", 5), "Hello...");
    }

    #[test]
    fn test_pagination_meta() {
        let meta = PaginationMeta::new(100, 15, 0, 15);
        assert_eq!(meta.total, 100);
        assert_eq!(meta.returned, 15);
        assert!(meta.has_more);

        let meta_with_hints = meta.with_standard_hints("get");
        assert!(!meta_with_hints.hints.is_empty());
    }

    #[test]
    fn test_apply_pagination_defaults() {
        // Default values
        let (limit, offset) = apply_pagination_defaults(None, None);
        assert_eq!(limit, limits::DEFAULT_PAGE_SIZE);
        assert_eq!(offset, 0);

        // Custom values within limits
        let (limit, offset) = apply_pagination_defaults(Some(25), Some(10));
        assert_eq!(limit, 25);
        assert_eq!(offset, 10);

        // Exceeds max - should be capped
        let (limit, _) = apply_pagination_defaults(Some(100), None);
        assert_eq!(limit, limits::MAX_PAGE_SIZE);
    }
}
