import { useState, useEffect } from 'react'
import { 
  Search, 
  Filter, 
  MoreVertical, 
  UserPlus, 
  Download,
  Mail,
  Calendar,
  Shield
} from 'lucide-react'

// Mock user data
const mockUsers = [
  { id: '1', email: 'alice@quantum.io', username: 'alice', tier: 'pro', createdAt: '2026-01-15', lastActive: '2 hours ago', messages: 234, jobs: 12 },
  { id: '2', email: 'bob@research.edu', username: 'bob_researcher', tier: 'enterprise', createdAt: '2026-01-10', lastActive: '5 min ago', messages: 892, jobs: 45 },
  { id: '3', email: 'carol@enterprise.com', username: 'carol', tier: 'free', createdAt: '2026-01-20', lastActive: '1 day ago', messages: 56, jobs: 3 },
  { id: '4', email: 'dave@startup.io', username: 'dave_quantum', tier: 'pro', createdAt: '2026-01-18', lastActive: '3 hours ago', messages: 178, jobs: 8 },
  { id: '5', email: 'eve@physics.lab', username: 'eve_physics', tier: 'free', createdAt: '2026-01-25', lastActive: '12 hours ago', messages: 23, jobs: 1 },
  { id: '6', email: 'frank@bigcorp.com', username: 'frank_admin', tier: 'enterprise', createdAt: '2026-01-05', lastActive: '30 min ago', messages: 1205, jobs: 89 },
  { id: '7', email: 'grace@university.edu', username: 'grace_phd', tier: 'free', createdAt: '2026-01-28', lastActive: '2 days ago', messages: 15, jobs: 0 },
  { id: '8', email: 'henry@tech.co', username: 'henry_dev', tier: 'pro', createdAt: '2026-01-12', lastActive: '1 hour ago', messages: 445, jobs: 22 },
]

interface User {
  id: string
  email: string
  username: string
  tier: string
  createdAt: string
  lastActive: string
  messages: number
  jobs: number
}

export default function Users() {
  const [users, setUsers] = useState<User[]>([])
  const [filteredUsers, setFilteredUsers] = useState<User[]>([])
  const [searchQuery, setSearchQuery] = useState('')
  const [tierFilter, setTierFilter] = useState('all')
  const [selectedUser, setSelectedUser] = useState<User | null>(null)
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    // Simulate API call
    setTimeout(() => {
      setUsers(mockUsers)
      setFilteredUsers(mockUsers)
      setIsLoading(false)
    }, 500)
  }, [])

  useEffect(() => {
    let result = users

    // Apply search filter
    if (searchQuery) {
      const query = searchQuery.toLowerCase()
      result = result.filter(user => 
        user.email.toLowerCase().includes(query) ||
        user.username.toLowerCase().includes(query)
      )
    }

    // Apply tier filter
    if (tierFilter !== 'all') {
      result = result.filter(user => user.tier === tierFilter)
    }

    setFilteredUsers(result)
  }, [searchQuery, tierFilter, users])

  const getTierBadge = (tier: string) => {
    switch (tier) {
      case 'enterprise':
        return <span className="badge badge-info">Enterprise</span>
      case 'pro':
        return <span className="badge badge-success">Pro</span>
      default:
        return <span className="badge" style={{ background: 'rgba(255,255,255,0.1)', border: '1px solid rgba(255,255,255,0.2)' }}>Free</span>
    }
  }

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
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold mb-1">Users</h1>
          <p className="text-gray-500">{users.length} total users registered</p>
        </div>
        <div className="flex items-center gap-3">
          <button className="glass-button flex items-center gap-2">
            <Download className="w-4 h-4" />
            Export
          </button>
          <button className="glass-button-primary flex items-center gap-2">
            <UserPlus className="w-4 h-4" />
            Add User
          </button>
        </div>
      </div>

      {/* Filters */}
      <div className="glass-card">
        <div className="flex flex-wrap items-center gap-4">
          {/* Search */}
          <div className="relative flex-1 min-w-64">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-500" />
            <input
              type="text"
              placeholder="Search by email or username..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="glass-input pl-10"
            />
          </div>

          {/* Tier Filter */}
          <div className="flex items-center gap-2">
            <Filter className="w-4 h-4 text-gray-500" />
            <select
              value={tierFilter}
              onChange={(e) => setTierFilter(e.target.value)}
              className="glass-input w-auto"
            >
              <option value="all">All Tiers</option>
              <option value="free">Free</option>
              <option value="pro">Pro</option>
              <option value="enterprise">Enterprise</option>
            </select>
          </div>

          {/* Results count */}
          <span className="text-sm text-gray-500">
            Showing {filteredUsers.length} of {users.length}
          </span>
        </div>
      </div>

      {/* Users Table */}
      <div className="glass-card overflow-hidden p-0">
        <table className="glass-table">
          <thead>
            <tr>
              <th>User</th>
              <th>Tier</th>
              <th>Messages</th>
              <th>Quantum Jobs</th>
              <th>Last Active</th>
              <th>Joined</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {filteredUsers.map((user) => (
              <tr 
                key={user.id}
                className="cursor-pointer"
                onClick={() => setSelectedUser(user)}
              >
                <td>
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-full bg-gradient-to-br from-cyan-500/20 to-blue-600/20 flex items-center justify-center">
                      <span className="text-sm font-medium text-cyan-400">
                        {user.username.slice(0, 2).toUpperCase()}
                      </span>
                    </div>
                    <div>
                      <p className="font-medium">{user.username}</p>
                      <p className="text-sm text-gray-500">{user.email}</p>
                    </div>
                  </div>
                </td>
                <td>{getTierBadge(user.tier)}</td>
                <td>{user.messages.toLocaleString()}</td>
                <td>{user.jobs}</td>
                <td className="text-gray-400">{user.lastActive}</td>
                <td className="text-gray-400">{user.createdAt}</td>
                <td>
                  <button 
                    className="p-2 rounded-lg hover:bg-white/5 transition-colors"
                    onClick={(e) => {
                      e.stopPropagation()
                      // Show menu
                    }}
                  >
                    <MoreVertical className="w-4 h-4 text-gray-500" />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>

        {filteredUsers.length === 0 && (
          <div className="text-center py-12 text-gray-500">
            No users found matching your criteria
          </div>
        )}
      </div>

      {/* User Details Slide-over */}
      {selectedUser && (
        <div 
          className="fixed inset-0 bg-black/50 z-50 flex justify-end"
          onClick={() => setSelectedUser(null)}
        >
          <div 
            className="w-full max-w-md glass-sidebar h-full animate-slide-in overflow-auto"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="p-6 space-y-6">
              {/* Header */}
              <div className="flex items-start justify-between">
                <div className="flex items-center gap-4">
                  <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-cyan-500/20 to-blue-600/20 flex items-center justify-center">
                    <span className="text-xl font-bold text-cyan-400">
                      {selectedUser.username.slice(0, 2).toUpperCase()}
                    </span>
                  </div>
                  <div>
                    <h2 className="text-xl font-bold">{selectedUser.username}</h2>
                    <p className="text-gray-500">{selectedUser.email}</p>
                  </div>
                </div>
                <button 
                  onClick={() => setSelectedUser(null)}
                  className="text-gray-500 hover:text-white transition-colors"
                >
                  ×
                </button>
              </div>

              {/* Tier */}
              <div className="flex items-center gap-3">
                <Shield className="w-5 h-5 text-gray-500" />
                <span className="text-gray-400">Tier:</span>
                {getTierBadge(selectedUser.tier)}
              </div>

              {/* Stats */}
              <div className="grid grid-cols-2 gap-4">
                <div className="glass-card">
                  <p className="stat-label">Messages</p>
                  <p className="text-2xl font-bold">{selectedUser.messages.toLocaleString()}</p>
                </div>
                <div className="glass-card">
                  <p className="stat-label">Quantum Jobs</p>
                  <p className="text-2xl font-bold">{selectedUser.jobs}</p>
                </div>
              </div>

              {/* Activity */}
              <div className="space-y-3">
                <div className="flex items-center gap-3 text-sm">
                  <Calendar className="w-4 h-4 text-gray-500" />
                  <span className="text-gray-400">Joined:</span>
                  <span>{selectedUser.createdAt}</span>
                </div>
                <div className="flex items-center gap-3 text-sm">
                  <Mail className="w-4 h-4 text-gray-500" />
                  <span className="text-gray-400">Last active:</span>
                  <span>{selectedUser.lastActive}</span>
                </div>
              </div>

              {/* Actions */}
              <div className="pt-4 border-t border-white/5 space-y-2">
                <button className="glass-button w-full">View Full History</button>
                <button className="glass-button w-full">Change Tier</button>
                <button className="glass-button w-full text-red-400 hover:bg-red-500/10">
                  Suspend User
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
