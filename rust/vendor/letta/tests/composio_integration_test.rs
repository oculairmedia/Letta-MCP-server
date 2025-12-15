//! Integration tests for Composio endpoints.

use letta::error::LettaResult;
use letta::LettaClient;
use serial_test::serial;

fn setup_test_client() -> LettaResult<LettaClient> {
    LettaClient::local()
}

#[tokio::test]
#[serial]
#[ignore = "ensure COMPOSIO_API_KEY is set in your server environment before running this test"]
async fn test_list_composio_apps() -> LettaResult<()> {
    let client = setup_test_client()?;

    // Try to list Composio apps
    let apps = client.tools().list_composio_apps().await?;

    println!("Found {} Composio apps", apps.len());
    for app in apps.iter().take(5) {
        println!("  - {} ({}): {}", app.name, app.key, app.description);
    }

    Ok(())
}

#[tokio::test]
#[serial]
#[ignore = "ensure COMPOSIO_API_KEY is set in your server environment before running this test"]
async fn test_list_composio_actions() -> LettaResult<()> {
    let client = setup_test_client()?;

    // First list apps to get a valid app name
    let apps = client.tools().list_composio_apps().await?;

    if let Some(first_app) = apps.first() {
        println!("Testing actions for app: {}", first_app.name);

        // List actions for the first app
        let actions = client
            .tools()
            .list_composio_actions(&first_app.name)
            .await?;

        println!("Found {} actions for {}", actions.len(), first_app.name);
        for action in actions.iter().take(5) {
            println!("  - {}: {}", action.name, action.description);
        }
    } else {
        println!("No Composio apps found to test actions");
    }

    Ok(())
}

#[tokio::test]
#[serial]
#[ignore = "ensure COMPOSIO_API_KEY is set in your server environment before running this test"]
async fn test_add_composio_tool() -> LettaResult<()> {
    let client = setup_test_client()?;

    // We know this action name is valid so long as the server has a Composio key configured
    let tool = client
        .tools()
        .add_composio_tool("resend_send_email")
        .await?;

    println!("Added Composio tool: {}", tool.name);

    // Clean up
    if let Some(id) = &tool.id {
        client.tools().delete(id).await?;
        println!("Cleaned up tool: {}", id);
    }

    Ok(())
}
