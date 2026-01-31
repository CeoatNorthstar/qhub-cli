use anyhow::{Context, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{AuthResponse, CreateUserRequest, LoginRequest, User, UserSession};

const TOKEN_EXPIRY_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // User ID
    pub email: String,
    pub tier: String,
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
}

pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Result<Self> {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "development-secret-key-change-in-production".to_string());
        
        if jwt_secret == "development-secret-key-change-in-production" {
            eprintln!("WARNING: Using default JWT secret. Set JWT_SECRET in production!");
        }

        Ok(Self { pool, jwt_secret })
    }

    /// Hash a password using Argon2
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
            .to_string();

        Ok(password_hash)
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Generate a JWT token
    pub fn generate_token(&self, user: &User) -> Result<(String, i64)> {
        let expiry_hours = std::env::var("TOKEN_EXPIRY_HOURS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(TOKEN_EXPIRY_HOURS);

        let now = Utc::now();
        let exp = (now + Duration::hours(expiry_hours)).timestamp();

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            tier: user.tier.clone(),
            exp,
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .context("Failed to generate JWT token")?;

        Ok((token, exp))
    }

    /// Verify and decode a JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .context("Invalid or expired token")?;

        Ok(token_data.claims)
    }

    /// Hash a token for storage
    pub fn hash_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        general_purpose::STANDARD.encode(hasher.finalize())
    }

    /// Register a new user
    pub async fn register(&self, req: CreateUserRequest) -> Result<AuthResponse> {
        // Validate email format
        if !req.email.contains('@') {
            anyhow::bail!("Invalid email format");
        }

        // Check if user already exists
        let existing = sqlx::query!("SELECT id FROM users WHERE email = $1", req.email)
            .fetch_optional(&self.pool)
            .await?;

        if existing.is_some() {
            anyhow::bail!("Email already registered");
        }

        // Hash password
        let password_hash = self.hash_password(&req.password)?;

        // Create user
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO users (id, email, username, password_hash, tier, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            user_id,
            req.email,
            req.username,
            password_hash,
            "free",
            now,
            now
        )
        .execute(&self.pool)
        .await
        .context("Failed to create user")?;

        // Fetch the created user
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, display_name, password_hash,
                   tier, created_at, updated_at, last_login_at,
                   is_active, email_verified
            FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Generate token
        let (token, exp) = self.generate_token(&user)?;
        let token_hash = self.hash_token(&token);

        // Create session
        let session_id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO user_sessions (id, user_id, token_hash, expires_at, created_at, last_active_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            session_id,
            user.id,
            token_hash,
            chrono::DateTime::from_timestamp(exp, 0),
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(AuthResponse {
            token,
            user,
            expires_at: chrono::DateTime::from_timestamp(exp, 0).unwrap(),
        })
    }

    /// Login a user
    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse> {
        // Fetch user
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, display_name, password_hash,
                   tier, created_at, updated_at, last_login_at,
                   is_active, email_verified
            FROM users WHERE email = $1
            "#,
            req.email
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;

        // Check if user is active
        if !user.is_active {
            anyhow::bail!("Account is deactivated");
        }

        // Verify password
        let password_hash = user.password_hash.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;

        if !self.verify_password(&req.password, password_hash)? {
            anyhow::bail!("Invalid email or password");
        }

        // Update last login
        sqlx::query!("UPDATE users SET last_login_at = $1 WHERE id = $2", Utc::now(), user.id)
            .execute(&self.pool)
            .await?;

        // Generate token
        let (token, exp) = self.generate_token(&user)?;
        let token_hash = self.hash_token(&token);

        // Create session
        let session_id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO user_sessions (id, user_id, token_hash, expires_at, created_at, last_active_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            session_id,
            user.id,
            token_hash,
            chrono::DateTime::from_timestamp(exp, 0),
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(AuthResponse {
            token,
            user,
            expires_at: chrono::DateTime::from_timestamp(exp, 0).unwrap(),
        })
    }

    /// Verify a session token
    pub async fn verify_session(&self, token: &str) -> Result<User> {
        let claims = self.verify_token(token)?;
        let token_hash = self.hash_token(token);

        // Check if session exists and is valid
        let session = sqlx::query_as!(
            UserSession,
            r#"
            SELECT id, user_id, token_hash, device_info, ip_address,
                   expires_at, created_at, last_active_at
            FROM user_sessions WHERE token_hash = $1 AND expires_at > $2
            "#,
            token_hash,
            Utc::now()
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid or expired session"))?;

        // Update last active
        sqlx::query!(
            "UPDATE user_sessions SET last_active_at = $1 WHERE id = $2",
            Utc::now(),
            session.id
        )
        .execute(&self.pool)
        .await?;

        // Fetch user
        let user_id = Uuid::parse_str(&claims.sub)?;
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, display_name, password_hash,
                   tier, created_at, updated_at, last_login_at,
                   is_active, email_verified
            FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Logout (invalidate session)
    pub async fn logout(&self, token: &str) -> Result<()> {
        let token_hash = self.hash_token(token);
        
        sqlx::query!("DELETE FROM user_sessions WHERE token_hash = $1", token_hash)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<u64> {
        let result = sqlx::query!("DELETE FROM user_sessions WHERE expires_at < $1", Utc::now())
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
