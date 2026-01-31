import { Hono } from 'hono';
import { Env, QuantumJobRequest, QuantumJobResponse, User, QuantumJob, Variables } from '../types';
import { generateId, now } from '../utils';
import { authMiddleware } from '../middleware/auth';

const quantum = new Hono<{ Bindings: Env; Variables: Variables }>();

// Tier job limits (concurrent jobs)
const JOB_LIMITS: Record<string, number> = {
  free: 3,
  pro: 10,
  enterprise: 50
};

/**
 * POST /quantum/submit
 * Submit a new quantum circuit job
 * 
 * Requires authentication
 * Body: { circuit_code, backend?, name? }
 * Returns: { job_id, status, created_at }
 */
quantum.post('/submit', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const body = await c.req.json<QuantumJobRequest>();
    const { circuit_code, backend, name } = body;

    // Validate input
    if (!circuit_code || circuit_code.trim().length === 0) {
      return c.json({ error: 'Circuit code is required' }, 400);
    }

    if (circuit_code.length > 50000) {
      return c.json({ error: 'Circuit code too large (max 50KB)' }, 400);
    }

    // Check concurrent job limits
    const jobLimit = JOB_LIMITS[user.tier] || JOB_LIMITS.free;
    const activeJobs = await c.env.DB.prepare(
      `SELECT COUNT(*) as count FROM quantum_jobs 
       WHERE user_id = ? AND status IN ('pending', 'running')`
    ).bind(user.id).first<{ count: number }>();

    if (activeJobs && activeJobs.count >= jobLimit) {
      return c.json({ 
        error: `Concurrent job limit reached (${jobLimit} jobs). Wait for jobs to complete or upgrade your plan.`,
        job_limit: jobLimit,
        active_jobs: activeJobs.count
      }, 429);
    }

    // Create quantum job
    const jobId = generateId();
    const timestamp = now();
    const jobBackend = backend || 'qiskit_aer_simulator';
    const provider = 'ibm'; // Default provider (placeholder for now)

    await c.env.DB.prepare(
      `INSERT INTO quantum_jobs (
        id, user_id, name, circuit_code, backend, provider, 
        status, result, error_message, created_at, started_at, completed_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`
    ).bind(
      jobId,
      user.id,
      name || null,
      circuit_code,
      jobBackend,
      provider,
      'pending',
      null,
      null,
      timestamp,
      null,
      null
    ).run();

    // TODO: In production, this would trigger a queue worker to process the job
    // For now, we just store it as pending
    // You would use Cloudflare Queues or Durable Objects for actual quantum execution

    const response: QuantumJobResponse = {
      job_id: jobId,
      status: 'pending',
      created_at: timestamp
    };

    return c.json(response, 201);
  } catch (error) {
    console.error('Submit job error:', error);
    return c.json({ error: 'Failed to submit job' }, 500);
  }
});

/**
 * GET /quantum/jobs
 * List all quantum jobs for current user
 * 
 * Requires authentication
 * Query params: status?, limit?, offset?
 * Returns: { jobs: [...], total }
 */
quantum.get('/jobs', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const status = c.req.query('status');
    const limit = parseInt(c.req.query('limit') || '50', 10);
    const offset = parseInt(c.req.query('offset') || '0', 10);

    // Validate limits
    const safeLimit = Math.min(Math.max(limit, 1), 100);
    const safeOffset = Math.max(offset, 0);

    // Build query
    let query = 'SELECT * FROM quantum_jobs WHERE user_id = ?';
    const params: any[] = [user.id];

    if (status && ['pending', 'running', 'completed', 'failed', 'cancelled'].includes(status)) {
      query += ' AND status = ?';
      params.push(status);
    }

    query += ' ORDER BY created_at DESC LIMIT ? OFFSET ?';
    params.push(safeLimit, safeOffset);

    // Get jobs
    const jobs = await c.env.DB.prepare(query).bind(...params).all<QuantumJob>();

    // Get total count
    let countQuery = 'SELECT COUNT(*) as total FROM quantum_jobs WHERE user_id = ?';
    const countParams: any[] = [user.id];

    if (status && ['pending', 'running', 'completed', 'failed', 'cancelled'].includes(status)) {
      countQuery += ' AND status = ?';
      countParams.push(status);
    }

    const countResult = await c.env.DB.prepare(countQuery).bind(...countParams).first<{ total: number }>();

    return c.json({
      jobs: jobs.results || [],
      total: countResult?.total || 0,
      limit: safeLimit,
      offset: safeOffset
    });
  } catch (error) {
    console.error('List jobs error:', error);
    return c.json({ error: 'Failed to fetch jobs' }, 500);
  }
});

/**
 * GET /quantum/jobs/:id
 * Get detailed information about a specific job
 * 
 * Requires authentication
 * Returns: { job: {...} }
 */
quantum.get('/jobs/:id', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const jobId = c.req.param('id');

    // Get job
    const job = await c.env.DB.prepare(
      'SELECT * FROM quantum_jobs WHERE id = ? AND user_id = ?'
    ).bind(jobId, user.id).first<QuantumJob>();

    if (!job) {
      return c.json({ error: 'Job not found' }, 404);
    }

    return c.json({ job });
  } catch (error) {
    console.error('Get job error:', error);
    return c.json({ error: 'Failed to fetch job' }, 500);
  }
});

/**
 * DELETE /quantum/jobs/:id
 * Cancel/delete a quantum job
 * 
 * Requires authentication
 * Returns: { message: 'Job cancelled' }
 */
quantum.delete('/jobs/:id', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const jobId = c.req.param('id');

    // Get job
    const job = await c.env.DB.prepare(
      'SELECT id, status FROM quantum_jobs WHERE id = ? AND user_id = ?'
    ).bind(jobId, user.id).first<QuantumJob>();

    if (!job) {
      return c.json({ error: 'Job not found' }, 404);
    }

    // Only allow cancelling pending or running jobs
    if (job.status === 'completed' || job.status === 'failed' || job.status === 'cancelled') {
      return c.json({ error: 'Cannot cancel a job that is already finished' }, 400);
    }

    // Update job status to cancelled
    const timestamp = now();
    await c.env.DB.prepare(
      'UPDATE quantum_jobs SET status = ?, completed_at = ? WHERE id = ?'
    ).bind('cancelled', timestamp, jobId).run();

    // TODO: In production, signal the worker to stop processing this job

    return c.json({ message: 'Job cancelled' });
  } catch (error) {
    console.error('Cancel job error:', error);
    return c.json({ error: 'Failed to cancel job' }, 500);
  }
});

/**
 * GET /quantum/stats
 * Get quantum job statistics for current user
 * 
 * Requires authentication
 * Returns: { stats: {...} }
 */
quantum.get('/stats', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;

    // Get job counts by status
    const stats = await c.env.DB.prepare(
      `SELECT 
        status,
        COUNT(*) as count
       FROM quantum_jobs
       WHERE user_id = ?
       GROUP BY status`
    ).bind(user.id).all();

    // Calculate totals
    const statusCounts: Record<string, number> = {
      pending: 0,
      running: 0,
      completed: 0,
      failed: 0,
      cancelled: 0
    };

    let total = 0;
    if (stats.results) {
      for (const row of stats.results as any[]) {
        statusCounts[row.status] = row.count;
        total += row.count;
      }
    }

    // Get most recent job
    const recentJob = await c.env.DB.prepare(
      `SELECT id, name, status, created_at 
       FROM quantum_jobs 
       WHERE user_id = ? 
       ORDER BY created_at DESC 
       LIMIT 1`
    ).bind(user.id).first();

    const jobLimit = JOB_LIMITS[user.tier] || JOB_LIMITS.free;

    return c.json({
      stats: {
        total,
        by_status: statusCounts,
        active: statusCounts.pending + statusCounts.running,
        limit: jobLimit,
        recent_job: recentJob || null,
        tier: user.tier
      }
    });
  } catch (error) {
    console.error('Job stats error:', error);
    return c.json({ error: 'Failed to fetch statistics' }, 500);
  }
});

/**
 * POST /quantum/jobs/:id/rerun
 * Rerun a completed or failed job
 * 
 * Requires authentication
 * Returns: { job_id, status, created_at }
 */
quantum.post('/jobs/:id/rerun', authMiddleware, async (c) => {
  try {
    const user = c.get('user') as User;
    const oldJobId = c.req.param('id');

    // Get original job
    const oldJob = await c.env.DB.prepare(
      'SELECT * FROM quantum_jobs WHERE id = ? AND user_id = ?'
    ).bind(oldJobId, user.id).first<QuantumJob>();

    if (!oldJob) {
      return c.json({ error: 'Job not found' }, 404);
    }

    // Check concurrent job limits
    const jobLimit = JOB_LIMITS[user.tier] || JOB_LIMITS.free;
    const activeJobs = await c.env.DB.prepare(
      `SELECT COUNT(*) as count FROM quantum_jobs 
       WHERE user_id = ? AND status IN ('pending', 'running')`
    ).bind(user.id).first<{ count: number }>();

    if (activeJobs && activeJobs.count >= jobLimit) {
      return c.json({ 
        error: `Concurrent job limit reached (${jobLimit} jobs)`,
        job_limit: jobLimit,
        active_jobs: activeJobs.count
      }, 429);
    }

    // Create new job with same parameters
    const newJobId = generateId();
    const timestamp = now();

    await c.env.DB.prepare(
      `INSERT INTO quantum_jobs (
        id, user_id, name, circuit_code, backend, provider, 
        status, result, error_message, created_at, started_at, completed_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`
    ).bind(
      newJobId,
      user.id,
      oldJob.name ? `${oldJob.name} (rerun)` : null,
      oldJob.circuit_code,
      oldJob.backend,
      oldJob.provider,
      'pending',
      null,
      null,
      timestamp,
      null,
      null
    ).run();

    const response: QuantumJobResponse = {
      job_id: newJobId,
      status: 'pending',
      created_at: timestamp
    };

    return c.json(response, 201);
  } catch (error) {
    console.error('Rerun job error:', error);
    return c.json({ error: 'Failed to rerun job' }, 500);
  }
});

export default quantum;
