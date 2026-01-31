import { Hono } from 'hono';
import { Env, ChatRequest, ChatResponse, User, Conversation, Message, Variables } from '../types';
import { generateId, now } from '../utils';
import { authMiddleware } from '../middleware/auth';

const ai = new Hono<{ Bindings: Env; Variables: Variables }>();

// Tier usage limits (messages per day)
const USAGE_LIMITS: Record<string, number> = {
  free: 10,
  pro: 100,
  enterprise: 1000
};

/**
 * POST /ai/chat
 * Send a message and get AI response
 * 
 * Requires authentication
 * Body: { message, conversation_id? }
 * Returns: { response, conversation_id, tokens_used }
 */
ai.post('/chat', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const body = await c.req.json<ChatRequest>();
    const { message, conversation_id } = body;

    // Validate input
    if (!message || message.trim().length === 0) {
      return c.json({ error: 'Message is required' }, 400);
    }

    if (message.length > 4000) {
      return c.json({ error: 'Message too long (max 4000 characters)' }, 400);
    }

    // Check usage limits
    const today = Math.floor(Date.now() / 86400000); // Days since epoch
    const usageLimit = USAGE_LIMITS[user.tier] || USAGE_LIMITS.free;

    const usageCount = await c.env.DB.prepare(
      `SELECT COUNT(*) as count FROM messages 
       WHERE conversation_id IN (
         SELECT id FROM conversations WHERE user_id = ?
       ) AND role = 'user' AND created_at >= ?`
    ).bind(user.id, today * 86400).first<{ count: number }>();

    if (usageCount && usageCount.count >= usageLimit) {
      return c.json({ 
        error: `Daily usage limit reached (${usageLimit} messages). Upgrade your plan for more.`,
        usage_limit: usageLimit,
        current_usage: usageCount.count
      }, 429);
    }

    // Get or create conversation
    let convId = conversation_id;
    const timestamp = now();

    if (convId) {
      // Verify conversation belongs to user
      const conv = await c.env.DB.prepare(
        'SELECT id FROM conversations WHERE id = ? AND user_id = ?'
      ).bind(convId, user.id).first();

      if (!conv) {
        return c.json({ error: 'Conversation not found' }, 404);
      }

      // Update conversation timestamp
      await c.env.DB.prepare(
        'UPDATE conversations SET updated_at = ? WHERE id = ?'
      ).bind(timestamp, convId).run();
    } else {
      // Create new conversation
      convId = generateId();
      await c.env.DB.prepare(
        `INSERT INTO conversations (id, user_id, title, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)`
      ).bind(convId, user.id, null, timestamp, timestamp).run();
    }

    // Save user message
    const userMessageId = generateId();
    await c.env.DB.prepare(
      `INSERT INTO messages (id, conversation_id, role, content, tokens, created_at)
       VALUES (?, ?, ?, ?, ?, ?)`
    ).bind(userMessageId, convId, 'user', message, null, timestamp).run();

    // Get conversation history (last 10 messages for context)
    const history = await c.env.DB.prepare(
      `SELECT role, content FROM messages 
       WHERE conversation_id = ? 
       ORDER BY created_at ASC 
       LIMIT 10`
    ).bind(convId).all<Message>();

    // Build messages array for AI
    const messages = (history.results || []).map(m => ({
      role: m.role,
      content: m.content
    }));

    // Call Cloudflare AI
    let aiResponse: string;
    let tokensUsed = 0;

    try {
      // Using Cloudflare Workers AI
      const aiResult = await c.env.AI.run('@cf/meta/llama-2-7b-chat-int8', {
        messages: messages,
        max_tokens: 1024,
        temperature: 0.7,
        top_p: 0.9,
      });

      // Extract response text
      if (aiResult && aiResult.response) {
        aiResponse = aiResult.response;
      } else if (typeof aiResult === 'string') {
        aiResponse = aiResult;
      } else {
        throw new Error('Unexpected AI response format');
      }

      // Estimate tokens (rough approximation: 1 token ≈ 4 characters)
      tokensUsed = Math.ceil((message.length + aiResponse.length) / 4);
    } catch (aiError) {
      console.error('AI API error:', aiError);
      
      // Fallback response if AI fails
      aiResponse = "I apologize, but I'm having trouble processing your request right now. Please try again in a moment.";
      tokensUsed = 50;
    }

    // Save AI response
    const assistantMessageId = generateId();
    await c.env.DB.prepare(
      `INSERT INTO messages (id, conversation_id, role, content, tokens, created_at)
       VALUES (?, ?, ?, ?, ?, ?)`
    ).bind(assistantMessageId, convId, 'assistant', aiResponse, tokensUsed, timestamp).run();

    // Auto-generate conversation title from first message
    if (!conversation_id) {
      const title = message.slice(0, 50) + (message.length > 50 ? '...' : '');
      await c.env.DB.prepare(
        'UPDATE conversations SET title = ? WHERE id = ?'
      ).bind(title, convId).run();
    }

    // Return response
    const response: ChatResponse = {
      response: aiResponse,
      conversation_id: convId,
      tokens_used: tokensUsed
    };

    return c.json(response);
  } catch (error) {
    console.error('Chat error:', error);
    return c.json({ error: 'Chat failed' }, 500);
  }
});

/**
 * GET /ai/conversations
 * List all conversations for current user
 * 
 * Requires authentication
 * Returns: { conversations: [...] }
 */
ai.get('/conversations', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;

    // Get conversations with message count
    const conversations = await c.env.DB.prepare(
      `SELECT c.id, c.title, c.created_at, c.updated_at,
              (SELECT COUNT(*) FROM messages WHERE conversation_id = c.id) as message_count
       FROM conversations c
       WHERE c.user_id = ?
       ORDER BY c.updated_at DESC
       LIMIT 100`
    ).bind(user.id).all();

    return c.json({ conversations: conversations.results || [] });
  } catch (error) {
    console.error('List conversations error:', error);
    return c.json({ error: 'Failed to fetch conversations' }, 500);
  }
});

/**
 * GET /ai/conversations/:id
 * Get a specific conversation with all messages
 * 
 * Requires authentication
 * Returns: { conversation: {...}, messages: [...] }
 */
ai.get('/conversations/:id', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const conversationId = c.req.param('id');

    // Get conversation
    const conversation = await c.env.DB.prepare(
      'SELECT * FROM conversations WHERE id = ? AND user_id = ?'
    ).bind(conversationId, user.id).first<Conversation>();

    if (!conversation) {
      return c.json({ error: 'Conversation not found' }, 404);
    }

    // Get all messages
    const messages = await c.env.DB.prepare(
      `SELECT id, role, content, tokens, created_at 
       FROM messages 
       WHERE conversation_id = ? 
       ORDER BY created_at ASC`
    ).bind(conversationId).all<Message>();

    return c.json({
      conversation,
      messages: messages.results || []
    });
  } catch (error) {
    console.error('Get conversation error:', error);
    return c.json({ error: 'Failed to fetch conversation' }, 500);
  }
});

/**
 * DELETE /ai/conversations/:id
 * Delete a conversation and all its messages
 * 
 * Requires authentication
 * Returns: { message: 'Conversation deleted' }
 */
ai.delete('/conversations/:id', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const conversationId = c.req.param('id');

    // Verify ownership
    const conversation = await c.env.DB.prepare(
      'SELECT id FROM conversations WHERE id = ? AND user_id = ?'
    ).bind(conversationId, user.id).first();

    if (!conversation) {
      return c.json({ error: 'Conversation not found' }, 404);
    }

    // Delete messages first (foreign key constraint)
    await c.env.DB.prepare(
      'DELETE FROM messages WHERE conversation_id = ?'
    ).bind(conversationId).run();

    // Delete conversation
    await c.env.DB.prepare(
      'DELETE FROM conversations WHERE id = ?'
    ).bind(conversationId).run();

    return c.json({ message: 'Conversation deleted' });
  } catch (error) {
    console.error('Delete conversation error:', error);
    return c.json({ error: 'Failed to delete conversation' }, 500);
  }
});

/**
 * GET /ai/usage
 * Get current usage statistics for the user
 * 
 * Requires authentication
 * Returns: { usage: {...} }
 */
ai.get('/usage', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const today = Math.floor(Date.now() / 86400000) * 86400;

    // Count messages today
    const todayUsage = await c.env.DB.prepare(
      `SELECT COUNT(*) as count FROM messages 
       WHERE conversation_id IN (
         SELECT id FROM conversations WHERE user_id = ?
       ) AND role = 'user' AND created_at >= ?`
    ).bind(user.id, today).first<{ count: number }>();

    // Total conversations
    const totalConversations = await c.env.DB.prepare(
      'SELECT COUNT(*) as count FROM conversations WHERE user_id = ?'
    ).bind(user.id).first<{ count: number }>();

    // Total messages
    const totalMessages = await c.env.DB.prepare(
      `SELECT COUNT(*) as count FROM messages 
       WHERE conversation_id IN (
         SELECT id FROM conversations WHERE user_id = ?
       )`
    ).bind(user.id).first<{ count: number }>();

    const usageLimit = USAGE_LIMITS[user.tier] || USAGE_LIMITS.free;

    return c.json({
      usage: {
        today: todayUsage?.count || 0,
        limit: usageLimit,
        total_conversations: totalConversations?.count || 0,
        total_messages: totalMessages?.count || 0,
        tier: user.tier
      }
    });
  } catch (error) {
    console.error('Usage stats error:', error);
    return c.json({ error: 'Failed to fetch usage statistics' }, 500);
  }
});

export default ai;
