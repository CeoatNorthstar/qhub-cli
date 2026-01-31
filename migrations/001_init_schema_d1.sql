-- QHub Database Schema for Cloudflare D1
-- Unified schema compatible with PostgreSQL dev environment
-- Uses TEXT for IDs and INTEGER for timestamps (Unix epoch seconds)

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE,
    display_name TEXT,
    password_hash TEXT,
    tier TEXT NOT NULL DEFAULT 'free',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    last_login_at INTEGER,
    is_active INTEGER NOT NULL DEFAULT 1,
    email_verified INTEGER NOT NULL DEFAULT 0
);

-- User sessions/tokens
CREATE TABLE IF NOT EXISTS user_sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    token_hash TEXT UNIQUE NOT NULL,
    device_info TEXT,
    ip_address TEXT,
    expires_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    last_active_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- OAuth connections
CREATE TABLE IF NOT EXISTS oauth_connections (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    provider_user_id TEXT NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(provider, provider_user_id)
);

-- API keys
CREATE TABLE IF NOT EXISTS api_keys (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    key_hash TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    last_used_at INTEGER,
    expires_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    is_active INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- User preferences
CREATE TABLE IF NOT EXISTS user_preferences (
    user_id TEXT PRIMARY KEY,
    ai_provider TEXT NOT NULL DEFAULT 'deepseek',
    ai_model TEXT,
    quantum_provider TEXT NOT NULL DEFAULT 'ibm',
    quantum_backend TEXT,
    ui_theme TEXT NOT NULL DEFAULT 'dark',
    preferences TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Usage tracking
CREATE TABLE IF NOT EXISTS usage_records (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_count INTEGER NOT NULL DEFAULT 1,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Quantum jobs
CREATE TABLE IF NOT EXISTS quantum_jobs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT,
    circuit_code TEXT NOT NULL,
    backend TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    result TEXT,
    error_message TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    started_at INTEGER,
    completed_at INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_tier ON users(tier);
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_token_hash ON user_sessions(token_hash);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON user_sessions(expires_at);
CREATE INDEX IF NOT EXISTS idx_oauth_user_id ON oauth_connections(user_id);
CREATE INDEX IF NOT EXISTS idx_oauth_provider ON oauth_connections(provider, provider_user_id);
CREATE INDEX IF NOT EXISTS idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX IF NOT EXISTS idx_usage_user_id_created ON usage_records(user_id, created_at);
CREATE INDEX IF NOT EXISTS idx_quantum_jobs_user_id ON quantum_jobs(user_id);
CREATE INDEX IF NOT EXISTS idx_quantum_jobs_status ON quantum_jobs(status);

-- Triggers for updated_at
CREATE TRIGGER IF NOT EXISTS update_users_updated_at
AFTER UPDATE ON users
BEGIN
    UPDATE users SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_oauth_updated_at
AFTER UPDATE ON oauth_connections
BEGIN
    UPDATE oauth_connections SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_preferences_updated_at
AFTER UPDATE ON user_preferences
BEGIN
    UPDATE user_preferences SET updated_at = strftime('%s', 'now') WHERE user_id = NEW.user_id;
END;
