import { Context } from 'hono';
import { Env, JWTPayload } from './types';
import * as bcrypt from 'bcryptjs';
import { SignJWT, jwtVerify } from 'jose';

// Generate UUID v4
export function generateId(): string {
  return crypto.randomUUID();
}

// Hash password with bcrypt
export async function hashPassword(password: string): Promise<string> {
  const salt = await bcrypt.genSalt(10);
  return bcrypt.hash(password, salt);
}

// Verify password
export async function verifyPassword(password: string, hash: string): Promise<boolean> {
  return bcrypt.compare(password, hash);
}

// Generate JWT token
export async function generateToken(user: { id: string; email: string; tier: string }, secret: string, expiryHours: number = 24): Promise<{ token: string; expiresAt: number }> {
  const now = Math.floor(Date.now() / 1000);
  const exp = now + (expiryHours * 3600);
  
  const payload: JWTPayload = {
    sub: user.id,
    email: user.email,
    tier: user.tier,
    exp,
    iat: now,
  };
  
  const encoder = new TextEncoder();
  const token = await new SignJWT(payload)
    .setProtectedHeader({ alg: 'HS256' })
    .setIssuedAt(now)
    .setExpirationTime(exp)
    .sign(encoder.encode(secret));
  
  return { token, expiresAt: exp };
}

// Verify JWT token
export async function verifyToken(token: string, secret: string): Promise<JWTPayload> {
  const encoder = new TextEncoder();
  const { payload } = await jwtVerify(token, encoder.encode(secret));
  return payload as unknown as JWTPayload;
}

// Hash token for storage (SHA-256)
export async function hashToken(token: string): Promise<string> {
  const encoder = new TextEncoder();
  const data = encoder.encode(token);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

// Get current Unix timestamp
export function now(): number {
  return Math.floor(Date.now() / 1000);
}

// Extract bearer token from Authorization header
export function extractBearerToken(c: Context): string | null {
  const auth = c.req.header('Authorization');
  if (!auth || !auth.startsWith('Bearer ')) {
    return null;
  }
  return auth.slice(7);
}

// Validate email format
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

// Error response helper
export function errorResponse(message: string, status: number = 400) {
  return Response.json({ error: message }, { status });
}

// Success response helper
export function successResponse(data: any, status: number = 200) {
  return Response.json(data, { status });
}

// CORS headers
export function corsHeaders() {
  return {
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
    'Access-Control-Allow-Headers': 'Content-Type, Authorization',
  };
}
