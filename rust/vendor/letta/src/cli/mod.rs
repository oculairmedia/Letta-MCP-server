//! Command-line interface module.

pub mod commands;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Orual", version, about = "Letta REST API client")]
/// Letta command-line interface
pub struct Args {
    /// Base URL for the Letta API (defaults to http://localhost:8283)
    #[arg(
        short = 'u',
        long,
        env = "LETTA_BASE_URL",
        default_value = "http://localhost:8283"
    )]
    pub base_url: String,

    /// API key for authentication (optional, can also use LETTA_API_KEY env var)
    #[arg(short = 'k', long, env = "LETTA_API_KEY")]
    pub api_key: Option<String>,

    /// Enable verbose output
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
}

/// Available commands.
#[derive(Parser, Debug)]
pub enum Command {
    /// Agent operations
    #[command(subcommand)]
    Agent(commands::agent::AgentCommand),
    /// Message operations
    #[command(subcommand)]
    Message(commands::message::MessageCommand),
    /// Memory operations
    #[command(subcommand)]
    Memory(commands::memory::MemoryCommand),
    /// Tool operations
    #[command(subcommand)]
    Tools(commands::tools::ToolsCommand),
    /// Source operations
    #[command(subcommand)]
    Sources(commands::sources::SourcesCommand),
    /// Health check
    Health,
}

/// Run the CLI application
pub async fn run() -> miette::Result<()> {
    // Install miette's fancy error handler for better diagnostics
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(3)
                .tab_width(4)
                .build(),
        )
    }))?;

    // Install panic hook to get nice error reports on panics
    miette::set_panic_hook();

    let args = Args::parse();

    if args.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    // Build the client configuration
    let mut config = crate::ClientConfig::new(&args.base_url)?;

    if let Some(api_key) = args.api_key {
        config = config.auth(crate::auth::AuthConfig::bearer(api_key));
    }

    let client = crate::LettaClient::new(config)?;

    // Execute the command
    match args.command {
        Command::Agent(agent_cmd) => commands::agent::handle(agent_cmd, &client).await?,
        Command::Message(message_cmd) => commands::message::handle(message_cmd, &client).await?,
        Command::Memory(memory_cmd) => commands::memory::handle(memory_cmd, &client).await?,
        Command::Tools(tools_cmd) => commands::tools::handle(tools_cmd, &client).await?,
        Command::Sources(sources_cmd) => commands::sources::handle(sources_cmd, &client).await?,
        Command::Health => {
            println!("Checking health...");
            commands::check_health(&client).await?;
        }
    }

    Ok(())
}
