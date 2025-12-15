//! Test agent deserialization step by step.

use letta::types::AgentState;

#[tokio::test]
async fn test_agent_deserialization() {
    // Get raw JSON from server
    let response = reqwest::get("http://localhost:8283/v1/agents/")
        .await
        .unwrap();

    let text = response.text().await.unwrap();
    let agents: serde_json::Value = serde_json::from_str(&text).unwrap();

    if let Some(array) = agents.as_array() {
        if !array.is_empty() {
            let first_agent = &array[0];
            println!(
                "First agent JSON:\n{}",
                serde_json::to_string_pretty(first_agent).unwrap()
            );

            // Try to deserialize this specific agent
            match serde_json::from_value::<AgentState>(first_agent.clone()) {
                Ok(agent) => {
                    println!("✅ Successfully deserialized agent: {}", agent.name);
                }
                Err(e) => {
                    println!("❌ Failed to deserialize agent: {}", e);
                }
            }
        }
    }
}
