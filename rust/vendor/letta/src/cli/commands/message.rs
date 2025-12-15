//! Message command implementations.

use crate::types::common::LettaId;
use crate::types::message::{
    CreateMessagesRequest, LettaMessageUnion, ListMessagesRequest, MessageCreate, MessageRole,
};
use crate::LettaClient;
use clap::Parser;
use miette::{Context, IntoDiagnostic};
use std::str::FromStr;

/// Message-related commands.
#[derive(Parser, Debug)]
pub enum MessageCommand {
    /// Send a message to an agent
    Send {
        /// Agent ID to send message to
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Message text to send
        message: String,
        /// Message role (user, system, assistant)
        #[arg(short = 'r', long, default_value = "user")]
        role: String,
        /// Maximum steps for processing
        #[arg(short = 's', long)]
        max_steps: Option<i32>,
        /// Disable streaming (by default, messages are streamed)
        #[arg(long)]
        no_stream: bool,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// List messages for an agent
    List {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Maximum number of messages to return
        #[arg(short = 'l', long, default_value = "20")]
        limit: i32,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
}

/// Handle message commands.
pub async fn handle(cmd: MessageCommand, client: &crate::LettaClient) -> miette::Result<()> {
    match cmd {
        MessageCommand::Send {
            agent_id,
            message,
            role,
            max_steps,
            no_stream,
            output,
        } => {
            send_message(
                client, &agent_id, &message, &role, max_steps,
                !no_stream, // invert no_stream to get stream
                &output,
            )
            .await
        }
        MessageCommand::List {
            agent_id,
            limit,
            output,
        } => list_messages(client, &agent_id, limit, &output).await,
    }
}

async fn send_message(
    client: &LettaClient,
    agent_id: &str,
    message: &str,
    role: &str,
    max_steps: Option<i32>,
    stream: bool,
    output: &str,
) -> miette::Result<()> {
    // Parse message role
    let role = match role.to_lowercase().as_str() {
        "user" => MessageRole::User,
        "system" => MessageRole::System,
        "assistant" => MessageRole::Assistant,
        _ => {
            eprintln!("Invalid role: {}. Using 'user'.", role);
            MessageRole::User
        }
    };

    // Create the message
    let message = MessageCreate {
        role,
        content: message.into(),
        ..Default::default()
    };

    // Create the request
    let mut request = CreateMessagesRequest {
        messages: vec![message],
        ..Default::default()
    };

    if let Some(steps) = max_steps {
        request.max_steps = Some(steps);
    }

    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    if stream {
        // Handle streaming response
        use futures::StreamExt;

        if output != "json" {
            println!("Streaming response from agent...\n");
        }

        // Use token streaming for better UX when not in JSON mode
        let stream_tokens = output != "json";
        let mut stream = client
            .messages()
            .create_stream(&agent_id, request, stream_tokens)
            .await?;

        while let Some(event) = stream.next().await {
            match event {
                Ok(crate::StreamingEvent::Message(msg)) => {
                    match output {
                        "json" => {
                            println!("{}", serde_json::to_string(&msg).into_diagnostic()?);
                        }
                        _ => {
                            // Pretty print the message based on type
                            match msg {
                                LettaMessageUnion::UserMessage(m) => {
                                    println!("User: {}", m.content);
                                }
                                LettaMessageUnion::AssistantMessage(m) => {
                                    println!("Assistant: {}", m.content);
                                }
                                LettaMessageUnion::SystemMessage(m) => {
                                    println!("System: {}", m.content);
                                }
                                LettaMessageUnion::ToolCallMessage(m) => {
                                    println!(
                                        "Tool Call: {} - {}",
                                        m.tool_call.name, m.tool_call.arguments
                                    );
                                }
                                LettaMessageUnion::ToolReturnMessage(m) => {
                                    println!("Tool Result: {}", m.tool_return);
                                }
                                _ => {
                                    // For other message types, show the JSON
                                    println!(
                                        "{}",
                                        serde_json::to_string_pretty(&msg).into_diagnostic()?
                                    );
                                }
                            }
                        }
                    }
                }
                Ok(crate::StreamingEvent::StopReason(reason)) => {
                    if output == "json" {
                        println!("{}", serde_json::to_string(&reason).into_diagnostic()?);
                    }
                }
                Ok(crate::StreamingEvent::Usage(usage)) => {
                    if output == "json" {
                        println!("{}", serde_json::to_string(&usage).into_diagnostic()?);
                    } else if output != "summary" {
                        println!("\nUsage Statistics:");
                        if let Some(steps) = usage.step_count {
                            println!("  Steps: {}", steps);
                        }
                        if let Some(tokens) = usage.total_tokens {
                            println!("  Total Tokens: {}", tokens);
                        }
                    }
                }
                Err(e) => return Err(e).wrap_err("Error streaming messages")?,
            }
        }
    } else {
        // Handle non-streaming response
        if output != "json" {
            println!("Sending message to agent...");
        }

        match client.messages().create(&agent_id, request).await {
            Ok(response) => match output {
                "json" => {
                    println!("{}", serde_json::to_string(&response).into_diagnostic()?);
                }
                "pretty" => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&response).into_diagnostic()?
                    );
                }
                _ => {
                    println!("Message sent successfully!\n");

                    // Display the conversation
                    for msg in &response.messages {
                        match msg {
                            LettaMessageUnion::UserMessage(m) => {
                                println!("User: {}", m.content);
                            }
                            LettaMessageUnion::AssistantMessage(m) => {
                                println!("Assistant: {}", m.content);
                            }
                            LettaMessageUnion::SystemMessage(m) => {
                                println!("System: {}", m.content);
                            }
                            LettaMessageUnion::ToolCallMessage(m) => {
                                println!(
                                    "Tool Call: {} - {}",
                                    m.tool_call.name, m.tool_call.arguments
                                );
                            }
                            LettaMessageUnion::ToolReturnMessage(m) => {
                                println!("Tool Result: {}", m.tool_return);
                            }
                            _ => {}
                        }
                    }

                    // Show stop reason
                    println!("\nStop Reason: {:?}", response.stop_reason.stop_reason);

                    // Show usage if available
                    println!("\nUsage:");
                    if let Some(steps) = response.usage.step_count {
                        println!("  Steps: {}", steps);
                    }
                    if let Some(tokens) = response.usage.total_tokens {
                        println!("  Total Tokens: {}", tokens);
                    }
                }
            },
            Err(e) => return Err(e).wrap_err("Error sending messages")?,
        }
    }

    Ok(())
}

async fn list_messages(
    client: &LettaClient,
    agent_id: &str,
    limit: i32,
    output: &str,
) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    let params = ListMessagesRequest {
        limit: Some(limit),
        ..Default::default()
    };

    match client.messages().list(&agent_id, Some(params)).await {
        Ok(messages) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&messages).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&messages).into_diagnostic()?
                );
            }
            _ => {
                if messages.is_empty() {
                    println!("No messages found.");
                } else {
                    println!("Found {} messages:\n", messages.len());

                    for msg in messages {
                        match msg {
                            LettaMessageUnion::UserMessage(m) => {
                                println!("User [{}]", m.date);
                                println!("   {}\n", m.content);
                            }
                            LettaMessageUnion::AssistantMessage(m) => {
                                println!("Assistant [{}]", m.date);
                                println!("   {}\n", m.content);
                            }
                            LettaMessageUnion::SystemMessage(m) => {
                                println!("System [{}]", m.date);
                                println!("   {}\n", m.content);
                            }
                            LettaMessageUnion::ToolCallMessage(m) => {
                                println!("Tool Call [{}]", m.date);
                                println!("   Tool: {}", m.tool_call.name);
                                println!("   Args: {}\n", m.tool_call.arguments);
                            }
                            LettaMessageUnion::ToolReturnMessage(m) => {
                                println!("Tool Result [{}]", m.date);
                                println!("   {}\n", m.tool_return);
                            }
                            LettaMessageUnion::ReasoningMessage(m) => {
                                println!("Reasoning [{}]", m.date);
                                println!("   {}\n", m.reasoning);
                            }
                            LettaMessageUnion::HiddenReasoningMessage(m) => {
                                println!("Hidden Reasoning [{}]", m.date);
                                println!("   State: {:?}\n", m.state);
                            }
                        }
                    }
                }
            }
        },
        Err(e) => return Err(e).wrap_err("Failed to get messages")?,
    }

    Ok(())
}
