//! HTTP client utilities (requires `http` feature).
//!
//! This module provides a simple HTTP client wrapper with
//! sensible defaults for common operations.

use crate::error::{FoundationError, Result};
use std::path::Path;
use std::time::Duration;

/// HTTP client with configurable timeout and retry logic.
///
/// # Example
/// ```no_run
/// use rh_foundation::http::HttpClient;
///
/// # async fn example() -> rh_foundation::Result<()> {
/// let client = HttpClient::new()?;
/// let data = client.download("https://example.com/data.json").await?;
/// # Ok(())
/// # }
/// ```
pub struct HttpClient {
    client: reqwest::Client,
    timeout: Duration,
}

impl HttpClient {
    /// Create a new HTTP client with default settings (30 second timeout).
    pub fn new() -> Result<Self> {
        Self::with_timeout(Duration::from_secs(30))
    }

    /// Create a new HTTP client with a custom timeout.
    pub fn with_timeout(timeout: Duration) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| {
                FoundationError::Other(anyhow::anyhow!("Failed to build HTTP client: {e}"))
            })?;

        Ok(Self { client, timeout })
    }

    /// Get the configured timeout.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Download content from a URL as bytes.
    pub async fn download(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.client.get(url).send().await.map_err(|e| {
            FoundationError::Other(anyhow::anyhow!("HTTP request failed for {url}: {e}"))
        })?;

        if !response.status().is_success() {
            return Err(FoundationError::Other(anyhow::anyhow!(
                "HTTP request to {url} failed with status: {}",
                response.status()
            )));
        }

        response.bytes().await.map(|b| b.to_vec()).map_err(|e| {
            FoundationError::Other(anyhow::anyhow!("Failed to read response body: {e}"))
        })
    }

    /// Download content from a URL as a string.
    pub async fn download_text(&self, url: &str) -> Result<String> {
        let bytes = self.download(url).await?;
        String::from_utf8(bytes)
            .map_err(|e| FoundationError::Parse(format!("Response body is not valid UTF-8: {e}")))
    }

    /// Download content from a URL and parse it as JSON.
    pub async fn download_json<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let bytes = self.download(url).await?;
        serde_json::from_slice(&bytes).map_err(Into::into)
    }

    /// Download content from a URL and save it to a file.
    pub async fn download_to_file(&self, url: &str, path: impl AsRef<Path>) -> Result<()> {
        let bytes = self.download(url).await?;
        let path = path.as_ref();
        std::fs::write(path, bytes).map_err(|e| {
            FoundationError::Io(std::io::Error::new(
                e.kind(),
                format!(
                    "Failed to write downloaded content to {}: {}",
                    path.display(),
                    e
                ),
            ))
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_client_creation() {
        let client = HttpClient::new().unwrap();
        assert_eq!(client.timeout(), Duration::from_secs(30));
    }

    #[tokio::test]
    async fn test_custom_timeout() {
        let client = HttpClient::with_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(client.timeout(), Duration::from_secs(10));
    }
}
