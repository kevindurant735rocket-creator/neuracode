import { useState } from 'react'
import { Search as SearchIcon, FileCode, Filter, SortAsc } from 'lucide-react'
import Editor from '@monaco-editor/react'

const mockResults = [
  {
    id: 1,
    name: 'authenticateUser',
    kind: 'function',
    file: 'src/auth.ts',
    line: 42,
    score: 0.95,
    context: `function authenticateUser(username: string, password: string): boolean {
  const user = findUser(username);
  if (!user) return false;
  return verifyPassword(password, user.passwordHash);
}`,
  },
  {
    id: 2,
    name: 'login',
    kind: 'function',
    file: 'src/auth.ts',
    line: 78,
    score: 0.88,
    context: `async function login(username: string, password: string) {
  const isValid = await authenticateUser(username, password);
  if (!isValid) throw new Error('Invalid credentials');
  return generateToken(username);
}`,
  },
  {
    id: 3,
    name: 'AuthService',
    kind: 'class',
    file: 'src/services/auth.ts',
    line: 15,
    score: 0.82,
    context: `class AuthService {
  async authenticate(credentials: Credentials) {
    // Authentication logic
  }
}`,
  },
]

export default function Search() {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState([])
  const [isSearching, setIsSearching] = useState(false)
  const [selectedResult, setSelectedResult] = useState(null)

  const handleSearch = async () => {
    if (!query.trim()) return
    
    setIsSearching(true)
    // Simulate search
    setTimeout(() => {
      setResults(mockResults)
      setIsSearching(false)
    }, 500)
  }

  return (
    <div className="p-8 h-full flex flex-col animate-fade-in">
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold mb-2">Search</h1>
        <p className="text-slate-400">Semantic search across your codebase</p>
      </div>

      {/* Search Bar */}
      <div className="glass rounded-xl p-4 mb-6">
        <div className="flex gap-4">
          <div className="flex-1 relative">
            <SearchIcon className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-400" />
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
              placeholder="Search for functions, classes, or concepts..."
              className="w-full pl-12 pr-4 py-3 bg-slate-800 border border-slate-700 rounded-lg focus:outline-none focus:border-blue-500 transition-colors"
            />
          </div>
          <button
            onClick={handleSearch}
            disabled={isSearching}
            className="px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-600/50 rounded-lg font-medium transition-colors flex items-center gap-2"
          >
            {isSearching ? (
              <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
            ) : (
              <SearchIcon className="w-5 h-5" />
            )}
            Search
          </button>
        </div>

        {/* Filters */}
        <div className="flex gap-4 mt-4">
          <button className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 transition-colors text-sm">
            <Filter className="w-4 h-4" />
            Filters
          </button>
          <button className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 transition-colors text-sm">
            <SortAsc className="w-4 h-4" />
            Sort by Relevance
          </button>
        </div>
      </div>

      {/* Results */}
      <div className="flex-1 flex gap-6 overflow-hidden">
        {/* Results List */}
        <div className="w-1/2 overflow-auto">
          {results.length > 0 ? (
            <div className="space-y-3">
              {results.map((result) => (
                <div
                  key={result.id}
                  onClick={() => setSelectedResult(result)}
                  className={`glass rounded-xl p-4 cursor-pointer transition-all duration-200 ${
                    selectedResult?.id === result.id 
                      ? 'border-blue-500' 
                      : 'hover:border-slate-600'
                  }`}
                >
                  <div className="flex items-start justify-between mb-2">
                    <div className="flex items-center gap-2">
                      <FileCode className="w-4 h-4 text-blue-400" />
                      <span className="font-medium">{result.name}</span>
                      <span className="px-2 py-0.5 rounded text-xs bg-purple-500/20 text-purple-400">
                        {result.kind}
                      </span>
                    </div>
                    <span className="text-sm text-slate-400">
                      {(result.score * 100).toFixed(0)}%
                    </span>
                  </div>
                  <p className="text-sm text-slate-400 mb-2">
                    {result.file}:{result.line}
                  </p>
                  <pre className="text-xs text-slate-500 overflow-hidden">
                    {result.context.substring(0, 100)}...
                  </pre>
                </div>
              ))}
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center h-full text-slate-500">
              <SearchIcon className="w-16 h-16 mb-4 opacity-50" />
              <p>Enter a query to search your codebase</p>
            </div>
          )}
        </div>

        {/* Code Preview */}
        <div className="w-1/2 glass rounded-xl overflow-hidden">
          {selectedResult ? (
            <div className="h-full flex flex-col">
              <div className="p-4 border-b border-slate-700">
                <h3 className="font-medium">{selectedResult.name}</h3>
                <p className="text-sm text-slate-400">{selectedResult.file}</p>
              </div>
              <div className="flex-1">
                <Editor
                  height="100%"
                  defaultLanguage="typescript"
                  value={selectedResult.context}
                  theme="vs-dark"
                  options={{
                    readOnly: true,
                    minimap: { enabled: false },
                    fontSize: 14,
                  }}
                />
              </div>
            </div>
          ) : (
            <div className="flex items-center justify-center h-full text-slate-500">
              <p>Select a result to preview</p>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
