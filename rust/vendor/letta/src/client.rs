//! HTTP client and configuration for the Letta API.

use crate::auth::AuthConfig;
use crate::environment::LettaEnvironment;
use crate::error::{LettaError, LettaResult};
use crate::retry::{retry_with_config, RetryConfig};
use reqwest::header::HeaderMap;
use std::time::Duration;
use url::Url;

/// Default maximum response body size (10 MB).
const DEFAULT_MAX_RESPONSE_BYTES: usize = 10 * 1024 * 1024;

/// Configuration for the Letta client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the Letta API.
    pub base_url: Url,
    /// Authentication configuration.
    pub auth: AuthConfig,
    /// Request timeout duration.
    pub timeout: Duration,
    /// Maximum response body size in bytes before rejecting.
    pub max_response_bytes: usize,
    /// Additional HTTP headers to include in every request.
    pub headers: HeaderMap,
}

impl ClientConfig {
    /// Create a new client configuration.
    pub fn new(base_url: impl AsRef<str>) -> LettaResult<Self> {
        let base_url = Url::parse(base_url.as_ref())?;
        Ok(Self {
            base_url,
            auth: AuthConfig::default(),
            timeout: Duration::from_secs(30),
            headers: HeaderMap::new(),
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
        })
    }

    /// Set the authentication configuration.
    pub fn auth(mut self, auth: AuthConfig) -> Self {
        self.auth = auth;
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set additional headers to include with all requests.
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    /// Add a single header to include with all requests.
    pub fn header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> LettaResult<Self> {
        let key = key.as_ref();
        let value = value.as_ref();

        let header_name = reqwest::header::HeaderName::from_bytes(key.as_bytes())
            .map_err(|_| LettaError::validation(format!("Invalid header name: {}", key)))?;
        let header_value = reqwest::header::HeaderValue::from_str(value).map_err(|_| {
            LettaError::validation(format!("Invalid header value for {}: {}", key, value))
        })?;

        self.headers.insert(header_name, header_value);
        Ok(self)
    }

    /// Set the X-Project header for all requests.
    ///
    /// This associates all operations with a specific project context.
    pub fn project(self, project_id: impl AsRef<str>) -> LettaResult<Self> {
        self.header("X-Project", project_id)
    }

    /// Set the user-id header for all requests.
    ///
    /// This identifies the user making the requests.
    pub fn user_id(self, user_id: impl AsRef<str>) -> LettaResult<Self> {
        self.header("user-id", user_id)
    }
}

/// Main Letta API client.
#[derive(Debug, Clone)]
pub struct LettaClient {
    http: reqwest::Client,
    config: ClientConfig,
    retry_config: RetryConfig,
}

impl LettaClient {
    /// Create a new Letta client.
    pub fn new(config: ClientConfig) -> LettaResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(config.headers.clone())
            .build()?;

        Ok(Self {
            http,
            config,
            retry_config: RetryConfig::default(),
        })
    }

    /// Create a new client for Letta Cloud with the given API token.
    pub fn cloud(token: impl Into<String>) -> LettaResult<Self> {
        ClientBuilder::new()
            .environment(LettaEnvironment::Cloud)
            .auth(AuthConfig::bearer(token))
            .build()
    }

    /// Create a new client for a self-hosted/local Letta server.
    pub fn local() -> LettaResult<Self> {
        ClientBuilder::new()
            .environment(LettaEnvironment::SelfHosted)
            .build()
    }

    /// Create a new client for Letta Cloud with project context.
    ///
    /// This creates a client that automatically includes the X-Project header
    /// with all requests, associating them with the specified project.
    pub fn cloud_with_project(
        token: impl Into<String>,
        project_id: impl AsRef<str>,
    ) -> LettaResult<Self> {
        ClientBuilder::new()
            .environment(LettaEnvironment::Cloud)
            .auth(AuthConfig::bearer(token))
            .project(project_id)?
            .build()
    }

    /// Create a new client builder.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &Url {
        &self.config.base_url
    }

    /// Get the HTTP client.
    pub fn http(&self) -> &reqwest::Client {
        &self.http
    }

    /// Get the authentication configuration.
    pub fn auth(&self) -> &AuthConfig {
        &self.config.auth
    }

    /// Get the agent API.
    pub fn agents(&self) -> crate::api::AgentApi<'_> {
        crate::api::AgentApi::new(self)
    }

    /// Get the message API.
    pub fn messages(&self) -> crate::api::MessageApi<'_> {
        crate::api::MessageApi::new(self)
    }

    /// Get the memory API.
    pub fn memory(&self) -> crate::api::MemoryApi<'_> {
        crate::api::MemoryApi::new(self)
    }

    /// Get the source API.
    pub fn sources(&self) -> crate::api::SourceApi<'_> {
        crate::api::SourceApi::new(self)
    }

    /// Get the archive API.
    pub fn archives(&self) -> crate::api::ArchiveApi<'_> {
        crate::api::ArchiveApi::new(self)
    }

    /// Get the tool API.
    pub fn tools(&self) -> crate::api::ToolApi<'_> {
        crate::api::ToolApi::new(self)
    }

    /// Get the health API.
    pub fn health(&self) -> crate::api::HealthApi<'_> {
        crate::api::HealthApi::new(self)
    }

    /// Get the blocks API.
    pub fn blocks(&self) -> crate::api::BlocksApi<'_> {
        crate::api::BlocksApi::new(self)
    }

    /// Get the folders API.
    pub fn folders(&self) -> crate::api::FolderApi<'_> {
        crate::api::FolderApi::new(self)
    }

    /// Get the retry configuration.
    pub fn retry_config(&self) -> &RetryConfig {
        &self.retry_config
    }

    /// Set the retry configuration.
    pub fn set_retry_config(&mut self, config: RetryConfig) {
        self.retry_config = config;
    }

    fn max_response_bytes(&self) -> usize {
        self.config.max_response_bytes
    }

    async fn guarded_json<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
        url: &url::Url,
        method: &str,
    ) -> LettaResult<T> {
        let content_length = response.content_length();
        let max = self.max_response_bytes();

        if let Some(len) = content_length {
            if len as usize > max {
                return Err(LettaError::config(format!(
                    "Response too large: {} bytes (limit: {} bytes) for {} {}. \
                     Use pagination (limit/offset) to reduce payload size.",
                    len, max, method, url
                )));
            }
        }

        let bytes = response.bytes().await?;
        let size = bytes.len();

        if size > max {
            return Err(LettaError::config(format!(
                "Response too large: {} bytes (limit: {} bytes) for {} {}. \
                 Use pagination (limit/offset) to reduce payload size.",
                size, max, method, url
            )));
        }

        serde_json::from_slice(&bytes).map_err(|e| {
            tracing::error!(
                "Failed to decode {} {} response ({} bytes): {}",
                method,
                url,
                size,
                e
            );
            LettaError::config(format!(
                "Failed to decode response ({} bytes) for {} {}: {}",
                size, method, url, e
            ))
        })
    }

    // HTTP helper methods

    /// Send a GET request and deserialize the JSON response.
    pub async fn get<T>(&self, path: &str) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.get_internal(path).await
    }

    /// Internal GET implementation with retry logic.
    #[tracing::instrument(skip(self), fields(path = %path))]
    async fn get_internal<T>(&self, path: &str) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;

            tracing::debug!("Sending GET request to {}", url);
            let response = self.http().get(url.clone()).headers(headers).send().await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("GET".to_string()),
                ));
            }

            self.guarded_json(response, &url, "GET").await
        })
        .await
    }

    /// Make a POST request with a JSON body.
    #[tracing::instrument(skip(self, body), fields(path = %path))]
    pub async fn post<T, B>(&self, path: &str, body: &B) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let body_json = serde_json::to_value(body)?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;
            headers.insert(
                "Content-Type",
                "application/json"
                    .parse()
                    .map_err(|_| LettaError::config("Failed to parse Content-Type header"))?,
            );

            let response = self
                .http()
                .post(url.clone())
                .headers(headers)
                .json(&body_json)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("POST".to_string()),
                ));
            }

            self.guarded_json(response, &url, "POST").await
        })
        .await
    }

    /// Make a PATCH request with a JSON body.
    #[tracing::instrument(skip(self, body), fields(path = %path))]
    pub async fn patch<T, B>(&self, path: &str, body: &B) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let body_json = serde_json::to_value(body)?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;
            headers.insert(
                "Content-Type",
                "application/json"
                    .parse()
                    .map_err(|_| LettaError::config("Failed to parse Content-Type header"))?,
            );

            let response = self
                .http()
                .patch(url.clone())
                .headers(headers)
                .json(&body_json)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("PATCH".to_string()),
                ));
            }

            self.guarded_json(response, &url, "PATCH").await
        })
        .await
    }

    /// Make a PATCH request without a body.
    #[tracing::instrument(skip(self), fields(path = %path))]
    pub async fn patch_no_body<T>(&self, path: &str) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;

            let response = self
                .http()
                .patch(url.clone())
                .headers(headers)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("PATCH".to_string()),
                ));
            }

            self.guarded_json(response, &url, "PATCH").await
        })
        .await
    }

    /// Make a PUT request with a JSON body.
    #[tracing::instrument(skip(self, body), fields(path = %path))]
    pub async fn put<T, B>(&self, path: &str, body: &B) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let body_json = serde_json::to_value(body)?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;
            headers.insert(
                "Content-Type",
                "application/json"
                    .parse()
                    .map_err(|_| LettaError::config("Failed to parse Content-Type header"))?,
            );

            let response = self
                .http()
                .put(url.clone())
                .headers(headers)
                .json(&body_json)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("PUT".to_string()),
                ));
            }

            self.guarded_json(response, &url, "PUT").await
        })
        .await
    }

    /// Make a PUT request with custom headers.
    #[tracing::instrument(skip(self, body, extra_headers), fields(path = %path))]
    pub async fn put_with_headers<T, B>(
        &self,
        path: &str,
        body: &B,
        extra_headers: HeaderMap,
    ) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let body_json = serde_json::to_value(body)?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;
            headers.insert(
                "Content-Type",
                "application/json"
                    .parse()
                    .map_err(|_| LettaError::config("Failed to parse Content-Type header"))?,
            );

            // Add extra headers
            for (key, value) in extra_headers.iter() {
                headers.insert(key.clone(), value.clone());
            }

            let response = self
                .http()
                .put(url.clone())
                .headers(headers)
                .json(&body_json)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("PUT".to_string()),
                ));
            }

            self.guarded_json(response, &url, "PUT").await
        })
        .await
    }

    /// Send a DELETE request and deserialize the JSON response.
    #[tracing::instrument(skip(self), fields(path = %path))]
    pub async fn delete<T>(&self, path: &str) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;

            let response = self
                .http()
                .delete(url.clone())
                .headers(headers)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("DELETE".to_string()),
                ));
            }

            self.guarded_json(response, &url, "DELETE").await
        })
        .await
    }

    /// Send a DELETE request without expecting a response body.
    #[tracing::instrument(skip(self), fields(path = %path))]
    pub async fn delete_no_response(&self, path: &str) -> LettaResult<()> {
        let url = self.base_url().join(path.trim_start_matches('/'))?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;

            let response = self
                .http()
                .delete(url.clone())
                .headers(headers)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("DELETE".to_string()),
                ));
            }

            Ok(())
        })
        .await
    }

    /// Send a GET request with query parameters and deserialize the JSON response.
    #[tracing::instrument(skip(self, query), fields(path = %path))]
    pub async fn get_with_query<T, Q>(&self, path: &str, query: &Q) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        Q: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;

            let response = self
                .http()
                .get(url.clone())
                .headers(headers)
                .query(query)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("GET".to_string()),
                ));
            }

            self.guarded_json(response, &url, "GET").await
        })
        .await
    }

    /// Make a POST request with a JSON body **without retry**.
    ///
    /// Use this for non-idempotent mutations (e.g. sending messages) where retrying
    /// on transient failures would create duplicate side effects.
    #[tracing::instrument(skip(self, body), fields(path = %path))]
    pub async fn post_no_retry<T, B>(&self, path: &str, body: &B) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;

        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;
        headers.insert(
            "Content-Type",
            "application/json"
                .parse()
                .map_err(|_| LettaError::config("Failed to parse Content-Type header"))?,
        );

        let response = self
            .http()
            .post(url.clone())
            .headers(headers)
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let headers = response.headers().clone();
            let body = response.text().await?;
            return Err(LettaError::from_response_with_context(
                status,
                body,
                Some(&headers),
                Some(url.clone()),
                Some("POST".to_string()),
            ));
        }

        self.guarded_json(response, &url, "POST").await
    }

    /// Send a POST request with additional headers and deserialize the JSON response.
    #[tracing::instrument(skip(self, body, extra_headers), fields(path = %path))]
    pub async fn post_with_headers<T, B>(
        &self,
        path: &str,
        body: &B,
        extra_headers: HeaderMap,
    ) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let body_json = serde_json::to_value(body)?;

        retry_with_config(&self.retry_config, || async {
            let mut headers = HeaderMap::new();
            self.auth().apply_to_headers(&mut headers)?;
            headers.insert(
                "Content-Type",
                "application/json"
                    .parse()
                    .map_err(|_| LettaError::config("Failed to parse Content-Type header"))?,
            );

            for (key, value) in extra_headers.iter() {
                headers.insert(key.clone(), value.clone());
            }

            let response = self
                .http()
                .post(url.clone())
                .headers(headers)
                .json(&body_json)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let headers = response.headers().clone();
                let body = response.text().await?;
                return Err(LettaError::from_response_with_context(
                    status,
                    body,
                    Some(&headers),
                    Some(url.clone()),
                    Some("POST".to_string()),
                ));
            }

            self.guarded_json(response, &url, "POST").await
        })
        .await
    }

    /// Send a POST request with a multipart form body and deserialize the JSON response.
    #[tracing::instrument(skip(self, form), fields(path = %path))]
    pub async fn post_multipart<T>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;

        // Note: We can't retry multipart uploads easily since the form is consumed
        // For now, we'll do a single attempt. In the future, we could implement
        // a more sophisticated retry mechanism for multipart uploads.
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;

        let response = self
            .http()
            .post(url.clone())
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let headers = response.headers().clone();
            let body = response.text().await?;
            return Err(LettaError::from_response_with_context(
                status,
                body,
                Some(&headers),
                Some(url.clone()),
                Some("POST".to_string()),
            ));
        }

        self.guarded_json(response, &url, "POST").await
    }
}

/// Builder for creating a Letta client.
#[derive(Debug, Default)]
pub struct ClientBuilder {
    environment: Option<LettaEnvironment>,
    base_url: Option<String>,
    auth: Option<AuthConfig>,
    timeout: Option<Duration>,
    headers: Option<HeaderMap>,
}

impl ClientBuilder {
    /// Create a new client builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the environment (Cloud or SelfHosted).
    pub fn environment(mut self, env: LettaEnvironment) -> Self {
        self.environment = Some(env);
        self
    }

    /// Set the base URL (overrides environment).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the authentication.
    pub fn auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Set the timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set any custom headers.
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Add a single header to include with all requests.
    pub fn header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> LettaResult<Self> {
        let key = key.as_ref();
        let value = value.as_ref();

        let header_name = reqwest::header::HeaderName::from_bytes(key.as_bytes())
            .map_err(|_| LettaError::validation(format!("Invalid header name: {}", key)))?;
        let header_value = reqwest::header::HeaderValue::from_str(value).map_err(|_| {
            LettaError::validation(format!("Invalid header value for {}: {}", key, value))
        })?;

        let headers = self.headers.get_or_insert_with(HeaderMap::new);
        headers.insert(header_name, header_value);
        Ok(self)
    }

    /// Set the X-Project header for all requests.
    ///
    /// This associates all operations with a specific project context.
    pub fn project(self, project_id: impl AsRef<str>) -> LettaResult<Self> {
        self.header("X-Project", project_id)
    }

    /// Set the user-id header for all requests.
    ///
    /// This identifies the user making the requests.
    pub fn user_id(self, user_id: impl AsRef<str>) -> LettaResult<Self> {
        self.header("user-id", user_id)
    }

    /// Build the client.
    pub fn build(self) -> LettaResult<LettaClient> {
        // Check if we have an explicit base URL
        let has_explicit_url = self.base_url.is_some();

        // Determine base URL: explicit base_url takes precedence over environment
        let base_url = if let Some(url) = self.base_url {
            url
        } else {
            // Use environment or default to Cloud
            let env = self.environment.unwrap_or_default();
            env.base_url().to_string()
        };

        let mut config = ClientConfig::new(base_url)?;

        // Apply authentication
        if let Some(auth) = self.auth {
            config = config.auth(auth);
        } else if self.environment.unwrap_or_default().requires_auth() && !has_explicit_url {
            // Warn if using cloud environment without auth
            eprintln!("Warning: Cloud environment typically requires authentication");
        }

        if let Some(timeout) = self.timeout {
            config = config.timeout(timeout);
        }

        if let Some(headers) = self.headers {
            config = config.headers(headers);
        }

        LettaClient::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        assert_eq!(config.base_url.as_str(), "http://localhost:8283/");
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_client_builder() {
        let client = ClientBuilder::new()
            .base_url("http://localhost:8283")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        assert_eq!(client.base_url().as_str(), "http://localhost:8283/");
    }

    #[test]
    fn test_environment_based_client() {
        // Test cloud environment
        let client = ClientBuilder::new()
            .environment(LettaEnvironment::Cloud)
            .auth(AuthConfig::bearer("test-token"))
            .build()
            .unwrap();
        assert_eq!(client.base_url().as_str(), "https://api.letta.com/");

        // Test self-hosted environment
        let client = ClientBuilder::new()
            .environment(LettaEnvironment::SelfHosted)
            .build()
            .unwrap();
        assert_eq!(client.base_url().as_str(), "http://localhost:8283/");
    }

    #[test]
    fn test_convenience_constructors() {
        // Test cloud constructor
        let client = LettaClient::cloud("test-token").unwrap();
        assert_eq!(client.base_url().as_str(), "https://api.letta.com/");

        // Test local constructor
        let client = LettaClient::local().unwrap();
        assert_eq!(client.base_url().as_str(), "http://localhost:8283/");
    }

    #[test]
    fn test_base_url_overrides_environment() {
        // base_url should override environment setting
        let client = ClientBuilder::new()
            .environment(LettaEnvironment::Cloud)
            .base_url("http://custom.example.com")
            .build()
            .unwrap();
        assert_eq!(client.base_url().as_str(), "http://custom.example.com/");
    }

    #[test]
    fn test_header_configuration() -> LettaResult<()> {
        // Test adding headers via builder
        let _client = ClientBuilder::new()
            .base_url("http://localhost:8283")
            .header("user-id", "test-user-123")?
            .header("x-custom-header", "custom-value")?
            .build()?;

        // The headers are stored in the internal HTTP client, so we can't directly
        // verify them in this test. But we've verified the builder pattern works.

        // Test adding headers via ClientConfig
        let mut config = ClientConfig::new("http://localhost:8283")?;
        config = config.header("user-id", "test-user-456")?;
        let _client = LettaClient::new(config)?;

        Ok(())
    }

    #[test]
    fn test_header_helpers() -> LettaResult<()> {
        // Test project helper
        let _client = ClientBuilder::new()
            .base_url("http://localhost:8283")
            .project("my-project-123")?
            .build()?;

        // Test user_id helper
        let _client = ClientBuilder::new()
            .base_url("http://localhost:8283")
            .user_id("user-456")?
            .build()?;

        // Test both helpers together
        let _client = ClientBuilder::new()
            .base_url("http://localhost:8283")
            .project("project-789")?
            .user_id("user-789")?
            .build()?;

        // Test on ClientConfig
        let config = ClientConfig::new("http://localhost:8283")?
            .project("config-project")?
            .user_id("config-user")?;
        let _client = LettaClient::new(config)?;

        Ok(())
    }

    #[test]
    fn test_cloud_with_project() -> LettaResult<()> {
        let client = LettaClient::cloud_with_project("test-token", "project-123")?;
        assert_eq!(client.base_url().as_str(), "https://api.letta.com/");
        // Headers are configured internally
        Ok(())
    }
}
