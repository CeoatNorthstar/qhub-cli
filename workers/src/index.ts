import { Hono } from 'hono';
import { cors } from 'hono/cors';
import { Env, Variables } from './types';
import auth from './routes/auth';
import ai from './routes/ai';
import quantum from './routes/quantum';

/**
 * QHub API - TypeScript Backend on Cloudflare Workers
 * 
 * Enterprise-grade API for quantum computing and AI integration
 * Built with Hono, D1 Database, Cloudflare AI, and JWT authentication
 */

// Initialize Hono app with environment bindings
const app = new Hono<{ Bindings: Env; Variables: Variables }>();

// Global CORS middleware - allows requests from any origin
app.use('/*', cors({
  origin: '*',
  allowMethods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
  allowHeaders: ['Content-Type', 'Authorization'],
  exposeHeaders: ['Content-Length'],
  maxAge: 600,
  credentials: true,
}));

// Global error handler
app.onError((err, c) => {
  console.error('Unhandled error:', err);
  return c.json({ 
    error: 'Internal server error',
    message: c.env.ENVIRONMENT === 'development' ? err.message : undefined
  }, 500);
});

// Health check endpoint
app.get('/', (c) => {
  return c.json({
    name: 'QHub API',
    version: '1.0.0',
    status: 'operational',
    environment: c.env.ENVIRONMENT || 'production',
    endpoints: {
      auth: '/auth',
      ai: '/ai',
      quantum: '/quantum'
    },
    documentation: 'https://github.com/your-org/qhub-cli'
  });
});

// Health check endpoint (explicit)
app.get('/health', (c) => {
  return c.json({ 
    status: 'healthy', 
    timestamp: Math.floor(Date.now() / 1000) 
  });
});

// API version info
app.get('/version', (c) => {
  return c.json({
    version: '1.0.0',
    api_version: 'v1',
    environment: c.env.ENVIRONMENT || 'production'
  });
});

// Mount route handlers
app.route('/auth', auth);
app.route('/ai', ai);
app.route('/quantum', quantum);

// 404 handler for unknown routes
app.notFound((c) => {
  return c.json({ 
    error: 'Not found',
    path: c.req.path,
    message: 'The requested endpoint does not exist'
  }, 404);
});

// Export the app as default for Cloudflare Workers
export default app;
