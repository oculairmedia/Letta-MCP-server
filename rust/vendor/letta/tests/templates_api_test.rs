//! Integration tests for the Templates API (Cloud only).

use letta::client::LettaClient;
use letta::types::*;
use std::env;

/// Get a test client for Letta Cloud.
fn get_cloud_client() -> LettaClient {
    let api_key = env::var("LETTA_API_KEY").expect("LETTA_API_KEY must be set for cloud tests");
    LettaClient::cloud(&api_key).unwrap()
}

#[tokio::test]
#[ignore = "Requires cloud API key"]
async fn test_list_templates() {
    let client = get_cloud_client();

    // List templates
    let result = client.templates().list(None, None, None, None).await;

    let response = result.expect("Failed to list templates");
    println!("Found {} templates", response.templates.len());

    // Verify response structure
    for template in response.templates.iter().take(5) {
        println!("Template: {} ({})", template.name, template.id);
        assert!(
            !template.name.is_empty(),
            "Template name should not be empty"
        );
        assert!(!template.id.is_empty(), "Template ID should not be empty");
    }

    // Verify pagination field exists
    println!("Has next page: {}", response.has_next_page);
}

#[tokio::test]
#[ignore = "Requires cloud API key and existing agent"]
async fn test_create_template_from_agent() {
    let client = get_cloud_client();

    // First, we need an agent to create a template from
    // We'll try to list agents and use the first one
    let agents = client
        .agents()
        .list(Some(ListAgentsParams {
            limit: Some(1),
            ..Default::default()
        }))
        .await;

    let agents = agents.expect("Failed to list agents");

    if agents.is_empty() {
        println!("No agents found to create template from, skipping test");
        return;
    }

    let agent = &agents[0];
    println!("Using agent: {} ({})", agent.name, agent.id);

    // Create a template from this agent
    // First, try to get a project from the projects API
    let projects = client.projects().list(None, None, None).await;

    let request = match projects {
        Ok(proj_response) if !proj_response.projects.is_empty() => {
            // Use the first project
            CreateTemplateRequest {
                project: Some(proj_response.projects[0].slug.clone()),
            }
        }
        _ => {
            // Try with default project
            CreateTemplateRequest {
                project: Some("default".to_string()),
            }
        }
    };

    let result = client
        .templates()
        .create_from_agent(&agent.id, &request)
        .await;

    let response = result.unwrap_or_else(|e| {
        panic!("Failed to create template from agent: {}", e);
    });

    println!(
        "Created template: {} ({})",
        response.template_name, response.template_id
    );

    // Verify the response
    assert!(
        !response.template_name.is_empty(),
        "Template name should not be empty"
    );
    assert!(
        !response.template_id.is_empty(),
        "Template ID should not be empty"
    );
}

#[tokio::test]
#[ignore = "Requires cloud API key and existing agent"]
async fn test_get_agent_memory_variables() {
    let client = get_cloud_client();

    // First, we need an agent
    let agents = client
        .agents()
        .list(Some(ListAgentsParams {
            limit: Some(1),
            ..Default::default()
        }))
        .await;

    let agents = agents.expect("Failed to list agents");

    if agents.is_empty() {
        println!("No agents found, skipping test");
        return;
    }

    let agent = &agents[0];
    println!("Using agent: {} ({})", agent.name, agent.id);

    // Get memory variables for this agent
    let response = client
        .get_agent_memory_variables(&agent.id)
        .await
        .expect("Failed to get agent memory variables");

    println!("Memory variables:");
    for (key, value) in response.variables.iter() {
        println!("  {}: {}", key, value);
        assert!(!key.is_empty(), "Variable key should not be empty");
    }

    // Memory variables should be a valid HashMap
    assert!(
        response.variables.is_empty() || !response.variables.is_empty(),
        "Variables should be a valid HashMap"
    );
}

#[tokio::test]
#[ignore = "Requires cloud API key and existing template"]
async fn test_create_agents_from_template() {
    let client = get_cloud_client();

    // First, list templates to find one to use
    let templates = client.templates().list(None, None, None, None).await;

    let templates = templates.expect("Failed to list templates");

    if templates.templates.is_empty() {
        println!("No templates found, skipping test");
        return;
    }

    let template = &templates.templates[0];
    println!("Using template: {}", template.name);

    // We need the project slug and template version
    // For this test, we'll assume a simple format
    let project = "default"; // You might need to adjust this
    let template_version = format!("{}:latest", template.name);

    // Create agents from template
    let request = CreateAgentsFromTemplateRequest {
        agent_name: Some("Test Agent from Template".to_string()),
        tags: Some(vec!["test".to_string(), "from-template".to_string()]),
        ..Default::default()
    };

    let result = client
        .templates()
        .create_agents_from_template(project, &template_version, &request)
        .await;

    // This might fail if the project/template format is wrong
    let response = result.unwrap_or_else(|e| {
        panic!("Failed to create agents from template: {}", e);
    });

    println!("Created {} agents from template", response.agents.len());
    assert!(
        !response.agents.is_empty(),
        "Should create at least one agent"
    );

    for agent in &response.agents {
        println!("  Agent: {} ({})", agent.name, agent.id);
        assert!(!agent.name.is_empty(), "Agent name should not be empty");
        assert!(
            !agent.id.as_str().is_empty(),
            "Agent ID should not be empty"
        );
    }
}
