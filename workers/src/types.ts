// Environment bindings type definition
export interface Env {
  DB: D1Database;
  AI: any;
  JWT_SECRET: string;
  ENVIRONMENT: string;
  JWT_EXPIRY_HOURS: string;
}

// Extend Hono context variables
export type Variables = {
  user: User;
  jwtPayload: JWTPayload;
}

// Database types
export interface User {
  id: string;
  email: string;
  username: string | null;
  display_name: string | null;
  password_hash: string;
  tier: string;
  created_at: number;
  updated_at: number;
  last_login_at: number | null;
  is_active: number;
  email_verified: number;
}

export interface UserSession {
  id: string;
  user_id: string;
  token_hash: string;
  device_info: string | null;
  ip_address: string | null;
  expires_at: number;
  created_at: number;
  last_active_at: number;
}

export interface Conversation {
  id: string;
  user_id: string;
  title: string | null;
  created_at: number;
  updated_at: number;
}

export interface Message {
  id: string;
  conversation_id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  tokens: number | null;
  created_at: number;
}

export interface QuantumJob {
  id: string;
  user_id: string;
  name: string | null;
  circuit_code: string;
  backend: string | null;
  provider: string;
  status: 'pending' | 'running' | 'completed' | 'failed' | 'cancelled';
  result: string | null;
  error_message: string | null;
  created_at: number;
  started_at: number | null;
  completed_at: number | null;
}

// API Request/Response types
export interface RegisterRequest {
  email: string;
  password: string;
  username?: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: {
    id: string;
    email: string;
    username: string | null;
    tier: string;
  };
  expires_at: number;
}

export interface ChatRequest {
  message: string;
  conversation_id?: string;
}

export interface ChatResponse {
  response: string;
  conversation_id: string;
  tokens_used: number;
}

export interface QuantumJobRequest {
  circuit_code: string;
  backend?: string;
  name?: string;
}

export interface QuantumJobResponse {
  job_id: string;
  status: string;
  created_at: number;
}

// JWT Payload
export interface JWTPayload {
  sub: string;       // user_id
  email: string;
  tier: string;
  exp: number;
  iat: number;
  [key: string]: any; // Allow additional properties for jose compatibility
}
