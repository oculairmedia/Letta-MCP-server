//! Response optimization utilities
//!
//! Shared utilities for response truncation, pagination, and formatting
//! across all Letta MCP tools. These help reduce token usage while
//! maintaining useful information.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ===================================================
// Response Size Limits (configurable defaults)
// ===================================================

/// Default limits for response truncation
pub mod limits {
    use std::env;
    use tracing::warn;

    /// Environment variable name for maximum value/truncation length
    pub const ENV_MAX_VALUE_LEN: &str = "LETTA_MCP_MAX_VALUE_LEN";
    /// Environment variable name for core memory preview length
    pub const ENV_CORE_MEMORY_PREVIEW_LEN: &str = "LETTA_MCP_CORE_MEMORY_PREVIEW_LEN";

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

    /// Default truncation length for block/passage/search values (500 chars)
    pub const DEFAULT_MAX_VALUE_LEN: usize = 500;
    /// Default truncation length for core memory previews (200 chars)
    pub const DEFAULT_CORE_MEMORY_PREVIEW_LEN: usize = 200;

    /// Get max value truncation length from env var or default.
    /// Reads LETTA_MCP_MAX_VALUE_LEN env var. Invalid values fall back to DEFAULT_MAX_VALUE_LEN.
    pub fn max_value_len() -> usize {
        read_env_usize(ENV_MAX_VALUE_LEN, DEFAULT_MAX_VALUE_LEN)
    }

    /// Get core memory preview length from env var or default.
    /// Reads LETTA_MCP_CORE_MEMORY_PREVIEW_LEN env var. Invalid values fall back to DEFAULT_CORE_MEMORY_PREVIEW_LEN.
    pub fn core_memory_preview_len() -> usize {
        read_env_usize(ENV_CORE_MEMORY_PREVIEW_LEN, DEFAULT_CORE_MEMORY_PREVIEW_LEN)
    }

    /// Read a usize env var with fallback.
    /// Returns fallback if env var is not set or invalid.
    fn read_env_usize(name: &str, fallback: usize) -> usize {
        match env::var(name) {
            Ok(val) => match val.parse() {
                Ok(val) => val,
                Err(e) => {
                    warn!(
                        "Invalid value for {}='{}': {}. Using default {}",
                        name, val, e, fallback
                    );
                    fallback
                }
            },
            Err(_) => fallback,
        }
    }
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
// Unified Tool Response
// ===================================================

/// Unified response struct for all tool handlers.
///
/// Replaces per-tool response structs (ToolManagerResponse, JobMonitorResponse,
/// SourceManagerResponse, McpOpsResponse, MemoryUnifiedResponse, FileFolderResponse)
/// with a single generic struct + builder pattern.
///
/// Core fields (success/operation/message) are always present.
/// Optional fields are skipped when None to keep JSON compact.
/// The `extra` field with `#[serde(flatten)]` allows tool-specific
/// fields to appear at the top level of the JSON output.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
    /// Additional tool-specific fields, flattened into the response.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

impl ToolResponse {
    /// Create a success response.
    pub fn success(operation: &str, message: impl Into<String>) -> Self {
        Self {
            success: true,
            operation: operation.to_string(),
            message: message.into(),
            data: None,
            count: None,
            pagination: None,
            extra: None,
        }
    }

    /// Create an error response.
    pub fn error(operation: &str, message: impl Into<String>) -> Self {
        Self {
            success: false,
            operation: operation.to_string(),
            message: message.into(),
            data: None,
            count: None,
            pagination: None,
            extra: None,
        }
    }

    /// Attach serializable data (converts via serde_json::to_value).
    pub fn with_data(mut self, data: impl Serialize) -> Result<Self, serde_json::Error> {
        self.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    /// Attach a pre-built JSON Value as data.
    pub fn with_json_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Set the count field.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }

    /// Attach pagination metadata.
    pub fn with_pagination(mut self, pagination: PaginationMeta) -> Self {
        self.pagination = Some(pagination);
        self
    }

    /// Attach extra fields that will be flattened into the top-level JSON.
    pub fn with_extra(mut self, extra: Value) -> Self {
        self.extra = Some(extra);
        self
    }
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
    use std::env;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

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
        let emoji_text = "😀😁😂 hello";
        let result = truncate_preview(emoji_text, 5);
        assert!(result.starts_with("😀"));
        assert!(result.ends_with("..."));

        let cjk_text = "你好世界abcdef";
        let result = truncate_with_indicator(cjk_text, 7);
        assert!(result.starts_with("你好"));
        assert!(result.contains("truncated"));

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
        let (limit, offset) = apply_pagination_defaults(None, None);
        assert_eq!(limit, limits::DEFAULT_PAGE_SIZE);
        assert_eq!(offset, 0);

        let (limit, offset) = apply_pagination_defaults(Some(25), Some(10));
        assert_eq!(limit, 25);
        assert_eq!(offset, 10);

        let (limit, _) = apply_pagination_defaults(Some(100), None);
        assert_eq!(limit, limits::MAX_PAGE_SIZE);
    }

    #[test]
    fn test_max_value_len_default() {
        let _guard = ENV_LOCK.lock().expect("env test lock poisoned");
        // Safety: env var manipulation in tests
        unsafe {
            env::remove_var(limits::ENV_MAX_VALUE_LEN);
        }
        assert_eq!(limits::max_value_len(), limits::DEFAULT_MAX_VALUE_LEN);
    }

    #[test]
    fn test_max_value_len_from_env() {
        let _guard = ENV_LOCK.lock().expect("env test lock poisoned");
        unsafe {
            env::set_var(limits::ENV_MAX_VALUE_LEN, "1000");
        }
        assert_eq!(limits::max_value_len(), 1000);
        unsafe {
            env::remove_var(limits::ENV_MAX_VALUE_LEN);
        }
    }

    #[test]
    fn test_max_value_len_invalid_falls_back() {
        let _guard = ENV_LOCK.lock().expect("env test lock poisoned");
        unsafe {
            env::set_var(limits::ENV_MAX_VALUE_LEN, "not-a-number");
        }
        assert_eq!(limits::max_value_len(), limits::DEFAULT_MAX_VALUE_LEN);
        unsafe {
            env::remove_var(limits::ENV_MAX_VALUE_LEN);
        }
    }

    #[test]
    fn test_core_memory_preview_len_default() {
        let _guard = ENV_LOCK.lock().expect("env test lock poisoned");
        unsafe {
            env::remove_var(limits::ENV_CORE_MEMORY_PREVIEW_LEN);
        }
        assert_eq!(
            limits::core_memory_preview_len(),
            limits::DEFAULT_CORE_MEMORY_PREVIEW_LEN
        );
    }

    #[test]
    fn test_core_memory_preview_len_from_env() {
        let _guard = ENV_LOCK.lock().expect("env test lock poisoned");
        unsafe {
            env::set_var(limits::ENV_CORE_MEMORY_PREVIEW_LEN, "300");
        }
        assert_eq!(limits::core_memory_preview_len(), 300);
        unsafe {
            env::remove_var(limits::ENV_CORE_MEMORY_PREVIEW_LEN);
        }
    }
}
