//! Server-sent events (SSE) streaming support.
//!
//! This module provides utilities for handling streaming responses from the Letta API,
//! particularly for real-time messaging features.

use crate::error::LettaResult;
use futures::Stream;
use std::pin::Pin;

/// Type alias for a streaming response.
pub type StreamingResponse<T> = Pin<Box<dyn Stream<Item = LettaResult<T>> + Send>>;

/// SSE event from the Letta API.
#[derive(Debug, Clone)]
pub struct SseEvent {
    /// Event type.
    pub event_type: Option<String>,
    /// Event data.
    pub data: String,
    /// Event ID.
    pub id: Option<String>,
}

/// Parse SSE events from a response stream.
pub async fn parse_sse_stream(
    _response: reqwest::Response,
) -> LettaResult<StreamingResponse<SseEvent>> {
    // Implementation will be added when we implement streaming
    todo!("SSE parsing implementation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sse_event_creation() {
        let event = SseEvent {
            event_type: Some("message".to_string()),
            data: "test data".to_string(),
            id: Some("123".to_string()),
        };

        assert_eq!(event.event_type.as_deref(), Some("message"));
        assert_eq!(event.data, "test data");
        assert_eq!(event.id.as_deref(), Some("123"));
    }
}
