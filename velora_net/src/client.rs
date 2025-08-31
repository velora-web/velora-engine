//! HTTP client for the Velora web engine

use velora_core::{VeloraResult, HttpStatus};
use std::collections::HashMap;

/// HTTP client for making network requests
#[derive(Debug)]
pub struct HttpClient {
    /// Client configuration
    config: HttpClientConfig,
}

/// HTTP client configuration
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// User agent string
    pub user_agent: String,
    
    /// Request timeout in seconds
    pub timeout: u64,
    
    /// Maximum redirects
    pub max_redirects: u32,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            user_agent: "Velora Engine/1.0".to_string(),
            timeout: 30,
            max_redirects: 10,
        }
    }
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> VeloraResult<Self> {
        Ok(Self {
            config: HttpClientConfig::default(),
        })
    }
    
    /// Create a new HTTP client with custom configuration
    pub fn with_config(config: HttpClientConfig) -> VeloraResult<Self> {
        Ok(Self { config })
    }
    
    /// Make a GET request
    pub async fn get(&self, _url: &str) -> VeloraResult<HttpResponse> {
        // TODO: Implement GET request using config
        // For now, return a mock response
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), self.config.user_agent.clone());
        
        Ok(HttpResponse::new(HttpStatus::ok(), headers, Vec::new()))
    }
    
    /// Make a POST request
    pub async fn post(&self, _url: &str, _body: &[u8]) -> VeloraResult<HttpResponse> {
        // TODO: Implement POST request using config
        // For now, return a mock response
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), self.config.user_agent.clone());
        
        Ok(HttpResponse::new(HttpStatus::ok(), headers, Vec::new()))
    }
}

/// HTTP response
#[derive(Debug)]
pub struct HttpResponse {
    /// Response status
    pub status: HttpStatus,
    
    /// Response headers
    pub headers: HashMap<String, String>,
    
    /// Response body
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Create a new HTTP response
    pub fn new(status: HttpStatus, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
    
    /// Get a header value
    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).map(|s| s.as_str())
    }
    
    /// Get the response body as text
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_http_client_creation() {
        let client = HttpClient::new();
        assert!(client.is_ok());
        
        let client = client.unwrap();
        assert_eq!(client.config.user_agent, "Velora Engine/1.0");
        assert_eq!(client.config.timeout, 30);
        assert_eq!(client.config.max_redirects, 10);
    }
    
    #[tokio::test]
    async fn test_http_client_with_config() {
        let config = HttpClientConfig {
            user_agent: "Custom Agent/2.0".to_string(),
            timeout: 60,
            max_redirects: 5,
        };
        
        let client = HttpClient::with_config(config);
        assert!(client.is_ok());
        
        let client = client.unwrap();
        assert_eq!(client.config.user_agent, "Custom Agent/2.0");
        assert_eq!(client.config.timeout, 60);
        assert_eq!(client.config.max_redirects, 5);
    }
    
    #[tokio::test]
    async fn test_get_request() {
        let client = HttpClient::new().unwrap();
        let response = client.get("https://example.com").await;
        assert!(response.is_ok());
        
        let response = response.unwrap();
        assert_eq!(response.status.code, 200);
        assert!(response.get_header("User-Agent").is_some());
    }
    
    #[tokio::test]
    async fn test_post_request() {
        let client = HttpClient::new().unwrap();
        let body = b"Hello, World!";
        let response = client.post("https://example.com", body).await;
        assert!(response.is_ok());
        
        let response = response.unwrap();
        assert_eq!(response.status.code, 200);
        assert!(response.get_header("User-Agent").is_some());
    }
    
    #[test]
    fn test_http_response() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        let body = b"Hello, World!".to_vec();
        
        let response = HttpResponse::new(HttpStatus::ok(), headers, body.clone());
        assert_eq!(response.status.code, 200);
        assert_eq!(response.get_header("Content-Type"), Some("text/plain"));
        assert_eq!(response.body, body);
        
        let text = response.text();
        assert!(text.is_ok());
        assert_eq!(text.unwrap(), "Hello, World!");
    }
}
