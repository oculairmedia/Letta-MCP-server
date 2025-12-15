//! Identity-related types.

use bon::Builder;
use serde::{Deserialize, Serialize};

use super::LettaId;

/// Identity type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IdentityType {
    /// Organization identity.
    Org,
    /// User identity.
    User,
    /// Other identity type.
    Other,
}

impl std::fmt::Display for IdentityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Org => write!(f, "org"),
            Self::User => write!(f, "user"),
            Self::Other => write!(f, "other"),
        }
    }
}

/// Property value in an identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProperty {
    /// Property key.
    pub key: String,
    /// Property value (any JSON value).
    pub value: serde_json::Value,
    /// Property type.
    #[serde(rename = "type")]
    pub property_type: String,
}

/// Identity represents a user, organization, or other entity in Letta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// Identity ID (prefixed with "identity-").
    pub id: LettaId,
    /// Unique identifier key.
    pub identifier_key: String,
    /// Identity name.
    pub name: String,
    /// Identity type.
    pub identity_type: IdentityType,
    /// Agent IDs associated with this identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<LettaId>>,
    /// Block IDs associated with this identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<Vec<LettaId>>,
    /// Project ID this identity belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<LettaId>,
    /// Identity properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<IdentityProperty>>,
}

/// Request to create a new identity.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct CreateIdentityRequest {
    /// Unique identifier key.
    pub identifier_key: String,
    /// Identity name.
    pub name: String,
    /// Identity type.
    pub identity_type: IdentityType,
    /// Project ID this identity belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Agent IDs to associate with this identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    /// Block IDs to associate with this identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<Vec<String>>,
    /// Identity properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<IdentityProperty>>,
}

/// Request to update an identity.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateIdentityRequest {
    /// Unique identifier key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_key: Option<String>,
    /// Identity name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identity type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<IdentityType>,
    /// Agent IDs to associate with this identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    /// Block IDs to associate with this identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<Vec<String>>,
    /// Identity properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<IdentityProperty>>,
}

/// Query parameters for listing identities.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListIdentitiesParams {
    /// Filter by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Filter by identifier key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_key: Option<String>,
    /// Filter by identity type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<IdentityType>,
    /// Cursor for pagination (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Cursor for pagination (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Maximum number of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_type_serialization() {
        // Test serialization
        let user_json = serde_json::to_string(&IdentityType::User).unwrap();
        assert_eq!(user_json, "\"user\"");

        let org_json = serde_json::to_string(&IdentityType::Org).unwrap();
        assert_eq!(org_json, "\"org\"");

        let other_json = serde_json::to_string(&IdentityType::Other).unwrap();
        assert_eq!(other_json, "\"other\"");

        // Test deserialization
        let user: IdentityType = serde_json::from_str("\"user\"").unwrap();
        assert_eq!(user, IdentityType::User);

        let org: IdentityType = serde_json::from_str("\"org\"").unwrap();
        assert_eq!(org, IdentityType::Org);

        let other: IdentityType = serde_json::from_str("\"other\"").unwrap();
        assert_eq!(other, IdentityType::Other);
    }
}
