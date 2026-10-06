import { useState } from 'react'
import { GitBranch, ZoomIn, ZoomOut, Maximize2, RefreshCw } from 'lucide-react'

export default function Graph() {
  const [isLoading, setIsLoading] = useState(false)

  return (
    <div className="p-8 h-full flex flex-col animate-fade-in">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-3xl font-bold mb-2">Code Graph</h1>
          <p className="text-slate-400">Visualize your codebase structure</p>
        </div>
        <div className="flex gap-2">
          <button className="p-2 rounded-lg bg-slate-800 hover:bg-slate-700 transition-colors">
            <ZoomIn className="w-5 h-5" />
          </button>
          <button className="p-2 rounded-lg bg-slate-800 hover:bg-slate-700 transition-colors">
            <ZoomOut className="w-5 h-5" />
          </button>
          <button className="p-2 rounded-lg bg-slate-800 hover:bg-slate-700 transition-colors">
            <Maximize2 className="w-5 h-5" />
          </button>
          <button 
            onClick={() => setIsLoading(true)}
            className="p-2 rounded-lg bg-slate-800 hover:bg-slate-700 transition-colors"
          >
            <RefreshCw className={`w-5 h-5 ${isLoading ? 'animate-spin' : ''}`} />
          </button>
        </div>
      </div>

      {/* Graph Container */}
      <div className="flex-1 glass rounded-xl overflow-hidden relative">
        {isLoading ? (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <div className="w-12 h-12 border-4 border-blue-500/30 border-t-blue-500 rounded-full animate-spin mx-auto mb-4"></div>
              <p className="text-slate-400">Loading code graph...</p>
            </div>
          </div>
        ) : (
          <div className="flex flex-col items-center justify-center h-full text-slate-500">
            <GitBranch className="w-24 h-24 mb-4 opacity-30" />
            <p className="text-lg mb-2">Code Graph Visualization</p>
            <p className="text-sm">Interactive graph will be rendered here</p>
            <p className="text-xs mt-4 text-slate-600">
              Showing relationships between functions, classes, and modules
            </p>
          </div>
        )}

        {/* Legend */}
        <div className="absolute bottom-4 left-4 glass rounded-lg p-4">
          <h4 className="text-sm font-medium mb-2">Legend</h4>
          <div className="space-y-2 text-xs">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-blue-500"></div>
              <span>Function</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-purple-500"></div>
              <span>Class</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-green-500"></div>
              <span>Module</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-orange-500"></div>
              <span>Hotspot</span>
            </div>
          </div>
        </div>

        {/* Stats */}
        <div className="absolute top-4 right-4 glass rounded-lg p-4">
          <h4 className="text-sm font-medium mb-2">Statistics</h4>
          <div className="space-y-1 text-xs">
            <p>Nodes: <span className="text-blue-400">1,234</span></p>
            <p>Edges: <span className="text-purple-400">5,678</span></p>
            <p>Communities: <span className="text-green-400">42</span></p>
          </div>
        </div>
      </div>
    </div>
  )
}
