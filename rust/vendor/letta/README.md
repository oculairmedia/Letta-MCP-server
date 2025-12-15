# letta

A Rust client library for the [Letta](https://letta.com) REST API, providing idiomatic Rust bindings for building stateful AI agents with persistent memory and context.

Unlike the Letta-provided TypeScript and Python libraries, this was not generated from the OpenAPI spec, but implemented by hand (with substantial LLM assistance). As such it exposes things in slightly different, mildly opinionated ways, and includes a number of Rust-oriented affordances.

[![Crates.io](https://img.shields.io/crates/v/letta.svg)](https://crates.io/crates/letta) [![Documentation](https://docs.rs/letta/badge.svg)](https://docs.rs/letta) [![License](https://img.shields.io/crates/l/letta.svg)](./LICENSE)

## Features

- **Pagination**: Automatic cursor-based pagination with `PaginatedStream`
- **Type Safety**: Comprehensive type definitions for all API requests/responses
- **Flexible Configuration**: Support for cloud and local deployments
- **Rich Error Handling**: Detailed error types
- **Well Tested**: Extensive test coverage with integration tests

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
letta = "0.1.3"
```

## CLI Installation

The letta crate includes an optional CLI tool for interacting with Letta servers:

```bash
# Install from crates.io
cargo install letta --features cli

# Or build from source
git clone https://github.com/orual/letta-rs
cd letta-rs
cargo install --path . --features cli
```

After installation, the `letta-client` command will be available in your PATH.

### CLI Configuration

Set your API key (for cloud deployments):
```bash
export LETTA_API_KEY=your-api-key
```

Or specify the base URL for local servers:
```bash
export LETTA_BASE_URL=http://localhost:8283
```

### CLI Usage Examples

```bash
# Check server health
letta-client health

# List all agents
letta-client agent list

# Create a new agent
letta-client agent create -n "My Assistant" -m letta/letta-free

# Send a message to an agent (with streaming)
letta-client message send -a <agent-id> "Hello, how are you?"

# View agent memory
letta-client memory view -a <agent-id>

# Upload a document to a source
letta-client sources create -n "docs" -e letta/letta-free
letta-client sources files upload <source-id> -f document.pdf

# Get help for any command
letta-client --help
letta-client agent --help
```

The CLI supports multiple output formats:
- `--output summary` (default) - Human-readable format
- `--output json` - JSON output for scripting
- `--output pretty` - Pretty-printed JSON

## Compatibility

| letta client | letta server |
|--------------|--------------|
| 0.1.3        | 0.8.8        |
| 0.1.0-0.1.2  | 0.8.x        |

## Quick Start

```rust
use letta::{ClientConfig, LettaClient};
use letta::types::{CreateAgentRequest, AgentType, ModelEndpointType, EmbeddingEndpointType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client for local Letta server
    let config = ClientConfig::new("http://localhost:8283")?;
    let client = LettaClient::new(config)?;

    // Create an agent
    let agent_request = CreateAgentRequest {
        name: "My Assistant".to_string(),
        agent_type: Some(AgentType::MemGPT),
        llm_config: Some(json!({
            "model_endpoint_type": ModelEndpointType::Openai,
            "model_endpoint": "https://api.openai.com/v1",
            "model": "gpt-4",
        })),
        embedding_config: Some(json!({
            "embedding_endpoint_type": EmbeddingEndpointType::Openai,
            "embedding_endpoint": "https://api.openai.com/v1",
            "embedding_model": "text-embedding-ada-002",
        })),
        ..Default::default()
    };

    let agent = client.agents().create(agent_request).await?;
    println!("Created agent: {}", agent.id);

    // Send a message to the agent
    let response = client
        .messages()
        .send(&agent.id, "Hello! How are you today?", None)
        .await?;

    // Stream responses
    let mut stream = response.into_stream();
    while let Some(chunk) = stream.next().await {
        match chunk? {
            MessageChunk::AssistantMessage(msg) => {
                print!("{}", msg.message);
            }
            MessageChunk::FunctionCall(call) => {
                println!("Function: {} with args: {}", call.name, call.arguments);
            }
            _ => {}
        }
    }

    Ok(())
}
```

## API Coverage

### Core APIs
- ✅ **Agents** - Create, update, delete, and manage AI agents
- ✅ **Messages** - Send messages and stream responses with SSE
- ✅ **Memory** - Manage core and archival memory with semantic search
- ✅ **Tools** - Register and manage agent tools (functions)
- ✅ **Sources** - Upload documents and manage knowledge sources
- ✅ **Blocks** - Manage memory blocks and persistent storage

### Advanced APIs
- ✅ **Groups** - Multi-agent conversations
- ✅ **Runs** - Execution tracking and debugging
- ✅ **Jobs** - Asynchronous job management
- ✅ **Batch** - Batch message processing
- ✅ **Templates** - Agent templates for quick deployment
- ✅ **Projects** - Project organization
- ✅ **Models** - LLM and embedding model configuration
- ✅ **Providers** - LLM provider management
- ✅ **Identities** - Identity and permissions management
- ✅ **Tags** - Tag-based organization
- ✅ **Telemetry** - Usage tracking and monitoring
- 🚧 **Voice** - Voice conversation support (beta)

## Examples

### Creating an Agent with Builder Pattern

```rust
use letta::types::{CreateAgentRequest, AgentType, Block, LLMConfig};

// Create agent using builder pattern
let request = CreateAgentRequest::builder()
    .name("My Assistant")
    .agent_type(AgentType::MemGPT)
    .description("A helpful coding assistant")
    .model("letta/letta-free")  // Shorthand for LLM config
    .embedding("letta/letta-free")  // Shorthand for embedding config
    .build();

let agent = client.agents().create(request).await?;

// Create custom memory blocks with builder
let human_block = Block::human("Name: Alice\nRole: Software Engineer")
    .label("human");

let persona_block = Block::persona("You are a helpful coding assistant.")
    .label("persona");
```

### Working with Archival Memory

```rust
// Add to archival memory
client
    .memory()
    .insert_archival_memory(&agent.id, "Important fact: Rust is memory safe")
    .await?;

// Search archival memory
let memories = client
    .memory()
    .search_archival_memory(&agent.id, "Rust safety", Some(10))
    .await?;

for memory in memories {
    println!("Found: {}", memory.text);
}
```

### Streaming with Pagination

```rust
// Get paginated list of agents
let mut agent_stream = client
    .agents()
    .paginated()
    .limit(10)
    .build();

while let Some(agent) = agent_stream.next().await {
    let agent = agent?;
    println!("Agent: {} ({})", agent.name, agent.id);
}
```

### Managing Tools

```rust
use letta::types::{CreateToolRequest, Tool};

// Create a custom tool
// Note: this example is simplified, see the tool documentation for details.
let tool = CreateToolRequest {
    name: "get_weather".to_string(),
    description: Some("Get current weather for a location".to_string()),
    source_code: r#"
def get_weather(location: str) -> str:
    """Get weather for a location."""
    return f"The weather in {location} is sunny and 72°F"
"#.to_string(),
    source_type: Some("python".to_string()),
    ..Default::default()
};

let created_tool = client.tools().create(tool).await?;

// Add tool to agent
client
    .agents()
    .add_tool(&agent.id, &created_tool.id)
    .await?;
```

## Configuration

### Local Development Server

```rust
// No authentication required for local server
let config = ClientConfig::new("http://localhost:8283")?;
let client = LettaClient::new(config)?;
```

### Letta Cloud

```rust
// Use API key for cloud deployment
let config = ClientConfig::new("https://api.letta.com")?
    .with_api_key("your-api-key");
let client = LettaClient::new(config)?;
```

### Custom Headers

```rust
// Add custom headers like X-Project
let config = ClientConfig::new("http://localhost:8283")?
    .with_header("X-Project", "my-project")?;
```

## Error Handling

The library provides comprehensive error handling with detailed context:

```rust
use letta::error::LettaError;

match client.agents().get(&agent_id).await {
    Ok(agent) => println!("Found agent: {}", agent.name),
    Err(LettaError::Api { status, message, .. }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/letta
cd letta

# Build the library
cargo build

# Run tests
cargo test

# Build documentation
cargo doc --open
```

### Running the Local Test Server

```bash
# Start local Letta server for testing
docker compose up -d

# Run integration tests
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [reqwest](https://github.com/seanmonstar/reqwest) for HTTP operations
- Uses [tokio](https://github.com/tokio-rs/tokio) for async runtime
- Streaming support via [eventsource-stream](https://github.com/jpopesculian/eventsource-stream)
- Error handling with [miette](https://github.com/zkat/miette)

## Related Projects

- [Letta](https://github.com/letta-ai/letta) - The official Letta server
- [letta-node](https://github.com/letta-ai/letta-node) - Official TypeScript/JavaScript SDK
- [letta-python](https://github.com/letta-ai/letta-python) - Official Python SDK
