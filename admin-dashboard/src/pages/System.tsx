import { useState, useEffect } from 'react'
import { 
  Server, 
  Database, 
  RefreshCw,
  CheckCircle,
  AlertCircle,
  XCircle,
  ExternalLink
} from 'lucide-react'

interface ServiceStatus {
  name: string
  status: 'healthy' | 'warning' | 'error'
  latency: string
  uptime: string
  lastCheck: string
}

const mockServices: ServiceStatus[] = [
  { name: 'Production API', status: 'healthy', latency: '45ms', uptime: '99.98%', lastCheck: '10 sec ago' },
  { name: 'Staging API', status: 'healthy', latency: '52ms', uptime: '99.95%', lastCheck: '10 sec ago' },
  { name: 'D1 Database (Prod)', status: 'healthy', latency: '12ms', uptime: '100%', lastCheck: '10 sec ago' },
  { name: 'D1 Database (Stage)', status: 'healthy', latency: '14ms', uptime: '100%', lastCheck: '10 sec ago' },
  { name: 'Cloudflare AI', status: 'healthy', latency: '1.2s', uptime: '99.9%', lastCheck: '10 sec ago' },
  { name: 'Rate Limiter', status: 'warning', latency: '2ms', uptime: '100%', lastCheck: '10 sec ago' },
]

const mockLogs = [
  { time: '19:41:23', level: 'info', message: 'User registration: alice@quantum.io', source: 'auth' },
  { time: '19:41:20', level: 'info', message: 'AI request completed (2.3s)', source: 'ai' },
  { time: '19:41:15', level: 'warn', message: 'Rate limit approaching for IP 192.168.1.45', source: 'ratelimit' },
  { time: '19:41:10', level: 'info', message: 'Quantum job submitted: job_abc123', source: 'quantum' },
  { time: '19:41:05', level: 'info', message: 'Health check passed', source: 'system' },
  { time: '19:40:58', level: 'error', message: 'Failed login attempt: unknown@spam.net', source: 'auth' },
  { time: '19:40:45', level: 'info', message: 'Token verified for bob@research.edu', source: 'auth' },
  { time: '19:40:30', level: 'info', message: 'Database query: 45ms', source: 'db' },
]

export default function System() {
  const [services, setServices] = useState<ServiceStatus[]>(mockServices)
  const [isRefreshing, setIsRefreshing] = useState(false)
  const [autoRefresh, setAutoRefresh] = useState(true)

  const refreshStatus = async () => {
    setIsRefreshing(true)
    
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    // In production, this would call the actual health endpoints
    setServices(mockServices.map(s => ({
      ...s,
      lastCheck: 'just now'
    })))
    
    setIsRefreshing(false)
  }

  useEffect(() => {
    if (autoRefresh) {
      const interval = setInterval(refreshStatus, 30000) // Every 30 seconds
      return () => clearInterval(interval)
    }
  }, [autoRefresh])

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'healthy':
        return <CheckCircle className="w-5 h-5 text-green-400" />
      case 'warning':
        return <AlertCircle className="w-5 h-5 text-yellow-400" />
      case 'error':
        return <XCircle className="w-5 h-5 text-red-400" />
      default:
        return null
    }
  }

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'healthy':
        return <span className="badge badge-success">Healthy</span>
      case 'warning':
        return <span className="badge badge-warning">Warning</span>
      case 'error':
        return <span className="badge badge-danger">Error</span>
      default:
        return null
    }
  }

  const getLogLevelStyle = (level: string) => {
    switch (level) {
      case 'error':
        return 'text-red-400 bg-red-400/10'
      case 'warn':
        return 'text-yellow-400 bg-yellow-400/10'
      default:
        return 'text-gray-400 bg-gray-400/10'
    }
  }

  return (
    <div className="space-y-6 animate-in">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold mb-1">System</h1>
          <p className="text-gray-500">Infrastructure health and monitoring</p>
        </div>
        <div className="flex items-center gap-3">
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={autoRefresh}
              onChange={(e) => setAutoRefresh(e.target.checked)}
              className="sr-only"
            />
            <div className={`w-10 h-6 rounded-full transition-colors ${autoRefresh ? 'bg-cyan-500' : 'bg-gray-600'}`}>
              <div className={`w-4 h-4 rounded-full bg-white transform transition-transform mt-1 ${autoRefresh ? 'translate-x-5' : 'translate-x-1'}`}></div>
            </div>
            <span className="text-sm text-gray-400">Auto-refresh</span>
          </label>
          <button 
            onClick={refreshStatus}
            disabled={isRefreshing}
            className="glass-button flex items-center gap-2"
          >
            <RefreshCw className={`w-4 h-4 ${isRefreshing ? 'animate-spin' : ''}`} />
            Refresh
          </button>
        </div>
      </div>

      {/* Overall Status */}
      <div className="glass-card">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-xl bg-green-400/20 flex items-center justify-center">
              <CheckCircle className="w-6 h-6 text-green-400" />
            </div>
            <div>
              <h2 className="text-xl font-semibold">All Systems Operational</h2>
              <p className="text-gray-500">Last updated: {isRefreshing ? 'Checking...' : '10 seconds ago'}</p>
            </div>
          </div>
          <a 
            href="https://dash.cloudflare.com" 
            target="_blank" 
            rel="noopener noreferrer"
            className="glass-button flex items-center gap-2"
          >
            Cloudflare Dashboard
            <ExternalLink className="w-4 h-4" />
          </a>
        </div>
      </div>

      {/* Services Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {services.map((service) => (
          <div key={service.name} className="glass-card">
            <div className="flex items-start justify-between mb-4">
              <div className="flex items-center gap-3">
                {getStatusIcon(service.status)}
                <h3 className="font-medium">{service.name}</h3>
              </div>
              {getStatusBadge(service.status)}
            </div>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-gray-500">Latency</span>
                <span>{service.latency}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-500">Uptime</span>
                <span className="text-green-400">{service.uptime}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-500">Last Check</span>
                <span className="text-gray-400">{service.lastCheck}</span>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Quick Actions & Info */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Environment Info */}
        <div className="glass-card">
          <h2 className="text-lg font-semibold mb-4">Environment Configuration</h2>
          <div className="space-y-3">
            <div className="flex items-center justify-between p-3 rounded-lg bg-white/3">
              <div className="flex items-center gap-3">
                <Server className="w-5 h-5 text-cyan-400" />
                <span>Production API</span>
              </div>
              <code className="text-xs text-gray-400 bg-white/5 px-2 py-1 rounded">
                qhub-api-production.a-contactnaol.workers.dev
              </code>
            </div>
            <div className="flex items-center justify-between p-3 rounded-lg bg-white/3">
              <div className="flex items-center gap-3">
                <Server className="w-5 h-5 text-yellow-400" />
                <span>Staging API</span>
              </div>
              <code className="text-xs text-gray-400 bg-white/5 px-2 py-1 rounded">
                qhub-api-staging.a-contactnaol.workers.dev
              </code>
            </div>
            <div className="flex items-center justify-between p-3 rounded-lg bg-white/3">
              <div className="flex items-center gap-3">
                <Database className="w-5 h-5 text-green-400" />
                <span>Production DB</span>
              </div>
              <code className="text-xs text-gray-400 bg-white/5 px-2 py-1 rounded">
                b607a2f3-...
              </code>
            </div>
            <div className="flex items-center justify-between p-3 rounded-lg bg-white/3">
              <div className="flex items-center gap-3">
                <Database className="w-5 h-5 text-yellow-400" />
                <span>Staging DB</span>
              </div>
              <code className="text-xs text-gray-400 bg-white/5 px-2 py-1 rounded">
                18a877b3-...
              </code>
            </div>
          </div>
        </div>

        {/* Recent Logs */}
        <div className="glass-card">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Recent Logs</h2>
            <button className="text-sm text-cyan-400 hover:text-cyan-300">
              View All
            </button>
          </div>
          <div className="space-y-2 max-h-64 overflow-auto">
            {mockLogs.map((log, index) => (
              <div 
                key={index}
                className="flex items-start gap-3 p-2 rounded-lg hover:bg-white/3 text-sm"
              >
                <span className="text-gray-600 font-mono text-xs">{log.time}</span>
                <span className={`px-2 py-0.5 rounded text-xs uppercase font-medium ${getLogLevelStyle(log.level)}`}>
                  {log.level}
                </span>
                <span className="text-gray-300 flex-1">{log.message}</span>
                <span className="text-gray-600 text-xs">{log.source}</span>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Version Info */}
      <div className="glass-card">
        <h2 className="text-lg font-semibold mb-4">Version Information</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div>
            <p className="text-sm text-gray-500">QHub CLI</p>
            <p className="font-medium">v0.1.0</p>
          </div>
          <div>
            <p className="text-sm text-gray-500">Workers API</p>
            <p className="font-medium">v1.0.0</p>
          </div>
          <div>
            <p className="text-sm text-gray-500">Admin Dashboard</p>
            <p className="font-medium">v1.0.0</p>
          </div>
          <div>
            <p className="text-sm text-gray-500">Deployment ID</p>
            <p className="font-medium font-mono text-sm">9606b93b</p>
          </div>
        </div>
      </div>
    </div>
  )
}
