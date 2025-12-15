//! Pagination utilities for working with Letta API list endpoints.

use crate::error::LettaResult;
use crate::types::{LettaId, PaginationParams};
use futures::stream::{self, Stream, StreamExt};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// A paginated stream that automatically fetches pages as needed.
///
/// This provides a convenient way to iterate through paginated results
/// without manually managing cursors.
pub struct PaginatedStream<T> {
    /// The inner stream of items
    inner: Pin<Box<dyn Stream<Item = LettaResult<T>> + Send>>,
}

impl<T> PaginatedStream<T> {
    /// Create a new paginated stream from a fetch function.
    ///
    /// The fetch function takes optional pagination parameters and returns
    /// a future that resolves to a vector of items.
    pub fn new<F, Fut>(initial_params: Option<PaginationParams>, fetch_fn: F) -> Self
    where
        F: Fn(Option<PaginationParams>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = LettaResult<Vec<T>>> + Send + 'static,
        T: Send + 'static,
    {
        let inner = create_paginated_stream(initial_params, fetch_fn);
        Self {
            inner: Box::pin(inner),
        }
    }

    /// Create a paginated stream that fetches items by ID-based cursor.
    ///
    /// This is for endpoints where items have an ID field that can be used
    /// as a cursor (most Letta endpoints work this way).
    pub fn new_with_id_cursor<F, Fut, I>(
        initial_params: Option<PaginationParams>,
        fetch_fn: F,
        get_id: I,
    ) -> Self
    where
        F: Fn(Option<PaginationParams>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = LettaResult<Vec<T>>> + Send + 'static,
        I: Fn(&T) -> &LettaId + Send + Sync + 'static,
        T: Send + 'static,
    {
        let inner = create_id_based_paginated_stream(initial_params, fetch_fn, get_id);
        Self {
            inner: Box::pin(inner),
        }
    }

    /// Create a paginated stream that uses string values as cursors.
    ///
    /// This is for endpoints where items don't have IDs but can use
    /// string values as cursors (like tags).
    pub fn new_with_string_cursor<F, Fut, S>(
        initial_params: Option<PaginationParams>,
        fetch_fn: F,
        get_cursor: S,
    ) -> Self
    where
        F: Fn(Option<PaginationParams>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = LettaResult<Vec<T>>> + Send + 'static,
        S: Fn(&T) -> String + Send + Sync + 'static,
        T: Send + 'static,
    {
        let inner = create_string_based_paginated_stream(initial_params, fetch_fn, get_cursor);
        Self {
            inner: Box::pin(inner),
        }
    }

    /// Collect all items into a vector.
    ///
    /// This will fetch all pages until exhausted.
    pub async fn collect(self) -> LettaResult<Vec<T>> {
        self.inner.collect::<Vec<_>>().await.into_iter().collect()
    }

    /// Take up to n items from the stream.
    pub fn take(self, n: usize) -> impl Stream<Item = LettaResult<T>> {
        self.inner.take(n)
    }

    /// Filter items using a predicate.
    pub fn filter<P>(self, predicate: P) -> impl Stream<Item = LettaResult<T>>
    where
        P: Fn(&T) -> bool + Send + Clone + 'static,
    {
        self.inner.filter_map(move |result| {
            let predicate = predicate.clone();
            async move {
                match result {
                    Ok(item) => {
                        if predicate(&item) {
                            Some(Ok(item))
                        } else {
                            None
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            }
        })
    }

    /// Map items to a different type.
    pub fn map<U, F>(self, f: F) -> impl Stream<Item = LettaResult<U>>
    where
        F: Fn(T) -> U + Send,
        U: Send,
    {
        self.inner.map(move |result| result.map(&f))
    }
}

impl<T> Stream for PaginatedStream<T> {
    type Item = LettaResult<T>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}

/// Create a paginated stream that fetches pages based on item count.
///
/// This is for basic pagination where we just keep fetching until we get
/// fewer items than the limit.
fn create_paginated_stream<T, F, Fut>(
    initial_params: Option<PaginationParams>,
    fetch_fn: F,
) -> impl Stream<Item = LettaResult<T>>
where
    F: Fn(Option<PaginationParams>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = LettaResult<Vec<T>>> + Send,
    T: Send + 'static,
{
    // Default limit if not specified
    let limit = initial_params.as_ref().and_then(|p| p.limit).unwrap_or(100);

    let fetch_fn = Arc::new(fetch_fn);

    stream::unfold(
        Some(initial_params.unwrap_or_default()),
        move |params_opt| {
            let fetch_fn = Arc::clone(&fetch_fn);
            async move {
                let params = params_opt?;

                match fetch_fn(Some(params.clone())).await {
                    Ok(items) => {
                        let item_count = items.len();

                        // If we got fewer items than the limit, we're done
                        let has_more = item_count as u32 >= limit;

                        // Convert to boxed stream to unify types
                        let items_stream: Pin<Box<dyn Stream<Item = LettaResult<T>> + Send>> =
                            Box::pin(stream::iter(items.into_iter().map(Ok)));

                        // Only continue if there might be more pages
                        let next_state = if has_more {
                            // For basic pagination, we can't create a meaningful cursor
                            // so we just stop after one page
                            None
                        } else {
                            None
                        };

                        Some((items_stream, next_state))
                    }
                    Err(e) => {
                        // Return error and stop pagination
                        let error_stream: Pin<Box<dyn Stream<Item = LettaResult<T>> + Send>> =
                            Box::pin(stream::once(async move { Err(e) }));
                        Some((error_stream, None))
                    }
                }
            }
        },
    )
    .flatten()
}

/// Create a paginated stream that uses item IDs as cursors.
///
/// This is for endpoints where items have an ID that can be used as
/// the "after" cursor for the next page.
fn create_id_based_paginated_stream<T, F, Fut, I>(
    initial_params: Option<PaginationParams>,
    fetch_fn: F,
    get_id: I,
) -> impl Stream<Item = LettaResult<T>>
where
    F: Fn(Option<PaginationParams>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = LettaResult<Vec<T>>> + Send,
    I: Fn(&T) -> &LettaId + Send + Sync + 'static,
    T: Send + 'static,
{
    let limit = initial_params.as_ref().and_then(|p| p.limit).unwrap_or(100);

    let fetch_fn = Arc::new(fetch_fn);
    let get_id = Arc::new(get_id);

    stream::unfold(
        Some(initial_params.unwrap_or_default()),
        move |params_opt| {
            let fetch_fn = Arc::clone(&fetch_fn);
            let get_id = Arc::clone(&get_id);
            async move {
                let mut params = params_opt?;

                match fetch_fn(Some(params.clone())).await {
                    Ok(items) => {
                        let item_count = items.len();

                        // Get the ID of the last item to use as cursor
                        let last_id = items.last().map(|item| get_id(item).to_string());

                        // If we got fewer items than the limit, we're done
                        let has_more = item_count as u32 >= limit && last_id.is_some();

                        // Convert to boxed stream to unify types
                        let items_stream: Pin<Box<dyn Stream<Item = LettaResult<T>> + Send>> =
                            Box::pin(stream::iter(items.into_iter().map(Ok)));

                        // Set up params for next page
                        let next_state = if has_more {
                            params.after = last_id;
                            params.before = None; // Clear before when using after
                            Some(params)
                        } else {
                            None
                        };

                        Some((items_stream, next_state))
                    }
                    Err(e) => {
                        // Return error and stop pagination
                        let error_stream: Pin<Box<dyn Stream<Item = LettaResult<T>> + Send>> =
                            Box::pin(stream::once(async move { Err(e) }));
                        Some((error_stream, None))
                    }
                }
            }
        },
    )
    .flatten()
}

/// Create a paginated stream that uses string values as cursors.
///
/// This is for endpoints where items are strings that can be used as cursors.
fn create_string_based_paginated_stream<T, F, Fut, S>(
    initial_params: Option<PaginationParams>,
    fetch_fn: F,
    get_cursor: S,
) -> impl Stream<Item = LettaResult<T>>
where
    F: Fn(Option<PaginationParams>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = LettaResult<Vec<T>>> + Send,
    S: Fn(&T) -> String + Send + Sync + 'static,
    T: Send + 'static,
{
    let limit = initial_params.as_ref().and_then(|p| p.limit).unwrap_or(100);

    let fetch_fn = Arc::new(fetch_fn);
    let get_cursor = Arc::new(get_cursor);

    stream::unfold(
        Some(initial_params.unwrap_or_default()),
        move |params_opt| {
            let fetch_fn = Arc::clone(&fetch_fn);
            let get_cursor = Arc::clone(&get_cursor);
            async move {
                let params = params_opt?;

                match fetch_fn(Some(params.clone())).await {
                    Ok(items) => {
                        let item_count = items.len();

                        // Get the last item's cursor value for the next page
                        let last_cursor = items.last().map(|item| get_cursor(item));

                        // Convert to boxed stream to unify types
                        let items_stream: Pin<Box<dyn Stream<Item = LettaResult<T>> + Send>> =
                            Box::pin(stream::iter(items.into_iter().map(Ok)));

                        // Continue if we got a full page and have a cursor
                        let next_state = if item_count as u32 >= limit && last_cursor.is_some() {
                            let mut params = params.clone();
                            params.after = last_cursor;
                            params.before = None; // Clear before when using after
                            Some(params)
                        } else {
                            None
                        };

                        Some((items_stream, next_state))
                    }
                    Err(e) => {
                        // Return error and stop pagination
                        let error_stream: Pin<Box<dyn Stream<Item = LettaResult<T>> + Send>> =
                            Box::pin(stream::once(async move { Err(e) }));
                        Some((error_stream, None))
                    }
                }
            }
        },
    )
    .flatten()
}

/// Extension trait to add pagination methods to API clients.
pub trait PaginationExt {
    /// The item type returned by list operations.
    type Item;

    /// Create a paginated stream for this API.buil
    fn paginated(&self, params: Option<PaginationParams>) -> PaginatedStream<Self::Item>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LettaId;
    use std::str::FromStr;

    #[derive(Debug, Clone)]
    struct TestItem {
        #[allow(dead_code)]
        id: LettaId,
        name: String,
    }

    #[tokio::test]
    async fn test_paginated_stream_basic() {
        let items = vec![
            TestItem {
                id: LettaId::from_str("test-00000000-0000-0000-0000-000000000001").unwrap(),
                name: "Item 1".to_string(),
            },
            TestItem {
                id: LettaId::from_str("test-00000000-0000-0000-0000-000000000002").unwrap(),
                name: "Item 2".to_string(),
            },
        ];

        let fetch_fn = move |_params| {
            let items = items.clone();
            async move { Ok(items) }
        };

        let stream = PaginatedStream::new(None, fetch_fn);
        let collected: Vec<TestItem> = stream.collect().await.unwrap();

        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].name, "Item 1");
        assert_eq!(collected[1].name, "Item 2");
    }

    #[tokio::test]
    async fn test_paginated_stream_with_filter() {
        let items = vec![
            TestItem {
                id: LettaId::from_str("test-00000000-0000-0000-0000-000000000001").unwrap(),
                name: "Item 1".to_string(),
            },
            TestItem {
                id: LettaId::from_str("test-00000000-0000-0000-0000-000000000002").unwrap(),
                name: "Item 2".to_string(),
            },
            TestItem {
                id: LettaId::from_str("test-00000000-0000-0000-0000-000000000003").unwrap(),
                name: "Skip Me".to_string(),
            },
        ];

        let fetch_fn = move |_params| {
            let items = items.clone();
            async move { Ok(items) }
        };

        let stream =
            PaginatedStream::new(None, fetch_fn).filter(|item| item.name.starts_with("Item"));

        let collected: Vec<TestItem> = stream
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(collected.len(), 2);
        assert!(collected.iter().all(|item| item.name.starts_with("Item")));
    }

    #[tokio::test]
    async fn test_paginated_stream_with_take() {
        let items = vec![
            TestItem {
                id: LettaId::from_str("test-00000000-0000-0000-0000-000000000001").unwrap(),
                name: "Item 1".to_string(),
            },
            TestItem {
                id: LettaId::from_str("test-00000000-0000-0000-0000-000000000002").unwrap(),
                name: "Item 2".to_string(),
            },
            TestItem {
                id: LettaId::from_str("test-00000000-0000-0000-0000-000000000003").unwrap(),
                name: "Item 3".to_string(),
            },
        ];

        let fetch_fn = move |_params| {
            let items = items.clone();
            async move { Ok(items) }
        };

        let stream = PaginatedStream::new(None, fetch_fn).take(2);

        let collected: Vec<TestItem> = stream
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(collected.len(), 2);
    }
}
