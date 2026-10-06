import { NavLink } from 'react-router-dom'
import { 
  LayoutDashboard, 
  Search, 
  GitBranch, 
  BarChart3, 
  Settings,
  ChevronLeft,
  ChevronRight,
  Brain
} from 'lucide-react'
import clsx from 'clsx'

const navItems = [
  { path: '/', icon: LayoutDashboard, label: 'Dashboard' },
  { path: '/search', icon: Search, label: 'Search' },
  { path: '/graph', icon: GitBranch, label: 'Code Graph' },
  { path: '/analysis', icon: BarChart3, label: 'Analysis' },
  { path: '/settings', icon: Settings, label: 'Settings' },
]

export default function Sidebar({ isOpen, onToggle }) {
  return (
    <aside 
      className={clsx(
        'fixed left-0 top-0 h-full bg-slate-900 border-r border-slate-700 transition-all duration-300 z-50',
        isOpen ? 'w-64' : 'w-16'
      )}
    >
      {/* Logo */}
      <div className="flex items-center gap-3 p-4 border-b border-slate-700">
        <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center">
          <Brain className="w-6 h-6 text-white" />
        </div>
        {isOpen && (
          <div className="animate-fade-in">
            <h1 className="text-lg font-bold gradient-text">NeuraCode</h1>
            <p className="text-xs text-slate-400">Cognitive Enhancement</p>
          </div>
        )}
      </div>

      {/* Navigation */}
      <nav className="p-2 space-y-1">
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) => clsx(
              'flex items-center gap-3 px-3 py-2.5 rounded-lg transition-all duration-200',
              isActive 
                ? 'bg-blue-600 text-white' 
                : 'text-slate-400 hover:bg-slate-800 hover:text-white'
            )}
          >
            <item.icon className="w-5 h-5 flex-shrink-0" />
            {isOpen && <span className="text-sm font-medium">{item.label}</span>}
          </NavLink>
        ))}
      </nav>

      {/* Toggle button */}
      <button
        onClick={onToggle}
        className="absolute -right-3 top-20 w-6 h-6 rounded-full bg-slate-700 border border-slate-600 flex items-center justify-center text-slate-400 hover:text-white hover:bg-slate-600 transition-colors"
      >
        {isOpen ? <ChevronLeft className="w-4 h-4" /> : <ChevronRight className="w-4 h-4" />}
      </button>
    </aside>
  )
}
