//! Agent-related types.

use crate::types::common::{LettaId, Metadata, Timestamp};
use crate::types::memory::Block;
use bon::Builder;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::collections::HashMap;

/// Environment variable for agent tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEnvironmentVariable {
    /// The ID of the user that created this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// The ID of the user that last updated this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// When the object was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the object was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// The unique identifier of the agent environment variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LettaId>,
    /// The name of the environment variable.
    pub key: String,
    /// The value of the environment variable.
    pub value: String,
    /// An optional description of the environment variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the agent this environment variable belongs to.
    pub agent_id: LettaId,
}

/// Agent type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, SmartDefault)]
pub enum AgentType {
    /// Standard memgpt-style agent.
    #[default]
    #[serde(rename = "memgpt_agent")]
    MemGPT,
    /// Version 2 memgpt agent.
    #[serde(rename = "memgpt_v2_agent")]
    MemGPTv2,
    /// React agent.
    #[serde(rename = "react_agent")]
    React,
    /// Workflow agent.
    #[serde(rename = "workflow_agent")]
    Workflow,
    /// Split-thread agent
    #[serde(rename = "split_thread_agent")]
    SplitThread,
    /// Sleeptime agent
    #[serde(rename = "sleeptime_agent")]
    Sleeptime,
    /// Voice Conversation agent
    #[serde(rename = "voice_convo_agent")]
    VoiceConvo,
    /// Voice sleeptime agent
    #[serde(rename = "voice_sleeptime_agent")]
    VoiceSleeptime,
    /// Other agent types.
    #[serde(other)]
    Other,
}

/// LLM configuration for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// Model name/identifier.
    pub model: String,
    /// Model endpoint type.
    pub model_endpoint_type: ModelEndpointType,
    /// Model endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_endpoint: Option<String>,
    /// Context window size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<u32>,
    /// Provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// Provider category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_category: Option<String>,
    /// Model wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_wrapper: Option<String>,
    /// Whether to include inner thoughts in function keyword arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_inner_thoughts_in_kwargs: Option<bool>,
    /// Provider-specific handle or identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Max tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Enable reasoner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_reasoner: Option<bool>,
    /// Reasoning effort.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    /// Max reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_reasoning_tokens: Option<u32>,
    /// Additional configuration.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl LLMConfig {
    /// Create a configuration for OpenAI models.
    ///
    /// # Example
    /// ```
    /// # use letta::types::agent::LLMConfig;
    /// let config = LLMConfig::openai("gpt-4");
    /// assert_eq!(config.model, "gpt-4");
    /// ```
    pub fn openai(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            model_endpoint_type: ModelEndpointType::Openai,
            model_endpoint: None,
            context_window: Some(128000), // GPT-4 default
            provider_name: Some("openai".to_string()),
            provider_category: None,
            model_wrapper: None,
            put_inner_thoughts_in_kwargs: None,
            handle: None,
            temperature: None,
            max_tokens: None,
            enable_reasoner: None,
            reasoning_effort: None,
            max_reasoning_tokens: None,
            extra: HashMap::new(),
        }
    }

    /// Create a configuration for Anthropic models.
    ///
    /// # Example
    /// ```
    /// # use letta::types::agent::LLMConfig;
    /// let config = LLMConfig::anthropic("claude-3-sonnet-20240229");
    /// assert_eq!(config.model, "claude-3-sonnet-20240229");
    /// ```
    pub fn anthropic(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            model_endpoint_type: ModelEndpointType::Anthropic,
            model_endpoint: None,
            context_window: Some(200000), // Claude 3 default
            provider_name: Some("anthropic".to_string()),
            provider_category: None,
            model_wrapper: None,
            put_inner_thoughts_in_kwargs: None,
            handle: None,
            temperature: None,
            max_tokens: None,
            enable_reasoner: None,
            reasoning_effort: None,
            max_reasoning_tokens: None,
            extra: HashMap::new(),
        }
    }

    /// Create a configuration for local models.
    ///
    /// # Example
    /// ```
    /// # use letta::types::agent::LLMConfig;
    /// let config = LLMConfig::local("llama2", "http://localhost:8080");
    /// assert_eq!(config.model, "llama2");
    /// ```
    pub fn local(model: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            model_endpoint_type: ModelEndpointType::Ollama,
            model_endpoint: Some(endpoint.into()),
            context_window: Some(4096), // Conservative default
            provider_name: Some("ollama".to_string()),
            provider_category: None,
            model_wrapper: None,
            put_inner_thoughts_in_kwargs: None,
            handle: None,
            temperature: None,
            max_tokens: None,
            enable_reasoner: None,
            reasoning_effort: None,
            max_reasoning_tokens: None,
            extra: HashMap::new(),
        }
    }

    /// Set the context window size.
    pub fn with_context_window(mut self, size: u32) -> Self {
        self.context_window = Some(size);
        self
    }

    /// Set the temperature.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the max tokens.
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set the model endpoint.
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.model_endpoint = Some(endpoint.into());
        self
    }

    /// Enable reasoner with optional effort and token limits.
    pub fn with_reasoner(mut self, effort: Option<&str>, max_tokens: Option<u32>) -> Self {
        self.enable_reasoner = Some(true);
        if let Some(e) = effort {
            self.reasoning_effort = Some(e.to_string());
        }
        if let Some(t) = max_tokens {
            self.max_reasoning_tokens = Some(t);
        }
        self
    }
}

/// Available model endpoint types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelEndpointType {
    /// OpenAI
    Openai,
    /// Anthropic
    Anthropic,
    /// Cohere
    Cohere,
    /// Google AI
    GoogleAi,
    /// Google Vertex AI
    GoogleVertex,
    /// Azure AI
    Azure,
    /// Groq
    Groq,
    /// Ollama
    Ollama,
    /// Open Webui
    Webui,
    /// Open Webui (legacy api)
    #[serde(rename = "webui-legacy")]
    WebuiLegacy,
    /// LM Studio
    Lmstudio,
    /// LM Studio (legacy api)
    #[serde(rename = "lmstudio-legacy")]
    LmstudioLegacy,
    /// LM Studio (chat completions API)
    #[serde(rename = "lmstudio-chatcompletions")]
    LmstudioChatCompletions,
    /// Llamacpp
    Llamacpp,
    /// Koboldcpp
    Koboldcpp,
    /// vLLM
    Vllm,
    /// Hugging Face
    #[serde(rename = "hugging-face")]
    HuggingFace,
    /// Mistral
    Mistral,
    /// Together
    Together,
    /// Other model types.
    #[serde(other)]
    Other,
}

/// Available embedding endpoint types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingEndpointType {
    /// OpenAI
    Openai,
    /// Azure
    Azure,
    /// Cohere
    Cohere,
    /// HuggingFace
    #[serde(rename = "hugging-face")]
    HuggingFace,
    /// Ollama
    Ollama,
    /// Other embedding types
    #[serde(other)]
    Other,
}

/// Azure-specific embedding configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AzureEmbeddingConfig {
    /// Azure endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_endpoint: Option<String>,
    /// Azure version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_version: Option<String>,
    /// Azure deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_deployment: Option<String>,
}

/// Embedding configuration for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Embedding model name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<String>,
    /// Embedding endpoint type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_endpoint_type: Option<EmbeddingEndpointType>,
    /// Embedding endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_endpoint: Option<String>,
    /// Embedding dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_dim: Option<u32>,
    /// Embedding chunk size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_chunk_size: Option<u32>,
    /// Provider-specific handle or identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Azure-specific configuration (flattened into the JSON).
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub azure_config: Option<AzureEmbeddingConfig>,
    /// Additional configuration.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Agent memory configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemory {
    /// Memory blocks.
    #[serde(default)]
    pub blocks: Vec<Block>,
    /// File-based memory blocks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_blocks: Vec<Block>,
    /// Prompt template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<String>,
}

/// Tool reference for agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolReference {
    /// Tool ID reference.
    Id(String),
    /// Full tool object.
    Object(serde_json::Value),
}

/// Tool rule for controlling agent behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolRule {
    /// Continue loop after tool execution.
    #[serde(rename = "continue_loop")]
    ContinueLoop {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Exit loop after tool execution.
    #[serde(rename = "exit_loop")]
    ExitLoop {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Terminal tool rule (deprecated, use exit_loop).
    Terminal {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Max count per step constraint.
    MaxCountPerStep {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
        /// Maximum number of times this tool can be invoked in a single step.
        max_count_limit: u32,
    },
    /// Conditional tool mapping based on output.
    Conditional {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
        /// The default child tool to be called.
        #[serde(skip_serializing_if = "Option::is_none")]
        default_child: Option<String>,
        /// Mapping from tool output values to child tool names.
        child_output_mapping: HashMap<String, String>,
        /// Whether to throw an error when output doesn't match any case.
        #[serde(default)]
        require_output_mapping: bool,
    },
    /// Child tool rule.
    Child {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
        /// The name of the tool that can be a child of this tool.
        child_tool_name: String,
    },
    /// Parent tool rule.
    Parent {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
        /// The name of the tool that can be a parent of this tool.
        parent_tool_name: String,
    },
    /// Required before exit rule.
    RequiredBeforeExit {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Init tool rule.
    Init {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Requires approval tool rule.
    #[serde(rename = "requires_approval")]
    RequiresApproval {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Run first tool rule.
    #[serde(rename = "run_first")]
    RunFirst {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Default on empty tool rule.
    #[serde(rename = "default_on_empty")]
    DefaultOnEmpty {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Unknown tool rule variant (catches new/unknown types).
    #[serde(other)]
    Unknown,
}

/// Conditional tool rule configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ConditionalToolRule {
    /// Tool name this rule applies to.
    pub tool_name: String,
    /// Optional prompt template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<String>,
    /// The default child tool to be called.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_child: Option<String>,
    /// Mapping from tool output values to child tool names.
    #[builder(default)]
    pub child_output_mapping: HashMap<String, String>,
    /// Whether to throw an error when output doesn't match any case.
    #[serde(default)]
    #[builder(default)]
    pub require_output_mapping: bool,
}

impl ConditionalToolRule {
    /// Add a mapping from output value to child tool name.
    pub fn with_mapping(mut self, output: impl Into<String>, child: impl Into<String>) -> Self {
        self.child_output_mapping
            .insert(output.into(), child.into());
        self
    }

    /// Convert to a ToolRule.
    pub fn build(self) -> ToolRule {
        ToolRule::Conditional {
            tool_name: self.tool_name,
            prompt_template: self.prompt_template,
            default_child: self.default_child,
            child_output_mapping: self.child_output_mapping,
            require_output_mapping: self.require_output_mapping,
        }
    }
}

impl ToolRule {
    /// Create a continue loop rule.
    pub fn continue_loop(tool_name: impl Into<String>) -> Self {
        Self::ContinueLoop {
            tool_name: tool_name.into(),
            prompt_template: None,
        }
    }

    /// Create an exit loop rule.
    pub fn exit_loop(tool_name: impl Into<String>) -> Self {
        Self::ExitLoop {
            tool_name: tool_name.into(),
            prompt_template: None,
        }
    }

    /// Create a terminal rule (deprecated, prefer exit_loop).
    pub fn terminal(tool_name: impl Into<String>) -> Self {
        Self::Terminal {
            tool_name: tool_name.into(),
            prompt_template: None,
        }
    }

    /// Create a max count per step rule.
    pub fn max_count_per_step(tool_name: impl Into<String>, max_count_limit: u32) -> Self {
        Self::MaxCountPerStep {
            tool_name: tool_name.into(),
            prompt_template: None,
            max_count_limit,
        }
    }

    /// Create a conditional tool rule builder.
    pub fn conditional(tool_name: impl Into<String>) -> ConditionalToolRule {
        ConditionalToolRule::builder()
            .tool_name(tool_name.into())
            .build()
    }

    /// Create a child tool rule.
    pub fn child(tool_name: impl Into<String>, child_tool_name: impl Into<String>) -> Self {
        Self::Child {
            tool_name: tool_name.into(),
            prompt_template: None,
            child_tool_name: child_tool_name.into(),
        }
    }

    /// Create a parent tool rule.
    pub fn parent(tool_name: impl Into<String>, parent_tool_name: impl Into<String>) -> Self {
        Self::Parent {
            tool_name: tool_name.into(),
            prompt_template: None,
            parent_tool_name: parent_tool_name.into(),
        }
    }

    /// Create a required before exit rule.
    pub fn required_before_exit(tool_name: impl Into<String>) -> Self {
        Self::RequiredBeforeExit {
            tool_name: tool_name.into(),
            prompt_template: None,
        }
    }

    /// Create an init rule.
    pub fn init(tool_name: impl Into<String>) -> Self {
        Self::Init {
            tool_name: tool_name.into(),
            prompt_template: None,
        }
    }

    /// Add a prompt template to this rule.
    pub fn with_prompt_template(mut self, template: impl Into<String>) -> Self {
        match &mut self {
            Self::ContinueLoop {
                prompt_template, ..
            }
            | Self::ExitLoop {
                prompt_template, ..
            }
            | Self::Terminal {
                prompt_template, ..
            }
            | Self::MaxCountPerStep {
                prompt_template, ..
            }
            | Self::Child {
                prompt_template, ..
            }
            | Self::Parent {
                prompt_template, ..
            }
            | Self::RequiredBeforeExit {
                prompt_template, ..
            }
            | Self::Init {
                prompt_template, ..
            }
            | Self::RequiresApproval {
                prompt_template, ..
            }
            | Self::RunFirst {
                prompt_template, ..
            }
            | Self::DefaultOnEmpty {
                prompt_template, ..
            } => {
                *prompt_template = Some(template.into());
            }
            Self::Conditional {
                prompt_template, ..
            } => {
                *prompt_template = Some(template.into());
            }
            Self::Unknown => {}
        }
        self
    }
}

/// Response format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, SmartDefault)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    /// Text response.
    #[default]
    Text,
    /// JSON object response.
    JsonObject,
    /// Other response format types.
    #[serde(other)]
    Other,
}

/// Response format configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    /// Response type.
    #[serde(rename = "type")]
    pub format_type: ResponseFormatType,
    /// JSON schema for validation (when type is json_object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
}

impl ResponseFormat {
    /// Create a text response format.
    pub fn text() -> Self {
        Self {
            format_type: ResponseFormatType::Text,
            json_schema: None,
        }
    }

    /// Create a JSON response format with optional schema.
    pub fn json(schema: Option<serde_json::Value>) -> Self {
        Self {
            format_type: ResponseFormatType::JsonObject,
            json_schema: schema,
        }
    }

    /// Create a JSON response format with a schema from a JSON string.
    pub fn json_with_schema(schema_str: &str) -> Result<Self, serde_json::Error> {
        let schema = serde_json::from_str(schema_str)?;
        Ok(Self {
            format_type: ResponseFormatType::JsonObject,
            json_schema: Some(schema),
        })
    }
}

/// Agent state and configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// Unique identifier for the agent.
    pub id: LettaId,
    /// Agent name.
    pub name: String,
    /// System prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Agent type.
    #[serde(default)]
    pub agent_type: AgentType,
    /// LLM configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_config: Option<LLMConfig>,
    /// Embedding configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
    /// Agent memory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<AgentMemory>,
    /// Available tools.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolReference>,
    /// Data sources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<serde_json::Value>,
    /// Agent tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Agent description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Agent metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<LettaId>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// When the agent was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the agent was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// Tool rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_rules: Option<Vec<ToolRule>>,
    /// Message IDs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub message_ids: Vec<LettaId>,
    /// Configuration for multi-agent group participation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_agent_group: Option<serde_json::Value>,
    /// Template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<LettaId>,
    /// Base template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_template_id: Option<LettaId>,
    /// Identity IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ids: Option<Vec<LettaId>>,
    /// Tool execution environment variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_exec_environment_variables: Option<Vec<AgentEnvironmentVariable>>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<LettaId>,
    /// Timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Last run completion time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_completion: Option<Timestamp>,
    /// Last run duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_duration_ms: Option<u64>,
    /// Whether to enable sleeptime mode for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sleeptime: Option<bool>,
    /// Response format configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Message buffer autoclear setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_buffer_autoclear: Option<bool>,
    /// The model handle used by the agent (format: provider/model-name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The embedding model handle used by the agent (format: provider/model-name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<String>,
    /// The model settings used by the agent (provider-specific).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<serde_json::Value>,
    /// The environment variables for tool execution specific to this agent.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<AgentEnvironmentVariable>,
    /// The id of the deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<LettaId>,
    /// The id of the entity within the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<LettaId>,
    /// The identities associated with this agent.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identities: Vec<serde_json::Value>,
    /// The multi-agent group that this agent manages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_group: Option<serde_json::Value>,
    /// The stop reason from the agent's last run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop_reason: Option<String>,
    /// Maximum number of files that can be open at once for this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files_open: Option<i32>,
    /// The per-file view window character limit for this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_file_view_window_char_limit: Option<i32>,
    /// If set to True, the agent will be hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// The memory blocks used by the agent.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<Block>,
}

/// Request to create a new agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAgentRequest {
    /// Agent name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// System prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Agent type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<AgentType>,
    /// LLM configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_config: Option<LLMConfig>,
    /// Embedding configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
    /// Memory blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_blocks: Option<Vec<Block>>,
    /// Tools to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
    /// Tool IDs to attach (alternative to tools).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<LettaId>>,
    /// Source IDs to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<Vec<LettaId>>,
    /// Agent tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Agent description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Agent metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Agent timezone (IANA format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Include base tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_base_tools: Option<bool>,
    /// Include multi-agent tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_multi_agent_tools: Option<bool>,
    /// Include base tool rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_base_tool_rules: Option<bool>,
    /// Include default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_default_source: Option<bool>,
    /// Tool rules to apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_rules: Option<Vec<ToolRule>>,
    /// Initial message sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_message_sequence: Option<Vec<serde_json::Value>>,
    /// Tool execution environment variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_exec_environment_variables: Option<HashMap<String, String>>,
    /// Response format configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Enable reasoner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_reasoner: Option<bool>,
    /// Whether to automatically clear the message buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_buffer_autoclear: Option<bool>,
    /// Block IDs to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<Vec<LettaId>>,
    /// Model shorthand (alternative to llm_config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Embedding shorthand (alternative to embedding_config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<String>,
    /// Context window limit shorthand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window_limit: Option<u32>,
    /// Embedding chunk size shorthand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_chunk_size: Option<u32>,
    /// Max tokens shorthand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Max reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_reasoning_tokens: Option<u32>,
    /// Create from template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_template: Option<String>,
    /// Mark as template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
    /// Memory variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_variables: Option<HashMap<String, String>>,
    /// Project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<LettaId>,
    /// Template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<LettaId>,
    /// Base template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_template_id: Option<LettaId>,
    /// Identity IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ids: Option<Vec<LettaId>>,
    /// Whether to enable sleeptime mode for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sleeptime: Option<bool>,
}

impl CreateAgentRequest {
    /// Create a new agent request builder.
    pub fn builder() -> CreateAgentRequestBuilder {
        CreateAgentRequestBuilder::default()
    }
}

/// Builder for CreateAgentRequest.
#[derive(Debug, Default)]
pub struct CreateAgentRequestBuilder {
    request: CreateAgentRequest,
}

impl CreateAgentRequestBuilder {
    /// Set the agent name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    /// Set the system prompt.
    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.request.system = Some(system.into());
        self
    }

    /// Set the agent type.
    pub fn agent_type(mut self, agent_type: AgentType) -> Self {
        self.request.agent_type = Some(agent_type);
        self
    }

    /// Set the LLM configuration.
    pub fn llm_config(mut self, config: LLMConfig) -> Self {
        self.request.llm_config = Some(config);
        self
    }

    /// Set the embedding configuration.
    pub fn embedding_config(mut self, config: EmbeddingConfig) -> Self {
        self.request.embedding_config = Some(config);
        self
    }

    /// Add memory blocks.
    pub fn memory_blocks(mut self, blocks: Vec<Block>) -> Self {
        self.request.memory_blocks = Some(blocks);
        self
    }

    /// Add a single memory block.
    pub fn memory_block(mut self, block: Block) -> Self {
        self.request
            .memory_blocks
            .get_or_insert_with(Vec::new)
            .push(block);
        self
    }

    /// Add tools.
    pub fn tools(mut self, tools: Vec<String>) -> Self {
        self.request.tools = Some(tools);
        self
    }

    /// Add tags.
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.request.tags = Some(tags);
        self
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// Set the timezone.
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.request.timezone = Some(timezone.into());
        self
    }

    /// Set include base tools.
    pub fn include_base_tools(mut self, include: bool) -> Self {
        self.request.include_base_tools = Some(include);
        self
    }

    /// Set tool rules.
    pub fn tool_rules(mut self, rules: Vec<ToolRule>) -> Self {
        self.request.tool_rules = Some(rules);
        self
    }

    /// Set initial message sequence.
    pub fn initial_message_sequence(mut self, sequence: Vec<serde_json::Value>) -> Self {
        self.request.initial_message_sequence = Some(sequence);
        self
    }

    /// Set tool execution environment variables.
    pub fn tool_exec_environment_variables(mut self, vars: HashMap<String, String>) -> Self {
        self.request.tool_exec_environment_variables = Some(vars);
        self
    }

    /// Set the model shorthand (alternative to llm_config).
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.request.model = Some(model.into());
        self
    }

    /// Set the embedding shorthand (alternative to embedding_config).
    pub fn embedding(mut self, embedding: impl Into<String>) -> Self {
        self.request.embedding = Some(embedding.into());
        self
    }

    /// Set tool IDs to attach (alternative to tools).
    pub fn tool_ids(mut self, tool_ids: Vec<LettaId>) -> Self {
        self.request.tool_ids = Some(tool_ids);
        self
    }

    /// Set source IDs to attach.
    pub fn source_ids(mut self, source_ids: Vec<LettaId>) -> Self {
        self.request.source_ids = Some(source_ids);
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.request.metadata = Some(metadata);
        self
    }

    /// Set include multi-agent tools.
    pub fn include_multi_agent_tools(mut self, include: bool) -> Self {
        self.request.include_multi_agent_tools = Some(include);
        self
    }

    /// Set include base tool rules.
    pub fn include_base_tool_rules(mut self, include: bool) -> Self {
        self.request.include_base_tool_rules = Some(include);
        self
    }

    /// Set include default source.
    pub fn include_default_source(mut self, include: bool) -> Self {
        self.request.include_default_source = Some(include);
        self
    }

    /// Set response format configuration.
    pub fn response_format(mut self, format: ResponseFormat) -> Self {
        self.request.response_format = Some(format);
        self
    }

    /// Set enable reasoner.
    pub fn enable_reasoner(mut self, enable: bool) -> Self {
        self.request.enable_reasoner = Some(enable);
        self
    }

    /// Set message buffer autoclear.
    pub fn message_buffer_autoclear(mut self, autoclear: bool) -> Self {
        self.request.message_buffer_autoclear = Some(autoclear);
        self
    }

    /// Set block IDs to attach.
    pub fn block_ids(mut self, block_ids: Vec<LettaId>) -> Self {
        self.request.block_ids = Some(block_ids);
        self
    }

    /// Set context window limit shorthand.
    pub fn context_window_limit(mut self, limit: u32) -> Self {
        self.request.context_window_limit = Some(limit);
        self
    }

    /// Set embedding chunk size shorthand.
    pub fn embedding_chunk_size(mut self, size: u32) -> Self {
        self.request.embedding_chunk_size = Some(size);
        self
    }

    /// Set max tokens shorthand.
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    /// Set max reasoning tokens.
    pub fn max_reasoning_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_reasoning_tokens = Some(max_tokens);
        self
    }

    /// Create from template.
    pub fn from_template(mut self, template_name: impl Into<String>) -> Self {
        self.request.from_template = Some(template_name.into());
        self
    }

    /// Mark as template.
    pub fn template(mut self, is_template: bool) -> Self {
        self.request.template = Some(is_template);
        self
    }

    /// Set memory variables.
    pub fn memory_variables(mut self, variables: HashMap<String, String>) -> Self {
        self.request.memory_variables = Some(variables);
        self
    }

    /// Set project ID.
    pub fn project_id(mut self, project_id: LettaId) -> Self {
        self.request.project_id = Some(project_id);
        self
    }

    /// Set template ID.
    pub fn template_id(mut self, template_id: LettaId) -> Self {
        self.request.template_id = Some(template_id);
        self
    }

    /// Set base template ID.
    pub fn base_template_id(mut self, base_template_id: LettaId) -> Self {
        self.request.base_template_id = Some(base_template_id);
        self
    }

    /// Set identity IDs.
    pub fn identity_ids(mut self, identity_ids: Vec<LettaId>) -> Self {
        self.request.identity_ids = Some(identity_ids);
        self
    }

    /// Set enable sleeptime mode.
    pub fn enable_sleeptime(mut self, enable: bool) -> Self {
        self.request.enable_sleeptime = Some(enable);
        self
    }

    // Convenience methods

    /// Add a single tool by name.
    pub fn tool(mut self, tool: impl Into<String>) -> Self {
        self.request
            .tools
            .get_or_insert_with(Vec::new)
            .push(tool.into());
        self
    }

    /// Add a single tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.request
            .tags
            .get_or_insert_with(Vec::new)
            .push(tag.into());
        self
    }

    /// Add a single tool ID.
    pub fn tool_id(mut self, tool_id: LettaId) -> Self {
        self.request
            .tool_ids
            .get_or_insert_with(Vec::new)
            .push(tool_id);
        self
    }

    /// Add a single source ID.
    pub fn source_id(mut self, source_id: LettaId) -> Self {
        self.request
            .source_ids
            .get_or_insert_with(Vec::new)
            .push(source_id);
        self
    }

    /// Add a single block ID.
    pub fn block_id(mut self, block_id: LettaId) -> Self {
        self.request
            .block_ids
            .get_or_insert_with(Vec::new)
            .push(block_id);
        self
    }

    /// Add a single identity ID.
    pub fn identity_id(mut self, identity_id: LettaId) -> Self {
        self.request
            .identity_ids
            .get_or_insert_with(Vec::new)
            .push(identity_id);
        self
    }

    /// Add a single tool rule.
    pub fn tool_rule(mut self, rule: ToolRule) -> Self {
        self.request
            .tool_rules
            .get_or_insert_with(Vec::new)
            .push(rule);
        self
    }

    /// Add a single memory variable.
    pub fn memory_variable(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request
            .memory_variables
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Add a single tool execution environment variable.
    pub fn tool_exec_env_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request
            .tool_exec_environment_variables
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Build the request.
    pub fn build(self) -> CreateAgentRequest {
        self.request
    }
}

/// Request to update an agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateAgentRequest {
    /// Agent name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// System prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Agent type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<AgentType>,
    /// LLM configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_config: Option<LLMConfig>,
    /// Embedding configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
    /// Agent tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Agent description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Agent metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Request parameters for importing an agent from file.
#[derive(Debug, Clone, Default)]
pub struct ImportAgentRequest {
    /// Whether to append a copy suffix to the agent name if it already exists.
    pub append_copy_suffix: Option<bool>,
    /// Whether to override existing tools with the same name.
    pub override_existing_tools: Option<bool>,
    /// The project ID to associate the uploaded agent with.
    pub project_id: Option<LettaId>,
    /// If set to True, strips all messages from the agent before importing.
    pub strip_messages: Option<bool>,
}

/// Search response for agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsSearchResponse {
    /// List of matching agents.
    pub agents: Vec<AgentState>,
    /// Cursor for pagination.
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
}

/// Search request for agents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentsSearchRequest {
    /// Search criteria (simplified for now).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<Vec<String>>,
    /// Project ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<LettaId>,
    /// Search combinator (only "AND" supported).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<String>,
    /// Maximum number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Sort field.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
}

/// Query parameters for listing agents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListAgentsParams {
    /// Filter by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Whether to match all tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_all_tags: Option<bool>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Search query text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    /// Filter by project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<LettaId>,
    /// Filter by template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<LettaId>,
    /// Filter by base template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_template_id: Option<LettaId>,
    /// Filter by identity ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<LettaId>,
    /// Search by identifier keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_keys: Option<Vec<String>>,
    /// Include relationships.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_relationships: Option<Vec<String>>,
    /// Sort order (ascending).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    /// Sort by field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListAgentsParams {
    /// Create a new list agents params builder.
    pub fn builder() -> ListAgentsParamsBuilder {
        ListAgentsParamsBuilder::default()
    }
}

/// Builder for ListAgentsParams.
#[derive(Debug, Default)]
pub struct ListAgentsParamsBuilder {
    params: ListAgentsParams,
}

impl ListAgentsParamsBuilder {
    /// Filter by name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    /// Filter by tags.
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.params.tags = Some(tags);
        self
    }

    /// Set pagination limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.params.limit = Some(limit);
        self
    }

    /// Set search query.
    pub fn query_text(mut self, query: impl Into<String>) -> Self {
        self.params.query_text = Some(query.into());
        self
    }

    /// Build the params.
    pub fn build(self) -> ListAgentsParams {
        self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_agent_serialization() {
        let agent = AgentState {
            id: LettaId::from_str("agent-00000000-0000-0000-0000-000000000000").unwrap(),
            name: "Test Agent".to_string(),
            system: Some("You are a helpful assistant".to_string()),
            agent_type: AgentType::MemGPT,
            llm_config: Some(LLMConfig {
                model: "gpt-4".to_string(),
                model_endpoint_type: ModelEndpointType::Openai,
                model_endpoint: None,
                context_window: Some(8192),
                provider_name: None,
                provider_category: None,
                model_wrapper: None,
                put_inner_thoughts_in_kwargs: None,
                handle: None,
                temperature: None,
                max_tokens: None,
                enable_reasoner: None,
                reasoning_effort: None,
                max_reasoning_tokens: None,
                extra: HashMap::new(),
            }),
            embedding_config: None,
            memory: None,
            tools: vec![],
            sources: vec![],
            tags: vec!["test".to_string()],
            description: Some("A test agent".to_string()),
            metadata: None,
            project_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            tool_rules: None,
            message_ids: vec![],
            multi_agent_group: None,
            template_id: None,
            base_template_id: None,
            identity_ids: None,
            tool_exec_environment_variables: None,
            organization_id: None,
            timezone: None,
            last_run_completion: None,
            last_run_duration_ms: None,
            enable_sleeptime: None,
            response_format: None,
            message_buffer_autoclear: None,
            model: None,
            embedding: None,
            model_settings: None,
            secrets: vec![],
            deployment_id: None,
            entity_id: None,
            identities: vec![],
            managed_group: None,
            last_stop_reason: None,
            max_files_open: None,
            per_file_view_window_char_limit: None,
            hidden: None,
            blocks: vec![],
        };

        let json = serde_json::to_string(&agent).unwrap();
        let deserialized: AgentState = serde_json::from_str(&json).unwrap();
        assert_eq!(agent.name, deserialized.name);
    }

    #[test]
    fn test_create_agent_request_builder() {
        let request = CreateAgentRequest::builder()
            .name("My Agent")
            .system("You are helpful")
            .agent_type(AgentType::MemGPTv2)
            .memory_block(Block {
                id: None,
                label: "human".to_string(),
                value: "The human's name is Bob.".to_string(),
                limit: Some(5000),
                is_template: false,
                preserve_on_migration: false,
                read_only: false,
                description: None,
                metadata: None,
                name: None,
                organization_id: None,
                created_by_id: None,
                last_updated_by_id: None,
                created_at: None,
                updated_at: None,
            })
            .memory_block(Block {
                id: None,
                label: "persona".to_string(),
                value: "I am Sam, a helpful assistant.".to_string(),
                limit: Some(5000),
                is_template: false,
                preserve_on_migration: false,
                read_only: false,
                description: None,
                metadata: None,
                name: None,
                organization_id: None,
                created_by_id: None,
                last_updated_by_id: None,
                created_at: None,
                updated_at: None,
            })
            .tags(vec!["test".to_string(), "demo".to_string()])
            .build();

        assert_eq!(request.name.as_deref(), Some("My Agent"));
        assert_eq!(request.memory_blocks.as_ref().unwrap().len(), 2);
        assert_eq!(request.tags.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_create_agent_request_builder_extended() {
        let request = CreateAgentRequest::builder()
            .name("Advanced Agent")
            .model("gpt-4")
            .embedding("text-embedding-ada-002")
            .include_base_tools(true)
            .include_multi_agent_tools(true)
            .enable_sleeptime(true)
            .enable_reasoner(true)
            .context_window_limit(8192)
            .max_tokens(2048)
            .max_reasoning_tokens(4096)
            .template(true)
            .tag("production")
            .tag("multiagent")
            .tool("send_message")
            .tool("archival_memory_search")
            .memory_variable("user_name", "Alice")
            .memory_variable("agent_role", "assistant")
            .tool_exec_env_var("API_KEY", "secret123")
            .response_format(ResponseFormat::json(None))
            .build();

        assert_eq!(request.name.as_deref(), Some("Advanced Agent"));
        assert_eq!(request.model.as_deref(), Some("gpt-4"));
        assert_eq!(request.enable_sleeptime, Some(true));
        assert_eq!(request.enable_reasoner, Some(true));
        assert_eq!(request.context_window_limit, Some(8192));
        assert_eq!(request.template, Some(true));
        assert_eq!(request.tags.as_ref().unwrap().len(), 2);
        assert_eq!(request.tools.as_ref().unwrap().len(), 2);
        assert_eq!(request.memory_variables.as_ref().unwrap().len(), 2);
        assert_eq!(
            request
                .tool_exec_environment_variables
                .as_ref()
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn test_agent_type_serialization() {
        let agent_type = AgentType::Sleeptime;
        let json = serde_json::to_string(&agent_type).unwrap();
        assert_eq!(json, r#""sleeptime_agent""#);

        let deserialized: AgentType = serde_json::from_str(&json).unwrap();
        assert_eq!(agent_type, deserialized);
    }

    #[test]
    fn test_memory_block_serialization() {
        let block = Block {
            id: Some(LettaId::from_str("block-550e8400-e29b-41d4-a716-446655440002").unwrap()),
            label: "human".to_string(),
            value: "The human's name is Alice.".to_string(),
            limit: Some(1000),
            is_template: false,
            preserve_on_migration: true,
            read_only: false,
            description: Some("Human information".to_string()),
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&block).unwrap();
        let deserialized: Block = serde_json::from_str(&json).unwrap();
        assert_eq!(block.label, deserialized.label);
        assert_eq!(block.value, deserialized.value);
    }

    #[test]
    fn test_list_agents_params_builder() {
        let params = ListAgentsParams::builder()
            .name("test")
            .tags(vec!["production".to_string()])
            .limit(50)
            .query_text("search term")
            .build();

        assert_eq!(params.name.as_deref(), Some("test"));
        assert_eq!(params.limit, Some(50));
    }

    #[test]
    fn test_embedding_endpoint_type_serialization() {
        let endpoint_type = EmbeddingEndpointType::Openai;
        let json = serde_json::to_string(&endpoint_type).unwrap();
        assert_eq!(json, r#""openai""#);

        let deserialized: EmbeddingEndpointType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, EmbeddingEndpointType::Openai);
    }

    #[test]
    fn test_model_endpoint_type_serialization() {
        let endpoint_type = ModelEndpointType::Anthropic;
        let json = serde_json::to_string(&endpoint_type).unwrap();
        assert_eq!(json, r#""anthropic""#);

        let deserialized: ModelEndpointType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ModelEndpointType::Anthropic);
    }

    #[test]
    fn test_azure_embedding_config_flattening() {
        let config = EmbeddingConfig {
            embedding_model: Some("text-embedding-ada-002".to_string()),
            embedding_endpoint_type: Some(EmbeddingEndpointType::Azure),
            embedding_endpoint: Some("https://myazure.openai.azure.com".to_string()),
            embedding_dim: Some(1536),
            embedding_chunk_size: Some(300),
            handle: None,
            azure_config: Some(AzureEmbeddingConfig {
                azure_endpoint: Some("https://myazure.openai.azure.com".to_string()),
                azure_version: Some("2023-05-15".to_string()),
                azure_deployment: Some("my-deployment".to_string()),
            }),
            extra: HashMap::new(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify Azure fields are flattened to top level
        assert_eq!(parsed["azure_endpoint"], "https://myazure.openai.azure.com");
        assert_eq!(parsed["azure_version"], "2023-05-15");
        assert_eq!(parsed["azure_deployment"], "my-deployment");

        // Verify we can deserialize back
        let deserialized: EmbeddingConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized
                .azure_config
                .as_ref()
                .unwrap()
                .azure_version
                .as_deref(),
            Some("2023-05-15")
        );
    }

    #[test]
    fn test_create_agent_request_shorthand_fields() {
        let request = CreateAgentRequest {
            name: Some("Test Agent".to_string()),
            model: Some("gpt-4".to_string()),
            embedding: Some("text-embedding-ada-002".to_string()),
            context_window_limit: Some(8192),
            embedding_chunk_size: Some(512),
            max_tokens: Some(2048),
            from_template: Some("customer-support".to_string()),
            template: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["model"], "gpt-4");
        assert_eq!(parsed["embedding"], "text-embedding-ada-002");
        assert_eq!(parsed["context_window_limit"], 8192);
        assert_eq!(parsed["template"], true);
    }
}
