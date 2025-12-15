//! Tools command implementations.

use crate::types::common::LettaId;
use crate::types::tool::{CreateToolRequest, ListToolsParams, SourceType};
use crate::LettaClient;
use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use std::str::FromStr;

/// Tools-related commands.
#[derive(Parser, Debug)]
pub enum ToolsCommand {
    /// List all tools
    List {
        /// Maximum number of tools to return
        #[arg(short = 'l', long, default_value = "20")]
        limit: u32,
        /// Filter by tags
        #[arg(short = 't', long)]
        tags: Vec<String>,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Create a new tool from Python file
    Create {
        /// Path to Python file containing the tool function
        #[arg(short = 'p', long)]
        python: String,
        /// Path to JSON file containing the function schema
        #[arg(short = 's', long)]
        schema: String,
        /// Path to JSON file containing args schema (optional, will extract from schema if not provided)
        #[arg(short = 'a', long)]
        args_schema: Option<String>,
        /// Tool description (optional, will use from schema if not provided)
        #[arg(short = 'd', long)]
        description: Option<String>,
        /// Tags for the tool
        #[arg(short = 't', long)]
        tags: Vec<String>,
        /// Return character limit
        #[arg(short = 'r', long)]
        return_limit: Option<u32>,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Get tool details
    Get {
        /// Tool ID
        id: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "pretty")]
        output: String,
    },
    /// Delete a tool
    Delete {
        /// Tool ID
        id: String,
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

/// Handle tools commands.
pub async fn handle(cmd: ToolsCommand, client: &crate::LettaClient) -> miette::Result<()> {
    match cmd {
        ToolsCommand::List {
            limit,
            tags,
            output,
        } => list_tools(client, limit, tags, &output).await,
        ToolsCommand::Create {
            python,
            schema,
            args_schema,
            description,
            tags,
            return_limit,
            output,
        } => {
            create_tool(
                client,
                &python,
                &schema,
                args_schema.as_deref(),
                description,
                tags,
                return_limit,
                &output,
            )
            .await
        }
        ToolsCommand::Get { id, output } => get_tool(client, &id, &output).await,
        ToolsCommand::Delete { id, yes } => delete_tool(client, &id, yes).await,
    }
}

async fn list_tools(
    client: &LettaClient,
    limit: u32,
    _tags: Vec<String>,
    output: &str,
) -> miette::Result<()> {
    println!("Listing tools...");

    let mut params = ListToolsParams::default();
    params.limit = Some(limit);
    // Note: tags filtering would need to be done client-side as API doesn't support it

    let tools = client.tools().list(Some(params)).await?;

    match output {
        "json" => {
            println!("{}", serde_json::to_string(&tools).into_diagnostic()?);
        }
        "pretty" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&tools).into_diagnostic()?
            );
        }
        _ => {
            if tools.is_empty() {
                println!("No tools found.");
            } else {
                println!("Found {} tools:\n", tools.len());
                for tool in tools {
                    println!(
                        "ID: {}",
                        tool.id
                            .as_ref()
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "N/A".to_string())
                    );
                    println!("Name: {}", tool.name);
                    if let Some(desc) = &tool.description {
                        println!("Description: {}", desc);
                    }
                    if let Some(tool_type) = &tool.tool_type {
                        println!("Type: {:?}", tool_type);
                    }
                    if let Some(tags) = &tool.tags {
                        if !tags.is_empty() {
                            println!("Tags: {:?}", tags);
                        }
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}

/// Validate that a JSON schema matches Letta's expected tool schema format
fn validate_tool_schema(schema: &serde_json::Value) -> miette::Result<()> {
    // Check required top-level fields
    if !schema.is_object() {
        return Err(miette!("Schema must be a JSON object"));
    }

    let obj = schema.as_object().unwrap();

    // Required field: name
    if !obj.contains_key("name") {
        return Err(miette!(
            r#"Schema missing required field 'name'. Example:
{{
  "name": "my_tool",
  "description": "Tool description",
  "parameters": {{ ... }}
}}"#
        ));
    }

    if !obj["name"].is_string() {
        return Err(miette!("Field 'name' must be a string"));
    }

    // Required field: parameters
    if !obj.contains_key("parameters") {
        return Err(miette!(
            r#"Schema missing required field 'parameters'. Example:
{{
  "name": "my_tool",
  "description": "...",
  "parameters": {{
    "type": "object",
    "properties": {{ ... }},
    "required": [ ... ]
  }}
}}"#
        ));
    }

    let params = &obj["parameters"];
    if !params.is_object() {
        return Err(miette!("Field 'parameters' must be a JSON object"));
    }

    // Validate parameters structure
    let params_obj = params.as_object().unwrap();

    // Check for 'type' field in parameters
    if !params_obj.contains_key("type") {
        return Err(miette!(
            r#"Parameters missing 'type' field. Should be:
"parameters": {{
  "type": "object",
  "properties": {{ ... }}
}}"#
        ));
    }

    if params_obj["type"] != "object" {
        return Err(miette!(
            r#"Parameters 'type' should be "object", got: {}"#,
            params_obj["type"]
        ));
    }

    // Check for 'properties' field
    if !params_obj.contains_key("properties") {
        return Err(miette!(
            r#"Parameters missing 'properties' field. Example:
"parameters": {{
  "type": "object",
  "properties": {{
    "arg1": {{ "type": "string", "description": "..." }},
    "arg2": {{ "type": "integer", "description": "..." }}
  }}
}}"#
        ));
    }

    let properties = &params_obj["properties"];
    if !properties.is_object() {
        return Err(miette!("Parameters 'properties' must be a JSON object"));
    }

    // Validate each property
    let props_obj = properties.as_object().unwrap();
    for (prop_name, prop_value) in props_obj {
        if !prop_value.is_object() {
            return Err(miette!(
                "Property '{}' must be an object with 'type' and 'description' fields",
                prop_name
            ));
        }

        let prop_obj = prop_value.as_object().unwrap();
        if !prop_obj.contains_key("type") {
            return Err(miette!(
                r#"Property '{}' missing 'type' field. Example:
"{}" : {{
  "type": "string",
  "description": "Description of {}"
}}"#,
                prop_name,
                prop_name,
                prop_name
            ));
        }

        if !prop_obj.contains_key("description") {
            return Err(miette!(
                r#"Property '{}' missing 'description' field. Properties should have:
{{
  "type": "...",
  "description": "What this parameter does"
}}"#,
                prop_name
            ));
        }
    }

    // Warn about optional but recommended fields
    if !obj.contains_key("description") {
        eprintln!("Warning: Schema missing 'description' field. It's recommended to include a tool description.");
    }

    Ok(())
}

/// Validate that Python source code has proper docstring formatting for Letta
fn validate_python_docstring(source_code: &str, function_name: &str) -> miette::Result<()> {
    // Quick and dirty docstring validation

    // Check if there's a docstring at all (look for triple quotes after function def)
    let has_docstring = source_code.contains(r#"""""#) || source_code.contains("'''");
    if !has_docstring {
        return Err(miette!(
            r#"Function '{}' missing docstring. Letta requires a docstring with Args and Returns sections:

def {}(...):
    """
    Brief description.
    
    Args:
        param1: Description
        param2: Description
    
    Returns:
        Description of return value
    """
    ..."#,
            function_name,
            function_name
        ));
    }

    // Extract docstring content (quick and dirty - find content between first set of triple quotes)
    let docstring_content = if let Some(start) = source_code.find(r#"""""#) {
        if let Some(end) = source_code[start + 3..].find(r#"""""#) {
            &source_code[start + 3..start + 3 + end]
        } else {
            return Err(miette!(
                "Malformed docstring - missing closing triple quotes"
            ));
        }
    } else if let Some(start) = source_code.find("'''") {
        if let Some(end) = source_code[start + 3..].find("'''") {
            &source_code[start + 3..start + 3 + end]
        } else {
            return Err(miette!(
                "Malformed docstring - missing closing triple quotes"
            ));
        }
    } else {
        return Err(miette!("Could not find docstring"));
    };

    // Check for Args section
    let has_args_section = docstring_content.contains("Args:")
        || docstring_content.contains("Arguments:")
        || docstring_content.contains("Parameters:");

    if !has_args_section {
        return Err(miette!(
            r#"Docstring missing 'Args:' section. Letta requires parameter documentation:

    Args:
        param_name: Description of parameter
        another_param: Description

Without this, you'll get: "Parameter 'X' in function '{}' lacks a description in the docstring""#,
            function_name
        ));
    }

    // Check for Returns section (warning only)
    let has_returns_section =
        docstring_content.contains("Returns:") || docstring_content.contains("Return:");

    if !has_returns_section {
        eprintln!("Warning: Docstring missing 'Returns:' section. It's recommended to document return values.");
    }

    Ok(())
}

async fn create_tool(
    client: &LettaClient,
    python_path: &str,
    schema_path: &str,
    args_schema_path: Option<&str>,
    description: Option<String>,
    tags: Vec<String>,
    return_limit: Option<u32>,
    output: &str,
) -> miette::Result<()> {
    // Read Python source code
    let source_code = std::fs::read_to_string(python_path)
        .into_diagnostic()
        .wrap_err(format!("Failed to read Python file: {}", python_path))?;

    // Read JSON schema
    let schema_content = std::fs::read_to_string(schema_path)
        .into_diagnostic()
        .wrap_err(format!("Failed to read schema file: {}", schema_path))?;

    let json_schema: serde_json::Value = serde_json::from_str(&schema_content)
        .into_diagnostic()
        .wrap_err("Failed to parse JSON schema")?;

    // Validate the schema format
    validate_tool_schema(&json_schema)?;

    // Extract function name for validation
    let function_name = json_schema
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("unknown");

    // Validate Python docstring
    validate_python_docstring(&source_code, function_name)?;

    // Extract name from schema if present
    let tool_name = json_schema
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or_else(|| miette!("Schema must contain a 'name' field"))?
        .to_string();

    // Extract description from schema if not provided
    let tool_description = description.or_else(|| {
        json_schema
            .get("description")
            .and_then(|d| d.as_str())
            .map(String::from)
    });

    // Read args schema if provided, otherwise try to extract from main schema
    let args_json_schema = if let Some(args_path) = args_schema_path {
        let args_content = std::fs::read_to_string(args_path)
            .into_diagnostic()
            .wrap_err(format!("Failed to read args schema file: {}", args_path))?;

        Some(
            serde_json::from_str(&args_content)
                .into_diagnostic()
                .wrap_err("Failed to parse args JSON schema")?,
        )
    } else {
        // Try to extract from main schema
        json_schema.get("parameters").cloned()
    };

    // Build the request
    let request = CreateToolRequest {
        source_code,
        source_type: Some(SourceType::Python),
        json_schema: Some(json_schema),
        args_json_schema,
        description: tool_description,
        tags: if tags.is_empty() { None } else { Some(tags) },
        return_char_limit: return_limit,
        ..Default::default()
    };

    if output != "json" {
        println!("Creating tool '{}'...", tool_name);
    }

    match client.tools().create(request).await {
        Ok(tool) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&tool).into_diagnostic()?);
            }
            "pretty" => {
                println!("{}", serde_json::to_string_pretty(&tool).into_diagnostic()?);
            }
            _ => {
                println!("Tool created successfully!");
                println!("\nTool Details:");
                println!(
                    "  ID: {}",
                    tool.id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                );
                println!("  Name: {}", tool.name);
                if let Some(desc) = &tool.description {
                    println!("  Description: {}", desc);
                }
                if let Some(tool_type) = &tool.tool_type {
                    println!("  Type: {:?}", tool_type);
                }
                println!(
                    "\nUse 'letta tools get {}' to see full details.",
                    tool.id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| tool.name.clone())
                );
            }
        },
        Err(e) => {
            return Err(e).wrap_err("Failed to create tool");
        }
    }

    Ok(())
}

async fn get_tool(client: &LettaClient, id: &str, output: &str) -> miette::Result<()> {
    let tool_id = LettaId::from_str(id).into_diagnostic()?;

    match client.tools().get(&tool_id).await {
        Ok(tool) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&tool).into_diagnostic()?);
            }
            "pretty" => {
                println!("{}", serde_json::to_string_pretty(&tool).into_diagnostic()?);
            }
            _ => {
                println!("Tool Details:");
                println!(
                    "  ID: {}",
                    tool.id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                );
                println!("  Name: {}", tool.name);
                if let Some(desc) = &tool.description {
                    println!("  Description: {}", desc);
                }
                if let Some(tool_type) = &tool.tool_type {
                    println!("  Type: {:?}", tool_type);
                }
                if let Some(tags) = &tool.tags {
                    if !tags.is_empty() {
                        println!("  Tags: {:?}", tags);
                    }
                }
                if let Some(limit) = tool.return_char_limit {
                    println!("  Return Limit: {} characters", limit);
                }

                // Show source code
                if let Some(ref source_code) = tool.source_code {
                    if !source_code.is_empty() {
                        println!("\nSource Code:");
                        println!("{}", "-".repeat(50));
                        println!("{}", source_code);
                        println!("{}", "-".repeat(50));
                    }
                }

                // Show JSON schema if available
                if let Some(schema) = &tool.json_schema {
                    println!("\nJSON Schema:");
                    println!(
                        "{}",
                        serde_json::to_string_pretty(schema).into_diagnostic()?
                    );
                }
            }
        },
        Err(e) => return Err(e).wrap_err("Failed to get tool")?,
    }

    Ok(())
}

async fn delete_tool(client: &LettaClient, id: &str, yes: bool) -> miette::Result<()> {
    if !yes {
        print!("Are you sure you want to delete tool {}? (y/N) ", id);
        use std::io::Write;
        std::io::stdout().flush().into_diagnostic()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).into_diagnostic()?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!("Deleting tool {}...", id);
    let tool_id = LettaId::from_str(id).into_diagnostic()?;

    match client.tools().delete(&tool_id).await {
        Ok(_) => {
            println!("Tool deleted successfully.");
        }
        Err(e) => return Err(e).wrap_err("Failed to delete tool")?,
    }

    Ok(())
}
