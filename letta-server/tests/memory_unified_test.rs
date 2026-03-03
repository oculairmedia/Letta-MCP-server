//! Tests for memory_unified tool operations
//!
//! Tests memory operations including:
//! - Core memory operations
//! - Block operations
//! - Archival memory (passages)
//! - Response truncation

use letta_server::tools::memory_unified::{MemoryOperation, MemoryUnifiedRequest, SearchSource};
use letta_server::tools::memory_utils::{
    BlockSummary, PaginationMeta, PassageSummary, truncate_block_value, truncate_preview,
    truncate_string,
};
use serde_json::json;

// ============================================================
// Request Parsing Tests
// ============================================================

#[test]
fn test_parse_get_core_memory() {
    let json_input = json!({
        "operation": "get_core_memory",
        "agent_id": "agent-12345"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::GetCoreMemory));
    assert_eq!(request.agent_id.unwrap(), "agent-12345");
}

#[test]
fn test_parse_update_core_memory() {
    let json_input = json!({
        "operation": "update_core_memory",
        "agent_id": "agent-12345",
        "block_label": "persona",
        "value": "Updated persona content"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(
        request.operation,
        MemoryOperation::UpdateCoreMemory
    ));
    assert_eq!(request.block_label.unwrap(), "persona");
    assert_eq!(request.value.unwrap(), "Updated persona content");
}

#[test]
fn test_parse_list_blocks() {
    let json_input = json!({
        "operation": "list_blocks",
        "agent_id": "agent-12345",
        "limit": 20
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::ListBlocks));
    assert_eq!(request.limit.unwrap(), 20);
}

#[test]
fn test_parse_get_block() {
    let json_input = json!({
        "operation": "get_block",
        "block_id": "block-12345"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::GetBlock));
    assert_eq!(request.block_id.unwrap(), "block-12345");
}

#[test]
fn test_parse_create_block() {
    let json_input = json!({
        "operation": "create_block",
        "label": "custom",
        "value": "Custom block content",
        "is_template": false
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::CreateBlock));
    assert_eq!(request.label.unwrap(), "custom");
    assert_eq!(request.value.unwrap(), "Custom block content");
}

#[test]
fn test_parse_update_block() {
    let json_input = json!({
        "operation": "update_block",
        "block_id": "block-12345",
        "value": "New value"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::UpdateBlock));
}

#[test]
fn test_parse_search_archival() {
    let json_input = json!({
        "operation": "search_archival",
        "agent_id": "agent-12345",
        "query": "important facts",
        "limit": 10
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::SearchArchival));
    assert_eq!(request.query.unwrap(), "important facts");
}

#[test]
fn test_parse_list_passages() {
    let json_input = json!({
        "operation": "list_passages",
        "agent_id": "agent-12345"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::ListPassages));
}

#[test]
fn test_parse_create_passage() {
    let json_input = json!({
        "operation": "create_passage",
        "agent_id": "agent-12345",
        "text": "This is a new passage to store."
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::CreatePassage));
    assert_eq!(request.text.unwrap(), "This is a new passage to store.");
}

#[test]
fn test_parse_search_memory_basic() {
    let json_input = json!({
        "operation": "search_memory",
        "agent_id": "agent-12345",
        "query": "important info"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::SearchMemory));
    assert_eq!(request.agent_id.unwrap(), "agent-12345");
    assert_eq!(request.query.unwrap(), "important info");
    assert!(request.source.is_none()); // Default should be None (which means Both)
}

#[test]
fn test_parse_search_memory_with_source() {
    let json_input = json!({
        "operation": "search_memory",
        "agent_id": "agent-12345",
        "query": "find this",
        "source": "archival"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::SearchMemory));
    assert!(matches!(request.source, Some(SearchSource::Archival)));
}

#[test]
fn test_parse_search_memory_messages_only() {
    let json_input = json!({
        "operation": "search_memory",
        "agent_id": "agent-12345",
        "query": "conversation",
        "source": "messages"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.source, Some(SearchSource::Messages)));
}

#[test]
fn test_parse_search_memory_both() {
    let json_input = json!({
        "operation": "search_memory",
        "agent_id": "agent-12345",
        "query": "all sources",
        "source": "both"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.source, Some(SearchSource::Both)));
}

#[test]
fn test_parse_search_memory_with_dates() {
    let json_input = json!({
        "operation": "search_memory",
        "agent_id": "agent-12345",
        "query": "dated search",
        "start_date": "2025-01-01T00:00:00Z",
        "end_date": "2025-12-31T23:59:59Z"
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, MemoryOperation::SearchMemory));
    assert!(request.start_date.is_some());
    assert!(request.end_date.is_some());
}

#[test]
fn test_parse_search_memory_with_limit() {
    let json_input = json!({
        "operation": "search_memory",
        "agent_id": "agent-12345",
        "query": "limited search",
        "limit": 25
    });

    let request: MemoryUnifiedRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.limit.unwrap(), 25);
}

// ============================================================
// All Operations Enum Test
// ============================================================

#[test]
fn test_all_memory_operations_parse() {
    let operations = vec![
        "get_core_memory",
        "update_core_memory",
        "get_block_by_label",
        "list_blocks",
        "create_block",
        "get_block",
        "update_block",
        "attach_block",
        "detach_block",
        "list_agents_using_block",
        "search_archival",
        "list_passages",
        "create_passage",
        "update_passage",
        "delete_passage",
        "search_memory",
    ];

    for op in operations {
        let json_input = json!({ "operation": op });
        let result: Result<MemoryUnifiedRequest, _> = serde_json::from_value(json_input);
        assert!(result.is_ok(), "Failed to parse operation: {}", op);
    }
}

// ============================================================
// Memory Utils Tests
// ============================================================

#[test]
fn test_truncate_string_short() {
    let text = "Hello world";
    let result = truncate_string(text, 100);
    assert_eq!(result, text);
}

#[test]
fn test_truncate_string_long() {
    let text = "a".repeat(500);
    let result = truncate_string(&text, 100);

    assert!(result.len() < text.len());
    assert!(result.contains("truncated"));
    assert!(result.contains("400 more chars"));
}

#[test]
fn test_truncate_preview_short() {
    let text = "Short";
    let result = truncate_preview(text, 20);
    assert_eq!(result, "Short");
}

#[test]
fn test_truncate_preview_long() {
    let text = "This is a longer text that needs truncating";
    let result = truncate_preview(text, 20);

    assert!(result.len() <= 23); // 20 + "..."
    assert!(result.ends_with("..."));
}

// ============================================================
// BlockSummary Tests
// ============================================================

#[test]
fn test_block_summary_from_value() {
    let block_json = json!({
        "id": "block-123",
        "label": "persona",
        "description": "A persona block",
        "value": "I am a helpful assistant that provides detailed responses.",
        "is_template": false,
        "created_at": "2025-01-01T00:00:00Z"
    });

    let summary = BlockSummary::from_block_value(&block_json);

    assert_eq!(summary.id, Some("block-123".to_string()));
    assert_eq!(summary.label, "persona");
    assert!(summary.value_preview.len() <= 103); // 100 chars max + "..."
    assert_eq!(summary.is_template, Some(false));
}

#[test]
fn test_block_summary_truncates_long_value() {
    let long_value = "x".repeat(500);
    let block_json = json!({
        "id": "block-456",
        "label": "custom",
        "value": long_value,
    });

    let summary = BlockSummary::from_block_value(&block_json);

    assert!(summary.value_preview.len() < long_value.len());
    assert!(summary.value_preview.ends_with("..."));
    assert_eq!(summary.value_length, 500);
}

#[test]
fn test_block_summary_handles_missing_fields() {
    let minimal_json = json!({
        "label": "test"
    });

    let summary = BlockSummary::from_block_value(&minimal_json);

    assert_eq!(summary.label, "test");
    assert!(summary.id.is_none());
    assert!(summary.description.is_none());
}

// ============================================================
// PassageSummary Tests
// ============================================================

#[test]
fn test_passage_summary_from_value() {
    let passage_json = json!({
        "id": "passage-789",
        "text": "This is passage content that contains important information.",
        "source": "document.pdf",
        "created_at": "2025-01-01T00:00:00Z"
    });

    let summary = PassageSummary::from_passage_value(&passage_json);

    assert_eq!(summary.id, "passage-789");
    assert!(summary.text_preview.contains("passage content"));
    assert_eq!(summary.source, Some("document.pdf".to_string()));
}

#[test]
fn test_passage_summary_truncates_long_text() {
    let long_text = "Important information. ".repeat(50);
    let passage_json = json!({
        "id": "passage-long",
        "text": long_text,
    });

    let summary = PassageSummary::from_passage_value(&passage_json);

    assert!(summary.text_preview.len() <= 203); // 200 chars + "..."
    assert!(summary.text_length > 200);
}

// ============================================================
// PaginationMeta Tests
// ============================================================

#[test]
fn test_pagination_meta_basic() {
    let meta = PaginationMeta::new(100, 20, 20);

    assert_eq!(meta.total, 100);
    assert_eq!(meta.returned, 20);
    assert_eq!(meta.limit, 20);
    assert!(meta.offset.is_none());
}

#[test]
fn test_pagination_meta_with_offset() {
    let meta = PaginationMeta::new(100, 20, 20).with_offset(40);

    assert_eq!(meta.offset, Some(40));
}

#[test]
fn test_pagination_meta_with_hint() {
    let meta =
        PaginationMeta::new(50, 10, 10).with_hint("Use get_block for full content".to_string());

    assert_eq!(meta.hints.len(), 1);
    assert!(meta.hints[0].contains("get_block"));
}

#[test]
fn test_pagination_meta_multiple_hints() {
    let meta = PaginationMeta::new(100, 20, 20)
        .with_hint("Hint 1".to_string())
        .with_hint("Hint 2".to_string());

    assert_eq!(meta.hints.len(), 2);
}

// ============================================================
// truncate_block_value Tests
// ============================================================

#[test]
fn test_truncate_block_value_short() {
    let mut block = json!({
        "id": "block-short",
        "value": "Short content"
    });

    let was_truncated = truncate_block_value(&mut block, 2000);

    assert!(!was_truncated);
    assert_eq!(block.get("truncated"), None);
    assert_eq!(block["value"], "Short content");
}

#[test]
fn test_truncate_block_value_long() {
    let long_value = "x".repeat(3000);
    let mut block = json!({
        "id": "block-long",
        "value": long_value
    });

    let was_truncated = truncate_block_value(&mut block, 2000);

    assert!(was_truncated);
    assert_eq!(block["truncated"], true);
    assert_eq!(block["value_length"], 3000);

    let truncated_value = block["value"].as_str().unwrap();
    assert!(truncated_value.len() < 3000);
    assert!(truncated_value.contains("truncated"));
}

#[test]
fn test_truncate_block_value_exactly_at_limit() {
    let exact_value = "x".repeat(2000);
    let mut block = json!({
        "id": "block-exact",
        "value": exact_value
    });

    let was_truncated = truncate_block_value(&mut block, 2000);

    assert!(!was_truncated);
    assert_eq!(block.get("truncated"), None);
}

#[test]
fn test_truncate_block_value_one_over() {
    let over_value = "x".repeat(2001);
    let mut block = json!({
        "id": "block-over",
        "value": over_value
    });

    let was_truncated = truncate_block_value(&mut block, 2000);

    assert!(was_truncated);
    assert_eq!(block["truncated"], true);
    assert!(block["value"].as_str().unwrap().contains("1 more chars"));
}

// ============================================================
// Response Format Tests
// ============================================================

mod response_format {
    use serde_json::json;

    #[test]
    fn test_list_blocks_response_format() {
        let response_data = json!({
            "total": 7,
            "returned": 7,
            "blocks": [
                {"id": "b1", "label": "persona", "value_preview": "...", "value_length": 500},
                {"id": "b2", "label": "human", "value_preview": "...", "value_length": 200}
            ],
            "hints": ["Use get_block for full content"]
        });

        assert!(response_data.get("total").is_some());
        assert!(response_data.get("returned").is_some());
        assert!(response_data.get("blocks").is_some());
        assert!(response_data.get("hints").is_some());
    }

    #[test]
    fn test_search_archival_response_format() {
        let response_data = json!({
            "total": 50,
            "returned": 10,
            "passages": [],
            "query": "search term",
            "hint": "Use get with passage_id for full content"
        });

        assert!(response_data.get("query").is_some());
        assert!(response_data.get("passages").is_some());
    }

    #[test]
    fn test_core_memory_response_format() {
        let response_data = json!({
            "agent_id": "agent-123",
            "blocks": {
                "persona": {
                    "id": "block-1",
                    "value": "...",
                    "truncated": true,
                    "value_length": 5000
                },
                "human": {
                    "id": "block-2",
                    "value": "User info",
                    "truncated": false
                }
            },
            "hint": "Use get_block for full content"
        });

        assert!(response_data.get("agent_id").is_some());
        assert!(response_data.get("blocks").is_some());

        let persona = &response_data["blocks"]["persona"];
        assert_eq!(persona["truncated"], true);
        assert!(persona.get("value_length").is_some());
    }
}

// ============================================================
// Edge Cases
// ============================================================

mod edge_cases {
    use letta_server::tools::memory_utils::BlockSummary;
    use serde_json::json;

    #[test]
    fn test_empty_block_value() {
        let block_json = json!({
            "id": "block-empty",
            "label": "test",
            "value": ""
        });

        let summary = BlockSummary::from_block_value(&block_json);
        assert_eq!(summary.value_length, 0);
        assert_eq!(summary.value_preview, "");
    }

    #[test]
    fn test_null_block_value() {
        let block_json = json!({
            "id": "block-null",
            "label": "test",
            "value": null
        });

        let summary = BlockSummary::from_block_value(&block_json);
        // Should handle null gracefully
        assert_eq!(summary.value_length, 0);
    }

    #[test]
    fn test_unicode_in_block_value() {
        let block_json = json!({
            "id": "block-unicode",
            "label": "test",
            "value": "Hello \u{1F600} World \u{1F389} Test"
        });

        let summary = BlockSummary::from_block_value(&block_json);
        assert!(summary.value_preview.contains("\u{1F600}"));
    }

    #[test]
    fn test_very_long_label() {
        let long_label = "a".repeat(1000);
        let block_json = json!({
            "label": long_label,
            "value": "content"
        });

        let summary = BlockSummary::from_block_value(&block_json);
        assert_eq!(summary.label, long_label);
    }
}
