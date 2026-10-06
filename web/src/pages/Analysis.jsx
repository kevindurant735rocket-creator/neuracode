import { useState } from 'react'
import { BarChart3, AlertTriangle, CheckCircle, XCircle, TrendingUp } from 'lucide-react'

const issues = [
  { severity: 'error', message: 'Unused variable in auth.ts:42', file: 'src/auth.ts', line: 42 },
  { severity: 'warning', message: 'High complexity in user-service.ts', file: 'src/services/user.ts', line: 156 },
  { severity: 'warning', message: 'Missing error handling in api.ts', file: 'src/api.ts', line: 89 },
  { severity: 'info', message: 'Consider adding tests for new-feature.ts', file: 'src/new-feature.ts', line: 1 },
]

const metrics = [
  { label: 'Code Coverage', value: 78, target: 80, unit: '%' },
  { label: 'Avg Complexity', value: 4.2, target: 5.0, unit: '' },
  { label: 'Technical Debt', value: 12, target: 10, unit: ' days' },
  { label: 'Code Duplication', value: 3.5, target: 5.0, unit: '%' },
]

export default function Analysis() {
  const [selectedTab, setSelectedTab] = useState('overview')

  return (
    <div className="p-8 animate-fade-in">
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Analysis</h1>
        <p className="text-slate-400">Deep code analysis and insights</p>
      </div>

      {/* Tabs */}
      <div className="flex gap-2 mb-6">
        {['overview', 'issues', 'metrics', 'recommendations'].map((tab) => (
          <button
            key={tab}
            onClick={() => setSelectedTab(tab)}
            className={`px-4 py-2 rounded-lg font-medium transition-colors ${
              selectedTab === tab
                ? 'bg-blue-600 text-white'
                : 'bg-slate-800 text-slate-400 hover:bg-slate-700'
            }`}
          >
            {tab.charAt(0).toUpperCase() + tab.slice(1)}
          </button>
        ))}
      </div>

      {/* Overview Tab */}
      {selectedTab === 'overview' && (
        <div className="space-y-6">
          {/* Metrics Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {metrics.map((metric) => (
              <div key={metric.label} className="glass rounded-xl p-6">
                <div className="flex items-center justify-between mb-4">
                  <span className="text-slate-400 text-sm">{metric.label}</span>
                  <TrendingUp className="w-4 h-4 text-slate-500" />
                </div>
                <p className="text-3xl font-bold mb-2">
                  {metric.value}{metric.unit}
                </p>
                <div className="h-2 bg-slate-700 rounded-full overflow-hidden">
                  <div 
                    className={`h-full rounded-full ${
                      metric.value <= metric.target ? 'bg-green-500' : 'bg-yellow-500'
                    }`}
                    style={{ width: `${(metric.value / metric.target) * 100}%` }}
                  ></div>
                </div>
                <p className="text-xs text-slate-500 mt-2">
                  Target: {metric.target}{metric.unit}
                </p>
              </div>
            ))}
          </div>

          {/* Summary */}
          <div className="glass rounded-xl p-6">
            <h3 className="text-lg font-semibold mb-4">Summary</h3>
            <div className="grid grid-cols-3 gap-6">
              <div className="text-center">
                <div className="w-16 h-16 rounded-full bg-green-500/20 flex items-center justify-center mx-auto mb-2">
                  <CheckCircle className="w-8 h-8 text-green-500" />
                </div>
                <p className="text-2xl font-bold text-green-500">156</p>
                <p className="text-sm text-slate-400">Passed</p>
              </div>
              <div className="text-center">
                <div className="w-16 h-16 rounded-full bg-yellow-500/20 flex items-center justify-center mx-auto mb-2">
                  <AlertTriangle className="w-8 h-8 text-yellow-500" />
                </div>
                <p className="text-2xl font-bold text-yellow-500">12</p>
                <p className="text-sm text-slate-400">Warnings</p>
              </div>
              <div className="text-center">
                <div className="w-16 h-16 rounded-full bg-red-500/20 flex items-center justify-center mx-auto mb-2">
                  <XCircle className="w-8 h-8 text-red-500" />
                </div>
                <p className="text-2xl font-bold text-red-500">3</p>
                <p className="text-sm text-slate-400">Errors</p>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Issues Tab */}
      {selectedTab === 'issues' && (
        <div className="glass rounded-xl overflow-hidden">
          <div className="divide-y divide-slate-700">
            {issues.map((issue, i) => (
              <div key={i} className="p-4 hover:bg-slate-800/50 transition-colors">
                <div className="flex items-start gap-4">
                  {issue.severity === 'error' && <XCircle className="w-5 h-5 text-red-500 flex-shrink-0 mt-0.5" />}
                  {issue.severity === 'warning' && <AlertTriangle className="w-5 h-5 text-yellow-500 flex-shrink-0 mt-0.5" />}
                  {issue.severity === 'info' && <CheckCircle className="w-5 h-5 text-blue-500 flex-shrink-0 mt-0.5" />}
                  <div className="flex-1">
                    <p className="font-medium">{issue.message}</p>
                    <p className="text-sm text-slate-400">{issue.file}:{issue.line}</p>
                  </div>
                  <span className={`px-2 py-1 rounded text-xs ${
                    issue.severity === 'error' ? 'bg-red-500/20 text-red-400' :
                    issue.severity === 'warning' ? 'bg-yellow-500/20 text-yellow-400' :
                    'bg-blue-500/20 text-blue-400'
                  }`}>
                    {issue.severity}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Metrics Tab */}
      {selectedTab === 'metrics' && (
        <div className="glass rounded-xl p-6">
          <h3 className="text-lg font-semibold mb-6">Detailed Metrics</h3>
          <div className="space-y-6">
            {metrics.map((metric) => (
              <div key={metric.label}>
                <div className="flex justify-between mb-2">
                  <span>{metric.label}</span>
                  <span className="text-slate-400">
                    {metric.value}{metric.unit} / {metric.target}{metric.unit}
                  </span>
                </div>
                <div className="h-3 bg-slate-700 rounded-full overflow-hidden">
                  <div 
                    className={`h-full rounded-full transition-all duration-500 ${
                      metric.value <= metric.target ? 'bg-green-500' : 'bg-yellow-500'
                    }`}
                    style={{ width: `${Math.min((metric.value / metric.target) * 100, 100)}%` }}
                  ></div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Recommendations Tab */}
      {selectedTab === 'recommendations' && (
        <div className="space-y-4">
          {[
            { priority: 'high', title: 'Add error handling to API endpoints', impact: 'Improves reliability' },
            { priority: 'high', title: 'Increase test coverage for auth module', impact: 'Reduces bugs' },
            { priority: 'medium', title: 'Refactor user-service.ts to reduce complexity', impact: 'Improves maintainability' },
            { priority: 'medium', title: 'Add input validation to forms', impact: 'Security improvement' },
            { priority: 'low', title: 'Update documentation for public APIs', impact: 'Better developer experience' },
          ].map((rec, i) => (
            <div key={i} className="glass rounded-xl p-4 flex items-center justify-between">
              <div>
                <p className="font-medium">{rec.title}</p>
                <p className="text-sm text-slate-400">{rec.impact}</p>
              </div>
              <span className={`px-3 py-1 rounded-full text-xs ${
                rec.priority === 'high' ? 'bg-red-500/20 text-red-400' :
                rec.priority === 'medium' ? 'bg-yellow-500/20 text-yellow-400' :
                'bg-green-500/20 text-green-400'
              }`}>
                {rec.priority}
              </span>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
