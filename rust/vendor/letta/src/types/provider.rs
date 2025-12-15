//! Provider-related types.

use bon::Builder;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::{LettaId, ProviderCategory};

/// Provider configuration for LLM/embedding models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    /// Provider ID (prefixed with "provider-").
    pub id: LettaId,
    /// Provider name.
    pub name: String,
    /// Provider type.
    pub provider_type: ProviderType,
    /// Provider category.
    pub provider_category: ProviderCategory,
    /// API key (encrypted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Base URL for the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Access key (for certain providers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// Secret key (for certain providers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// Region (for providers like AWS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Additional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Last updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Provider types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    /// Anthropic
    Anthropic,
    /// OpenAI
    Openai,
    /// Azure OpenAI
    Azure,
    /// Google AI
    #[serde(rename = "google_ai")]
    GoogleAi,
    /// Groq
    Groq,
    /// Cohere
    Cohere,
    /// Together AI
    Together,
    /// Mistral
    Mistral,
    /// Ollama (local)
    Ollama,
    /// vLLM
    #[serde(rename = "vllm")]
    Vllm,
    /// LM Studio
    #[serde(rename = "lmstudio")]
    LmStudio,
    /// Kobold
    Kobold,
    /// Replicate
    Replicate,
    /// OpenRouter
    #[serde(rename = "openrouter")]
    OpenRouter,
    /// Perplexity
    Perplexity,
    /// Recursal
    Recursal,
    /// Fireworks
    Fireworks,
    /// Anyscale
    Anyscale,
    /// Cloudflare
    Cloudflare,
    /// Voyage
    Voyage,
    /// AWS Bedrock
    Bedrock,
    /// Other/Custom
    Other,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anthropic => write!(f, "anthropic"),
            Self::Openai => write!(f, "openai"),
            Self::Azure => write!(f, "azure"),
            Self::GoogleAi => write!(f, "google_ai"),
            Self::Groq => write!(f, "groq"),
            Self::Cohere => write!(f, "cohere"),
            Self::Together => write!(f, "together"),
            Self::Mistral => write!(f, "mistral"),
            Self::Ollama => write!(f, "ollama"),
            Self::Vllm => write!(f, "vllm"),
            Self::LmStudio => write!(f, "lmstudio"),
            Self::Kobold => write!(f, "kobold"),
            Self::Replicate => write!(f, "replicate"),
            Self::OpenRouter => write!(f, "openrouter"),
            Self::Perplexity => write!(f, "perplexity"),
            Self::Recursal => write!(f, "recursal"),
            Self::Fireworks => write!(f, "fireworks"),
            Self::Anyscale => write!(f, "anyscale"),
            Self::Cloudflare => write!(f, "cloudflare"),
            Self::Voyage => write!(f, "voyage"),
            Self::Bedrock => write!(f, "bedrock"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl std::str::FromStr for ProviderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "anthropic" => Ok(Self::Anthropic),
            "openai" => Ok(Self::Openai),
            "azure" => Ok(Self::Azure),
            "google_ai" => Ok(Self::GoogleAi),
            "groq" => Ok(Self::Groq),
            "cohere" => Ok(Self::Cohere),
            "together" => Ok(Self::Together),
            "mistral" => Ok(Self::Mistral),
            "ollama" => Ok(Self::Ollama),
            "vllm" => Ok(Self::Vllm),
            "lmstudio" => Ok(Self::LmStudio),
            "kobold" => Ok(Self::Kobold),
            "replicate" => Ok(Self::Replicate),
            "openrouter" => Ok(Self::OpenRouter),
            "perplexity" => Ok(Self::Perplexity),
            "recursal" => Ok(Self::Recursal),
            "fireworks" => Ok(Self::Fireworks),
            "anyscale" => Ok(Self::Anyscale),
            "cloudflare" => Ok(Self::Cloudflare),
            "voyage" => Ok(Self::Voyage),
            "bedrock" => Ok(Self::Bedrock),
            "other" => Ok(Self::Other),
            _ => Err(format!("Unknown provider type: {}", s)),
        }
    }
}

impl<'a> TryFrom<&'a str> for ProviderType {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

/// Request to create a new provider.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ProviderCreate {
    /// Provider name.
    pub name: String,
    /// Provider type.
    pub provider_type: ProviderType,
    /// Provider category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_category: Option<ProviderCategory>,
    /// API key (required).
    pub api_key: String,
    /// Base URL for the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Access key (for certain providers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// Secret key (for certain providers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// Region (for providers like AWS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Additional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Request to update a provider.
///
/// Only `api_key`, `access_key`, and `region` can be updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderUpdate {
    /// API key (required).
    pub api_key: String,
    /// Access key (for certain providers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// Region (for providers like AWS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Query parameters for listing providers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListProvidersParams {
    /// Filter by provider category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_category: Option<ProviderCategory>,
    /// Filter by provider type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<ProviderType>,
    /// Cursor for pagination - return results after this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Maximum number of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Response from provider check endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCheckResponse {
    /// Whether the provider is reachable.
    pub status: bool,
    /// Optional error message if check failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Response from provider delete endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDeleteResponse {
    /// Success message.
    pub message: String,
}
