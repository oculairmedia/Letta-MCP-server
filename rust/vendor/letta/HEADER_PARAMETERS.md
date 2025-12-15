# Header Parameters in Letta API

This document describes the header parameters used by various Letta API endpoints.

## Standard Headers

All requests include these standard headers:
- **Authorization**: Bearer token authentication (when configured)
- **Content-Type**: application/json (for POST/PUT/PATCH requests)

## Custom Headers

### X-Project Header
The `X-Project` header is used to associate operations with a specific project context.

**Endpoints that accept X-Project:**
- `POST /v1/agents` - Create agent (optional)
- `POST /v1/identities/` - Create identity (optional)
- `PUT /v1/identities/` - Upsert identity (optional)

**Usage:**
```rust
// Agent creation with project
let agent = client.agents()
    .create_with_project(request, "project-123")
    .await?;

// Identity creation with project
let identity = client.identities()
    .create_with_project(request, "project-123")
    .await?;

// Identity upsert with project
let identity = client.identities()
    .upsert_with_project(request, "project-123")
    .await?;

// Or configure globally on the client
let client = LettaClient::builder()
    .base_url("http://localhost:8283")
    .header("X-Project", "project-123")?
    .build()?;
```

### user-id Header
The `user-id` header is used to identify the user making the request in certain contexts.

**Endpoints that accept user-id:**
- `POST /v1/voice-beta/{agent_id}/chat/completions` - Voice chat completions (optional header)

**Usage:**
```rust
// Voice API with user-id
let response = client.voice()
    .create_voice_chat_completions(&agent_id, request, Some("user-123"))
    .await?;

// Or configure globally on the client
let client = LettaClient::builder()
    .base_url("http://localhost:8283") 
    .header("user-id", "user-123")?
    .build()?;
```

## Client Configuration

Headers can be configured at two levels:

### 1. Global Headers (on Client)
Set headers that will be sent with every request:

```rust
use letta_rs::{LettaClient, ClientConfig};
use reqwest::header::HeaderMap;

// Using convenient helper methods (recommended)
let client = LettaClient::builder()
    .base_url("http://localhost:8283")
    .project("my-project")?      // Sets X-Project header
    .user_id("user-123")?         // Sets user-id header
    .build()?;

// For cloud clients with project
let client = LettaClient::cloud_with_project("api-token", "my-project")?;

// Using generic header method
let client = LettaClient::builder()
    .base_url("http://localhost:8283")
    .header("X-Project", "my-project")?
    .header("user-id", "user-123")?
    .build()?;

// Using ClientConfig
let config = ClientConfig::new("http://localhost:8283")?
    .project("my-project")?
    .user_id("user-123")?;
let client = LettaClient::new(config)?;

// Using HeaderMap for bulk headers
let mut headers = HeaderMap::new();
headers.insert("X-Project", "my-project".parse()?);
headers.insert("user-id", "user-123".parse()?);

let config = ClientConfig::new("http://localhost:8283")?
    .headers(headers);
let client = LettaClient::new(config)?;
```

### 2. Per-Request Headers
Override or add headers for specific requests:

```rust
// Agent creation with project
let agent = client.agents()
    .create_with_project(request, "specific-project")
    .await?;

// Voice API with user-id
let response = client.voice()
    .create_voice_chat_completions(&agent_id, request, Some("specific-user"))
    .await?;
```

## Implementation Pattern

To add header support to an endpoint:

1. Create a variant method that accepts the header parameter:
```rust
pub async fn method_with_header(
    &self,
    // ... regular parameters
    header_value: &str
) -> LettaResult<Response> {
    let mut headers = HeaderMap::new();
    headers.insert("Header-Name", header_value.parse().map_err(|_| {
        LettaError::validation("Invalid header value")
    })?);
    
    self.client.post_with_headers(path, &body, headers).await
}
```

2. The client's `post_with_headers` method will merge these with the standard headers.

## Query Parameters vs Headers

Some endpoints accept user context as query parameters instead of headers:

### user-id Query Parameter
- `GET /v1/tools/mcp/servers?user-id={user_id}` - List MCP servers (optional query param)

**Usage:**
```rust
// MCP servers with user-id query parameter
let servers = client.tools()
    .list_mcp_servers_with_user("user-123")
    .await?;
```

## Notes

- Headers configured on the client are sent with every request
- Per-request headers override client-level headers if they have the same name
- Invalid header values will return a validation error
- The Voice API is currently in beta and may have additional requirements
- Some endpoints use query parameters instead of headers for user context