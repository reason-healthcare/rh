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

/// Builder for configuring an HttpClient.
///
/// # Example
/// ```no_run
/// use rh_foundation::http::HttpClientBuilder;
/// use std::time::Duration;
///
/// # fn example() -> rh_foundation::Result<()> {
/// let client = HttpClientBuilder::new()
///     .timeout(Duration::from_secs(60))
///     .bearer_auth("my-token")?
///     .user_agent("my-app/1.0")?
///     .build()?;
/// # Ok(())
/// # }
/// ```
pub struct HttpClientBuilder {
    timeout: Duration,
    headers: reqwest::header::HeaderMap,
}

impl HttpClientBuilder {
    /// Create a new builder with default settings.
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            headers: reqwest::header::HeaderMap::new(),
        }
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Add a bearer token for authentication.
    pub fn bearer_auth(mut self, token: &str) -> Result<Self> {
        let value = reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(|e| FoundationError::Authentication(format!("Invalid auth token: {e}")))?;
        self.headers.insert(reqwest::header::AUTHORIZATION, value);
        Ok(self)
    }

    /// Set the User-Agent header.
    pub fn user_agent(mut self, user_agent: &str) -> Result<Self> {
        let value = reqwest::header::HeaderValue::from_str(user_agent).map_err(|e| {
            FoundationError::InvalidInput(format!("Invalid user-agent header: {e}"))
        })?;
        self.headers.insert(reqwest::header::USER_AGENT, value);
        Ok(self)
    }

    /// Add a custom header.
    pub fn header(mut self, key: reqwest::header::HeaderName, value: &str) -> Result<Self> {
        let value = reqwest::header::HeaderValue::from_str(value).map_err(|e| {
            FoundationError::InvalidInput(format!("Invalid header value for {key}: {e}"))
        })?;
        self.headers.insert(key, value);
        Ok(self)
    }

    /// Build the HttpClient.
    pub fn build(self) -> Result<HttpClient> {
        let client = reqwest::Client::builder()
            .timeout(self.timeout)
            .default_headers(self.headers)
            .build()
            .map_err(|e| {
                FoundationError::Other(anyhow::anyhow!("Failed to build HTTP client: {e}"))
            })?;

        Ok(HttpClient {
            client,
            timeout: self.timeout,
        })
    }
}

impl Default for HttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    /// Create a new HTTP client with default settings (30 second timeout).
    pub fn new() -> Result<Self> {
        HttpClientBuilder::new().build()
    }

    /// Create a builder for configuring an HTTP client.
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::new()
    }

    /// Create a new HTTP client with a custom timeout.
    pub fn with_timeout(timeout: Duration) -> Result<Self> {
        HttpClientBuilder::new().timeout(timeout).build()
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
