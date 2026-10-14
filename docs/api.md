# API Reference

## CLI (`neuracode`)

Run `neuracode --help` for full flags. Config resolution order:
`--config PATH` (or `NEURACODE_CONFIG`) → `.neuracode/config.toml` (partial
keys) → built-in defaults.

| Command | What it does | Status |
|---|---|---|
| `init [path]` | create `.neuracode/config.toml` + `cache/` | ✅ |
| `index [path] [--force]` | parse + persist code graph | ✅ (`--force` currently re-indexes; no cache-bypass yet) |
| `search <query>` | lexical search (see `docs/retrieval.md`) | ✅ |
| `predict <task>` | task classification + context package | ✅ heuristic |
| `impact <target>` | search first hit → dependents + risk | ✅ |
| `architecture` | pattern heuristics from graph | ✅ |
| `hotspots` | degree + PageRank composite | ✅ |
| `understand <image>` | file-exists check + filename heuristic | 🚧 placeholder, see roadmap |
| `install [--agent NAME] [--all]` | merge `neuracode` MCP entry (never overwrites) | ✅ |
| `uninstall` | remove only the `neuracode` entry/section | ✅ |
| `stats` | node/edge counts + top hotspots | ✅ |
| `serve` | execs `neuracode-mcp` (stdio) | ✅ bridge |
| `config [--full]` | show effective config | ✅ |
| `analyze/export/watch/compare/suggest/version` | composites of the above | ✅ best-effort (see `--help`) |

## MCP (`neuracode-mcp`, stdio, protocol `2024-11-05`)

Tools (all backed by `neuracode-core`, same ranking as CLI):

`neuracode_search`, `neuracode_predict`, `neuracode_impact`,
`neuracode_architecture`, `neuracode_hotspots`, `neuracode_analyze`,
`neuracode_suggest`, `neuracode_stats`, `neuracode_compare`, `neuracode_export`.

`notifications/initialized` gets no response (spec-compliant); unknown methods
return `METHOD_NOT_FOUND`.

## Rust core (`neuracode-core`)

Entry: `NeuraCode::new(config)` →
`index_codebase(path)` / `search(query)` / `predict_context(task)` /
`learn(session)` / `understand_image(path)` / `install_all_agents()`.
Config: `NeuraCodeConfig::load_or_default(explicit_path)`.
Secrets note: `.env*`, `*.pem`, `*.key` are default-ignored (see
`docs/privacy.md`).
