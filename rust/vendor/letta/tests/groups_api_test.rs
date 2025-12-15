//! Integration tests for the Groups API.

use letta::client::{ClientConfig, LettaClient};
use letta::types::*;

/// Get a test client for the local server.
fn get_test_client() -> LettaClient {
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    LettaClient::new(config).unwrap()
}

#[tokio::test]
async fn test_list_groups() {
    let client = get_test_client();

    // List groups (might be empty)
    let result = client.groups().list(None).await;

    let groups = result.expect("Failed to list groups");

    println!("Found {} groups", groups.len());
    for group in groups.iter().take(5) {
        println!("Group: {:?} - Type: {:?}", group.id, group.manager_type);
    }
}

#[tokio::test]
async fn test_create_round_robin_group() {
    let client = get_test_client();

    // First, create two agents for the group
    let agent1 = client
        .agents()
        .create(CreateAgentRequest {
            name: Some("Group Test Agent 1".to_string()),
            model: Some("letta/letta-free".to_string()),
            embedding: Some("letta/letta-free".to_string()),
            agent_type: Some(AgentType::MemGPT),
            system: Some("You are agent 1 in a group test.".to_string()),
            tools: Some(vec!["send_message".to_string()]),
            ..Default::default()
        })
        .await
        .expect("Failed to create agent 1");

    let agent2 = client
        .agents()
        .create(CreateAgentRequest {
            name: Some("Group Test Agent 2".to_string()),
            model: Some("letta/letta-free".to_string()),
            embedding: Some("letta/letta-free".to_string()),
            agent_type: Some(AgentType::MemGPT),
            system: Some("You are agent 2 in a group test.".to_string()),
            tools: Some(vec!["send_message".to_string()]),
            ..Default::default()
        })
        .await
        .expect("Failed to create agent 2");

    // Create a round-robin group
    let group_request = GroupCreate {
        agent_ids: vec![agent1.id.clone(), agent2.id.clone()],
        description: "Test round-robin group".to_string(),
        manager_config: Some(GroupCreateManagerConfig::RoundRobin(RoundRobinManager {
            max_turns: Some(10),
        })),
        shared_block_ids: None,
    };

    let group = client
        .groups()
        .create(group_request)
        .await
        .expect("Failed to create group");

    println!("Created group: {:?}", group.id);
    println!("Manager type: {:?}", group.manager_type);
    println!("Agent count: {}", group.agent_ids.len());

    // Test getting the group
    let fetched_group = client
        .groups()
        .get(&group.id)
        .await
        .expect("Failed to get group");
    assert_eq!(group.id, fetched_group.id);
    assert_eq!(group.manager_type, fetched_group.manager_type);

    // Test sending a message to the group
    let message_result = client
        .groups()
        .send_message(
            &group.id,
            vec![MessageCreate {
                role: MessageRole::User,
                content: "Hello group!".into(),
                ..Default::default()
            }],
        )
        .await;

    match message_result {
        Ok(response) => {
            println!("Group responded with {} messages", response.messages.len());
        }
        Err(e) => {
            // This is expected to fail on local server (returns 500)
            println!(
                "Note: Group messaging returned error (expected on local server): {}",
                e
            );
        }
    }

    // Clean up
    let _ = client.groups().delete(&group.id).await;

    // Clean up agents
    let _ = client.agents().delete(&agent1.id).await;
    let _ = client.agents().delete(&agent2.id).await;
}

#[tokio::test]
async fn test_group_agent_management() {
    let client = get_test_client();

    // Create three agents
    let mut agents = Vec::new();
    for i in 1..=3 {
        let agent = client
            .agents()
            .create(CreateAgentRequest {
                name: Some(format!("Group Management Agent {}", i)),
                model: Some("letta/letta-free".to_string()),
                embedding: Some("letta/letta-free".to_string()),
                agent_type: Some(AgentType::MemGPT),
                system: Some(format!("You are agent {} for group management test.", i)),
                tools: Some(vec!["send_message".to_string()]),
                ..Default::default()
            })
            .await
            .expect(&format!("Failed to create agent {}", i));
        agents.push(agent);
    }

    // Create a group with first two agents
    let group_request = GroupCreate {
        agent_ids: vec![agents[0].id.clone(), agents[1].id.clone()],
        description: "Test group for agent management".to_string(),
        manager_config: Some(GroupCreateManagerConfig::RoundRobin(RoundRobinManager {
            max_turns: None,
        })),
        shared_block_ids: None,
    };

    let group = client
        .groups()
        .create(group_request)
        .await
        .expect("Failed to create group");

    println!("Created group with {} agents", group.agent_ids.len());

    // List agents in group
    let group_agents = client
        .groups()
        .get(&group.id)
        .await
        .expect("Failed to list agents in group");
    println!("Group has {} agents", group_agents.agent_ids.len());
    assert_eq!(group_agents.agent_ids.len(), 2);

    // Add the third agent
    client
        .groups()
        .update(
            &group.id,
            GroupUpdate {
                agent_ids: Some(vec![
                    agents[0].id.clone(),
                    agents[1].id.clone(),
                    agents[2].id.clone(),
                ]),
                manager_config: Some(GroupUpdateManagerConfig::RoundRobin(
                    RoundRobinManagerUpdate { max_turns: None },
                )),
                shared_block_ids: group_agents.shared_block_ids.clone(),
                description: Some(group_agents.description.clone()),
            },
        )
        .await
        .expect("Failed to add third agent to group");
    println!("Added third agent to group");

    // Verify the agent was added
    let updated_group = client
        .groups()
        .get(&group.id)
        .await
        .expect("Failed to get updated group after adding agent");
    assert_eq!(updated_group.agent_ids.len(), 3);

    // remove an agent
    client
        .groups()
        .update(
            &group.id,
            GroupUpdate {
                agent_ids: Some(vec![agents[1].id.clone(), agents[2].id.clone()]),
                manager_config: Some(GroupUpdateManagerConfig::RoundRobin(
                    RoundRobinManagerUpdate { max_turns: None },
                )),
                shared_block_ids: group_agents.shared_block_ids.clone(),
                description: Some(group_agents.description.clone()),
            },
        )
        .await
        .expect("Failed to update group configuration");

    // Verify the agent was removed
    let updated_group = client
        .groups()
        .get(&group.id)
        .await
        .expect("Failed to get updated group after removing agent");
    assert_eq!(updated_group.agent_ids.len(), 2);

    // Clean up group
    let _ = client.groups().delete(&group.id).await;

    // Clean up agents
    for agent in agents {
        let _ = client.agents().delete(&agent.id).await;
    }
}

#[tokio::test]
#[ignore = "Supervisor groups require a specific supervisor agent setup"]
async fn test_create_supervisor_group() {
    let _client = get_test_client();

    // This test would require:
    // 1. Creating a supervisor agent with special capabilities
    // 2. Creating worker agents
    // 3. Creating a supervisor-type group

    // For now, just test the type serialization
    let manager_config = GroupCreateManagerConfig::Supervisor(SupervisorManager {
        manager_agent_id: LettaId::new_prefixed("agent", uuid::Uuid::new_v4()),
    });

    let json = serde_json::to_string(&manager_config).unwrap();
    assert!(json.contains("\"manager_type\":\"supervisor\""));
}

#[tokio::test]
async fn test_agent_list_groups() {
    let client = get_test_client();

    // Create an agent
    let agent = client
        .agents()
        .create(CreateAgentRequest {
            name: Some("Agent Groups List Test".to_string()),
            model: Some("letta/letta-free".to_string()),
            embedding: Some("letta/letta-free".to_string()),
            agent_type: Some(AgentType::MemGPT),
            system: Some("You are an agent to test group listing.".to_string()),
            tools: Some(vec!["send_message".to_string()]),
            ..Default::default()
        })
        .await
        .expect("Failed to create agent");

    // Create another agent for the group
    let agent2 = client
        .agents()
        .create(CreateAgentRequest {
            name: Some("Agent Groups List Test 2".to_string()),
            model: Some("letta/letta-free".to_string()),
            embedding: Some("letta/letta-free".to_string()),
            agent_type: Some(AgentType::MemGPT),
            system: Some("You are the second agent in the group.".to_string()),
            tools: Some(vec!["send_message".to_string()]),
            ..Default::default()
        })
        .await
        .expect("Failed to create agent 2");

    // Create a group with both agents
    let group = client
        .groups()
        .create(GroupCreate {
            agent_ids: vec![agent.id.clone(), agent2.id.clone()],
            description: "Test group for agent list groups".to_string(),
            manager_config: Some(GroupCreateManagerConfig::RoundRobin(RoundRobinManager {
                max_turns: Some(5),
            })),
            shared_block_ids: None,
        })
        .await
        .expect("Failed to create group");

    // List groups for the first agent
    let groups = client
        .agents()
        .list_groups(&agent.id)
        .await
        .expect("Failed to list agent groups");

    println!("Agent {} belongs to {} groups", agent.id, groups.len());
    assert!(groups.len() >= 1, "Agent should belong to at least 1 group");

    // Verify our group is in the list
    assert!(
        groups.iter().any(|g| g.id == group.id),
        "Agent's group list should contain the created group"
    );

    // Clean up
    let _ = client.groups().delete(&group.id).await;
    let _ = client.agents().delete(&agent.id).await;
    let _ = client.agents().delete(&agent2.id).await;
}
