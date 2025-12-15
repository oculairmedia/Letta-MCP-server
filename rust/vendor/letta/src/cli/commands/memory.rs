//! Memory command implementations.

use crate::types::common::LettaId;
use crate::types::memory::{
    ArchivalMemoryQueryParams, CreateArchivalMemoryRequest, UpdateMemoryBlockRequest,
};
use crate::LettaClient;
use clap::Parser;
use miette::{Context, IntoDiagnostic};
use std::str::FromStr;

/// Memory-related commands.
#[derive(Parser, Debug)]
pub enum MemoryCommand {
    /// View agent's memory
    View {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Edit core memory
    Edit {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Memory block to edit (human or persona)
        #[arg(short = 'b', long)]
        block: String,
        /// New value for the memory block
        #[arg(short = 'v', long)]
        value: String,
    },
    /// Search archival memory
    Archival {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Query text to search for
        #[arg(short = 'q', long)]
        query: String,
        /// Maximum number of results
        #[arg(short = 'l', long, default_value = "10")]
        limit: u32,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Add to archival memory
    Add {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Text to add to archival memory
        text: String,
    },
}

/// Handle memory commands.
pub async fn handle(cmd: MemoryCommand, client: &crate::LettaClient) -> miette::Result<()> {
    match cmd {
        MemoryCommand::View { agent_id, output } => view_memory(client, &agent_id, &output).await,
        MemoryCommand::Edit {
            agent_id,
            block,
            value,
        } => edit_memory_block(client, &agent_id, &block, &value, "summary").await,
        MemoryCommand::Archival {
            agent_id,
            query,
            limit,
            output,
        } => list_archival_memory(client, &agent_id, Some(query), limit, &output).await,
        MemoryCommand::Add { agent_id, text } => {
            add_archival_memory(client, &agent_id, &text, "summary").await
        }
    }
}

async fn view_memory(client: &LettaClient, agent_id: &str, output: &str) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    match client.memory().get_core_memory(&agent_id).await {
        Ok(memory) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&memory).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&memory).into_diagnostic()?
                );
            }
            _ => {
                println!("Core Memory for Agent:\n");
                for block in &memory.blocks {
                    println!("Block: {} ({})", block.label, block.limit.unwrap_or(0));
                    println!("{}", "-".repeat(50));
                    println!("{}\n", block.value);
                }
            }
        },
        Err(e) => return Err(e).wrap_err("Failed to retrieve memory")?,
    }

    Ok(())
}

async fn edit_memory_block(
    client: &LettaClient,
    agent_id: &str,
    block_label: &str,
    new_value: &str,
    output: &str,
) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    let request = UpdateMemoryBlockRequest {
        value: Some(new_value.to_string()),
        ..Default::default()
    };

    match client
        .memory()
        .update_core_memory_block(&agent_id, block_label, request)
        .await
    {
        Ok(block) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&block).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&block).into_diagnostic()?
                );
            }
            _ => {
                println!("Memory block updated successfully!");
                println!("\nUpdated Block:");
                println!("  Label: {}", block.label);
                println!("  Value: {}", block.value);
                if let Some(limit) = block.limit {
                    println!("  Limit: {} characters", limit);
                }
            }
        },
        Err(e) => return Err(e).wrap_err("Failed to update memory block")?,
    }

    Ok(())
}

async fn list_archival_memory(
    client: &LettaClient,
    agent_id: &str,
    query: Option<String>,
    limit: u32,
    output: &str,
) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    let params = ArchivalMemoryQueryParams {
        search: query,
        limit: Some(limit),
        ..Default::default()
    };

    match client
        .memory()
        .list_archival_memory(&agent_id, Some(params))
        .await
    {
        Ok(passages) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&passages).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&passages).into_diagnostic()?
                );
            }
            _ => {
                if passages.is_empty() {
                    println!("No archival memory found.");
                } else {
                    println!("Found {} archival memory passages:\n", passages.len());
                    for (i, passage) in passages.iter().enumerate() {
                        println!(
                            "{}. [{}]",
                            i + 1,
                            passage
                                .created_at
                                .as_ref()
                                .map(|t| t.to_string())
                                .unwrap_or_else(|| "unknown".to_string())
                        );
                        println!("   {}", passage.text);
                        if let Some(embed_vec) = &passage.embedding {
                            println!("   Embedding: {} dimensions", embed_vec.len());
                        }
                        println!();
                    }
                }
            }
        },
        Err(e) => return Err(e).wrap_err("Failed to list archival memory")?,
    }

    Ok(())
}

async fn add_archival_memory(
    client: &LettaClient,
    agent_id: &str,
    text: &str,
    output: &str,
) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    let request = CreateArchivalMemoryRequest {
        text: text.to_string(),
    };

    match client
        .memory()
        .create_archival_memory(&agent_id, request)
        .await
    {
        Ok(passages) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&passages).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&passages).into_diagnostic()?
                );
            }
            _ => {
                println!("Archival memory added successfully!");
                if let Some(passage) = passages.first() {
                    println!("\nAdded Passage:");
                    println!("  ID: {}", passage.id);
                    println!("  Text: {}", passage.text);
                    println!(
                        "  Created: {}",
                        passage
                            .created_at
                            .as_ref()
                            .map(|t| t.to_string())
                            .unwrap_or_else(|| "unknown".to_string())
                    );
                }
            }
        },
        Err(e) => return Err(e).wrap_err("Failed to add archival memory")?,
    }

    Ok(())
}
