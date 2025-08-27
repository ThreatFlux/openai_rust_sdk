//! Stream operation functions for collecting and processing streams

use crate::error::Result;
use futures::Stream;
use futures::StreamExt as FuturesStreamExt;
use std::pin::Pin;

use super::types::ResponseStream;

/// Helper trait for streaming operations
pub trait ResponseStreamExt {
    /// Collect content from stream chunks
    fn collect_content(self) -> Pin<Box<dyn futures::Future<Output = Result<String>> + Send>>;
}

impl ResponseStreamExt for ResponseStream {
    fn collect_content(self) -> Pin<Box<dyn futures::Future<Output = Result<String>> + Send>> {
        Box::pin(collect_stream_response(self))
    }
}

/// Collect all chunks from a stream into a single response
pub async fn collect_stream_response(mut stream: ResponseStream) -> Result<String> {
    let mut content = String::new();

    while let Some(chunk_result) = FuturesStreamExt::next(&mut stream).await {
        let chunk = chunk_result?;

        for choice in chunk.choices {
            if let Some(delta_content) = &choice.delta.content {
                content.push_str(delta_content);
            }

            // Check if we're done
            if choice.finish_reason.is_some() {
                break;
            }
        }
    }

    Ok(content)
}
