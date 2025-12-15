//! Integration tests for the Jobs & Steps API.

use letta::client::{ClientConfig, LettaClient};
use letta::types::*;

/// Get a test client for the local server.
fn get_test_client() -> LettaClient {
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    LettaClient::new(config).unwrap()
}

#[tokio::test]
async fn test_list_jobs() {
    let client = get_test_client();

    // List jobs (might be empty)
    let result = client.jobs().list(None, None, None).await;

    let jobs = result.expect("Failed to list jobs");

    println!("Found {} jobs", jobs.len());
    for job in jobs.iter().take(5) {
        println!("Job: {:?} - Status: {:?}", job.id, job.status);
    }
}

#[tokio::test]
#[ignore = "GET /v1/steps returns 500 on local server - server bug with 'tags' keyword argument"]
async fn test_step_api() {
    let client = get_test_client();

    // List all steps - this should work but currently returns 500
    let steps = client
        .steps()
        .list(None)
        .await
        .expect("Step listing should work");

    println!("Found {} total steps", steps.len());

    // If we have steps, test getting a specific one
    if let Some(step) = steps.first() {
        let fetched_step = client
            .steps()
            .get(&step.id)
            .await
            .expect("Getting specific step should work");
        println!("Fetched step: {:?}", fetched_step.id);
        assert_eq!(step.id, fetched_step.id);
    }
}

#[tokio::test]
#[ignore = "GET /v1/steps returns 500 on local server - server bug with 'tags' keyword argument"]
async fn test_step_feedback() {
    let client = get_test_client();

    // First, we need to get a step to provide feedback on
    let steps = client
        .steps()
        .list(Some(ListStepsParams {
            limit: Some(1),
            ..Default::default()
        }))
        .await
        .expect("Step listing should work");

    if let Some(step) = steps.first() {
        let response = client
            .steps()
            .provide_feedback(&step.id, StepFeedback::Positive)
            .await
            .expect("Providing feedback should work");
        println!("Feedback response: {}", response);
    } else {
        println!("No steps available to provide feedback on");
    }
}

#[tokio::test]
async fn test_list_jobs_with_filters() {
    let client = get_test_client();

    // Test listing with status filter
    let running_jobs = client
        .jobs()
        .list(Some(JobStatus::Running), None, None)
        .await
        .expect("Failed to list running jobs");

    println!("Found {} running jobs", running_jobs.len());
    for job in &running_jobs {
        assert_eq!(job.status, Some(JobStatus::Running));
    }

    // Test listing with limit
    let limited_jobs = client
        .jobs()
        .list(None, Some(5), None)
        .await
        .expect("Failed to list jobs with limit");

    println!("Found {} jobs (limited to 5)", limited_jobs.len());
    assert!(limited_jobs.len() <= 5);
}
