import { Hono } from 'hono';
import { Env, RegisterRequest, LoginRequest, AuthResponse, User, Variables } from '../types';
import { 
  generateId, 
  hashPassword, 
  verifyPassword, 
  generateToken, 
  hashToken,
  now,
  isValidEmail 
} from '../utils';
import { authMiddleware } from '../middleware/auth';

const auth = new Hono<{ Bindings: Env; Variables: Variables }>();

/**
 * POST /auth/register
 * Register a new user account
 * 
 * Body: { email, password, username? }
 * Returns: { token, user, expires_at }
 */
auth.post('/register', async (c) => {
  try {
    const body = await c.req.json<RegisterRequest>();
    const { email, password, username } = body;

    // Validate input
    if (!email || !password) {
      return c.json({ error: 'Email and password are required' }, 400);
    }

    if (!isValidEmail(email)) {
      return c.json({ error: 'Invalid email format' }, 400);
    }

    if (password.length < 8) {
      return c.json({ error: 'Password must be at least 8 characters' }, 400);
    }

    // Check if email already exists
    const existingUser = await c.env.DB.prepare(
      'SELECT id FROM users WHERE email = ?'
    ).bind(email.toLowerCase()).first();

    if (existingUser) {
      return c.json({ error: 'Email already registered' }, 409);
    }

    // Check if username is taken (if provided)
    if (username) {
      const existingUsername = await c.env.DB.prepare(
        'SELECT id FROM users WHERE username = ?'
      ).bind(username).first();

      if (existingUsername) {
        return c.json({ error: 'Username already taken' }, 409);
      }
    }

    // Hash password
    const passwordHash = await hashPassword(password);

    // Create user
    const userId = generateId();
    const timestamp = now();

    await c.env.DB.prepare(
      `INSERT INTO users (
        id, email, username, password_hash, tier, 
        created_at, updated_at, is_active, email_verified
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`
    ).bind(
      userId,
      email.toLowerCase(),
      username || null,
      passwordHash,
      'free',
      timestamp,
      timestamp,
      1,
      0
    ).run();

    // Generate JWT token
    const expiryHours = parseInt(c.env.JWT_EXPIRY_HOURS || '24', 10);
    const { token, expiresAt } = await generateToken(
      { id: userId, email: email.toLowerCase(), tier: 'free' },
      c.env.JWT_SECRET,
      expiryHours
    );

    // Create session
    const sessionId = generateId();
    const tokenHash = await hashToken(token);
    const ipAddress = c.req.header('CF-Connecting-IP') || null;
    const userAgent = c.req.header('User-Agent') || null;

    await c.env.DB.prepare(
      `INSERT INTO user_sessions (
        id, user_id, token_hash, device_info, ip_address, 
        expires_at, created_at, last_active_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)`
    ).bind(
      sessionId,
      userId,
      tokenHash,
      userAgent,
      ipAddress,
      expiresAt,
      timestamp,
      timestamp
    ).run();

    // Return auth response
    const response: AuthResponse = {
      token,
      user: {
        id: userId,
        email: email.toLowerCase(),
        username: username || null,
        tier: 'free'
      },
      expires_at: expiresAt
    };

    return c.json(response, 201);
  } catch (error) {
    console.error('Register error:', error);
    return c.json({ error: 'Registration failed' }, 500);
  }
});

/**
 * POST /auth/login
 * Login with email and password
 * 
 * Body: { email, password }
 * Returns: { token, user, expires_at }
 */
auth.post('/login', async (c) => {
  try {
    const body = await c.req.json<LoginRequest>();
    const { email, password } = body;

    // Validate input
    if (!email || !password) {
      return c.json({ error: 'Email and password are required' }, 400);
    }

    // Find user by email
    const user = await c.env.DB.prepare(
      'SELECT * FROM users WHERE email = ? AND is_active = 1'
    ).bind(email.toLowerCase()).first<User>();

    if (!user) {
      return c.json({ error: 'Invalid email or password' }, 401);
    }

    // Verify password
    const passwordValid = await verifyPassword(password, user.password_hash);
    if (!passwordValid) {
      return c.json({ error: 'Invalid email or password' }, 401);
    }

    // Update last login timestamp
    const timestamp = now();
    await c.env.DB.prepare(
      'UPDATE users SET last_login_at = ? WHERE id = ?'
    ).bind(timestamp, user.id).run();

    // Generate JWT token
    const expiryHours = parseInt(c.env.JWT_EXPIRY_HOURS || '24', 10);
    const { token, expiresAt } = await generateToken(
      { id: user.id, email: user.email, tier: user.tier },
      c.env.JWT_SECRET,
      expiryHours
    );

    // Create session
    const sessionId = generateId();
    const tokenHash = await hashToken(token);
    const ipAddress = c.req.header('CF-Connecting-IP') || null;
    const userAgent = c.req.header('User-Agent') || null;

    await c.env.DB.prepare(
      `INSERT INTO user_sessions (
        id, user_id, token_hash, device_info, ip_address, 
        expires_at, created_at, last_active_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)`
    ).bind(
      sessionId,
      user.id,
      tokenHash,
      userAgent,
      ipAddress,
      expiresAt,
      timestamp,
      timestamp
    ).run();

    // Return auth response
    const response: AuthResponse = {
      token,
      user: {
        id: user.id,
        email: user.email,
        username: user.username,
        tier: user.tier
      },
      expires_at: expiresAt
    };

    return c.json(response);
  } catch (error) {
    console.error('Login error:', error);
    return c.json({ error: 'Login failed' }, 500);
  }
});

/**
 * POST /auth/logout
 * Logout current session (requires authentication)
 * 
 * Returns: { message: 'Logged out successfully' }
 */
auth.post('/logout', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const token = c.req.header('Authorization')?.slice(7);
    
    if (!token) {
      return c.json({ error: 'No token provided' }, 400);
    }

    // Delete session by token hash
    const tokenHash = await hashToken(token);
    await c.env.DB.prepare(
      'DELETE FROM user_sessions WHERE user_id = ? AND token_hash = ?'
    ).bind(user.id, tokenHash).run();

    return c.json({ message: 'Logged out successfully' });
  } catch (error) {
    console.error('Logout error:', error);
    return c.json({ error: 'Logout failed' }, 500);
  }
});

/**
 * POST /auth/logout-all
 * Logout from all sessions (requires authentication)
 * 
 * Returns: { message: 'Logged out from all devices' }
 */
auth.post('/logout-all', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;

    // Delete all sessions for user
    await c.env.DB.prepare(
      'DELETE FROM user_sessions WHERE user_id = ?'
    ).bind(user.id).run();

    return c.json({ message: 'Logged out from all devices' });
  } catch (error) {
    console.error('Logout all error:', error);
    return c.json({ error: 'Logout failed' }, 500);
  }
});

/**
 * GET /auth/verify
 * Verify current token and return user info (requires authentication)
 * 
 * Returns: { user: { id, email, username, tier } }
 */
auth.get('/verify', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;

    return c.json({
      user: {
        id: user.id,
        email: user.email,
        username: user.username,
        tier: user.tier,
        email_verified: user.email_verified === 1,
        created_at: user.created_at
      }
    });
  } catch (error) {
    console.error('Verify error:', error);
    return c.json({ error: 'Verification failed' }, 500);
  }
});

/**
 * GET /auth/sessions
 * List all active sessions for current user (requires authentication)
 * 
 * Returns: { sessions: [...] }
 */
auth.get('/sessions', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const timestamp = now();

    // Get active sessions (not expired)
    const sessions = await c.env.DB.prepare(
      `SELECT id, device_info, ip_address, created_at, last_active_at, expires_at
       FROM user_sessions 
       WHERE user_id = ? AND expires_at > ?
       ORDER BY last_active_at DESC`
    ).bind(user.id, timestamp).all();

    return c.json({ sessions: sessions.results || [] });
  } catch (error) {
    console.error('Sessions list error:', error);
    return c.json({ error: 'Failed to fetch sessions' }, 500);
  }
});

/**
 * DELETE /auth/sessions/:id
 * Delete a specific session (requires authentication)
 * 
 * Returns: { message: 'Session deleted' }
 */
auth.delete('/sessions/:id', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const sessionId = c.req.param('id');

    // Delete session only if it belongs to the user
    const result = await c.env.DB.prepare(
      'DELETE FROM user_sessions WHERE id = ? AND user_id = ?'
    ).bind(sessionId, user.id).run();

    if (result.meta.changes === 0) {
      return c.json({ error: 'Session not found' }, 404);
    }

    return c.json({ message: 'Session deleted' });
  } catch (error) {
    console.error('Session delete error:', error);
    return c.json({ error: 'Failed to delete session' }, 500);
  }
});

export default auth;
