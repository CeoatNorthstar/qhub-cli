import { useState } from 'react'
import { 
  TrendingUp, 
  TrendingDown,
  Clock,
  Zap,
  Database,
  Globe
} from 'lucide-react'
import { 
  LineChart, 
  Line, 
  AreaChart, 
  Area, 
  XAxis, 
  YAxis, 
  Tooltip, 
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell
} from 'recharts'

// Mock data
const apiUsageData = [
  { time: '00:00', requests: 120, errors: 2 },
  { time: '04:00', requests: 80, errors: 1 },
  { time: '08:00', requests: 340, errors: 5 },
  { time: '12:00', requests: 580, errors: 8 },
  { time: '16:00', requests: 620, errors: 6 },
  { time: '20:00', requests: 420, errors: 3 },
  { time: '24:00', requests: 180, errors: 2 },
]

const endpointStats = [
  { endpoint: '/auth/login', calls: 2847, avgTime: '45ms', status: 'healthy' },
  { endpoint: '/auth/register', calls: 892, avgTime: '120ms', status: 'healthy' },
  { endpoint: '/ai/chat', calls: 5623, avgTime: '2.3s', status: 'healthy' },
  { endpoint: '/quantum/submit', calls: 156, avgTime: '850ms', status: 'healthy' },
  { endpoint: '/health', calls: 12847, avgTime: '5ms', status: 'healthy' },
]

const tierDistribution = [
  { name: 'Free', value: 156, color: '#6b7280' },
  { name: 'Pro', value: 72, color: '#22c55e' },
  { name: 'Enterprise', value: 19, color: '#00d4ff' },
]

const errorRates = [
  { hour: '1h', rate: 0.2 },
  { hour: '2h', rate: 0.3 },
  { hour: '3h', rate: 0.1 },
  { hour: '4h', rate: 0.5 },
  { hour: '5h', rate: 0.2 },
  { hour: '6h', rate: 0.1 },
  { hour: '7h', rate: 0.4 },
  { hour: '8h', rate: 0.3 },
  { hour: '9h', rate: 0.2 },
  { hour: '10h', rate: 0.6 },
  { hour: '11h', rate: 0.3 },
  { hour: '12h', rate: 0.2 },
]

export default function Analytics() {
  const [timeRange, setTimeRange] = useState('24h')

  return (
    <div className="space-y-6 animate-in">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold mb-1">Analytics</h1>
          <p className="text-gray-500">API usage, performance metrics, and insights</p>
        </div>
        <select 
          value={timeRange}
          onChange={(e) => setTimeRange(e.target.value)}
          className="glass-input w-auto"
        >
          <option value="1h">Last hour</option>
          <option value="24h">Last 24 hours</option>
          <option value="7d">Last 7 days</option>
          <option value="30d">Last 30 days</option>
        </select>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="glass-card">
          <div className="flex items-center gap-2 text-gray-400 mb-2">
            <Globe className="w-4 h-4" />
            <span className="text-sm">Total Requests</span>
          </div>
          <p className="text-3xl font-bold">12,847</p>
          <div className="flex items-center gap-1 text-sm text-green-400 mt-1">
            <TrendingUp className="w-3 h-3" />
            <span>+18.2% vs yesterday</span>
          </div>
        </div>

        <div className="glass-card">
          <div className="flex items-center gap-2 text-gray-400 mb-2">
            <Clock className="w-4 h-4" />
            <span className="text-sm">Avg Response Time</span>
          </div>
          <p className="text-3xl font-bold">245<span className="text-lg text-gray-500">ms</span></p>
          <div className="flex items-center gap-1 text-sm text-green-400 mt-1">
            <TrendingDown className="w-3 h-3" />
            <span>-12ms vs yesterday</span>
          </div>
        </div>

        <div className="glass-card">
          <div className="flex items-center gap-2 text-gray-400 mb-2">
            <Zap className="w-4 h-4" />
            <span className="text-sm">Error Rate</span>
          </div>
          <p className="text-3xl font-bold">0.24<span className="text-lg text-gray-500">%</span></p>
          <div className="flex items-center gap-1 text-sm text-green-400 mt-1">
            <TrendingDown className="w-3 h-3" />
            <span>-0.05% vs yesterday</span>
          </div>
        </div>

        <div className="glass-card">
          <div className="flex items-center gap-2 text-gray-400 mb-2">
            <Database className="w-4 h-4" />
            <span className="text-sm">AI Tokens Used</span>
          </div>
          <p className="text-3xl font-bold">1.2<span className="text-lg text-gray-500">M</span></p>
          <div className="flex items-center gap-1 text-sm text-yellow-400 mt-1">
            <TrendingUp className="w-3 h-3" />
            <span>+32% vs yesterday</span>
          </div>
        </div>
      </div>

      {/* Charts Row */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* API Usage Chart */}
        <div className="lg:col-span-2 glass-card">
          <h2 className="text-lg font-semibold mb-6">API Requests Over Time</h2>
          <div className="h-72">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={apiUsageData}>
                <defs>
                  <linearGradient id="colorRequests" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#00d4ff" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#00d4ff" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <XAxis dataKey="time" stroke="#666" fontSize={12} />
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
                  strokeWidth={2}
                  fillOpacity={1} 
                  fill="url(#colorRequests)" 
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Tier Distribution */}
        <div className="glass-card">
          <h2 className="text-lg font-semibold mb-6">User Distribution</h2>
          <div className="h-48">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={tierDistribution}
                  cx="50%"
                  cy="50%"
                  innerRadius={50}
                  outerRadius={70}
                  paddingAngle={5}
                  dataKey="value"
                >
                  {tierDistribution.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: 'rgba(20,20,20,0.9)', 
                    border: '1px solid rgba(255,255,255,0.1)',
                    borderRadius: '8px'
                  }}
                />
              </PieChart>
            </ResponsiveContainer>
          </div>
          <div className="flex justify-center gap-4 mt-4">
            {tierDistribution.map((tier) => (
              <div key={tier.name} className="flex items-center gap-2">
                <div className="w-3 h-3 rounded-full" style={{ backgroundColor: tier.color }}></div>
                <span className="text-sm text-gray-400">{tier.name}: {tier.value}</span>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Error Rate Chart */}
      <div className="glass-card">
        <h2 className="text-lg font-semibold mb-6">Error Rate (Last 12 Hours)</h2>
        <div className="h-48">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={errorRates}>
              <XAxis dataKey="hour" stroke="#666" fontSize={12} />
              <YAxis stroke="#666" fontSize={12} unit="%" />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: 'rgba(20,20,20,0.9)', 
                  border: '1px solid rgba(255,255,255,0.1)',
                  borderRadius: '8px'
                }}
                formatter={(value) => [`${value}%`, 'Error Rate']}
              />
              <Line 
                type="monotone" 
                dataKey="rate" 
                stroke="#ef4444" 
                strokeWidth={2}
                dot={{ fill: '#ef4444', strokeWidth: 2 }}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Endpoint Stats Table */}
      <div className="glass-card overflow-hidden p-0">
        <div className="p-4 border-b border-white/5">
          <h2 className="text-lg font-semibold">Endpoint Performance</h2>
        </div>
        <table className="glass-table">
          <thead>
            <tr>
              <th>Endpoint</th>
              <th>Total Calls</th>
              <th>Avg Response Time</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            {endpointStats.map((endpoint) => (
              <tr key={endpoint.endpoint}>
                <td>
                  <code className="text-cyan-400 bg-cyan-400/10 px-2 py-1 rounded text-sm">
                    {endpoint.endpoint}
                  </code>
                </td>
                <td>{endpoint.calls.toLocaleString()}</td>
                <td>{endpoint.avgTime}</td>
                <td>
                  <span className="badge badge-success">{endpoint.status}</span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}
