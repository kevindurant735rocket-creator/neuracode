import { useState, useEffect } from 'react'
import { 
  FileCode, 
  GitBranch, 
  Zap, 
  TrendingUp,
  Activity,
  Clock,
  CheckCircle,
  AlertCircle
} from 'lucide-react'
import { Line, Doughnut } from 'recharts'

const stats = [
  { label: 'Files Indexed', value: '1,234', icon: FileCode, color: 'blue' },
  { label: 'Code Nodes', value: '5,678', icon: GitBranch, color: 'purple' },
  { label: 'Queries Today', value: '89', icon: Zap, color: 'green' },
  { label: 'Avg Response', value: '12ms', icon: TrendingUp, color: 'orange' },
]

const activityData = [
  { name: 'Mon', queries: 45 },
  { name: 'Tue', queries: 52 },
  { name: 'Wed', queries: 49 },
  { name: 'Thu', queries: 63 },
  { name: 'Fri', queries: 58 },
  { name: 'Sat', queries: 32 },
  { name: 'Sun', queries: 28 },
]

const languageData = [
  { name: 'TypeScript', value: 45, color: '#3178c6' },
  { name: 'Python', value: 30, color: '#3776ab' },
  { name: 'Rust', value: 15, color: '#dea584' },
  { name: 'Go', value: 10, color: '#00add8' },
]

export default function Dashboard() {
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    // Simulate loading
    setTimeout(() => setIsLoading(false), 1000)
  }, [])

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
      </div>
    )
  }

  return (
    <div className="p-8 animate-fade-in">
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Dashboard</h1>
        <p className="text-slate-400">Welcome to NeuraCode - Your AI Agent's Second Brain</p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        {stats.map((stat) => (
          <div 
            key={stat.label}
            className="glass rounded-xl p-6 hover:scale-105 transition-transform duration-200"
          >
            <div className="flex items-center justify-between mb-4">
              <div className={`w-12 h-12 rounded-lg bg-${stat.color}-500/20 flex items-center justify-center`}>
                <stat.icon className={`w-6 h-6 text-${stat.color}-500`} />
              </div>
              <Activity className="w-4 h-4 text-slate-500" />
            </div>
            <p className="text-3xl font-bold mb-1">{stat.value}</p>
            <p className="text-sm text-slate-400">{stat.label}</p>
          </div>
        ))}
      </div>

      {/* Charts Row */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        {/* Activity Chart */}
        <div className="lg:col-span-2 glass rounded-xl p-6">
          <h3 className="text-lg font-semibold mb-4">Query Activity</h3>
          <div className="h-64">
            {/* Chart would go here */}
            <div className="flex items-end justify-between h-full gap-2">
              {activityData.map((item) => (
                <div key={item.name} className="flex-1 flex flex-col items-center gap-2">
                  <div 
                    className="w-full bg-blue-500/30 rounded-t-lg transition-all duration-300 hover:bg-blue-500/50"
                    style={{ height: `${(item.queries / 70) * 100}%` }}
                  ></div>
                  <span className="text-xs text-slate-400">{item.name}</span>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Language Distribution */}
        <div className="glass rounded-xl p-6">
          <h3 className="text-lg font-semibold mb-4">Languages</h3>
          <div className="space-y-4">
            {languageData.map((lang) => (
              <div key={lang.name}>
                <div className="flex justify-between text-sm mb-1">
                  <span>{lang.name}</span>
                  <span className="text-slate-400">{lang.value}%</span>
                </div>
                <div className="h-2 bg-slate-700 rounded-full overflow-hidden">
                  <div 
                    className="h-full rounded-full transition-all duration-500"
                    style={{ 
                      width: `${lang.value}%`,
                      backgroundColor: lang.color 
                    }}
                  ></div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Recent Activity */}
      <div className="glass rounded-xl p-6">
        <h3 className="text-lg font-semibold mb-4">Recent Activity</h3>
        <div className="space-y-4">
          {[
            { type: 'success', message: 'Indexed 15 new files', time: '2 minutes ago' },
            { type: 'info', message: 'Search query: "authentication"', time: '5 minutes ago' },
            { type: 'success', message: 'Impact analysis completed', time: '10 minutes ago' },
            { type: 'warning', message: 'High complexity detected in auth.ts', time: '15 minutes ago' },
          ].map((activity, i) => (
            <div key={i} className="flex items-center gap-4 p-3 rounded-lg hover:bg-slate-800/50 transition-colors">
              {activity.type === 'success' && <CheckCircle className="w-5 h-5 text-green-500" />}
              {activity.type === 'info' && <Clock className="w-5 h-5 text-blue-500" />}
              {activity.type === 'warning' && <AlertCircle className="w-5 h-5 text-yellow-500" />}
              <div className="flex-1">
                <p className="text-sm">{activity.message}</p>
                <p className="text-xs text-slate-500">{activity.time}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
