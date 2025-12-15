//! Tags API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::pagination::PaginatedStream;
use crate::types::{ListTagsParams, PaginationParams};

/// Tags API operations.
#[derive(Debug)]
pub struct TagsApi<'a> {
    client: &'a LettaClient,
}

impl<'a> TagsApi<'a> {
    /// Create a new tags API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all tags in the database.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<ListTagsParams>) -> LettaResult<Vec<String>> {
        let mut query_params = Vec::new();

        if let Some(params) = params {
            if let Some(after) = params.after {
                query_params.push(("after", after));
            }
            if let Some(limit) = params.limit {
                query_params.push(("limit", limit.to_string()));
            }
            if let Some(query_text) = params.query_text {
                query_params.push(("queryText", query_text));
            }
        }

        if query_params.is_empty() {
            self.client.get("v1/tags").await
        } else {
            self.client.get_with_query("v1/tags", &query_params).await
        }
    }

    /// Get a paginated stream of tags.
    ///
    /// This method returns a [`PaginatedStream`] that automatically handles pagination
    /// and allows streaming through all tags using async iteration.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional pagination parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::client::{ClientConfig, LettaClient};
    /// # use letta::{types::PaginationParams, pagination::PaginationExt};
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    ///
    /// let mut stream = client.tags().paginated(None);
    /// while let Some(tag) = stream.next().await {
    ///     println!("Tag: {}", tag?);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn paginated(&self, params: Option<PaginationParams>) -> PaginatedStream<String> {
        let client = self.client.clone();
        let fetch_fn = move |params: Option<PaginationParams>| {
            let client = client.clone();
            async move {
                let mut query_params = Vec::new();

                if let Some(params) = params {
                    if let Some(after) = params.after {
                        query_params.push(("after", after));
                    }
                    if let Some(limit) = params.limit {
                        query_params.push(("limit", limit.to_string()));
                    }
                }

                if query_params.is_empty() {
                    client.get("v1/tags").await
                } else {
                    client.get_with_query("v1/tags", &query_params).await
                }
            }
        };

        PaginatedStream::new_with_string_cursor(params, fetch_fn, |tag: &String| tag.clone())
    }
}

/// Convenience method for tags operations.
impl LettaClient {
    /// Get the tags API for this client.
    pub fn tags(&self) -> TagsApi {
        TagsApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_tags_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = TagsApi::new(&client);
    }
}
