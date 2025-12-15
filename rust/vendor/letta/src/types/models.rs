//! Model-related types.

use serde::{Deserialize, Serialize};

use super::ModelEndpointType;

/// LLM configuration with model details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Model name/identifier.
    pub model: String,
    /// Model endpoint type.
    pub model_endpoint_type: ModelEndpointType,
    /// Context window size.
    pub context_window: u32,
    /// Provider details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Provider type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// Provider category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_category: Option<String>,
    /// Model wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_wrapper: Option<String>,
    /// Maximum tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Temperature for sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Frequency penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Presence penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Number of output sequences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Random seed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Maximum number of retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,
    /// Request timeout in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<u32>,
    /// Whether reasoning is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_reasoning: Option<bool>,
    /// Reasoning effort level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    /// Top-k sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    /// Repetition penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,
    /// Endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

/// Reasoning effort levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    /// Low reasoning effort.
    Low,
    /// Medium reasoning effort.
    Medium,
    /// High reasoning effort.
    High,
}

/// Embedding model configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingModel {
    /// Embedding endpoint type.
    pub embedding_endpoint_type: String,
    /// Embedding endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_endpoint: Option<String>,
    /// Embedding model name/identifier.
    pub embedding_model: String,
    /// Embedding dimension.
    pub embedding_dim: u32,
    /// Maximum chunk size.
    pub embedding_chunk_size: u32,
    /// Model handle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
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

/// Query parameters for listing models.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListModelsParams {
    /// Filter by provider category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_category: Option<Vec<ProviderCategory>>,
    /// Filter by provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// Filter by provider type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
}

/// Provider category for embedding modules
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderCategory {
    /// base
    #[serde(rename = "base")]
    Base,
    /// byok
    #[serde(rename = "byok")]
    Byok,
}

impl std::fmt::Display for ProviderCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderCategory::Base => f.write_str("base"),
            ProviderCategory::Byok => f.write_str("byok"),
        }
    }
}

/// Query parameters for listing embedding models.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListEmbeddingModelsParams {
    /// Filter by provider category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_category: Option<Vec<ProviderCategory>>,
}
