use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,  // UUID as TEXT for cross-DB compatibility
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub tier: String,
    #[sqlx(try_from = "i64")]
    pub created_at: i64,  // Unix timestamp for cross-DB compatibility
    #[sqlx(try_from = "i64")]
    pub updated_at: i64,
    pub last_login_at: Option<i64>,
    pub is_active: bool,
    pub email_verified: bool,
}

impl User {
    /// Convert created_at timestamp to DateTime
    pub fn created_at_dt(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.created_at, 0).unwrap()
    }
    
    /// Convert updated_at timestamp to DateTime
    pub fn updated_at_dt(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.updated_at, 0).unwrap()
    }
    
    /// Convert last_login_at timestamp to DateTime
    pub fn last_login_at_dt(&self) -> Option<DateTime<Utc>> {
        self.last_login_at.map(|ts| Utc.timestamp_opt(ts, 0).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSession {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    #[sqlx(try_from = "i64")]
    pub expires_at: i64,
    #[sqlx(try_from = "i64")]
    pub created_at: i64,
    #[sqlx(try_from = "i64")]
    pub last_active_at: i64,
}

impl UserSession {
    pub fn expires_at_dt(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.expires_at, 0).unwrap()
    }
    
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        self.expires_at < now
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OAuthConnection {
    pub id: String,
    pub user_id: String,
    pub provider: String,
    pub provider_user_id: String,
    #[serde(skip_serializing)]
    pub access_token: Option<String>,
    #[serde(skip_serializing)]
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<i64>,
    #[sqlx(try_from = "i64")]
    pub created_at: i64,
    #[sqlx(try_from = "i64")]
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiKey {
    pub id: String,
    pub user_id: String,
    pub key_hash: String,
    pub name: String,
    pub last_used_at: Option<i64>,
    pub expires_at: Option<i64>,
    #[sqlx(try_from = "i64")]
    pub created_at: i64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPreferences {
    pub user_id: String,
    pub ai_provider: String,
    pub ai_model: Option<String>,
    pub quantum_provider: String,
    pub quantum_backend: Option<String>,
    pub ui_theme: String,
    pub preferences: serde_json::Value,
    #[sqlx(try_from = "i64")]
    pub created_at: i64,
    #[sqlx(try_from = "i64")]
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UsageRecord {
    pub id: String,
    pub user_id: String,
    pub resource_type: String,
    pub resource_count: i32,
    pub metadata: serde_json::Value,
    #[sqlx(try_from = "i64")]
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuantumJob {
    pub id: String,
    pub user_id: String,
    pub name: Option<String>,
    pub circuit_code: String,
    pub backend: Option<String>,
    pub status: String,
    pub result: Option<serde_json::Value>,
    pub error_message: Option<String>,
    #[sqlx(try_from = "i64")]
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
}

// DTOs for API requests
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
    pub expires_at: i64,  // Unix timestamp
}

impl AuthResponse {
    pub fn expires_at_dt(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.expires_at, 0).unwrap()
    }
}
