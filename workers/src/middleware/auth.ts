import { Context, Next } from 'hono';
import { Env, JWTPayload, User, Variables } from '../types';
import { extractBearerToken, verifyToken } from '../utils';

// Extend Context to include authenticated user
export interface AuthContext {
  user: User;
  jwtPayload: JWTPayload;
}

/**
 * Authentication middleware
 * Verifies JWT token and attaches user to context
 * Returns 401 if token is invalid or user not found
 */
export async function authMiddleware(c: Context<{ Bindings: Env; Variables: Variables }>, next: Next) {
  try {
    // Extract token from Authorization header
    const token = extractBearerToken(c);
    if (!token) {
      return c.json({ error: 'Missing authorization token' }, 401);
    }

    // Verify JWT token
    let payload: JWTPayload;
    try {
      payload = await verifyToken(token, c.env.JWT_SECRET);
    } catch (err) {
      return c.json({ error: 'Invalid or expired token' }, 401);
    }

    // Check token expiration
    const now = Math.floor(Date.now() / 1000);
    if (payload.exp < now) {
      return c.json({ error: 'Token expired' }, 401);
    }

    // Fetch user from database
    const userResult = await c.env.DB.prepare(
      'SELECT * FROM users WHERE id = ? AND is_active = 1'
    ).bind(payload.sub).first<User>();

    if (!userResult) {
      return c.json({ error: 'User not found or inactive' }, 401);
    }

    // Attach user and JWT payload to context
    c.set('user', userResult);
    c.set('jwtPayload', payload);

    // Continue to next handler
    await next();
  } catch (error) {
    console.error('Auth middleware error:', error);
    return c.json({ error: 'Authentication failed' }, 500);
  }
}

/**
 * Optional authentication middleware
 * Attaches user if token is valid, but doesn't fail if missing
 */
export async function optionalAuthMiddleware(c: Context<{ Bindings: Env; Variables: Variables }>, next: Next) {
  try {
    const token = extractBearerToken(c);
    
    if (token) {
      try {
        const payload = await verifyToken(token, c.env.JWT_SECRET);
        const now = Math.floor(Date.now() / 1000);
        
        if (payload.exp >= now) {
          const userResult = await c.env.DB.prepare(
            'SELECT * FROM users WHERE id = ? AND is_active = 1'
          ).bind(payload.sub).first<User>();
          
          if (userResult) {
            c.set('user', userResult);
            c.set('jwtPayload', payload);
          }
        }
      } catch (err) {
        // Silently fail for optional auth
        console.warn('Optional auth failed:', err);
      }
    }
    
    await next();
  } catch (error) {
    console.error('Optional auth middleware error:', error);
    await next();
  }
}
