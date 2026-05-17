use letta::client::ClientBuilder;
use letta::types::LettaId;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn archive_delete_accepts_204_no_content() {
    let mock_server = MockServer::start().await;
    let archive_id: LettaId = "archive-00000000-0000-0000-0000-000000000001"
        .parse()
        .expect("valid archive id");

    Mock::given(method("DELETE"))
        .and(path(format!("/v1/archives/{archive_id}")))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .expect("client builds");

    let result = client.archives().delete(&archive_id).await;
    assert!(result.is_ok(), "204 No Content delete should succeed");
}

#[tokio::test]
async fn mcp_refresh_sends_agent_id_when_provided() {
    let mock_server = MockServer::start().await;
    let server_id: LettaId = "mcp-server-00000000-0000-0000-0000-000000000001"
        .parse()
        .expect("valid MCP server id");
    let agent_id: LettaId = "agent-00000000-0000-0000-0000-000000000001"
        .parse()
        .expect("valid agent id");

    Mock::given(method("PATCH"))
        .and(path(format!("/v1/mcp-servers/{server_id}/refresh")))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_string("null"),
        )
        .mount(&mock_server)
        .await;

    let client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .expect("client builds");

    let result = client
        .mcp_servers()
        .refresh(&server_id, Some(&agent_id))
        .await;
    assert!(result.is_ok(), "refresh should succeed");

    let requests = mock_server
        .received_requests()
        .await
        .expect("read requests");
    assert_eq!(requests.len(), 1, "expected one refresh request");
    assert_eq!(
        requests[0].url.query(),
        Some(format!("agent_id={agent_id}").as_str())
    );
}
