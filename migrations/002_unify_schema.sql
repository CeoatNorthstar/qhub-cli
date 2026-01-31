-- Migration 002: Unify PostgreSQL schema to match D1
-- This makes both databases work identically with the same application code
-- Changes: Use TEXT for UUIDs, BIGINT for timestamps to match D1's INTEGER

-- Drop existing tables (dev only - be careful in production!)
DROP TABLE IF EXISTS qhub.quantum_jobs CASCADE;
DROP TABLE IF EXISTS qhub.usage_records CASCADE;
DROP TABLE IF EXISTS qhub.user_preferences CASCADE;
DROP TABLE IF EXISTS qhub.api_keys CASCADE;
DROP TABLE IF EXISTS qhub.oauth_connections CASCADE;
DROP TABLE IF EXISTS qhub.user_sessions CASCADE;
DROP TABLE IF EXISTS qhub.users CASCADE;

-- Users table (unified schema)
CREATE TABLE qhub.users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE,
    display_name TEXT,
    password_hash TEXT,
    tier TEXT NOT NULL DEFAULT 'free',
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
    updated_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
    last_login_at BIGINT,
    is_active BOOLEAN DEFAULT true,
    email_verified BOOLEAN DEFAULT false
);

-- User sessions/tokens
CREATE TABLE qhub.user_sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    token_hash TEXT UNIQUE NOT NULL,
    device_info TEXT,
    ip_address TEXT,
    expires_at BIGINT NOT NULL,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
    last_active_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
);

-- OAuth connections
CREATE TABLE qhub.oauth_connections (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    provider_user_id TEXT NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at BIGINT,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
    updated_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
    UNIQUE(provider, provider_user_id)
);

-- API keys (for programmatic access)
CREATE TABLE qhub.api_keys (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    key_hash TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    last_used_at BIGINT,
    expires_at BIGINT,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
    is_active BOOLEAN DEFAULT true
);

-- User preferences
CREATE TABLE qhub.user_preferences (
    user_id TEXT PRIMARY KEY REFERENCES qhub.users(id) ON DELETE CASCADE,
    ai_provider TEXT NOT NULL DEFAULT 'deepseek',
    ai_model TEXT,
    quantum_provider TEXT NOT NULL DEFAULT 'ibm',
    quantum_backend TEXT,
    ui_theme TEXT NOT NULL DEFAULT 'dark',
    preferences JSONB DEFAULT '{}'::jsonb,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
    updated_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
);

-- Usage tracking
CREATE TABLE qhub.usage_records (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    resource_type TEXT NOT NULL,
    resource_count INTEGER NOT NULL DEFAULT 1,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
);

-- Quantum jobs
CREATE TABLE qhub.quantum_jobs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    name TEXT,
    circuit_code TEXT NOT NULL,
    backend TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    result JSONB,
    error_message TEXT,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
    started_at BIGINT,
    completed_at BIGINT
);

-- Indexes for performance
CREATE INDEX idx_user_sessions_user_id ON qhub.user_sessions(user_id);
CREATE INDEX idx_user_sessions_token_hash ON qhub.user_sessions(token_hash);
CREATE INDEX idx_user_sessions_expires_at ON qhub.user_sessions(expires_at);
CREATE INDEX idx_oauth_user_id ON qhub.oauth_connections(user_id);
CREATE INDEX idx_oauth_provider ON qhub.oauth_connections(provider, provider_user_id);
CREATE INDEX idx_api_keys_user_id ON qhub.api_keys(user_id);
CREATE INDEX idx_api_keys_key_hash ON qhub.api_keys(key_hash);
CREATE INDEX idx_usage_user_id ON qhub.usage_records(user_id);
CREATE INDEX idx_usage_created_at ON qhub.usage_records(created_at);
CREATE INDEX idx_quantum_jobs_user_id ON qhub.quantum_jobs(user_id);
CREATE INDEX idx_quantum_jobs_status ON qhub.quantum_jobs(status);
CREATE INDEX idx_quantum_jobs_created_at ON qhub.quantum_jobs(created_at);

-- Trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION qhub.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = EXTRACT(EPOCH FROM NOW())::BIGINT;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON qhub.users
    FOR EACH ROW EXECUTE FUNCTION qhub.update_updated_at_column();

CREATE TRIGGER update_oauth_updated_at BEFORE UPDATE ON qhub.oauth_connections
    FOR EACH ROW EXECUTE FUNCTION qhub.update_updated_at_column();

CREATE TRIGGER update_preferences_updated_at BEFORE UPDATE ON qhub.user_preferences
    FOR EACH ROW EXECUTE FUNCTION qhub.update_updated_at_column();
