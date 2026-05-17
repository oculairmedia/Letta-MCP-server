use letta::client::ClientBuilder;
use letta::types::memory::ArchivalMemoryQueryParams;
use letta::types::LettaId;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn search_params_hit_archival_search_endpoint() {
    let mock_server = MockServer::start().await;

    let agent_id: LettaId = "agent-00000000-0000-0000-0000-000000000001"
        .parse()
        .expect("valid agent id");

    let expected_search_path = format!("/v1/agents/{agent_id}/archival-memory/search");

    Mock::given(method("GET"))
        .and(path(expected_search_path))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_string(r#"{"results":[],"count":0}"#),
        )
        .mount(&mock_server)
        .await;

    let client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .expect("client builds");

    let params = ArchivalMemoryQueryParams {
        search: Some("cats".to_string()),
        limit: Some(5),
        before: None,
        after: None,
        ascending: None,
    };

    let result = client
        .memory()
        .list_archival_memory(&agent_id, Some(params))
        .await;
    assert!(
        result.is_ok(),
        "search call should succeed against archival search endpoint"
    );

    let requests = mock_server
        .received_requests()
        .await
        .expect("read requests");
    assert_eq!(requests.len(), 1, "expected exactly one outbound request");

    let request = &requests[0];
    assert_eq!(
        request.url.path(),
        format!("/v1/agents/{agent_id}/archival-memory/search"),
        "SDK should call dedicated archival search endpoint"
    );

    let query = request.url.query().unwrap_or_default().to_string();
    assert!(
        query.contains("query=cats"),
        "query should include search term as Letta 0.16 search param"
    );
    assert!(
        query.contains("top_k=5"),
        "query should map limit to Letta 0.16 top_k param"
    );
    assert!(
        !query.contains("search=") && !query.contains("limit="),
        "search endpoint should not receive list-endpoint param names"
    );
}

#[tokio::test]
async fn list_without_search_hits_archival_list_endpoint() {
    let mock_server = MockServer::start().await;

    let agent_id: LettaId = "agent-00000000-0000-0000-0000-000000000001"
        .parse()
        .expect("valid agent id");

    let expected_list_path = format!("/v1/agents/{agent_id}/archival-memory");

    Mock::given(method("GET"))
        .and(path(expected_list_path))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_string("[]"),
        )
        .mount(&mock_server)
        .await;

    let client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .expect("client builds");

    let result = client.memory().list_archival_memory(&agent_id, None).await;
    assert!(
        result.is_ok(),
        "list call should succeed against list endpoint"
    );

    let requests = mock_server
        .received_requests()
        .await
        .expect("read requests");
    assert_eq!(requests.len(), 1, "expected exactly one outbound request");
    assert_eq!(
        requests[0].url.path(),
        format!("/v1/agents/{agent_id}/archival-memory")
    );
}
