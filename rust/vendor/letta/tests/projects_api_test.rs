//! Integration tests for the Projects API (Cloud only).

use letta::client::LettaClient;
use std::env;

/// Get a test client for Letta Cloud.
fn get_cloud_client() -> LettaClient {
    let api_key = env::var("LETTA_API_KEY").expect("LETTA_API_KEY must be set for cloud tests");
    LettaClient::cloud(&api_key).unwrap()
}

#[tokio::test]
#[ignore = "Requires cloud API key"]
async fn test_list_projects() {
    let client = get_cloud_client();

    // List all projects
    let result = client.projects().list(None, None, None).await;

    let response = result.expect("Failed to list projects");

    println!("Found {} projects", response.projects.len());

    for project in response.projects.iter() {
        println!(
            "Project: {} (slug: {}, id: {})",
            project.name, project.slug, project.id
        );

        // Verify project fields
        assert!(!project.name.is_empty(), "Project name should not be empty");
        assert!(!project.slug.is_empty(), "Project slug should not be empty");
        assert!(!project.id.is_empty(), "Project ID should not be empty");
    }

    println!("Has next page: {}", response.has_next_page);
}

#[tokio::test]
#[ignore = "Requires cloud API key"]
async fn test_list_projects_with_pagination() {
    let client = get_cloud_client();

    // List projects with limit
    let result = client
        .projects()
        .list(None, None, Some("2".to_string()))
        .await;

    let response = result.expect("Failed to list projects with pagination");

    println!("Found {} projects (limited to 2)", response.projects.len());
    assert!(
        response.projects.len() <= 2,
        "Should return at most 2 projects when limit is 2"
    );

    if response.has_next_page {
        println!("There are more projects available");
    }
}

#[tokio::test]
#[ignore = "Requires cloud API key"]
async fn test_list_projects_by_name() {
    let client = get_cloud_client();

    // First, list all projects to get a name to search for
    let all_projects = client.projects().list(None, None, None).await;

    let all_response = all_projects.expect("Failed to list all projects");

    if all_response.projects.is_empty() {
        println!("No projects found to search for, skipping test");
        return;
    }

    let target_name = &all_response.projects[0].name;
    println!("Searching for project with name: {}", target_name);

    // Search by name
    let search_response = client
        .projects()
        .list(Some(target_name.clone()), None, None)
        .await
        .expect("Failed to search projects by name");

    println!(
        "Found {} projects matching name '{}'",
        search_response.projects.len(),
        target_name
    );

    // Verify search results
    assert!(
        !search_response.projects.is_empty(),
        "Should find at least one project when searching for existing name"
    );

    for project in &search_response.projects {
        assert!(
            project.name.contains(target_name),
            "Project name '{}' should contain search term '{}'",
            project.name,
            target_name
        );
        println!("  Matched: {} ({})", project.name, project.slug);
    }
}
