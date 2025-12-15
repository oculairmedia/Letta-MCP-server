//! Common types used across the Letta API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smart_default::SmartDefault;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Letta resource identifier that can be either a bare UUID or a prefixed UUID.
///
/// # Examples
///
/// ```
/// use letta::types::LettaId;
/// use std::str::FromStr;
///
/// // Bare UUID
/// let id1 = LettaId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
///
/// // Prefixed UUID
/// let id2 = LettaId::from_str("agent-550e8400-e29b-41d4-a716-446655440000").unwrap();
///
/// // Get the UUID part
/// assert_eq!(id1.uuid(), id2.uuid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LettaId {
    /// Optional prefix (e.g., "agent", "run", "tool")
    prefix: Option<String>,
    /// The UUID part
    uuid: Uuid,
}

impl LettaId {
    /// Create a new ID with a prefix.
    pub fn new_prefixed(prefix: impl Into<String>, uuid: Uuid) -> Self {
        Self {
            prefix: Some(prefix.into()),
            uuid,
        }
    }

    /// Create a new ID without a prefix (bare UUID).
    pub fn new_bare(uuid: Uuid) -> Self {
        Self { prefix: None, uuid }
    }

    /// Get the prefix, if any.
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Get the UUID part.
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Check if this is a bare UUID (no prefix).
    pub fn is_bare(&self) -> bool {
        self.prefix.is_none()
    }

    /// Convert to string representation.
    pub fn as_str(&self) -> String {
        match &self.prefix {
            Some(prefix) => format!("{}-{}", prefix, self.uuid),
            None => self.uuid.to_string(),
        }
    }
}

impl fmt::Display for LettaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for LettaId {
    type Err = LettaIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try to parse as bare UUID first
        if let Ok(uuid) = Uuid::from_str(s) {
            return Ok(Self::new_bare(uuid));
        }

        // Try to parse as prefixed UUID
        // UUIDs have a specific format with dashes at positions 8, 13, 18, 23
        // So we need to check if the string has the pattern: prefix-uuid
        // where uuid is 36 characters long (32 hex + 4 dashes)
        if s.len() > 36 {
            let uuid_start = s.len() - 36;
            if uuid_start > 0 && s.chars().nth(uuid_start - 1) == Some('-') {
                let potential_uuid = &s[uuid_start..];
                if let Ok(uuid) = Uuid::from_str(potential_uuid) {
                    let prefix = &s[..uuid_start - 1];
                    // Basic validation: prefix should not be empty and should be alphanumeric + underscores
                    // But not allow prefixes that start/end with dash or are just dashes
                    if !prefix.is_empty()
                        && !prefix.starts_with('-')
                        && !prefix.ends_with('-')
                        && prefix.chars().any(|c| c.is_alphanumeric())
                        && prefix
                            .chars()
                            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                    {
                        return Ok(Self::new_prefixed(prefix, uuid));
                    }
                }
            }
        }

        Err(LettaIdError::InvalidFormat(s.to_string()))
    }
}

impl Serialize for LettaId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}

impl<'de> Deserialize<'de> for LettaId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Error type for LettaId parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LettaIdError {
    /// Invalid ID format.
    InvalidFormat(String),
}

impl fmt::Display for LettaIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(s) => write!(f, "Invalid LettaId format: {}", s),
        }
    }
}

impl std::error::Error for LettaIdError {}

// Convenience conversions for easier API usage
impl From<LettaId> for String {
    fn from(id: LettaId) -> Self {
        id.as_str()
    }
}

// Allow using &LettaId where &str is expected by converting to string
impl<'a> From<&'a LettaId> for String {
    fn from(id: &'a LettaId) -> Self {
        id.as_str()
    }
}

// Allow direct conversion from &str to LettaId for convenience
impl<'a> TryFrom<&'a str> for LettaId {
    type Error = LettaIdError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

/// Type alias for optional LettaId fields in structs.
/// This helps with backwards compatibility and allows easier migration.
pub type OptionalId = Option<LettaId>;

/// Convenience type alias for legacy code compatibility.
pub type ResourceId = String;

/// Timestamp type used throughout the API.
pub type Timestamp = DateTime<Utc>;

/// Generic pagination parameters for list operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationParams {
    /// Cursor for pagination (before this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Cursor for pagination (after this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Whether to return results in ascending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
}

impl PaginationParams {
    /// Create new pagination parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the before cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the after cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set ascending order.
    pub fn ascending(mut self, ascending: bool) -> Self {
        self.ascending = Some(ascending);
        self
    }
}

/// Pagination parameters for APIs that only support forward pagination.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AfterOnlyPaginationParams {
    /// Cursor for pagination (after this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl AfterOnlyPaginationParams {
    /// Create new pagination parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the after cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Generic response wrapper for paginated results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Total count of items (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    /// Cursor for the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Cursor for the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    /// Whether there are more items available.
    #[serde(default)]
    pub has_more: bool,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response.
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            total: None,
            next_cursor: None,
            prev_cursor: None,
            has_more: false,
        }
    }

    /// Get the number of items in this page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if this page is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get an iterator over the items.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
}

impl<T> IntoIterator for PaginatedResponse<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// Standard error response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error message.
    pub message: String,
    /// Optional error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Optional additional details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, serde_json::Value>>,
}

/// Health check response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Health status.
    pub status: String,
    /// Optional version information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Timestamp of the health check.
    pub timestamp: Timestamp,
}

/// Count response for count endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountResponse {
    /// The count value.
    pub count: u64,
}

/// Generic metadata for resources.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    /// Arbitrary key-value metadata.
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl Metadata {
    /// Create new empty metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a key-value pair.
    pub fn insert(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }

    /// Get a value by key.
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Check if metadata is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Sort order for list operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, SmartDefault)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending order.
    Asc,
    /// Descending order.
    #[default]
    Desc,
}

/// Common query parameters for list operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub pagination: PaginationParams,
    /// Optional search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Optional sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

impl ListParams {
    /// Create new list parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the search query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Set the sort field.
    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = Some(sort_by.into());
        self
    }

    /// Set the sort order.
    pub fn sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = Some(order);
        self
    }

    /// Set pagination parameters.
    pub fn pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Set the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.pagination.limit = Some(limit);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params() {
        let params = PaginationParams::new()
            .limit(10)
            .after("cursor123")
            .ascending(true);

        assert_eq!(params.limit, Some(10));
        assert_eq!(params.after.as_deref(), Some("cursor123"));
        assert_eq!(params.ascending, Some(true));
    }

    #[test]
    fn test_paginated_response() {
        let items = vec![1, 2, 3];
        let response = PaginatedResponse::new(items);

        assert_eq!(response.len(), 3);
        assert!(!response.is_empty());
        assert!(!response.has_more);
    }

    #[test]
    fn test_metadata() {
        let mut metadata = Metadata::new();
        assert!(metadata.is_empty());

        metadata.insert(
            "key".to_string(),
            serde_json::Value::String("value".to_string()),
        );
        assert!(!metadata.is_empty());
        assert!(metadata.get("key").is_some());
    }

    #[test]
    fn test_list_params() {
        let params = ListParams::new()
            .query("search term")
            .sort_by("created_at")
            .sort_order(SortOrder::Asc)
            .limit(20);

        assert_eq!(params.query.as_deref(), Some("search term"));
        assert_eq!(params.sort_by.as_deref(), Some("created_at"));
        assert!(matches!(params.sort_order, Some(SortOrder::Asc)));
        assert_eq!(params.pagination.limit, Some(20));
    }

    #[test]
    fn test_letta_id_bare_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = LettaId::from_str(uuid_str).unwrap();

        assert!(id.is_bare());
        assert_eq!(id.prefix(), None);
        assert_eq!(id.as_str(), uuid_str);
    }

    #[test]
    fn test_letta_id_prefixed() {
        let prefixed_str = "agent-550e8400-e29b-41d4-a716-446655440000";
        let id = LettaId::from_str(prefixed_str).unwrap();

        assert!(!id.is_bare());
        assert_eq!(id.prefix(), Some("agent"));
        assert_eq!(id.as_str(), prefixed_str);
    }

    #[test]
    fn test_letta_id_various_prefixes() {
        let test_cases = vec![
            "run-550e8400-e29b-41d4-a716-446655440000",
            "tool-550e8400-e29b-41d4-a716-446655440000",
            "source-550e8400-e29b-41d4-a716-446655440000",
            "block-550e8400-e29b-41d4-a716-446655440000",
            "memory_block-550e8400-e29b-41d4-a716-446655440000",
        ];

        for case in test_cases {
            let id = LettaId::from_str(case).unwrap();
            assert_eq!(id.as_str(), case);
        }
    }

    #[test]
    fn test_letta_id_invalid() {
        let invalid_cases = vec![
            "not-a-uuid",
            "agent-not-a-uuid",
            "-550e8400-e29b-41d4-a716-446655440000", // Empty prefix
            "agent--550e8400-e29b-41d4-a716-446655440000", // Double dash
        ];

        for case in invalid_cases {
            assert!(LettaId::from_str(case).is_err());
        }
    }

    #[test]
    fn test_letta_id_serialization() {
        let id = LettaId::from_str("agent-550e8400-e29b-41d4-a716-446655440000").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"agent-550e8400-e29b-41d4-a716-446655440000\"");

        let deserialized: LettaId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }
}
