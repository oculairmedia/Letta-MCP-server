//! Group and multi-agent management types for the Letta API.

use crate::types::common::{LettaId, Timestamp};
use serde::{Deserialize, Serialize};

/// Manager type for multi-agent groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagerType {
    /// Round-robin message routing.
    RoundRobin,
    /// Supervisor-based coordination.
    Supervisor,
    /// Dynamic routing.
    Dynamic,
    /// Sleep-time based scheduling.
    Sleeptime,
    /// Voice-optimized sleep-time scheduling.
    VoiceSleeptime,
    /// Swarm-based coordination.
    Swarm,
}

/// Group representation for multi-agent conversations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Group ID.
    pub id: LettaId,
    /// Manager type for the group.
    pub manager_type: ManagerType,
    /// IDs of agents in the group.
    pub agent_ids: Vec<LettaId>,
    /// Group description.
    pub description: String,
    /// Shared memory block IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_block_ids: Option<Vec<LettaId>>,
    /// Manager agent ID (for supervisor/dynamic/sleeptime managers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_agent_id: Option<LettaId>,
    /// Termination token (for dynamic manager).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_token: Option<String>,
    /// Maximum turns per conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<i32>,
    /// Sleep-time agent frequency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleeptime_agent_frequency: Option<i32>,
    /// Turn counter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turns_counter: Option<i32>,
    /// Last processed message ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_processed_message_id: Option<LettaId>,
    /// Maximum message buffer length (voice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_message_buffer_length: Option<i32>,
    /// Minimum message buffer length (voice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_message_buffer_length: Option<i32>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// When the group was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the group was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Manager configuration for group creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "manager_type", rename_all = "snake_case")]
pub enum GroupCreateManagerConfig {
    /// Round-robin manager configuration.
    RoundRobin(RoundRobinManager),
    /// Supervisor manager configuration.
    Supervisor(SupervisorManager),
    /// Dynamic manager configuration.
    Dynamic(DynamicManager),
    /// Sleep-time manager configuration.
    Sleeptime(SleeptimeManager),
    /// Voice sleep-time manager configuration.
    VoiceSleeptime(VoiceSleeptimeManager),
}

/// Manager configuration for group updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "manager_type", rename_all = "snake_case")]
pub enum GroupUpdateManagerConfig {
    /// Round-robin manager update.
    RoundRobin(RoundRobinManagerUpdate),
    /// Supervisor manager update.
    Supervisor(SupervisorManagerUpdate),
    /// Dynamic manager update.
    Dynamic(DynamicManagerUpdate),
    /// Sleep-time manager update.
    Sleeptime(SleeptimeManagerUpdate),
    /// Voice sleep-time manager update.
    VoiceSleeptime(VoiceSleeptimeManagerUpdate),
}

/// Round-robin manager configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundRobinManager {
    /// Maximum turns per conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<i32>,
}

/// Round-robin manager update.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoundRobinManagerUpdate {
    /// Maximum turns per conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<i32>,
}

/// Supervisor manager configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorManager {
    /// The agent ID that acts as supervisor.
    pub manager_agent_id: LettaId,
}

/// Supervisor manager update.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupervisorManagerUpdate {
    /// The agent ID that acts as supervisor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_agent_id: Option<LettaId>,
}

/// Dynamic manager configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicManager {
    /// The agent ID that manages dynamic routing.
    pub manager_agent_id: LettaId,
    /// Token that signals conversation termination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_token: Option<String>,
    /// Maximum turns per conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<i32>,
}

/// Dynamic manager update.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DynamicManagerUpdate {
    /// The agent ID that manages dynamic routing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_agent_id: Option<LettaId>,
    /// Token that signals conversation termination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_token: Option<String>,
    /// Maximum turns per conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<i32>,
}

/// Sleep-time manager configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleeptimeManager {
    /// The agent ID that manages sleep-time scheduling.
    pub manager_agent_id: LettaId,
    /// Frequency of sleep-time agent activation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleeptime_agent_frequency: Option<i32>,
}

/// Sleep-time manager update.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SleeptimeManagerUpdate {
    /// The agent ID that manages sleep-time scheduling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_agent_id: Option<LettaId>,
    /// Frequency of sleep-time agent activation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleeptime_agent_frequency: Option<i32>,
}

/// Voice sleep-time manager configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSleeptimeManager {
    /// The agent ID that manages voice sleep-time.
    pub manager_agent_id: LettaId,
    /// Maximum message buffer length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_message_buffer_length: Option<i32>,
    /// Minimum message buffer length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_message_buffer_length: Option<i32>,
}

/// Voice sleep-time manager update.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoiceSleeptimeManagerUpdate {
    /// The agent ID that manages voice sleep-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_agent_id: Option<LettaId>,
    /// Maximum message buffer length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_message_buffer_length: Option<i32>,
    /// Minimum message buffer length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_message_buffer_length: Option<i32>,
}

/// Request to create a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupCreate {
    /// Agent IDs to include in the group.
    pub agent_ids: Vec<LettaId>,
    /// Group description.
    pub description: String,
    /// Manager configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_config: Option<GroupCreateManagerConfig>,
    /// Shared memory block IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_block_ids: Option<Vec<LettaId>>,
}

/// Request to update a group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupUpdate {
    /// Agent IDs to include in the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<LettaId>>,
    /// Group description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Manager configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_config: Option<GroupUpdateManagerConfig>,
    /// Shared memory block IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_block_ids: Option<Vec<LettaId>>,
}

/// Query parameters for listing groups.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupsListRequest {
    /// Filter by manager type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_type: Option<ManagerType>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Maximum number of groups to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Project ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_type_serialization() {
        assert_eq!(
            serde_json::to_string(&ManagerType::RoundRobin).unwrap(),
            "\"round_robin\""
        );
        assert_eq!(
            serde_json::to_string(&ManagerType::Supervisor).unwrap(),
            "\"supervisor\""
        );
        assert_eq!(
            serde_json::to_string(&ManagerType::VoiceSleeptime).unwrap(),
            "\"voice_sleeptime\""
        );
    }

    #[test]
    fn test_manager_config_serialization() {
        let config = GroupCreateManagerConfig::RoundRobin(RoundRobinManager {
            max_turns: Some(10),
        });
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"manager_type\":\"round_robin\""));
        assert!(json.contains("\"max_turns\":10"));
    }
}
