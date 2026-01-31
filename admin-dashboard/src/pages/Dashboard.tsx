import { useEffect, useState } from 'react'
import { 
  Users, 
  MessageSquare, 
  Cpu,
  ArrowUp,
  ArrowDown,
  Activity
} from 'lucide-react'
import { AreaChart, Area, XAxis, YAxis, Tooltip, ResponsiveContainer, BarChart, Bar } from 'recharts'

// Mock data - will be replaced with API calls
const mockStats = {
  totalUsers: 247,
  activeUsers: 89,
  totalMessages: 1834,
  aiRequests: 892,
  quantumJobs: 156,
  apiCalls: 12847
}

const mockChartData = [
  { name: 'Mon', users: 40, requests: 240, ai: 120 },
  { name: 'Tue', users: 45, requests: 280, ai: 140 },
  { name: 'Wed', users: 55, requests: 420, ai: 210 },
  { name: 'Thu', users: 48, requests: 380, ai: 190 },
  { name: 'Fri', users: 62, requests: 520, ai: 260 },
  { name: 'Sat', users: 35, requests: 180, ai: 90 },
  { name: 'Sun', users: 28, requests: 140, ai: 70 },
]

const mockRecentActivity = [
  { id: 1, action: 'User registered', user: 'alice@quantum.io', time: '2 min ago', type: 'success' },
  { id: 2, action: 'AI request completed', user: 'bob@research.edu', time: '5 min ago', type: 'info' },
  { id: 3, action: 'Quantum job submitted', user: 'carol@enterprise.com', time: '12 min ago', type: 'info' },
  { id: 4, action: 'Failed login attempt', user: 'unknown@spam.net', time: '18 min ago', type: 'warning' },
  { id: 5, action: 'Pro tier upgrade', user: 'dave@startup.io', time: '1 hour ago', type: 'success' },
]

interface StatCardProps {
  title: string
  value: string | number
  icon: React.ElementType
  change?: number
  changeLabel?: string
}

function StatCard({ title, value, icon: Icon, change, changeLabel }: StatCardProps) {
  const isPositive = change && change > 0
  
  return (
    <div className="glass-card group">
      <div className="flex items-start justify-between">
        <div>
          <p className="stat-label">{title}</p>
          <p className="stat-value">{value}</p>
          {change !== undefined && (
            <div className={`flex items-center gap-1 text-sm mt-2 ${isPositive ? 'text-green-400' : 'text-red-400'}`}>
              {isPositive ? <ArrowUp className="w-3 h-3" /> : <ArrowDown className="w-3 h-3" />}
              <span>{Math.abs(change)}%</span>
              <span className="text-gray-500">{changeLabel}</span>
            </div>
          )}
        </div>
        <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-cyan-500/20 to-blue-600/20 flex items-center justify-center group-hover:from-cyan-500/30 group-hover:to-blue-600/30 transition-all">
          <Icon className="w-6 h-6 text-cyan-400" />
        </div>
      </div>
    </div>
  )
}

export default function Dashboard() {
  const [stats] = useState(mockStats)
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    // Simulate API call
    const timer = setTimeout(() => {
      setIsLoading(false)
    }, 500)
    return () => clearTimeout(timer)
  }, [])

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="spinner"></div>
      </div>
    )
  }

  return (
    <div className="space-y-6 animate-in">
      {/* Page Header */}
      <div>
        <h1 className="text-2xl font-bold mb-1">Dashboard</h1>
        <p className="text-gray-500">Welcome back! Here's what's happening.</p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard 
          title="Total Users" 
          value={stats.totalUsers.toLocaleString()} 
          icon={Users}
          change={12}
          changeLabel="vs last week"
        />
        <StatCard 
          title="Active Today" 
          value={stats.activeUsers.toLocaleString()} 
          icon={Activity}
          change={8}
          changeLabel="vs yesterday"
        />
        <StatCard 
          title="AI Requests" 
          value={stats.aiRequests.toLocaleString()} 
          icon={MessageSquare}
          change={24}
          changeLabel="vs last week"
        />
        <StatCard 
          title="Quantum Jobs" 
          value={stats.quantumJobs.toLocaleString()} 
          icon={Cpu}
          change={-5}
          changeLabel="vs last week"
        />
      </div>

      {/* Charts Row */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Activity Chart */}
        <div className="glass-card">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-lg font-semibold">Weekly Activity</h2>
            <select className="glass-input w-auto text-sm py-1.5 px-3">
              <option>Last 7 days</option>
              <option>Last 30 days</option>
              <option>Last 90 days</option>
            </select>
          </div>
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={mockChartData}>
                <defs>
                  <linearGradient id="colorUsers" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#00d4ff" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#00d4ff" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <XAxis dataKey="name" stroke="#666" fontSize={12} />
                <YAxis stroke="#666" fontSize={12} />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: 'rgba(20,20,20,0.9)', 
                    border: '1px solid rgba(255,255,255,0.1)',
                    borderRadius: '8px'
                  }}
                />
                <Area 
                  type="monotone" 
                  dataKey="requests" 
                  stroke="#00d4ff" 
                  fillOpacity={1} 
                  fill="url(#colorUsers)" 
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* AI vs Quantum Chart */}
        <div className="glass-card">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-lg font-semibold">AI vs Quantum Usage</h2>
            <div className="flex items-center gap-4 text-sm">
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 rounded-full bg-cyan-400"></div>
                <span className="text-gray-400">AI Requests</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 rounded-full bg-purple-400"></div>
                <span className="text-gray-400">Quantum Jobs</span>
              </div>
            </div>
          </div>
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={mockChartData}>
                <XAxis dataKey="name" stroke="#666" fontSize={12} />
                <YAxis stroke="#666" fontSize={12} />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: 'rgba(20,20,20,0.9)', 
                    border: '1px solid rgba(255,255,255,0.1)',
                    borderRadius: '8px'
                  }}
                />
                <Bar dataKey="ai" fill="#00d4ff" radius={[4, 4, 0, 0]} />
                <Bar dataKey="users" fill="#a855f7" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      {/* Recent Activity & Quick Stats */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Recent Activity */}
        <div className="lg:col-span-2 glass-card">
          <h2 className="text-lg font-semibold mb-4">Recent Activity</h2>
          <div className="space-y-3">
            {mockRecentActivity.map((activity) => (
              <div 
                key={activity.id} 
                className="flex items-center justify-between p-3 rounded-lg bg-white/3 hover:bg-white/5 transition-colors"
              >
                <div className="flex items-center gap-3">
                  <div className={`w-2 h-2 rounded-full ${
                    activity.type === 'success' ? 'bg-green-400' :
                    activity.type === 'warning' ? 'bg-yellow-400' :
                    'bg-cyan-400'
                  }`}></div>
                  <div>
                    <p className="text-sm font-medium">{activity.action}</p>
                    <p className="text-xs text-gray-500">{activity.user}</p>
                  </div>
                </div>
                <span className="text-xs text-gray-500">{activity.time}</span>
              </div>
            ))}
          </div>
        </div>

        {/* System Health */}
        <div className="glass-card">
          <h2 className="text-lg font-semibold mb-4">System Health</h2>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-400">API Status</span>
              <span className="badge badge-success">Healthy</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-400">Database</span>
              <span className="badge badge-success">Connected</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-400">Cloudflare AI</span>
              <span className="badge badge-success">Active</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-400">Rate Limiting</span>
              <span className="badge badge-warning">75% capacity</span>
            </div>
            
            <div className="pt-4 border-t border-white/5">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm text-gray-400">Uptime</span>
                <span className="text-sm font-medium text-green-400">99.98%</span>
              </div>
              <div className="h-2 bg-white/5 rounded-full overflow-hidden">
                <div className="h-full bg-gradient-to-r from-green-400 to-cyan-400" style={{ width: '99.98%' }}></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
