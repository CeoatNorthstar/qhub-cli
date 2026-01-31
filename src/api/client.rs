use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// API client errors
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Authentication failed: {0}")]
    Unauthorized(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Standard API error response
#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}

/// Authentication request/response types
#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
    pub expires_at: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub tier: String,
}

/// AI chat request/response types
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub message: String,
    pub conversation_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub response: String,
    pub conversation_id: String,
    pub tokens_used: i32,
}

/// API health check response
#[derive(Debug, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

/// Main API client with enterprise features
#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    /// Create a new API client with enterprise defaults
    pub fn new(base_url: String) -> Result<Self, ApiError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(90))
            .tcp_keepalive(Duration::from_secs(60))
            .build()?;
        
        Ok(Self {
            client,
            base_url,
            token: None,
        })
    }
    
    /// Set authentication token
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
    
    /// Clear authentication token
    pub fn clear_token(&mut self) {
        self.token = None;
    }
    
    /// Build full URL from endpoint
    fn url(&self, endpoint: &str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }
    
    /// Handle API response with proper error mapping
    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: Response,
    ) -> Result<T, ApiError> {
        let status = response.status();
        
        match status {
            StatusCode::OK | StatusCode::CREATED => {
                let data = response.json::<T>().await?;
                Ok(data)
            }
            StatusCode::BAD_REQUEST => {
                let err = response.json::<ErrorResponse>().await
                    .unwrap_or_else(|_| ErrorResponse { error: "Bad request".to_string() });
                Err(ApiError::Validation(err.error))
            }
            StatusCode::UNAUTHORIZED => {
                let err = response.json::<ErrorResponse>().await
                    .unwrap_or_else(|_| ErrorResponse { error: "Unauthorized".to_string() });
                Err(ApiError::Unauthorized(err.error))
            }
            StatusCode::NOT_FOUND => {
                let err = response.json::<ErrorResponse>().await
                    .unwrap_or_else(|_| ErrorResponse { error: "Not found".to_string() });
                Err(ApiError::NotFound(err.error))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                Err(ApiError::RateLimit)
            }
            StatusCode::INTERNAL_SERVER_ERROR | StatusCode::BAD_GATEWAY | StatusCode::SERVICE_UNAVAILABLE => {
                let err = response.json::<ErrorResponse>().await
                    .unwrap_or_else(|_| ErrorResponse { error: "Server error".to_string() });
                Err(ApiError::ServerError(err.error))
            }
            _ => {
                let err = response.json::<ErrorResponse>().await
                    .unwrap_or_else(|_| ErrorResponse { 
                        error: format!("Unexpected status: {}", status) 
                    });
                Err(ApiError::Unknown(err.error))
            }
        }
    }
    
    /// Health check endpoint
    pub async fn health(&self) -> Result<HealthResponse, ApiError> {
        let response = self.client
            .get(self.url("/health"))
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Register a new user account
    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, ApiError> {
        let response = self.client
            .post(self.url("/auth/register"))
            .json(&req)
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Login to existing account
    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, ApiError> {
        let response = self.client
            .post(self.url("/auth/login"))
            .json(&req)
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Logout (invalidate session)
    pub async fn logout(&self) -> Result<(), ApiError> {
        let token = self.token.as_ref()
            .ok_or_else(|| ApiError::Unauthorized("No token set".to_string()))?;
        
        let response = self.client
            .post(self.url("/auth/logout"))
            .bearer_auth(token)
            .send()
            .await?;
        
        match response.status() {
            StatusCode::OK => Ok(()),
            _ => {
                let err: Result<ErrorResponse, _> = response.json().await;
                match err {
                    Ok(e) => Err(ApiError::ServerError(e.error)),
                    Err(_) => Err(ApiError::ServerError("Logout failed".to_string())),
                }
            }
        }
    }
    
    /// Verify token and get user info
    pub async fn verify_token(&self) -> Result<User, ApiError> {
        let token = self.token.as_ref()
            .ok_or_else(|| ApiError::Unauthorized("No token set".to_string()))?;
        
        let response = self.client
            .get(self.url("/auth/verify"))
            .bearer_auth(token)
            .send()
            .await?;
        
        #[derive(Deserialize)]
        struct VerifyResponse {
            user: User,
        }
        
        let verify_resp: VerifyResponse = self.handle_response(response).await?;
        Ok(verify_resp.user)
    }
    
    /// Send AI chat message
    pub async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, ApiError> {
        let token = self.token.as_ref()
            .ok_or_else(|| ApiError::Unauthorized("No token set".to_string()))?;
        
        let response = self.client
            .post(self.url("/ai/chat"))
            .bearer_auth(token)
            .json(&req)
            .send()
            .await?;
        
        self.handle_response(response).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_client_creation() {
        let client = ApiClient::new("http://localhost:8787".to_string());
        assert!(client.is_ok());
    }
    
    #[test]
    fn test_url_building() {
        let client = ApiClient::new("http://localhost:8787".to_string()).unwrap();
        assert_eq!(client.url("/health"), "http://localhost:8787/health");
        assert_eq!(client.url("/auth/login"), "http://localhost:8787/auth/login");
    }
}
