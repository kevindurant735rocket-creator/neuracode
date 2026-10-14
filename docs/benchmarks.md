# Benchmarks

> Policy: no performance number is published in the README. Figures below are
> reproduced by the commands shown; environment is recorded with each run.

## Regression suite (runs in CI)

```bash
cargo test -p neuracode-core
```

Covers: 10 unit + 4 code-brain integration + 5 predict-engine + retrieval eval.
The `test_index_rust_file` and E2E tests guard against indexing regressions;
`test_retrieval_eval` guards recall.

## Manual measurements (maintainer workstation, 2026-10)

| Scenario | Command | Result |
|---|---|---|
| Index own `crates/` (41 files) | `neuracode index ./crates` | ~79 nodes, ~49 edges (see run output; time varies by machine) |
| Re-index stability | index twice, count `CodeBrain` rows | stays 1 (dedup verified) |
| Search latency | `time neuracode search CodeBrain` | local-only, no published SLA yet |

## Criterion benches (planned for CI)

Target layout `crates/neuracode-core/benches/`:

- `index_10_files`, `index_100_files` (synthetic projects)
- `search_1k_nodes` recall + latency

Until wired, `tests/benchmarks/indexing.rs` documents the intended scenarios
but is not executed by `cargo test` (see roadmap v0.2).
