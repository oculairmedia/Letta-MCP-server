//! Integration tests for pagination functionality.

use futures::StreamExt;
use letta::client::{ClientConfig, LettaClient};
use letta::error::LettaResult;
use letta::types::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_agent_pagination() -> LettaResult<()> {
    let config = ClientConfig::new("http://localhost:8283")?;
    let client = LettaClient::new(config)?;

    // First, create some test agents to ensure we have something to paginate
    println!("Creating test agents for pagination...");
    let mut created_agents = Vec::new();
    for i in 1..=5 {
        let request = CreateAgentRequest::builder()
            .name(format!("Pagination Test Agent {}", i))
            .agent_type(AgentType::MemGPT)
            .model("letta/letta-free")
            .embedding("letta/letta-free")
            .build();

        let agent = client.agents().create(request).await?;
        println!("  Created agent: {} ({})", agent.name, agent.id);
        created_agents.push(agent.id);
    }

    // Test 1: Use pagination to iterate through agents
    println!("\nTesting paginated iteration with limit 2...");
    let mut stream = client
        .agents()
        .paginated(Some(PaginationParams::new().limit(2)));

    let mut count = 0;
    while let Some(result) = stream.next().await {
        let agent = result?;
        println!("  Found agent: {} ({})", agent.name, agent.id);
        count += 1;
    }

    println!("Total agents found via pagination: {}", count);
    assert!(count >= 5, "Should find at least the 5 agents we created");

    // Test 2: Use pagination with filter
    println!("\nTesting pagination with filter...");
    let filtered_agents: Vec<_> = client
        .agents()
        .paginated(Some(PaginationParams::new().limit(3)))
        .filter(|agent| agent.name.contains("Pagination Test"))
        .take(3)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    println!("Found {} agents matching filter", filtered_agents.len());
    assert!(
        filtered_agents.len() >= 3,
        "Should find at least 3 test agents"
    );

    // Test 3: Collect all agents at once
    println!("\nTesting collect all agents...");
    let all_agents = client
        .agents()
        .paginated(Some(PaginationParams::new().limit(10)))
        .collect()
        .await?;

    println!("Collected {} agents total", all_agents.len());
    assert!(
        all_agents.len() >= 5,
        "Should have at least our test agents"
    );

    // Cleanup: delete the test agents
    println!("\nCleaning up test agents...");
    for agent_id in created_agents {
        client.agents().delete(&agent_id).await?;
        println!("  Deleted agent: {}", agent_id);
    }

    println!("✅ Pagination tests completed!");
    Ok(())
}

#[tokio::test]
#[ignore = "Requires many agents to test real pagination"]
async fn test_real_pagination_with_cursor() -> LettaResult<()> {
    // This test would require having many agents (>100) to actually test
    // cursor-based pagination across multiple pages.

    let config = ClientConfig::new("http://localhost:8283")?;
    let client = LettaClient::new(config)?;

    // Test with a small limit to force multiple pages
    let agents: Vec<_> = client
        .agents()
        .paginated(Some(PaginationParams::new().limit(5)))
        .take(15) // Take only first 15 even if there are more
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    println!("Collected {} agents using cursor pagination", agents.len());

    // Verify we got distinct agents (no duplicates from pagination)
    let unique_ids: std::collections::HashSet<_> = agents.iter().map(|a| &a.id).collect();
    assert_eq!(
        unique_ids.len(),
        agents.len(),
        "All agents should be unique"
    );

    Ok(())
}
