//! Integration tests for the local Letta server.

use letta::types::{
    AgentType, AgentsSearchRequest, CreateAgentRequest, ImportAgentRequest, ListAgentsParams,
    ToolRule,
};
use letta::{ClientConfig, LettaClient, LettaId};
use std::path::Path;
use std::str::FromStr;

#[tokio::test]
async fn test_local_server_agent_operations() {
    // Create client for local server (no auth required)
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Test 1: Create a new agent
    println!("Testing agent creation...");
    let create_request = CreateAgentRequest {
        name: Some("Rust SDK Test Agent".to_string()),
        system: Some("You are a test agent created by the Rust SDK integration test.".to_string()),
        agent_type: Some(AgentType::MemGPT),
        model: Some("letta/letta-free".to_string()),
        embedding: Some("letta/letta-free".to_string()),
        tags: Some(vec![
            "rust-sdk-test".to_string(),
            "integration-test".to_string(),
        ]),
        ..Default::default()
    };

    let created_agent = client.agents().create(create_request).await.unwrap();
    println!("Created agent with ID: {}", created_agent.id);
    assert_eq!(created_agent.name, "Rust SDK Test Agent");
    assert_eq!(created_agent.agent_type, AgentType::MemGPT);

    // Test 5: Verify the created agent exists
    let retrieved_agent = client.agents().get(&created_agent.id).await.unwrap();
    assert_eq!(retrieved_agent.id, created_agent.id);
    assert_eq!(retrieved_agent.name, created_agent.name);

    // Test 2: List agents
    println!("Testing agent list...");
    let agents = client.agents().list(None).await.unwrap();
    println!("Found {} agents", agents.len());
    assert!(!agents.is_empty(), "Server should have at least one agent");

    // Test 3: Get a specific agent
    let first_agent = &agents[0];
    println!("Testing agent get for ID: {}", first_agent.id);
    let agent = client.agents().get(&first_agent.id).await.unwrap();
    assert_eq!(agent.id, first_agent.id);

    // Test 4: List with parameters
    println!("Testing agent list with parameters...");
    let params = ListAgentsParams::builder().limit(1).build();
    let limited_agents = client.agents().list(Some(params)).await.unwrap();
    assert_eq!(limited_agents.len(), 1);

    // Test 5: Clean up - delete the created agent
    println!("Cleaning up - deleting test agent...");
    client.agents().delete(&created_agent.id).await.unwrap();

    // Test 6: Test summarize conversation (before deletion)
    // First recreate an agent since we deleted it
    let create_request2 = CreateAgentRequest {
        name: Some("Summarize Test Agent".to_string()),
        system: Some("You are a test agent for summarization testing.".to_string()),
        agent_type: Some(AgentType::MemGPT),
        model: Some("letta/letta-free".to_string()),
        embedding: Some("letta/letta-free".to_string()),
        tags: Some(vec!["summarize-test".to_string()]),
        ..Default::default()
    };

    let summarize_agent = client.agents().create(create_request2).await.unwrap();
    println!("Testing agent conversation summarization...");

    // Test summarize with a reasonable max_message_length
    let summarized_agent = client
        .agents()
        .summarize_agent_conversation(&summarize_agent.id, 10)
        .await
        .unwrap();

    assert_eq!(summarized_agent.id, summarize_agent.id);
    println!("✅ Agent conversation summarized successfully");

    // Clean up the summarize test agent
    client.agents().delete(&summarize_agent.id).await.unwrap();

    // Test 8: Test count and export_file
    println!("Testing agent count...");
    let count = client.agents().count().await.unwrap();
    println!("✅ Agent count: {}", count);

    // Test export_file and import_file with the first agent we can find
    let current_agents = client.agents().list(None).await.unwrap();
    if !current_agents.is_empty() {
        let first_agent = &current_agents[0];
        println!("Testing agent export for ID: {}", first_agent.id);
        let exported_data = client.agents().export_file(&first_agent.id).await.unwrap();
        println!(
            "✅ Agent exported successfully, data length: {}",
            exported_data.len()
        );
        assert!(
            !exported_data.is_empty(),
            "Exported data should not be empty"
        );

        // Test import_file: write exported data to temp file and import it back
        let temp_file_path = "/tmp/test_agent_export.json";
        tokio::fs::write(temp_file_path, &exported_data)
            .await
            .unwrap();

        let import_request = ImportAgentRequest {
            append_copy_suffix: Some(true),
            strip_messages: Some(true),
            ..Default::default()
        };

        println!("Testing agent import from file...");
        let imported_agent = client
            .agents()
            .import_file(Path::new(temp_file_path), import_request)
            .await
            .unwrap();

        println!(
            "✅ Agent imported successfully with ID: {}",
            imported_agent.id
        );
        assert!(
            imported_agent.name.contains("copy") || imported_agent.name.contains("Copy"),
            "Imported agent should have copy suffix"
        );

        // Clean up the imported agent and temp file
        client.agents().delete(&imported_agent.id).await.unwrap();
        tokio::fs::remove_file(temp_file_path).await.ok();
        println!("✅ Cleaned up imported agent and temp file");
    }

    // Test search functionality (skip for local server - not supported)
    println!("Testing agent search...");
    let search_request = AgentsSearchRequest {
        limit: Some(5),
        ..Default::default()
    };

    match client.agents().search(search_request).await {
        Ok(search_results) => {
            println!("✅ Search returned {} agents", search_results.agents.len());
            assert!(
                !search_results.agents.is_empty(),
                "Search should return at least one agent"
            );
        }
        Err(err) => {
            println!(
                "⚠️  Search not supported on local server (Method Not Allowed): {}",
                err
            );
        }
    }

    // Test 9: Verify original deletion
    let delete_result = client.agents().get(&created_agent.id).await;
    assert!(
        delete_result.is_err(),
        "Agent should be deleted and not found"
    );

    println!("✅ All local server tests passed!");
}

#[tokio::test]
async fn test_local_server_error_handling() {
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Test getting a non-existent agent
    let fake_id = LettaId::from_str("agent-00000000-0000-0000-0000-000000000000").unwrap();
    let result = client.agents().get(&fake_id).await;
    assert!(
        result.is_err(),
        "Should return error for non-existent agent"
    );

    // Test deleting a non-existent agent
    let result = client.agents().delete(&fake_id).await;
    assert!(
        result.is_err(),
        "Should return error when deleting non-existent agent"
    );

    println!("✅ Error handling tests passed!");
}

#[test]
fn test_tool_rule_serialization() {
    // Test that our tool rule enum serializes correctly
    let rules = vec![
        ToolRule::ContinueLoop {
            tool_name: "test_tool".to_string(),
            prompt_template: None,
        },
        ToolRule::MaxCountPerStep {
            tool_name: "limited_tool".to_string(),
            prompt_template: Some("Limited to {{max_count_limit}} calls".to_string()),
            max_count_limit: 3,
        },
        ToolRule::Conditional {
            tool_name: "conditional_tool".to_string(),
            prompt_template: None,
            default_child: Some("default_action".to_string()),
            child_output_mapping: std::collections::HashMap::from([
                ("success".to_string(), "success_handler".to_string()),
                ("error".to_string(), "error_handler".to_string()),
            ]),
            require_output_mapping: true,
        },
    ];

    for rule in rules {
        let json = serde_json::to_string_pretty(&rule).unwrap();
        println!("Tool rule JSON:\n{}\n", json);

        // Verify we can deserialize it back
        let deserialized: ToolRule = serde_json::from_str(&json).unwrap();

        // Verify the deserialized rule matches the original
        match (&rule, &deserialized) {
            (
                ToolRule::ContinueLoop { tool_name: t1, .. },
                ToolRule::ContinueLoop { tool_name: t2, .. },
            ) => {
                assert_eq!(t1, t2);
            }
            (
                ToolRule::MaxCountPerStep {
                    max_count_limit: l1,
                    ..
                },
                ToolRule::MaxCountPerStep {
                    max_count_limit: l2,
                    ..
                },
            ) => {
                assert_eq!(l1, l2);
            }
            (
                ToolRule::Conditional {
                    require_output_mapping: r1,
                    ..
                },
                ToolRule::Conditional {
                    require_output_mapping: r2,
                    ..
                },
            ) => {
                assert_eq!(r1, r2);
            }
            _ => panic!("Rule type mismatch"),
        }
    }

    println!("✅ Tool rule serialization tests passed!");
}
