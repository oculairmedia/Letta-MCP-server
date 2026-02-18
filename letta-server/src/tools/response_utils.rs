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

/// Find the nearest valid UTF-8 char boundary at or before `index`.
/// Returns 0 if no valid boundary is found (should not happen for valid UTF-8).
fn floor_char_boundary(text: &str, index: usize) -> usize {
    if index >= text.len() {
        return text.len();
    }
    let mut end = index;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    end
}

/// Truncate text with indicator showing how many chars were truncated.
/// Use this for content that the user might want to know the full length of.
/// Safe for multi-byte UTF-8 characters (emoji, CJK, etc.).
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
        let end = floor_char_boundary(text, max_chars);
        let remaining = text.len() - end;
        format!("{}...[truncated, {} more chars]", &text[..end], remaining)
    }
}

/// Truncate text with simple ellipsis.
/// Use this for previews where exact length isn't important.
/// Safe for multi-byte UTF-8 characters (emoji, CJK, etc.).
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
        let end = floor_char_boundary(text, max_chars);
        format!("{}...", &text[..end])
    }
}

/// Truncate text without any indicator (for internal use).
/// Returns the original if shorter than max_chars.
/// Safe for multi-byte UTF-8 characters.
pub fn truncate_silent(text: &str, max_chars: usize) -> &str {
    if text.len() <= max_chars {
        text
    } else {
        let end = floor_char_boundary(text, max_chars);
        &text[..end]
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
            self.hints.push(format!(
                "Use offset={} for next page",
                self.offset + self.returned
            ));
        }
        self
    }
}

/// Apply pagination defaults and caps using the standard page size limits.
pub fn apply_pagination_defaults(limit: Option<usize>, offset: Option<usize>) -> (usize, usize) {
    paginate(
        limit,
        offset,
        limits::DEFAULT_PAGE_SIZE,
        limits::MAX_PAGE_SIZE,
    )
}

/// Apply pagination with custom default and maximum limits.
///
/// Use this when a tool needs different pagination bounds than the global defaults
/// (e.g., tool listings may want larger pages than agent listings).
pub fn paginate(
    limit: Option<usize>,
    offset: Option<usize>,
    default_limit: usize,
    max_limit: usize,
) -> (usize, usize) {
    let limit = limit.map(|l| l.min(max_limit)).unwrap_or(default_limit);
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
        assert_eq!(truncate_with_indicator("Hello", 10), "Hello");
        assert_eq!(truncate_with_indicator("Hello", 5), "Hello");

        let result = truncate_with_indicator("Hello, World!", 5);
        assert!(result.starts_with("Hello"));
        assert!(result.contains("truncated"));
        assert!(result.contains("8 more chars"));
    }

    #[test]
    fn test_truncate_preview() {
        assert_eq!(truncate_preview("Hello", 10), "Hello");
        assert_eq!(truncate_preview("Hello, World!", 5), "Hello...");
    }

    #[test]
    fn test_truncate_utf8_multibyte_safety() {
        // Emoji: each is 4 bytes. "😀😁😂" = 12 bytes
        let emoji_text = "😀😁😂 hello";
        // Cutting at byte 5 would land mid-emoji; should snap back to byte 4
        let result = truncate_preview(emoji_text, 5);
        assert!(result.starts_with("😀"));
        assert!(result.ends_with("..."));

        // CJK: each char is 3 bytes. "你好世界" = 12 bytes
        let cjk_text = "你好世界abcdef";
        let result = truncate_with_indicator(cjk_text, 7);
        // byte 7 lands mid-char (世 starts at 6), should snap to 6
        assert!(result.starts_with("你好"));
        assert!(result.contains("truncated"));

        // Exact boundary should work fine
        let result = truncate_preview("abc", 3);
        assert_eq!(result, "abc");
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
