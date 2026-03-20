//! Centralized API endpoint constants and path builders.
//!
//! All Letta REST API paths are defined here to avoid hardcoded strings
//! scattered across individual API modules. Static paths are `&str` constants;
//! dynamic paths use inline `format!` via helper functions.

// ---------------------------------------------------------------------------
// Agents
// ---------------------------------------------------------------------------
pub mod agents {
    pub const LIST: &str = "v1/agents/";
    pub const CREATE: &str = "v1/agents";
    pub const COUNT: &str = "v1/agents/count";
    pub const SEARCH: &str = "v1/agents/search";
    pub const IMPORT: &str = "v1/agents/import";
    pub const MESSAGE_SEARCH: &str = "v1/agents/messages/search";

    pub fn get(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}")
    }
    pub fn delete(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}")
    }
    pub fn update(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}")
    }
    pub fn context(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}/context")
    }
    pub fn reset_messages(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}/reset-messages")
    }
    pub fn summarize(agent_id: &impl std::fmt::Display, max_len: usize) -> String {
        format!("v1/agents/{agent_id}/summarize?max_message_length={max_len}")
    }
    pub fn export(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}/export")
    }
    pub fn groups(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}/groups")
    }
    pub fn template(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}/template")
    }
    pub fn version_template(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}/version-template")
    }
    pub fn migrate(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}/migrate")
    }
    pub fn core_memory_variables(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{agent_id}/core-memory/variables")
    }

    pub mod messages {
        pub fn list(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/messages")
        }
        pub fn send(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/messages")
        }
        pub fn stream(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/messages/stream")
        }
        pub fn get(
            agent_id: &impl std::fmt::Display,
            message_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/messages/{message_id}")
        }
        pub fn send_async(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/messages/async")
        }
        pub fn cancel(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/messages/cancel")
        }
        pub fn preview(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/messages/preview-raw-payload")
        }
    }

    pub mod core_memory {
        pub fn root(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/core-memory")
        }
        pub fn blocks(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/core-memory/blocks")
        }
        pub fn block(agent_id: &impl std::fmt::Display, label: &str) -> String {
            format!("v1/agents/{agent_id}/core-memory/blocks/{label}")
        }
        pub fn attach_block(
            agent_id: &impl std::fmt::Display,
            block_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/core-memory/blocks/attach/{block_id}")
        }
        pub fn detach_block(
            agent_id: &impl std::fmt::Display,
            block_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/core-memory/blocks/detach/{block_id}")
        }
    }

    pub mod archival {
        pub fn list(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/archival-memory")
        }
        pub fn search(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/archival-memory/search")
        }
        pub fn create(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/archival-memory")
        }
        pub fn update(
            agent_id: &impl std::fmt::Display,
            memory_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/archival-memory/{memory_id}")
        }
        pub fn delete(
            agent_id: &impl std::fmt::Display,
            memory_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/archival-memory/{memory_id}")
        }
    }

    pub mod tools {
        pub fn list(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/tools")
        }
        pub fn attach(
            agent_id: &impl std::fmt::Display,
            tool_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/tools/attach/{tool_id}")
        }
        pub fn detach(
            agent_id: &impl std::fmt::Display,
            tool_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/tools/detach/{tool_id}")
        }
    }

    pub mod files {
        pub fn list(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/files")
        }
        pub fn open(agent_id: &impl std::fmt::Display, file_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/files/{file_id}/open")
        }
        pub fn close(
            agent_id: &impl std::fmt::Display,
            file_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/files/{file_id}/close")
        }
        pub fn close_all(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/files/close-all")
        }
    }

    pub mod sources {
        pub fn list(agent_id: &impl std::fmt::Display) -> String {
            format!("v1/agents/{agent_id}/sources")
        }
        pub fn attach(
            agent_id: &impl std::fmt::Display,
            source_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/sources/attach/{source_id}")
        }
        pub fn detach(
            agent_id: &impl std::fmt::Display,
            source_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/sources/detach/{source_id}")
        }
    }

    pub mod archives {
        pub fn attach(
            agent_id: &impl std::fmt::Display,
            archive_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/archives/attach/{archive_id}")
        }
        pub fn detach(
            agent_id: &impl std::fmt::Display,
            archive_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/archives/detach/{archive_id}")
        }
    }

    pub mod folders {
        pub fn attach(
            agent_id: &impl std::fmt::Display,
            folder_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/folders/attach/{folder_id}")
        }
        pub fn detach(
            agent_id: &impl std::fmt::Display,
            folder_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/agents/{agent_id}/folders/detach/{folder_id}")
        }
    }
}

// ---------------------------------------------------------------------------
// Archives
// ---------------------------------------------------------------------------
pub mod archives {
    pub const LIST: &str = "v1/archives/";
    pub const CREATE: &str = "v1/archives/";

    pub fn get(archive_id: &impl std::fmt::Display) -> String {
        format!("v1/archives/{archive_id}")
    }
    pub fn update(archive_id: &impl std::fmt::Display) -> String {
        format!("v1/archives/{archive_id}")
    }
    pub fn delete(archive_id: &impl std::fmt::Display) -> String {
        format!("v1/archives/{archive_id}")
    }
    pub fn list_agents(archive_id: &impl std::fmt::Display) -> String {
        format!("v1/archives/{archive_id}/agents")
    }

    pub mod passages {
        pub fn list(archive_id: &impl std::fmt::Display) -> String {
            format!("v1/archives/{archive_id}/passages")
        }
        pub fn create(archive_id: &impl std::fmt::Display) -> String {
            format!("v1/archives/{archive_id}/passages")
        }
        pub fn batch_create(archive_id: &impl std::fmt::Display) -> String {
            format!("v1/archives/{archive_id}/passages/batch")
        }
        pub fn delete(
            archive_id: &impl std::fmt::Display,
            passage_id: &impl std::fmt::Display,
        ) -> String {
            format!("v1/archives/{archive_id}/passages/{passage_id}")
        }
    }
}

// ---------------------------------------------------------------------------
// Batch
// ---------------------------------------------------------------------------
pub mod batch {
    pub const LIST: &str = "v1/messages/batches/";
    pub const CREATE: &str = "v1/messages/batches";

    pub fn get(batch_id: &impl std::fmt::Display) -> String {
        format!("v1/messages/batches/{batch_id}")
    }
    pub fn cancel(batch_id: &impl std::fmt::Display) -> String {
        format!("v1/messages/batches/{batch_id}/cancel")
    }
    pub fn messages(batch_id: &impl std::fmt::Display) -> String {
        format!("v1/messages/batches/{batch_id}/messages")
    }
}

// ---------------------------------------------------------------------------
// Blocks
// ---------------------------------------------------------------------------
pub mod blocks {
    pub const LIST: &str = "v1/blocks/";
    pub const CREATE: &str = "v1/blocks/";
    pub const COUNT: &str = "v1/blocks/count";

    pub fn get(block_id: &impl std::fmt::Display) -> String {
        format!("v1/blocks/{block_id}")
    }
    pub fn update(block_id: &impl std::fmt::Display) -> String {
        format!("v1/blocks/{block_id}")
    }
    pub fn delete(block_id: &impl std::fmt::Display) -> String {
        format!("v1/blocks/{block_id}")
    }
    pub fn list_agents(block_id: &impl std::fmt::Display) -> String {
        format!("v1/blocks/{block_id}/agents")
    }
}

// ---------------------------------------------------------------------------
// Conversations
// ---------------------------------------------------------------------------
pub mod conversations {
    pub const LIST: &str = "v1/conversations/";
    pub const CREATE: &str = "v1/conversations/";

    pub fn get(conversation_id: &impl std::fmt::Display) -> String {
        format!("v1/conversations/{conversation_id}")
    }
    pub fn update(conversation_id: &impl std::fmt::Display) -> String {
        format!("v1/conversations/{conversation_id}")
    }
    pub fn delete(conversation_id: &impl std::fmt::Display) -> String {
        format!("v1/conversations/{conversation_id}")
    }
    pub fn cancel(conversation_id: &impl std::fmt::Display) -> String {
        format!("v1/conversations/{conversation_id}/cancel")
    }
    pub fn compact(conversation_id: &impl std::fmt::Display) -> String {
        format!("v1/conversations/{conversation_id}/compact")
    }
    pub fn messages(conversation_id: &impl std::fmt::Display) -> String {
        format!("v1/conversations/{conversation_id}/messages")
    }
    pub fn stream(conversation_id: &impl std::fmt::Display) -> String {
        format!("v1/conversations/{conversation_id}/stream")
    }
}

// ---------------------------------------------------------------------------
// Folders
// ---------------------------------------------------------------------------
pub mod folders {
    pub const LIST: &str = "v1/folders/";
    pub const CREATE: &str = "v1/folders/";
    pub const COUNT: &str = "v1/folders/count";
    pub const METADATA: &str = "v1/folders/metadata";

    pub fn get(folder_id: &impl std::fmt::Display) -> String {
        format!("v1/folders/{folder_id}")
    }
    pub fn update(folder_id: &impl std::fmt::Display) -> String {
        format!("v1/folders/{folder_id}")
    }
    pub fn delete(folder_id: &impl std::fmt::Display) -> String {
        format!("v1/folders/{folder_id}")
    }
    pub fn get_by_name(name: &str) -> String {
        format!("v1/folders/name/{name}")
    }
    pub fn upload(folder_id: &impl std::fmt::Display) -> String {
        format!("v1/folders/{folder_id}/upload")
    }
    pub fn list_agents(folder_id: &impl std::fmt::Display) -> String {
        format!("v1/folders/{folder_id}/agents")
    }
    pub fn list_passages(folder_id: &impl std::fmt::Display) -> String {
        format!("v1/folders/{folder_id}/passages")
    }
    pub fn list_files(folder_id: &impl std::fmt::Display) -> String {
        format!("v1/folders/{folder_id}/files")
    }
    pub fn delete_file(
        folder_id: &impl std::fmt::Display,
        file_id: &impl std::fmt::Display,
    ) -> String {
        format!("v1/folders/{folder_id}/{file_id}")
    }
}

// ---------------------------------------------------------------------------
// Groups
// ---------------------------------------------------------------------------
pub mod groups {
    pub const LIST: &str = "v1/groups/";
    pub const CREATE: &str = "v1/groups";

    pub fn get(group_id: &impl std::fmt::Display) -> String {
        format!("v1/groups/{group_id}")
    }
    pub fn update(group_id: &impl std::fmt::Display) -> String {
        format!("v1/groups/{group_id}")
    }
    pub fn delete(group_id: &impl std::fmt::Display) -> String {
        format!("v1/groups/{group_id}")
    }
    pub fn send_message(group_id: &impl std::fmt::Display) -> String {
        format!("v1/groups/{group_id}/messages")
    }
    pub fn stream(group_id: &impl std::fmt::Display) -> String {
        format!("v1/groups/{group_id}/messages/stream")
    }
    pub fn list_messages(group_id: &impl std::fmt::Display) -> String {
        format!("v1/groups/{group_id}/messages")
    }
    pub fn get_message(
        group_id: &impl std::fmt::Display,
        message_id: &impl std::fmt::Display,
    ) -> String {
        format!("v1/groups/{group_id}/messages/{message_id}")
    }
    pub fn reset_messages(group_id: &impl std::fmt::Display) -> String {
        format!("v1/agents/{group_id}/reset-messages")
    }
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------
pub mod health {
    pub const CHECK: &str = "v1/health/";
}

// ---------------------------------------------------------------------------
// Identities
// ---------------------------------------------------------------------------
pub mod identities {
    pub const LIST: &str = "v1/identities/";
    pub const CREATE: &str = "v1/identities/";
    pub const UPSERT: &str = "v1/identities/";
    pub const COUNT: &str = "v1/identities/count";

    pub fn get(identity_id: &impl std::fmt::Display) -> String {
        format!("v1/identities/{identity_id}")
    }
    pub fn update(identity_id: &impl std::fmt::Display) -> String {
        format!("v1/identities/{identity_id}")
    }
    pub fn delete(identity_id: &impl std::fmt::Display) -> String {
        format!("v1/identities/{identity_id}")
    }
}

// ---------------------------------------------------------------------------
// Jobs
// ---------------------------------------------------------------------------
pub mod jobs {
    pub const LIST: &str = "v1/jobs/";
    pub const LIST_ACTIVE: &str = "v1/jobs/active/";

    pub fn get(job_id: &impl std::fmt::Display) -> String {
        format!("v1/jobs/{job_id}")
    }
    pub fn delete(job_id: &impl std::fmt::Display) -> String {
        format!("v1/jobs/{job_id}")
    }
}

// ---------------------------------------------------------------------------
// Steps
// ---------------------------------------------------------------------------
pub mod steps {
    pub const LIST: &str = "v1/steps/";

    pub fn get(step_id: &impl std::fmt::Display) -> String {
        format!("v1/steps/{step_id}")
    }
    pub fn feedback(step_id: &impl std::fmt::Display) -> String {
        format!("v1/steps/{step_id}/feedback")
    }
    pub fn feedback_with_value(
        step_id: &impl std::fmt::Display,
        feedback: &impl std::fmt::Display,
    ) -> String {
        format!("v1/steps/{step_id}/feedback?feedback={feedback}")
    }
    pub fn messages(step_id: &impl std::fmt::Display) -> String {
        format!("v1/steps/{step_id}/messages")
    }
    pub fn metrics(step_id: &impl std::fmt::Display) -> String {
        format!("v1/steps/{step_id}/metrics")
    }
    pub fn trace(step_id: &impl std::fmt::Display) -> String {
        format!("v1/steps/{step_id}/trace")
    }
    pub fn transaction(step_id: &impl std::fmt::Display, transaction_id: &str) -> String {
        format!("v1/steps/{step_id}/transaction/{transaction_id}")
    }
}

// ---------------------------------------------------------------------------
// MCP Servers (v2)
// ---------------------------------------------------------------------------
pub mod mcp_servers {
    pub const LIST: &str = "v1/mcp-servers/";
    pub const CREATE: &str = "v1/mcp-servers/";

    pub fn get(server_id: &impl std::fmt::Display) -> String {
        format!("v1/mcp-servers/{server_id}")
    }
    pub fn update(server_id: &impl std::fmt::Display) -> String {
        format!("v1/mcp-servers/{server_id}")
    }
    pub fn delete(server_id: &impl std::fmt::Display) -> String {
        format!("v1/mcp-servers/{server_id}")
    }
    pub fn connect(server_id: &impl std::fmt::Display) -> String {
        format!("v1/mcp-servers/connect/{server_id}")
    }
    pub fn refresh(server_id: &impl std::fmt::Display) -> String {
        format!("v1/mcp-servers/{server_id}/refresh")
    }
    pub fn list_tools(server_id: &impl std::fmt::Display) -> String {
        format!("v1/mcp-servers/{server_id}/tools")
    }
    pub fn get_tool(
        server_id: &impl std::fmt::Display,
        tool_id: &impl std::fmt::Display,
    ) -> String {
        format!("v1/mcp-servers/{server_id}/tools/{tool_id}")
    }
    pub fn run_tool(
        server_id: &impl std::fmt::Display,
        tool_id: &impl std::fmt::Display,
    ) -> String {
        format!("v1/mcp-servers/{server_id}/tools/{tool_id}/run")
    }
}

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------
pub mod models {
    pub const LIST: &str = "v1/models/";
    pub const LIST_EMBEDDING: &str = "v1/models/embedding/";
}

// ---------------------------------------------------------------------------
// Projects
// ---------------------------------------------------------------------------
pub mod projects {
    pub const LIST: &str = "v1/projects/";
}

// ---------------------------------------------------------------------------
// Providers
// ---------------------------------------------------------------------------
pub mod providers {
    pub const LIST: &str = "v1/providers/";
    pub const CREATE: &str = "v1/providers";

    pub fn delete(provider_id: &impl std::fmt::Display) -> String {
        format!("v1/providers/{provider_id}")
    }
    pub fn update(provider_id: &impl std::fmt::Display) -> String {
        format!("v1/providers/{provider_id}")
    }
    pub fn check(provider_id: &impl std::fmt::Display) -> String {
        format!("v1/providers/{provider_id}/check")
    }
}

// ---------------------------------------------------------------------------
// Runs
// ---------------------------------------------------------------------------
pub mod runs {
    pub const LIST: &str = "v1/runs/";
    pub const LIST_ACTIVE: &str = "v1/runs/active/";

    pub fn get(run_id: &impl std::fmt::Display) -> String {
        format!("v1/runs/{run_id}")
    }
    pub fn messages(run_id: &impl std::fmt::Display) -> String {
        format!("v1/runs/{run_id}/messages")
    }
    pub fn steps(run_id: &impl std::fmt::Display) -> String {
        format!("v1/runs/{run_id}/steps")
    }
    pub fn metrics(run_id: &impl std::fmt::Display) -> String {
        format!("v1/runs/{run_id}/metrics")
    }
    pub fn usage(run_id: &impl std::fmt::Display) -> String {
        format!("v1/runs/{run_id}/usage")
    }
    pub fn trace(run_id: &impl std::fmt::Display) -> String {
        format!("v1/runs/{run_id}/trace")
    }
    pub fn stream(run_id: &impl std::fmt::Display) -> String {
        format!("v1/runs/{run_id}/stream")
    }
}

// ---------------------------------------------------------------------------
// Sources
// ---------------------------------------------------------------------------
pub mod sources {
    pub const LIST: &str = "v1/sources/";
    pub const CREATE: &str = "v1/sources/";
    pub const COUNT: &str = "v1/sources/count";

    pub fn get(source_id: &impl std::fmt::Display) -> String {
        format!("v1/sources/{source_id}")
    }
    pub fn update(source_id: &impl std::fmt::Display) -> String {
        format!("v1/sources/{source_id}")
    }
    pub fn delete(source_id: &impl std::fmt::Display) -> String {
        format!("v1/sources/{source_id}")
    }
    pub fn get_by_name(name: &str) -> String {
        format!("v1/sources/name/{name}")
    }
    pub fn upload(source_id: &impl std::fmt::Display) -> String {
        format!("v1/sources/{source_id}/upload")
    }
    pub fn list_files(source_id: &impl std::fmt::Display) -> String {
        format!("v1/sources/{source_id}/files")
    }
    pub fn get_file(
        source_id: &impl std::fmt::Display,
        file_id: &impl std::fmt::Display,
    ) -> String {
        format!("v1/sources/{source_id}/files/{file_id}")
    }
    pub fn delete_file(
        source_id: &impl std::fmt::Display,
        file_id: &impl std::fmt::Display,
    ) -> String {
        format!("v1/sources/{source_id}/{file_id}")
    }
    pub fn list_passages(source_id: &impl std::fmt::Display) -> String {
        format!("v1/sources/{source_id}/passages")
    }
}

// ---------------------------------------------------------------------------
// Tags
// ---------------------------------------------------------------------------
pub mod tags {
    pub const LIST: &str = "v1/tags/";
}

// ---------------------------------------------------------------------------
// Telemetry
// ---------------------------------------------------------------------------
pub mod telemetry {
    pub fn get(step_id: &(impl std::fmt::Display + ?Sized)) -> String {
        format!("v1/telemetry/{step_id}")
    }
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------
pub mod templates {
    pub const LIST: &str = "v1/templates/";

    pub fn create_agents(project: &str, template_version: &str) -> String {
        format!("v1/templates/{project}/{template_version}/agents")
    }
}

// ---------------------------------------------------------------------------
// Tools (core CRUD)
// ---------------------------------------------------------------------------
pub mod tools {
    pub const LIST: &str = "v1/tools/";
    pub const CREATE: &str = "v1/tools/";
    pub const UPSERT: &str = "v1/tools/";
    pub const COUNT: &str = "v1/tools/count";
    pub const RUN_FROM_SOURCE: &str = "v1/tools/run";
    pub const ADD_BASE_TOOLS: &str = "v1/tools/add-base-tools";

    pub fn get(tool_id: &impl std::fmt::Display) -> String {
        format!("v1/tools/{tool_id}")
    }
    pub fn update(tool_id: &impl std::fmt::Display) -> String {
        format!("v1/tools/{tool_id}")
    }
    pub fn delete(tool_id: &impl std::fmt::Display) -> String {
        format!("v1/tools/{tool_id}")
    }

    pub mod composio {
        pub const LIST_APPS: &str = "v1/tools/composio/apps";

        pub fn list_actions(app_name: &str) -> String {
            format!("v1/tools/composio/apps/{app_name}/actions")
        }
        pub fn add_tool(action_name: &str) -> String {
            format!("v1/tools/composio/{action_name}")
        }
    }
}

// ---------------------------------------------------------------------------
// Tools (legacy MCP via /v1/tools/mcp/servers)
// ---------------------------------------------------------------------------
pub mod tools_mcp {
    pub const LIST_SERVERS: &str = "v1/tools/mcp/servers";
    pub const ADD_SERVER: &str = "v1/tools/mcp/servers";
    pub const TEST_SERVER: &str = "v1/tools/mcp/servers/test";

    pub fn list_servers_with_user(user_id: &str) -> String {
        format!("v1/tools/mcp/servers?user-id={user_id}")
    }
    pub fn list_tools(server_name: &str) -> String {
        format!("v1/tools/mcp/servers/{server_name}/tools")
    }
    pub fn register_tool(server_name: &str, tool_name: &str) -> String {
        format!("v1/tools/mcp/servers/{server_name}/{tool_name}")
    }
    pub fn delete_server(server_name: &str) -> String {
        format!("v1/tools/mcp/servers/{server_name}")
    }
    pub fn update_server(server_name: &str) -> String {
        format!("v1/tools/mcp/servers/{server_name}")
    }
}

// ---------------------------------------------------------------------------
// Voice
// ---------------------------------------------------------------------------
pub mod voice {
    pub fn chat(agent_id: &impl std::fmt::Display) -> String {
        format!("v1/voice-beta/{agent_id}/chat/completions")
    }
}
