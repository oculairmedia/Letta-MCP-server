# Agent Endpoints from OpenAPI Spec v0.12.1

Reference guide for implementing missing agent endpoints in the Rust SDK.

## Missing Operations to Implement

### 1. Update Agent
**Endpoint:** `PATCH /v1/agents/{agent_id}`
**Status:** ❌ NOT IN SDK v0.1.2
**Schema:** `UpdateAgent`
**Fields:**
- name: Option<String>
- tool_ids: Option<Vec<String>>
- source_ids: Option<Vec<String>>
- block_ids: Option<Vec<String>>
- tags: Option<Vec<String>>
- system: Option<String>
- llm_config: Option<LLMConfig>
- embedding_config: Option<EmbeddingConfig>
- metadata: Option<Object>
- description: Option<String>

### 2. Get Context
**Endpoint:** `GET /v1/agents/{agent_id}/context`
**Status:** ❌ NOT IN SDK
**Returns:** Context window information

### 3. Reset Messages
**Endpoint:** `POST /v1/agents/{agent_id}/reset-messages`
**Status:** ❌ NOT IN SDK
**Action:** Clears conversation history

### 4. Count Agents
**Endpoint:** `GET /v1/agents/count`
**Status:** ✅ IN SDK (verified)

### 5. Export Agent
**Endpoint:** `GET /v1/agents/{agent_id}/export`
**Status:** ✅ IN SDK as `export_file()`

### 6. Import Agent
**Endpoint:** `POST /v1/agents/import`
**Status:** ✅ IN SDK as `import_file()`

### 7. Summarize
**Endpoint:** `POST /v1/agents/{agent_id}/summarize`
**Status:** ✅ IN SDK as `summarize_agent_conversation()`

### 8. Files Operations
**Endpoints:**
- `GET /v1/agents/{agent_id}/files` - list
- `POST /v1/agents/{agent_id}/files/{file_id}/open` - open
- `POST /v1/agents/{agent_id}/files/{file_id}/close` - close
- `POST /v1/agents/{agent_id}/files/close-all` - close_all
**Status:** ✅ IMPLEMENTED in new files.rs

### 9. Folders Operations
**Endpoints:**
- `GET /v1/agents/{agent_id}/folders` - list (returns folder IDs)
- `PATCH /v1/agents/{agent_id}/folders/attach/{folder_id}` - attach
- `PATCH /v1/agents/{agent_id}/folders/detach/{folder_id}` - detach
**Status:** ✅ IMPLEMENTED in new folders.rs

## Agent File Session Endpoints (DONE)
- ✅ `GET /v1/agents/{agent_id}/files`
- ✅ `POST /v1/agents/{agent_id}/files/{file_id}/open`
- ✅ `POST /v1/agents/{agent_id}/files/{file_id}/close`
- ✅ `POST /v1/agents/{agent_id}/files/close-all`

## Agent Folder Endpoints (DONE)
- ✅ `PATCH /v1/agents/{agent_id}/folders/attach/{folder_id}`
- ✅ `PATCH /v1/agents/{agent_id}/folders/detach/{folder_id}`

## To Implement in agents.rs
1. ❌ `update()` - PATCH /v1/agents/{agent_id}
2. ❌ `get_context()` - GET /v1/agents/{agent_id}/context
3. ❌ `reset_messages()` - POST /v1/agents/{agent_id}/reset-messages
4. ❌ `files()` accessor - Returns AgentFileApi
5. ❌ `folders()` accessor - Returns AgentFolderApi (already in folders.rs)

## Clone and Bulk Delete
**NOT IN OPENAPI SPEC** - These may be custom/deprecated endpoints. Will check Node.js implementation.
