import { useState } from 'react'
import { Eye, EyeOff, Zap, Lock, Mail } from 'lucide-react'

interface LoginProps {
  onLogin: (token: string) => void
}

export default function Login({ onLogin }: LoginProps) {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsLoading(true)
    setError('')

    // Admin authentication
    // Default: founder@axionslab.com with strong password
    const ADMIN_EMAIL = 'founder@axionslab.com'
    const ADMIN_PASSWORD = 'Axions$ecure2026!'

    // Simulate API call delay
    await new Promise(resolve => setTimeout(resolve, 1000))

    if (email === ADMIN_EMAIL && password === ADMIN_PASSWORD) {
      // Generate a simple token (in production, this would come from backend)
      const token = btoa(JSON.stringify({ 
        email: ADMIN_EMAIL, 
        role: 'admin',
        exp: Date.now() + 24 * 60 * 60 * 1000 // 24 hours
      }))
      onLogin(token)
    } else {
      setError('Invalid credentials. Access denied.')
      setIsLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-gradient-animated flex items-center justify-center p-4 noise-overlay">
      {/* Decorative elements */}
      <div className="fixed top-1/4 left-1/4 w-96 h-96 bg-cyan-500/10 rounded-full blur-3xl"></div>
      <div className="fixed bottom-1/4 right-1/4 w-96 h-96 bg-purple-500/5 rounded-full blur-3xl"></div>

      <div className="w-full max-w-md animate-in">
        {/* Logo Section */}
        <div className="text-center mb-8">
          <div className="inline-flex items-center justify-center w-16 h-16 rounded-2xl glass mb-4 glow-accent">
            <Zap className="w-8 h-8 text-cyan-400" />
          </div>
          <h1 className="text-3xl font-bold mb-2 bg-gradient-to-r from-white to-gray-400 bg-clip-text text-transparent">
            QHub Admin
          </h1>
          <p className="text-gray-500 text-sm">Enterprise Control Center</p>
        </div>

        {/* Login Card */}
        <div className="glass-card">
          <form onSubmit={handleSubmit} className="space-y-6">
            {/* Email Input */}
            <div className="space-y-2">
              <label className="text-sm text-gray-400 flex items-center gap-2">
                <Mail className="w-4 h-4" />
                Email Address
              </label>
              <input
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder="admin@company.com"
                className="glass-input"
                required
              />
            </div>

            {/* Password Input */}
            <div className="space-y-2">
              <label className="text-sm text-gray-400 flex items-center gap-2">
                <Lock className="w-4 h-4" />
                Password
              </label>
              <div className="relative">
                <input
                  type={showPassword ? 'text' : 'password'}
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="••••••••••••"
                  className="glass-input pr-12"
                  required
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-300 transition-colors"
                >
                  {showPassword ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
                </button>
              </div>
            </div>

            {/* Error Message */}
            {error && (
              <div className="p-3 rounded-lg bg-red-500/10 border border-red-500/30 text-red-400 text-sm animate-slide-in">
                {error}
              </div>
            )}

            {/* Submit Button */}
            <button
              type="submit"
              disabled={isLoading}
              className="w-full glass-button-primary py-3 flex items-center justify-center gap-2 disabled:opacity-50"
            >
              {isLoading ? (
                <>
                  <div className="spinner"></div>
                  <span>Authenticating...</span>
                </>
              ) : (
                <>
                  <Lock className="w-4 h-4" />
                  <span>Access Dashboard</span>
                </>
              )}
            </button>
          </form>

          {/* Footer */}
          <div className="mt-6 pt-6 border-t border-white/5 text-center">
            <p className="text-gray-600 text-xs">
              Protected by enterprise-grade security
            </p>
          </div>
        </div>

        {/* Version */}
        <p className="text-center text-gray-700 text-xs mt-6">
          QHub Admin v1.0.0 • Powered by Cloudflare
        </p>
      </div>
    </div>
  )
}
