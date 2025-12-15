# Letta SDK Enhancements Plan

**Fork Repository:** https://github.com/oculairmedia/letta-rs.git
**Purpose:** Add missing endpoints to support letta-MCP-server Rust refactor
**Based On:** SDK_API_COVERAGE_ANALYSIS.md findings

## Overview

This document outlines the endpoints we need to add to the forked Letta Rust SDK to achieve 100% API coverage for the MCP server.

**Current Coverage:** ~63% (55/87 operations)
**Target Coverage:** ~95% (83/87 operations, MCP-specific routes excluded)

---

## Priority 1: File & Folder Operations (Critical Gap)

### 1.1 Agent File Sessions API

**File:** `src/api/files.rs` (NEW)
**Endpoint Base:** `/v1/agents/{agent_id}/files`

These are LRU-based file session management operations:

```rust
/// File session API operations for agents.
pub struct FileApi<'a> {
    client: &'a LettaClient,
}

impl<'a> FileApi<'a> {
    /// List files currently in an agent's file session.
    /// GET /v1/agents/{agent_id}/files
    pub async fn list(&self, agent_id: &LettaId) -> LettaResult<PaginatedAgentFiles>

    /// Open a file for an agent (adds to LRU session).
    /// POST /v1/agents/{agent_id}/files/{file_id}/open
    /// Returns: List of evicted file IDs due to LRU
    pub async fn open(&self, agent_id: &LettaId, file_id: &LettaId) -> LettaResult<Vec<String>>

    /// Close a specific file in agent's session.
    /// POST /v1/agents/{agent_id}/files/{file_id}/close
    pub async fn close(&self, agent_id: &LettaId, file_id: &LettaId) -> LettaResult<()>

    /// Close all files in agent's session.
    /// POST /v1/agents/{agent_id}/files/close-all
    /// Returns: List of closed file IDs
    pub async fn close_all(&self, agent_id: &LettaId) -> LettaResult<Vec<String>>
}
```

**Types to Add:** `src/types/file.rs`
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedAgentFiles {
    pub files: Vec<AgentFile>,
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentFile {
    pub id: LettaId,
    pub filename: String,
    pub size: u64,
    pub mime_type: String,
    pub is_open: bool,
    pub opened_at: Option<String>,
}
```

### 1.2 Folders API

**File:** `src/api/folders.rs` (NEW)
**Endpoint Base:** `/v1/folders`

```rust
/// Folder API operations.
pub struct FolderApi<'a> {
    client: &'a LettaClient,
}

impl<'a> FolderApi<'a> {
    /// List all folders.
    /// GET /v1/folders
    pub async fn list(&self) -> LettaResult<Vec<Folder>>

    /// Create a new folder.
    /// POST /v1/folders
    pub async fn create(&self, request: CreateFolderRequest) -> LettaResult<Folder>

    /// Get a folder by ID.
    /// GET /v1/folders/{folder_id}
    pub async fn get(&self, folder_id: &LettaId) -> LettaResult<Folder>

    /// Update a folder.
    /// PATCH /v1/folders/{folder_id}
    pub async fn update(&self, folder_id: &LettaId, request: UpdateFolderRequest) -> LettaResult<Folder>

    /// Delete a folder.
    /// DELETE /v1/folders/{folder_id}
    pub async fn delete(&self, folder_id: &LettaId) -> LettaResult<()>

    /// Get folder agents sub-API.
    pub fn agents(&self, folder_id: LettaId) -> FolderAgentsApi
}

/// Agent-folder relationship operations.
pub struct FolderAgentsApi<'a> {
    client: &'a LettaClient,
    folder_id: LettaId,
}

impl<'a> FolderAgentsApi<'a> {
    /// List agents in a folder.
    /// GET /v1/folders/{folder_id}/agents
    pub async fn list(&self) -> LettaResult<Vec<String>>  // Returns agent IDs
}

/// Agent folder operations (add to AgentApi).
impl<'a> AgentApi<'a> {
    /// Get agent folders sub-API.
    pub fn folders(&self, agent_id: LettaId) -> AgentFoldersApi
}

pub struct AgentFoldersApi<'a> {
    client: &'a LettaClient,
    agent_id: LettaId,
}

impl<'a> AgentFoldersApi<'a> {
    /// Attach folder to agent.
    /// PATCH /v1/agents/{agent_id}/folders/attach/{folder_id}
    pub async fn attach(&self, folder_id: &LettaId) -> LettaResult<AgentState>

    /// Detach folder from agent.
    /// PATCH /v1/agents/{agent_id}/folders/detach/{folder_id}
    pub async fn detach(&self, folder_id: &LettaId) -> LettaResult<AgentState>
}
```

**Types to Add:** `src/types/folder.rs`
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: LettaId,
    pub name: String,
    pub path: String,
    pub file_count: u32,
    pub agent_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFolderRequest {
    pub name: Option<String>,
    pub path: Option<String>,
}
```

---

## Priority 2: Missing Agent Operations

**File:** `src/api/agents.rs` (MODIFY)

Add the following methods to `impl<'a> AgentApi<'a>`:

```rust
/// Clone an agent.
/// POST /v1/agents/{agent_id}/clone
pub async fn clone(&self, agent_id: &LettaId, name: Option<String>) -> LettaResult<AgentState> {
    let mut body = serde_json::json!({});
    if let Some(n) = name {
        body["name"] = serde_json::Value::String(n);
    }
    self.client
        .post(&format!("v1/agents/{}/clone", agent_id), &body)
        .await
}

/// Get agent configuration.
/// GET /v1/agents/{agent_id}/config
pub async fn get_config(&self, agent_id: &LettaId) -> LettaResult<AgentConfig> {
    self.client
        .get(&format!("v1/agents/{}/config", agent_id))
        .await
}

/// Update agent (general update endpoint).
/// PATCH /v1/agents/{agent_id}
pub async fn update(&self, agent_id: &LettaId, request: UpdateAgentRequest) -> LettaResult<AgentState> {
    self.client
        .patch(&format!("v1/agents/{}", agent_id), &request)
        .await
}

/// Bulk delete agents.
/// POST /v1/agents/bulk-delete
pub async fn bulk_delete(&self, agent_ids: Vec<LettaId>) -> LettaResult<BulkDeleteResponse> {
    self.client
        .post("v1/agents/bulk-delete", &serde_json::json!({ "agent_ids": agent_ids }))
        .await
}

/// Get agent context window information.
/// GET /v1/agents/{agent_id}/context
pub async fn get_context(&self, agent_id: &LettaId) -> LettaResult<AgentContext> {
    self.client
        .get(&format!("v1/agents/{}/context", agent_id))
        .await
}

/// Reset agent messages (clear conversation history).
/// DELETE /v1/agents/{agent_id}/messages
pub async fn reset_messages(&self, agent_id: &LettaId) -> LettaResult<()> {
    self.client
        .delete_no_response(&format!("v1/agents/{}/messages", agent_id))
        .await
}

/// Get files API for agent.
pub fn files(&self, agent_id: LettaId) -> FileApi {
    FileApi::new(self.client, agent_id)
}
```

**Types to Add:** `src/types/agent.rs` (MODIFY)
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentConfig {
    pub llm_config: LLMConfig,
    pub embedding_config: EmbeddingConfig,
    pub system: String,
    pub tools: Vec<LettaId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAgentRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub system: Option<String>,
    pub llm_config: Option<LLMConfig>,
    pub embedding_config: Option<EmbeddingConfig>,
    pub tool_ids: Option<Vec<LettaId>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkDeleteResponse {
    pub deleted_count: u32,
    pub failed_ids: Vec<LettaId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentContext {
    pub context_window_size: u32,
    pub current_usage: u32,
    pub available: u32,
}
```

---

## Priority 3: Missing Message Operations

**File:** `src/api/messages.rs` (MODIFY)

```rust
impl<'a> MessageApi<'a> {
    /// Send async message (non-blocking).
    /// POST /v1/agents/{agent_id}/messages/async
    pub async fn create_async(&self, agent_id: &LettaId, request: CreateMessagesRequest) -> LettaResult<AsyncMessageResponse> {
        self.client
            .post(&format!("v1/agents/{}/messages/async", agent_id), &request)
            .await
    }

    /// Cancel a pending async message.
    /// DELETE /v1/agents/{agent_id}/messages/{message_id}
    pub async fn cancel(&self, agent_id: &LettaId, message_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/agents/{}/messages/{}", agent_id, message_id))
            .await
    }

    /// Preview message payload without sending.
    /// POST /v1/agents/{agent_id}/messages/preview
    pub async fn preview(&self, agent_id: &LettaId, request: CreateMessagesRequest) -> LettaResult<MessagePreview> {
        self.client
            .post(&format!("v1/agents/{}/messages/preview", agent_id), &request)
            .await
    }

    /// Search messages.
    /// POST /v1/agents/{agent_id}/messages/search
    pub async fn search(&self, agent_id: &LettaId, request: MessageSearchRequest) -> LettaResult<Vec<Message>> {
        self.client
            .post(&format!("v1/agents/{}/messages/search", agent_id), &request)
            .await
    }
}
```

**Types to Add:** `src/types/message.rs` (MODIFY)
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct AsyncMessageResponse {
    pub job_id: LettaId,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePreview {
    pub payload_size: u32,
    pub token_count: u32,
    pub will_fit: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSearchRequest {
    pub query: String,
    pub limit: Option<u32>,
}
```

---

## Priority 4: Missing Source Operations

**File:** `src/api/sources.rs` (MODIFY)

```rust
impl<'a> SourceApi<'a> {
    /// Process/ingest a source.
    /// POST /v1/sources/{source_id}/process
    pub async fn process(&self, source_id: &LettaId) -> LettaResult<SourceProcessResponse> {
        self.client
            .post(&format!("v1/sources/{}/process", source_id), &serde_json::json!({}))
            .await
    }

    /// Get jobs for a source.
    /// GET /v1/sources/{source_id}/jobs
    pub async fn get_jobs(&self, source_id: &LettaId) -> LettaResult<Vec<Job>> {
        self.client
            .get(&format!("v1/sources/{}/jobs", source_id))
            .await
    }
}
```

---

## Priority 5: Client Accessor Methods

**File:** `src/client.rs` (MODIFY)

Add accessor methods to `impl LettaClient`:

```rust
/// Get the files API.
pub fn files(&self) -> crate::api::FilesApi<'_> {
    crate::api::FilesApi::new(self)
}

/// Get the folders API.
pub fn folders(&self) -> crate::api::FolderApi<'_> {
    crate::api::FolderApi::new(self)
}
```

---

## Module Registration

**File:** `src/api/mod.rs` (MODIFY)

```rust
pub mod agents;
pub mod batch;
pub mod blocks;
pub mod files;      // NEW
pub mod folders;    // NEW
pub mod groups;
// ... existing modules ...

pub use agents::AgentApi;
pub use files::{FileApi, AgentFoldersApi, FolderAgentsApi};  // NEW
pub use folders::FolderApi;  // NEW
// ... existing exports ...
```

**File:** `src/types/mod.rs` (MODIFY)

```rust
pub mod agent;
pub mod file;    // NEW
pub mod folder;  // NEW
// ... existing modules ...

pub use file::*;   // NEW
pub use folder::*; // NEW
// ... existing exports ...
```

---

## Testing Strategy

1. **Unit Tests**: Add tests in each new module
2. **Integration Tests**: Test against real Letta instance
3. **Compatibility Tests**: Ensure MCP server works with new SDK

---

## Implementation Order

1. ✅ Switch Cargo.toml to use forked SDK
2. 🔄 Create `src/api/files.rs` + `src/types/file.rs` (4 operations)
3. 🔄 Create `src/api/folders.rs` + `src/types/folder.rs` (4 operations)
4. 🔄 Add missing agent methods to `src/api/agents.rs` (7 operations)
5. 🔄 Add missing message methods to `src/api/messages.rs` (4 operations)
6. 🔄 Add missing source methods to `src/api/sources.rs` (2 operations)
7. 🔄 Update `mod.rs` files for new modules
8. 🔄 Update `client.rs` for new accessors
9. 🔄 Test with MCP server handlers
10. 🔄 Push to fork, update letta-MCP-server

---

## Notes

- **MCP Operations**: Not included in SDK as they're custom server endpoints
- **Jobs API**: Already exists in SDK, verify coverage
- **Blocks API**: Already exists, verify coverage
- **Tools API**: Already exists, verify coverage

**Estimated Effort:** 21 new operations + types + tests
**New Files:** 4 (files.rs, folder.rs, file.rs in types, folder.rs in types)
**Modified Files:** 5 (agents.rs, messages.rs, sources.rs, client.rs, mod.rs files)
