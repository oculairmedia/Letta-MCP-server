use letta::client::ClientBuilder;
use letta::types::agent::AgentState;
use letta::types::LettaId;
use letta::LettaClient;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

const VALID_AGENT_STATE_JSON: &str = r#"{
  "id": "agent-00000000-0000-0000-0000-000000000001",
  "name": "test-agent",
  "agent_type": "memgpt_agent",
  "tools": [],
  "sources": [],
  "tags": [],
  "message_ids": []
}"#;

struct TestIds {
    agent_id: LettaId,
    tool_id: LettaId,
    block_id: LettaId,
    folder_id: LettaId,
}

fn test_ids() -> TestIds {
    TestIds {
        agent_id: "agent-00000000-0000-0000-0000-000000000001"
            .parse()
            .expect("valid agent id"),
        tool_id: "tool-00000000-0000-0000-0000-000000000001"
            .parse()
            .expect("valid tool id"),
        block_id: "block-00000000-0000-0000-0000-000000000001"
            .parse()
            .expect("valid block id"),
        folder_id: "source-00000000-0000-0000-0000-000000000001"
            .parse()
            .expect("valid folder id"),
    }
}

async fn setup_mock(path_pattern: &str, response_body: &str) -> (MockServer, LettaClient) {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path_regex(path_pattern))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_string(response_body),
        )
        .mount(&mock_server)
        .await;

    let client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .expect("client builds");

    (mock_server, client)
}

fn assert_ok_none(result: Result<Option<AgentState>, letta::error::LettaError>) {
    match result {
        Ok(agent) => {
            assert!(agent.is_none(), "expected Ok(None), got Ok(Some(_))");
        }
        Err(err) => panic!("expected Ok(None), got Err({err})"),
    }
}

fn assert_ok_some_named(
    result: Result<Option<AgentState>, letta::error::LettaError>,
    expected_name: &str,
) {
    match result {
        Ok(agent) => {
            if let Some(agent) = agent {
                assert_eq!(agent.name, expected_name);
            } else {
                panic!("expected Ok(Some(_)), got Ok(None)");
            }
        }
        Err(err) => panic!("expected Ok(Some(_)), got Err({err})"),
    }
}

macro_rules! endpoint_contract_tests {
    (
        null: $null_name:ident,
        valid: $valid_name:ident,
        empty: $empty_name:ident,
        malformed: $malformed_name:ident,
        path: $path_pattern:expr,
        call: |$client:ident, $ids:ident| $call:expr
    ) => {
        #[tokio::test]
        async fn $null_name() {
            let ids = test_ids();
            let (_mock_server, client) = setup_mock($path_pattern, "null").await;
            let result = {
                let $client = &client;
                let $ids = &ids;
                $call
            }
            .await;
            assert_ok_none(result);
        }

        #[tokio::test]
        async fn $valid_name() {
            let ids = test_ids();
            let (_mock_server, client) = setup_mock($path_pattern, VALID_AGENT_STATE_JSON).await;
            let result = {
                let $client = &client;
                let $ids = &ids;
                $call
            }
            .await;
            assert_ok_some_named(result, "test-agent");
        }

        #[tokio::test]
        async fn $empty_name() {
            let ids = test_ids();
            let (_mock_server, client) = setup_mock($path_pattern, "").await;
            let result = {
                let $client = &client;
                let $ids = &ids;
                $call
            }
            .await;
            assert!(
                result.is_err(),
                "expected deserialization error for empty body"
            );
        }

        #[tokio::test]
        async fn $malformed_name() {
            let ids = test_ids();
            let (_mock_server, client) = setup_mock($path_pattern, "{invalid").await;
            let result = {
                let $client = &client;
                let $ids = &ids;
                $call
            }
            .await;
            assert!(
                result.is_err(),
                "expected deserialization error for malformed JSON"
            );
        }
    };
}

endpoint_contract_tests!(
    null: test_attach_tool_null_response,
    valid: test_attach_tool_valid_agent_state,
    empty: test_attach_tool_empty_body,
    malformed: test_attach_tool_malformed_json,
    path: r"^/v1/agents/.+/tools/attach/.+$",
    call: |client, ids| client
        .memory()
        .attach_tool_to_agent(&ids.agent_id, &ids.tool_id)
);

endpoint_contract_tests!(
    null: test_detach_tool_null_response,
    valid: test_detach_tool_valid_agent_state,
    empty: test_detach_tool_empty_body,
    malformed: test_detach_tool_malformed_json,
    path: r"^/v1/agents/.+/tools/detach/.+$",
    call: |client, ids| client
        .memory()
        .detach_tool_from_agent(&ids.agent_id, &ids.tool_id)
);

endpoint_contract_tests!(
    null: test_attach_block_null_response,
    valid: test_attach_block_valid_agent_state,
    empty: test_attach_block_empty_body,
    malformed: test_attach_block_malformed_json,
    path: r"^/v1/agents/.+/core-memory/blocks/attach/.+$",
    call: |client, ids| client
        .memory()
        .attach_memory_block(&ids.agent_id, &ids.block_id)
);

endpoint_contract_tests!(
    null: test_detach_block_null_response,
    valid: test_detach_block_valid_agent_state,
    empty: test_detach_block_empty_body,
    malformed: test_detach_block_malformed_json,
    path: r"^/v1/agents/.+/core-memory/blocks/detach/.+$",
    call: |client, ids| client
        .memory()
        .detach_memory_block(&ids.agent_id, &ids.block_id)
);

endpoint_contract_tests!(
    null: test_attach_folder_null_response,
    valid: test_attach_folder_valid_agent_state,
    empty: test_attach_folder_empty_body,
    malformed: test_attach_folder_malformed_json,
    path: r"^/v1/agents/.+/folders/attach/.+$",
    call: |client, ids| client
        .agents()
        .folders(ids.agent_id.clone())
        .attach(&ids.folder_id)
);

endpoint_contract_tests!(
    null: test_detach_folder_null_response,
    valid: test_detach_folder_valid_agent_state,
    empty: test_detach_folder_empty_body,
    malformed: test_detach_folder_malformed_json,
    path: r"^/v1/agents/.+/folders/detach/.+$",
    call: |client, ids| client
        .agents()
        .folders(ids.agent_id.clone())
        .detach(&ids.folder_id)
);
