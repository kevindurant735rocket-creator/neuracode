# NeuraCode Roadmap

Status legend: ✅ shipped · 🚧 in progress · 📋 planned.

## Now (v0.1.x — correctness)

- ✅ Stable indexing without duplication (FK-ordered purge, absolute paths)
- ✅ Real MCP tools backed by core (search/predict/impact/architecture/hotspots/stats)
- ✅ Integration tests wired into `cargo test` (19 tests)
- 🚧 Honest docs (this roadmap, retrieval policy, benchmarks)

## Next (v0.2 — scale & trust)

- 📋 Per-project database isolation (`.neuracode/db/` when initialized)
- 📋 Incremental indexing (mtime/hash-based, `notify` watcher)
- 📋 Published retrieval evaluation set with recall numbers (`docs/retrieval.md`)
- 📋 Criterion benches in CI (`docs/benchmarks.md`)
- 📋 Remaining 7 agent install scripts
- 📋 Release-matrix alignment (install.sh × release.yml × Docker)

## Later (v0.3+ — intelligence)

- 📋 Optional vector index (behind a feature flag; keyword recall stays default)
- 📋 Vision-model integration for `understand` (behind `enable_multimodal`)
- 📋 Real multi-agent orchestration in `collab_reasoning`

## Explicitly non-goals

- No cloud upload of code; all analysis stays local unless the user opts in.
- No claimed accuracy numbers without a published evaluation.
