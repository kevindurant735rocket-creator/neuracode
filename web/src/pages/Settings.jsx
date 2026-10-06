import { useState } from 'react'
import { Save, RefreshCw, Trash2, Plus } from 'lucide-react'

export default function Settings() {
  const [settings, setSettings] = useState({
    cacheSize: 512,
    enablePrediction: true,
    enableLearning: true,
    enableMultimodal: true,
    maxFileSize: 1024 * 1024,
    languages: ['rust', 'javascript', 'typescript', 'python', 'go', 'java'],
  })

  const handleSave = () => {
    // Save settings
    console.log('Saving settings:', settings)
  }

  return (
    <div className="p-8 animate-fade-in max-w-4xl">
      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold mb-2">Settings</h1>
          <p className="text-slate-400">Configure NeuraCode behavior</p>
        </div>
        <button
          onClick={handleSave}
          className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg font-medium transition-colors"
        >
          <Save className="w-4 h-4" />
          Save Changes
        </button>
      </div>

      {/* General Settings */}
      <div className="glass rounded-xl p-6 mb-6">
        <h3 className="text-lg font-semibold mb-4">General</h3>
        
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-2">Cache Size (MB)</label>
            <input
              type="number"
              value={settings.cacheSize}
              onChange={(e) => setSettings({ ...settings, cacheSize: Number(e.target.value) })}
              className="w-full px-4 py-2 bg-slate-800 border border-slate-700 rounded-lg focus:outline-none focus:border-blue-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">Max File Size (bytes)</label>
            <input
              type="number"
              value={settings.maxFileSize}
              onChange={(e) => setSettings({ ...settings, maxFileSize: Number(e.target.value) })}
              className="w-full px-4 py-2 bg-slate-800 border border-slate-700 rounded-lg focus:outline-none focus:border-blue-500"
            />
          </div>
        </div>
      </div>

      {/* Feature Flags */}
      <div className="glass rounded-xl p-6 mb-6">
        <h3 className="text-lg font-semibold mb-4">Features</h3>
        
        <div className="space-y-4">
          {[
            { key: 'enablePrediction', label: 'Enable Prediction', description: 'Predict context for tasks' },
            { key: 'enableLearning', label: 'Enable Learning', description: 'Learn from sessions' },
            { key: 'enableMultimodal', label: 'Enable Multi-Modal', description: 'Understand images and diagrams' },
          ].map((feature) => (
            <div key={feature.key} className="flex items-center justify-between">
              <div>
                <p className="font-medium">{feature.label}</p>
                <p className="text-sm text-slate-400">{feature.description}</p>
              </div>
              <button
                onClick={() => setSettings({ ...settings, [feature.key]: !settings[feature.key] })}
                className={`w-12 h-6 rounded-full transition-colors ${
                  settings[feature.key] ? 'bg-blue-600' : 'bg-slate-700'
                }`}
              >
                <div 
                  className={`w-5 h-5 rounded-full bg-white transition-transform ${
                    settings[feature.key] ? 'translate-x-6' : 'translate-x-0.5'
                  }`}
                ></div>
              </button>
            </div>
          ))}
        </div>
      </div>

      {/* Language Support */}
      <div className="glass rounded-xl p-6 mb-6">
        <h3 className="text-lg font-semibold mb-4">Language Support</h3>
        
        <div className="flex flex-wrap gap-2 mb-4">
          {settings.languages.map((lang) => (
            <span 
              key={lang}
              className="px-3 py-1.5 bg-slate-800 rounded-lg flex items-center gap-2"
            >
              {lang}
              <button 
                onClick={() => setSettings({
                  ...settings,
                  languages: settings.languages.filter(l => l !== lang)
                })}
                className="text-slate-400 hover:text-red-400"
              >
                ×
              </button>
            </span>
          ))}
        </div>

        <button className="flex items-center gap-2 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 rounded-lg text-sm transition-colors">
          <Plus className="w-4 h-4" />
          Add Language
        </button>
      </div>

      {/* Danger Zone */}
      <div className="glass rounded-xl p-6 border border-red-500/20">
        <h3 className="text-lg font-semibold text-red-400 mb-4">Danger Zone</h3>
        
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium">Clear Cache</p>
              <p className="text-sm text-slate-400">Remove all cached data</p>
            </div>
            <button className="flex items-center gap-2 px-4 py-2 bg-slate-800 hover:bg-slate-700 rounded-lg transition-colors">
              <RefreshCw className="w-4 h-4" />
              Clear
            </button>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium">Reset All Data</p>
              <p className="text-sm text-slate-400">Delete all indexed data and settings</p>
            </div>
            <button className="flex items-center gap-2 px-4 py-2 bg-red-600/20 hover:bg-red-600/30 text-red-400 rounded-lg transition-colors">
              <Trash2 className="w-4 h-4" />
              Reset
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}
