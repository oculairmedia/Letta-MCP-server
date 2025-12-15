//! # letta
//!
//! A Rust client library for the [Letta](https://letta.com) REST API, providing idiomatic Rust bindings
//! for building stateful AI agents with persistent memory and context.
//!
//! Unlike the Letta-provided TypeScript and Python libraries, this was not generated from the OpenAPI spec,
//! but implemented by hand (with substantial LLM assistance). As such it exposes things in slightly different,
//! mildly opinionated ways, and includes a number of Rust-oriented affordances.
//!
//! ## Features
//!
//! - **Pagination**: Automatic cursor-based pagination with `PaginatedStream`
//! - **Type Safety**: Comprehensive type definitions for all API requests/responses
//! - **Flexible Configuration**: Support for cloud and local deployments
//! - **Rich Error Handling**: Detailed error types
//! - **Well Tested**: Extensive test coverage with integration tests
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! letta = "0.1.3"
//! ```
//!
//! ### CLI Installation
//!
//! The letta crate includes an optional CLI tool for interacting with Letta servers:
//!
//! ```bash
//! # Install from crates.io
//! cargo install letta --features cli
//!
//! # Or build from source
//! git clone https://github.com/orual/letta-rs
//! cd letta-rs
//! cargo install --path . --features cli
//! ```
//!
//! After installation, the `letta` command will be available in your PATH.
//!
//! ### CLI Configuration
//!
//! Set your API key (for cloud deployments):
//! ```bash
//! export LETTA_API_KEY=your-api-key
//! ```
//!
//! Or specify the base URL for local servers:
//! ```bash
//! export LETTA_BASE_URL=http://localhost:8283
//! ```
//!
//! ### CLI Usage Examples
//!
//! ```bash
//! # Check server health
//! letta health
//!
//! # List all agents
//! letta agent list
//!
//! # Create a new agent
//! letta agent create -n "My Assistant" -m letta/letta-free
//!
//! # Send a message to an agent (with streaming)
//! letta message send -a <agent-id> "Hello, how are you?"
//!
//! # View agent memory
//! letta memory view -a <agent-id>
//!
//! # Upload a document to a source
//! letta sources create -n "docs" -e letta/letta-free
//! letta sources files upload <source-id> -f document.pdf
//!
//! # Get help for any command
//! letta --help
//! letta agent --help
//! ```
//!
//! The CLI supports multiple output formats:
//! - `--output summary` (default) - Human-readable format
//! - `--output json` - JSON output for scripting
//! - `--output pretty` - Pretty-printed JSON
//!
//! ## Compatibility
//!
//! | letta client | letta server |
//! |--------------|--------------|
//! | 0.1.3        | 0.8.8        |
//! | 0.1.0-0.1.2  | 0.8.x        |
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use letta::{ClientConfig, LettaClient};
//! use letta::types::{CreateAgentRequest, AgentType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client for local Letta server
//!     let config = ClientConfig::new("http://localhost:8283")?;
//!     let client = LettaClient::new(config)?;
//!
//!     // Create an agent using builder pattern
//!     let agent_request = CreateAgentRequest::builder()
//!         .name("My Assistant")
//!         .agent_type(AgentType::MemGPT)
//!         .model("letta/letta-free")  // Shorthand for LLM config
//!         .embedding("letta/letta-free")  // Shorthand for embedding config
//!         .build();
//!
//!     let agent = client.agents().create(agent_request).await?;
//!     println!("Created agent: {}", agent.id);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Creating and Interacting with Agents
//!
//! ```rust,no_run
//! use letta::{LettaClient, types::*};
//! use futures::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!
//!     // Create a new agent
//!     let agent_request = CreateAgentRequest {
//!         name: Some("Assistant".to_string()),
//!         agent_type: Some(AgentType::MemGPT),
//!         model: Some("letta/letta-free".to_string()),
//!         embedding: Some("letta/letta-free".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let agent = client.agents().create(agent_request).await?;
//!     println!("Created agent: {}", agent.id);
//!
//!     // Send a simple message
//!     let request = CreateMessagesRequest {
//!         messages: vec![MessageCreate::user("Hello! Can you help me with Rust?")],
//!         ..Default::default()
//!     };
//!
//!     // Or create a more complex message with multiple content parts
//!     let complex_message = MessageCreate {
//!         role: MessageRole::User,
//!         content: MessageCreateContent::ContentParts(vec![
//!             ContentPart::Text(TextContent {
//!                 text: "Here's an image that shows my error:".to_string(),
//!             }),
//!             ContentPart::Image(ImageContent {
//!                 image_url: ImageUrl {
//!                     url: "https://example.com/error-screenshot.png".to_string(),
//!                     detail: None,
//!                 },
//!             }),
//!         ]),
//!         name: Some("Developer".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let response = client
//!         .messages()
//!         .create(&agent.id, request)
//!         .await?;
//!
//!     // Process the response messages
//!     for message in response.messages {
//!         match &message {
//!             LettaMessageUnion::ReasoningMessage(m) => {
//!                 println!("Thinking: {}", m.reasoning);
//!             }
//!             LettaMessageUnion::ToolReturnMessage(t) => {
//!                 println!("Tool returned: {:?}", t.tool_return);
//!             }
//!             LettaMessageUnion::AssistantMessage(m) => {
//!                 println!("Assistant: {}", m.content);
//!             }
//!             _ => {} // Other message types
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Memory Management
//!
//! ```rust,no_run
//! use letta::{LettaClient, types::*};
//! use std::str::FromStr;
//! use futures::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!     let agent_id = LettaId::from_str("agent-00000000-0000-0000-0000-000000000000")?;
//!
//!     // Get a specific core memory block
//!     let block = client
//!         .memory()
//!         .get_core_memory_block(&agent_id, "human")
//!         .await?;
//!
//!     println!("Current human memory: {}", block.value);
//!
//!     // Update core memory block
//!     let request = UpdateMemoryBlockRequest {
//!         value: Some("Name: Bob\nRole: Manager".to_string()),
//!         label: None,
//!         limit: None,
//!         name: None,
//!         preserve_on_migration: None,
//!         read_only: None,
//!         description: None,
//!         metadata: None,
//!     };
//!     let updated = client
//!         .memory()
//!         .update_core_memory_block(&agent_id, "human", request)
//!         .await?;
//!
//!     println!("Updated memory block: {}", updated.id.as_ref().unwrap());
//!
//!     // Add to archival memory
//!     let archival_request = CreateArchivalMemoryRequest {
//!         text: "Important: Project deadline is next Friday".to_string(),
//!     };
//!     let memories = client
//!         .memory()
//!         .create_archival_memory(&agent_id, archival_request)
//!         .await?;
//!
//!     println!("Inserted {} archival memory passages", memories.len());
//!
//!     // Search archival memory with semantic search
//!     let search_params = PaginationParams {
//!         limit: Some(5),
//!         ..Default::default()
//!     };
//!     let mut stream = client
//!         .memory()
//!         .archival_paginated(&agent_id, Some(search_params));
//!
//!     while let Some(result) = stream.next().await {
//!         let result = result?;
//!         println!("Found: {}", result.text);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Working with Tools
//!
//! ```rust,no_run
//! use letta::{LettaClient, types::*};
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!     let agent_id = "agent-00000000-0000-0000-0000-000000000000";
//!
//!     // Create a custom tool
//!     let tool = CreateToolRequest {
//!         description: Some("Get current weather for a location".to_string()),
//!         source_code: r#"
//! def get_weather(location: str) -> str:
//!     """Get weather for a location.
//!
//!     Args:
//!         location: The location to get weather for
//!
//!     Returns:
//!         Weather information as a string
//!     """
//!     return f"The weather in {location} is sunny and 72°F"
//! "#.to_string(),
//!         source_type: Some(SourceType::Python),
//!         json_schema: Some(json!({
//!             "name": "get_weather",
//!             "description": "Get current weather for a location",
//!             "parameters": {
//!                 "type": "object",
//!                 "properties": {
//!                     "location": {
//!                         "type": "string",
//!                         "description": "The location to get weather for"
//!                     }
//!                 },
//!                 "required": ["location"]
//!             }
//!         })),
//!         ..Default::default()
//!     };
//!
//!     let created_tool = client.tools().create(tool).await?;
//!
//!     // Tools are automatically available to agents after creation
//!     println!("Created tool: {}", created_tool.name);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Pagination Support
//!
//! ```rust,no_run
//! use letta::{LettaClient, types::*};
//! use futures::StreamExt;
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!
//!     // Use pagination to handle large result sets
//!     let params = PaginationParams {
//!         limit: Some(10),
//!         ..Default::default()
//!     };
//!     let mut agent_stream = client
//!         .agents()
//!         .paginated(Some(params));
//!
//!     // Automatically fetches next pages as needed
//!     while let Some(agent) = agent_stream.next().await {
//!         let agent = agent?;
//!         println!("Agent: {} ({})", agent.name, agent.id);
//!     }
//!
//!     // Also works with archival memory
//!     let agent_id = LettaId::from_str("agent-00000000-0000-0000-0000-000000000000")?;
//!     let memory_params = PaginationParams {
//!         limit: Some(20),
//!         ..Default::default()
//!     };
//!     let mut memory_stream = client
//!         .memory()
//!         .archival_paginated(&agent_id, Some(memory_params));
//!
//!     while let Some(memory) = memory_stream.next().await {
//!         let memory = memory?;
//!         println!("Memory: {}", memory.text);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! ### Local Development Server
//!
//! ```rust,no_run
//! use letta::{ClientConfig, LettaClient};
//!
//! // No authentication required for local server
//! let config = ClientConfig::new("http://localhost:8283")?;
//! let client = LettaClient::new(config)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Letta Cloud
//!
//! ```rust,no_run
//! use letta::{ClientConfig, LettaClient};
//! use letta::auth::AuthConfig;
//!
//! // Use API key for cloud deployment
//! let config = ClientConfig::new("https://api.letta.com")?
//!     .auth(AuthConfig::bearer("your-api-key"));
//! let client = LettaClient::new(config)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Custom Headers
//!
//! ```rust,no_run
//! use letta::{ClientConfig, LettaClient};
//!
//! // Add custom headers like X-Project
//! let config = ClientConfig::new("http://localhost:8283")?
//!     .header("X-Project", "my-project")?;
//! let client = LettaClient::new(config)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## API Coverage
//!
//! ### Core APIs
//! - ✅ **Agents** - Create, update, delete, and manage AI agents
//! - ✅ **Messages** - Send messages and stream responses with SSE
//! - ✅ **Memory** - Manage core and archival memory with semantic search
//! - ✅ **Tools** - Register and manage agent tools (functions)
//! - ✅ **Sources** - Upload documents and manage knowledge sources
//! - ✅ **Blocks** - Manage memory blocks and persistent storage
//!
//! ### Advanced APIs
//! - ✅ **Groups** - Multi-agent conversations
//! - ✅ **Runs** - Execution tracking and debugging
//! - ✅ **Jobs** - Asynchronous job management
//! - ✅ **Batch** - Batch message processing
//! - ✅ **Templates** - Agent templates for quick deployment
//! - ✅ **Projects** - Project organization
//! - ✅ **Models** - LLM and embedding model configuration
//! - ✅ **Providers** - LLM provider management
//! - ✅ **Identities** - Identity and permissions management
//! - ✅ **Tags** - Tag-based organization
//! - ✅ **Telemetry** - Usage tracking and monitoring
//! - 🚧 **Voice** - Voice conversation support (beta)
//!
//! ## API Sections
//!
//! - [`agents`](crate::api::agents) - Agent management and lifecycle
//! - [`messages`](crate::api::messages) - Real-time messaging with streaming
//! - [`memory`](crate::api::memory) - Memory management (core, archival, blocks)
//! - [`tools`](crate::api::tools) - Tool management and execution
//! - [`sources`](crate::api::sources) - Document and data source management
//! - [`blocks`](crate::api::blocks) - Memory block operations
//! - [`groups`](crate::api::groups) - Multi-agent conversations
//! - [`jobs`](crate::api::jobs) - Asynchronous job management
//! - [`projects`](crate::api::projects) - Project organization
//! - [`templates`](crate::api::templates) - Agent templates
//! - [`runs`](crate::api::runs) - Execution tracking
//! - [`models`](crate::api::models) - Model configuration
//! - [`providers`](crate::api::providers) - LLM provider management
//! - [`identities`](crate::api::identities) - Identity management
//! - [`tags`](crate::api::tags) - Tag-based organization
//! - [`batch`](crate::api::batch) - Batch processing
//! - [`telemetry`](crate::api::telemetry) - Usage tracking
//! - [`voice`](crate::api::voice) - Voice conversations (beta)
//!
//! ## Error Handling
//!
//! All operations return [`Result<T, LettaError>`](crate::error::LettaError) with
//! comprehensive error information and suggestions via [`miette`].

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod api;
pub mod auth;
pub mod client;
pub mod environment;
pub mod error;
pub mod pagination;
pub mod retry;
pub mod streaming;
pub mod types;
pub mod utils;

#[cfg(test)]
pub mod test_helpers;

#[cfg(feature = "cli")]
#[cfg_attr(docsrs, doc(cfg(feature = "cli")))]
pub mod cli;

// Re-export main types for convenience
pub use client::{ClientBuilder, ClientConfig, LettaClient};
pub use environment::LettaEnvironment;
pub use error::{ErrorContext, LettaError, LettaResult};
pub use types::*;

/// Maximum number of retries for API calls
pub const MAX_RETRIES: u32 = 3;

/// Convenience type alias for Results in this crate.
pub type Result<T> = std::result::Result<T, LettaError>;

// Re-export streaming types
pub use api::messages::{MessageStream, StreamingEvent};
