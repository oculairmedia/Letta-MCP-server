//! Integration test for memory response optimizations
//!
//! Tests that memory operations properly truncate large responses

use letta_server::tools::memory_utils::{truncate_block_value, BlockSummary, PassageSummary};
use letta_server::tools::response_utils::{
    truncate_preview, truncate_with_indicator, PaginationMeta,
};
use serde_json::json;

#[test]
fn test_truncate_with_indicator() {
    // Short string should not be truncated
    let short = "Hello, world!";
    let result = truncate_with_indicator(short, 100);
    assert_eq!(result, short);

    // Long string should be truncated with indicator
    let long = "a".repeat(1000);
    let result = truncate_with_indicator(&long, 100);
    assert!(result.len() < 200); // Should be much shorter than original
    assert!(result.contains("truncated"));
    assert!(result.contains("900 more chars"));
}

#[test]
fn test_truncate_preview() {
    let short = "Hello";
    assert_eq!(truncate_preview(short, 10), "Hello");

    let long = "Hello, this is a very long string that needs truncation";
    let result = truncate_preview(long, 10);
    assert_eq!(result, "Hello, thi...");
}

#[test]
fn test_block_summary_from_value() {
    let block_json = json!({
        "id": "block-123",
        "label": "persona",
        "description": "This is a very long description that should be truncated to 100 characters maximum for the summary view",
        "value": "This is a very long value that represents the actual block content. It could be thousands of characters long in a real scenario.",
        "is_template": false,
        "created_at": "2025-01-01T00:00:00Z"
    });

    let summary = BlockSummary::from_block_value(&block_json);

    assert_eq!(summary.id, Some("block-123".to_string()));
    assert_eq!(summary.label, "persona");
    assert!(summary.value_preview.len() <= 103); // 100 chars + "..."
    assert!(summary.value_preview.contains("..."));
    assert_eq!(summary.value_length, 128); // Original length
}

#[test]
fn test_passage_summary_from_value() {
    let passage_json = json!({
        "id": "passage-456",
        "text": "This is a passage with some content that should be truncated to 200 characters in the summary view to reduce token usage when listing many passages.",
        "created_at": "2025-01-01T00:00:00Z",
        "source": "uploaded_file.txt"
    });

    let summary = PassageSummary::from_passage_value(&passage_json);

    assert_eq!(summary.id, "passage-456");
    assert!(summary.text_preview.len() <= 203); // 200 chars + "..."
    assert_eq!(summary.text_length, 148);
    assert_eq!(summary.source, Some("uploaded_file.txt".to_string()));
}

#[test]
fn test_pagination_meta() {
    let meta = PaginationMeta::new(100, 20, 0, 20).with_hint("Use 'get_block' for full content");

    assert_eq!(meta.total, 100);
    assert_eq!(meta.returned, 20);
    assert_eq!(meta.limit, 20);
    assert_eq!(meta.offset, 0);
    assert_eq!(meta.hints.len(), 1);
}

#[test]
fn test_truncate_block_value() {
    let mut block = json!({
        "id": "block-789",
        "label": "test",
        "value": "x".repeat(3000)
    });

    let was_truncated = truncate_block_value(&mut block, 2000);

    assert!(was_truncated);
    assert!(block.get("truncated").unwrap().as_bool().unwrap());
    assert_eq!(block.get("value_length").unwrap().as_u64().unwrap(), 3000);

    let value_str = block.get("value").unwrap().as_str().unwrap();
    assert!(value_str.len() < 2100); // Truncated + indicator
    assert!(value_str.contains("truncated"));
    assert!(value_str.contains("1000 more chars"));
}

#[test]
fn test_block_value_not_truncated_when_short() {
    let mut block = json!({
        "id": "block-short",
        "label": "test",
        "value": "Short content"
    });

    let was_truncated = truncate_block_value(&mut block, 2000);

    assert!(!was_truncated);
    assert_eq!(block.get("truncated"), None);
    assert_eq!(
        block.get("value").unwrap().as_str().unwrap(),
        "Short content"
    );
}
