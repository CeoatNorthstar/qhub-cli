-- QHub Database Schema
-- PostgreSQL version for local development
-- Uses 'qhub' schema to avoid conflicts in shared database

-- Set schema
SET search_path TO qhub, public;

-- Users table
CREATE TABLE IF NOT EXISTS qhub.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE,
    display_name VARCHAR(255),
    password_hash VARCHAR(255), -- For email/password auth
    tier VARCHAR(50) NOT NULL DEFAULT 'free', -- free, pro, enterprise
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT true,
    email_verified BOOLEAN DEFAULT false
);

-- User sessions/tokens
CREATE TABLE IF NOT EXISTS qhub.user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) UNIQUE NOT NULL,
    device_info TEXT,
    ip_address INET,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_active_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- OAuth connections
CREATE TABLE IF NOT EXISTS qhub.oauth_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL, -- github, google, etc
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(provider, provider_user_id)
);

-- API keys (for programmatic access)
CREATE TABLE IF NOT EXISTS qhub.api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    key_hash VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    last_used_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

-- User preferences
CREATE TABLE IF NOT EXISTS qhub.user_preferences (
    user_id UUID PRIMARY KEY REFERENCES qhub.users(id) ON DELETE CASCADE,
    ai_provider VARCHAR(50) DEFAULT 'deepseek',
    ai_model VARCHAR(100),
    quantum_provider VARCHAR(50) DEFAULT 'ibm',
    quantum_backend VARCHAR(100),
    ui_theme VARCHAR(50) DEFAULT 'default',
    preferences JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Usage tracking (for quotas and analytics)
CREATE TABLE IF NOT EXISTS qhub.usage_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    resource_type VARCHAR(50) NOT NULL, -- ai_request, quantum_job, etc
    resource_count INTEGER DEFAULT 1,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Quantum jobs (stored quantum computations)
CREATE TABLE IF NOT EXISTS qhub.quantum_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES qhub.users(id) ON DELETE CASCADE,
    name VARCHAR(255),
    circuit_code TEXT NOT NULL,
    backend VARCHAR(100),
    status VARCHAR(50) DEFAULT 'pending', -- pending, running, completed, failed
    result JSONB,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_users_email ON qhub.users(email);
CREATE INDEX IF NOT EXISTS idx_users_tier ON qhub.users(tier);
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON qhub.user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_token_hash ON qhub.user_sessions(token_hash);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON qhub.user_sessions(expires_at);
CREATE INDEX IF NOT EXISTS idx_oauth_user_id ON qhub.oauth_connections(user_id);
CREATE INDEX IF NOT EXISTS idx_oauth_provider ON qhub.oauth_connections(provider, provider_user_id);
CREATE INDEX IF NOT EXISTS idx_api_keys_user_id ON qhub.api_keys(user_id);
CREATE INDEX IF NOT EXISTS idx_usage_user_id_created ON qhub.usage_records(user_id, created_at);
CREATE INDEX IF NOT EXISTS idx_quantum_jobs_user_id ON qhub.quantum_jobs(user_id);
CREATE INDEX IF NOT EXISTS idx_quantum_jobs_status ON qhub.quantum_jobs(status);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION qhub.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at triggers
DROP TRIGGER IF EXISTS update_users_updated_at ON qhub.users;
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON qhub.users
    FOR EACH ROW EXECUTE FUNCTION qhub.update_updated_at_column();

DROP TRIGGER IF EXISTS update_oauth_updated_at ON qhub.oauth_connections;
CREATE TRIGGER update_oauth_updated_at BEFORE UPDATE ON qhub.oauth_connections
    FOR EACH ROW EXECUTE FUNCTION qhub.update_updated_at_column();

DROP TRIGGER IF EXISTS update_preferences_updated_at ON qhub.user_preferences;
CREATE TRIGGER update_preferences_updated_at BEFORE UPDATE ON qhub.user_preferences
    FOR EACH ROW EXECUTE FUNCTION qhub.update_updated_at_column();
