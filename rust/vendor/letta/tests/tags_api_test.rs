//! Integration tests for the Tags API.

use letta::client::{ClientConfig, LettaClient};
use letta::error::LettaResult;
use letta::types::*;

/// Get a test client for the local server.
fn get_test_client() -> LettaResult<LettaClient> {
    let config = ClientConfig::new("http://localhost:8283")?;
    LettaClient::new(config)
}

#[tokio::test]
async fn test_list_tags() -> LettaResult<()> {
    let client = get_test_client()?;

    // List all tags
    let tags = client.tags().list(None).await?;

    println!("Found {} tags", tags.len());
    for tag in tags.iter().take(10) {
        println!("Tag: {}", tag);
    }

    Ok(())
}

#[tokio::test]
async fn test_list_tags_with_pagination() -> LettaResult<()> {
    let client = get_test_client()?;

    // List tags with pagination
    let params = ListTagsParams {
        limit: Some(5),
        ..Default::default()
    };

    let tags = client.tags().list(Some(params)).await?;

    println!("Found {} tags (limited to 5)", tags.len());
    assert!(tags.len() <= 5, "Should respect limit parameter");

    Ok(())
}

#[tokio::test]
async fn test_list_tags_with_query() -> LettaResult<()> {
    let client = get_test_client()?;

    // List tags with query filter
    let params = ListTagsParams {
        query_text: Some("test".to_string()),
        ..Default::default()
    };

    let tags = client.tags().list(Some(params)).await?;

    println!("Found {} tags matching 'test'", tags.len());
    for tag in &tags {
        println!("Matching tag: {}", tag);
    }

    Ok(())
}

#[tokio::test]
async fn test_tags_pagination_stream() -> LettaResult<()> {
    use futures::StreamExt;
    use letta::types::PaginationParams;

    let client = get_test_client()?;

    // Use paginated stream
    let params = PaginationParams {
        limit: Some(2),
        ..Default::default()
    };

    let mut stream = client.tags().paginated(Some(params));
    let mut count = 0;

    while let Some(result) = stream.next().await {
        let tag = result?;
        println!("Tag from stream: {}", tag);
        count += 1;
        if count >= 10 {
            break; // Limit test to 10 items
        }
    }

    println!("Streamed {} tags", count);

    Ok(())
}
