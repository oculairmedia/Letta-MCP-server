//! Sources command implementations.

use crate::types::agent::EmbeddingConfig;
use crate::types::common::LettaId;
use crate::types::source::{
    CreateSourceRequest, FileUploadResponse, ListFilesParams, ListPassagesParams,
};
use crate::LettaClient;
use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use std::collections::HashMap;
use std::str::FromStr;

/// Sources-related commands.
#[derive(Parser, Debug)]
pub enum SourcesCommand {
    /// List all sources
    List {
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Create a new source
    Create {
        /// Source name
        #[arg(short = 'n', long)]
        name: String,
        /// Source description
        #[arg(short = 'd', long)]
        description: Option<String>,
        /// Instructions for using the source
        #[arg(short = 'i', long)]
        instructions: Option<String>,
        /// Embedding model (defaults to letta/letta-free)
        #[arg(short = 'e', long, default_value = "letta/letta-free")]
        embedding_model: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Get source details
    Get {
        /// Source ID
        id: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "pretty")]
        output: String,
    },
    /// Delete a source
    Delete {
        /// Source ID
        id: String,
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
    /// File operations on a source
    #[command(subcommand)]
    Files(FilesCommand),
    /// Passage operations on a source
    #[command(subcommand)]
    Passages(PassagesCommand),
}

/// Files subcommands.
#[derive(Parser, Debug)]
pub enum FilesCommand {
    /// List files in a source
    List {
        /// Source ID
        source_id: String,
        /// Maximum number of files to return
        #[arg(short = 'l', long, default_value = "20")]
        limit: u32,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Upload a file to a source
    Upload {
        /// Source ID
        source_id: String,
        /// Path to file to upload
        #[arg(short = 'f', long)]
        file: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Get file details
    Get {
        /// Source ID
        source_id: String,
        /// File ID
        file_id: String,
        /// Include file content
        #[arg(short = 'c', long)]
        content: bool,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "pretty")]
        output: String,
    },
    /// Delete a file from a source
    Delete {
        /// Source ID
        source_id: String,
        /// File ID
        file_id: String,
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

/// Passages subcommands.
#[derive(Parser, Debug)]
pub enum PassagesCommand {
    /// List passages from a source
    List {
        /// Source ID
        source_id: String,
        /// Maximum number of passages to return
        #[arg(short = 'l', long, default_value = "20")]
        limit: u32,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
}

/// Handle sources commands.
pub async fn handle(cmd: SourcesCommand, client: &crate::LettaClient) -> miette::Result<()> {
    match cmd {
        SourcesCommand::List { output } => list_sources(client, &output).await,
        SourcesCommand::Create {
            name,
            description,
            instructions,
            embedding_model,
            output,
        } => {
            create_source(
                client,
                &name,
                &embedding_model,
                description,
                instructions,
                &output,
            )
            .await
        }
        SourcesCommand::Get { id, output } => get_source(client, &id, &output).await,
        SourcesCommand::Delete { id, yes } => delete_source(client, &id, yes).await,
        SourcesCommand::Files(files_cmd) => handle_files(files_cmd, client).await,
        SourcesCommand::Passages(passages_cmd) => handle_passages(passages_cmd, client).await,
    }
}

async fn list_sources(client: &LettaClient, output: &str) -> miette::Result<()> {
    println!("Listing sources...");

    let sources = client.sources().list().await?;

    match output {
        "json" => {
            println!("{}", serde_json::to_string(&sources).into_diagnostic()?);
        }
        "pretty" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&sources).into_diagnostic()?
            );
        }
        _ => {
            if sources.is_empty() {
                println!("No sources found.");
            } else {
                println!("Found {} sources:\n", sources.len());
                for source in sources {
                    println!(
                        "ID: {}",
                        source
                            .id
                            .as_ref()
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "N/A".to_string())
                    );
                    println!("Name: {}", source.name);
                    if let Some(desc) = &source.description {
                        println!("Description: {}", desc);
                    }
                    if let Some(created) = &source.created_at {
                        println!("Created: {}", created);
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}

async fn create_source(
    client: &LettaClient,
    name: &str,
    embedding_model: &str,
    description: Option<String>,
    instructions: Option<String>,
    output: &str,
) -> miette::Result<()> {
    if output != "json" {
        println!("Creating source '{}'...", name);
    }

    let request = CreateSourceRequest {
        name: name.to_string(),
        embedding_config: Some(EmbeddingConfig {
            embedding_model: Some(embedding_model.to_string()),
            embedding_endpoint_type: None,
            embedding_endpoint: None,
            embedding_dim: None,
            embedding_chunk_size: None,
            handle: None,
            azure_config: None,
            extra: HashMap::new(),
        }),
        description,
        instructions,
        embedding: None,
        embedding_chunk_size: None,
        metadata: None,
    };

    match client.sources().create(request).await {
        Ok(source) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&source).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&source).into_diagnostic()?
                );
            }
            _ => {
                println!("Source created successfully!");
                println!("\nSource Details:");
                println!(
                    "  ID: {}",
                    source
                        .id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                );
                println!("  Name: {}", source.name);
                if let Some(desc) = &source.description {
                    println!("  Description: {}", desc);
                }
                println!(
                    "\nUse 'letta sources get {}' to see full details.",
                    source
                        .id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                );
            }
        },
        Err(e) => {
            return Err(e).wrap_err("Failed to create source")?;
        }
    }

    Ok(())
}

async fn get_source(client: &LettaClient, id: &str, output: &str) -> miette::Result<()> {
    let source_id = LettaId::from_str(id).into_diagnostic()?;

    match client.sources().get(&source_id).await {
        Ok(source) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&source).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&source).into_diagnostic()?
                );
            }
            _ => {
                println!("Source Details:");
                println!(
                    "  ID: {}",
                    source
                        .id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                );
                println!("  Name: {}", source.name);
                if let Some(desc) = &source.description {
                    println!("  Description: {}", desc);
                }
                if let Some(instructions) = &source.instructions {
                    println!("  Instructions: {}", instructions);
                }
                if let Some(created) = &source.created_at {
                    println!("  Created: {}", created);
                }
                if let Some(updated) = &source.updated_at {
                    println!("  Updated: {}", updated);
                }
                let embedding_config = &source.embedding_config;
                if let Some(ref model) = embedding_config.embedding_model {
                    println!("  Embedding Model: {}", model);
                }
            }
        },
        Err(e) => return Err(e).wrap_err("Failed to get source")?,
    }

    Ok(())
}

async fn delete_source(client: &LettaClient, id: &str, yes: bool) -> miette::Result<()> {
    if !yes {
        print!("Are you sure you want to delete source {}? (y/N) ", id);
        use std::io::Write;
        std::io::stdout().flush().into_diagnostic()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).into_diagnostic()?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!("Deleting source {}...", id);
    let source_id = LettaId::from_str(id).into_diagnostic()?;

    match client.sources().delete(&source_id).await {
        Ok(_) => {
            println!("Source deleted successfully.");
        }
        Err(e) => return Err(e).wrap_err("Failed to delete source")?,
    }

    Ok(())
}

/// Handle file subcommands.
async fn handle_files(cmd: FilesCommand, client: &LettaClient) -> miette::Result<()> {
    match cmd {
        FilesCommand::List {
            source_id,
            limit,
            output,
        } => list_files(client, &source_id, limit, &output).await,
        FilesCommand::Upload {
            source_id,
            file,
            output,
        } => upload_file(client, &source_id, &file, &output).await,
        FilesCommand::Get {
            source_id,
            file_id,
            content: _,
            output,
        } => get_file(client, &source_id, &file_id, &output).await,
        FilesCommand::Delete {
            source_id,
            file_id,
            yes,
        } => delete_file(client, &source_id, &file_id, yes).await,
    }
}

async fn list_files(
    client: &LettaClient,
    source_id: &str,
    limit: u32,
    output: &str,
) -> miette::Result<()> {
    println!("Listing files in source...");
    let source_id = LettaId::from_str(source_id).into_diagnostic()?;

    let params = ListFilesParams {
        limit: Some(limit as i32),
        ..Default::default()
    };

    let files = client
        .sources()
        .list_files(&source_id, Some(params))
        .await?;

    match output {
        "json" => {
            println!("{}", serde_json::to_string(&files).into_diagnostic()?);
        }
        "pretty" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&files).into_diagnostic()?
            );
        }
        _ => {
            if files.is_empty() {
                println!("No files found in source.");
            } else {
                println!("Found {} files:\n", files.len());
                for file in files {
                    println!(
                        "ID: {}",
                        file.id
                            .as_ref()
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "N/A".to_string())
                    );
                    println!(
                        "Name: {}",
                        file.file_name.as_ref().unwrap_or(&"N/A".to_string())
                    );
                    if let Some(status) = &file.processing_status {
                        println!("Status: {:?}", status);
                    }
                    if let Some(size) = file.file_size {
                        println!("Size: {} bytes", size);
                    }
                    if let Some(mime) = &file.file_type {
                        println!("Type: {}", mime);
                    }
                    if let Some(created) = &file.created_at {
                        println!("Created: {}", created);
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}

async fn upload_file(
    client: &LettaClient,
    source_id: &str,
    file_path: &str,
    output: &str,
) -> miette::Result<()> {
    // Read file
    let file_data = std::fs::read(file_path)
        .into_diagnostic()
        .wrap_err(format!("Failed to read file: {}", file_path))?;

    let filename = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| miette!("Invalid filename"))?;

    if output != "json" {
        println!("Uploading file '{}' to source...", filename);
    }

    let source_id = LettaId::from_str(source_id).into_diagnostic()?;

    // Detect content type from file extension
    let content_type = match std::path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("pdf") => Some("application/pdf".to_string()),
        Some("txt") => Some("text/plain".to_string()),
        Some("md") => Some("text/markdown".to_string()),
        Some("json") => Some("application/json".to_string()),
        Some("csv") => Some("text/csv".to_string()),
        Some("doc") | Some("docx") => Some("application/msword".to_string()),
        _ => None,
    };

    use bytes::Bytes;
    let file_bytes = Bytes::from(file_data);

    match client
        .sources()
        .upload_file(&source_id, filename.to_string(), file_bytes, content_type)
        .await
    {
        Ok(file) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&file).into_diagnostic()?);
            }
            "pretty" => {
                println!("{}", serde_json::to_string_pretty(&file).into_diagnostic()?);
            }
            _ => {
                println!("File uploaded successfully!");
                println!("\nFile Details:");
                match &file {
                    FileUploadResponse::Job(job) => {
                        println!("  Job ID: {}", job.id);
                        println!("  Status: {}", job.status);
                        if let Some(metadata) = &job.metadata {
                            println!("  Filename: {}", metadata.filename);
                        }
                    }
                    FileUploadResponse::FileMetadata(metadata) => {
                        println!(
                            "  ID: {}",
                            metadata
                                .id
                                .as_ref()
                                .map(|id| id.to_string())
                                .unwrap_or_else(|| "N/A".to_string())
                        );
                        println!(
                            "  Name: {}",
                            metadata.file_name.as_ref().unwrap_or(&"N/A".to_string())
                        );
                        if let Some(status) = &metadata.processing_status {
                            println!("  Status: {:?}", status);
                        }
                        if let Some(size) = metadata.file_size {
                            println!("  Size: {} bytes", size);
                        }
                    }
                }
                println!("\nProcessing may take a moment. Check status with:");
                println!("  letta sources files get {} <file-id>", source_id);
            }
        },
        Err(e) => {
            return Err(e).wrap_err("Failed to upload file")?;
        }
    }

    Ok(())
}

async fn get_file(
    client: &LettaClient,
    source_id: &str,
    file_id: &str,
    output: &str,
) -> miette::Result<()> {
    let source_id = LettaId::from_str(source_id).into_diagnostic()?;
    let file_id = LettaId::from_str(file_id).into_diagnostic()?;

    match client.sources().get_file(&source_id, &file_id, None).await {
        Ok(file) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&file).into_diagnostic()?);
            }
            "pretty" => {
                println!("{}", serde_json::to_string_pretty(&file).into_diagnostic()?);
            }
            _ => {
                println!("File Details:");
                println!(
                    "  ID: {}",
                    file.id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                );
                println!(
                    "  Name: {}",
                    file.file_name.as_ref().unwrap_or(&"N/A".to_string())
                );
                if let Some(status) = &file.processing_status {
                    println!("  Status: {:?}", status);
                }
                if let Some(size) = file.file_size {
                    println!("  Size: {} bytes", size);
                }
                if let Some(mime) = &file.file_type {
                    println!("  Type: {}", mime);
                }
                if let Some(created) = &file.created_at {
                    println!("  Created: {}", created);
                }
                if let Some(updated) = &file.updated_at {
                    println!("  Updated: {}", updated);
                }
            }
        },
        Err(e) => return Err(e).wrap_err("Failed to get file")?,
    }

    Ok(())
}

async fn delete_file(
    client: &LettaClient,
    source_id: &str,
    file_id: &str,
    yes: bool,
) -> miette::Result<()> {
    if !yes {
        print!("Are you sure you want to delete file {}? (y/N) ", file_id);
        use std::io::Write;
        std::io::stdout().flush().into_diagnostic()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).into_diagnostic()?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!("Deleting file {}...", file_id);
    let source_id = LettaId::from_str(source_id).into_diagnostic()?;
    let file_id = LettaId::from_str(file_id).into_diagnostic()?;

    match client.sources().delete_file(&source_id, &file_id).await {
        Ok(_) => {
            println!("File deleted successfully.");
        }
        Err(e) => return Err(e).wrap_err("Failed to delete file")?,
    }

    Ok(())
}

/// Handle passage subcommands.
async fn handle_passages(cmd: PassagesCommand, client: &LettaClient) -> miette::Result<()> {
    match cmd {
        PassagesCommand::List {
            source_id,
            limit,
            output,
        } => list_passages(client, &source_id, limit, &output).await,
    }
}

async fn list_passages(
    client: &LettaClient,
    source_id: &str,
    limit: u32,
    output: &str,
) -> miette::Result<()> {
    println!("Listing passages in source...");
    let source_id = LettaId::from_str(source_id).into_diagnostic()?;

    let params = ListPassagesParams {
        limit: Some(limit as i32),
        ..Default::default()
    };

    let passages = client
        .sources()
        .list_passages(&source_id, Some(params))
        .await?;

    match output {
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
                println!("No passages found in source.");
            } else {
                println!("Found {} passages:\n", passages.len());
                for (i, passage) in passages.iter().enumerate() {
                    println!("{}. [ID: {}]", i + 1, passage.id);
                    println!("   Text: {}", passage.text);
                    if let Some(file_id) = &passage.file_id {
                        println!("   File ID: {}", file_id);
                    }
                    if let Some(created) = &passage.created_at {
                        println!("   Created: {}", created);
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}
