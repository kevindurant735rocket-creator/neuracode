# Retrieval & Ranking Policy

This document is the single source of truth for how NeuraCode finds code.
CLI (`neuracode search`) and MCP (`neuracode_search`) share the same
`CodeBrain::semantic_search` implementation, so results are consistent.

## Current method (v0.1.x): lexical, no vectors

1. Normalize query: lowercase, drop single chars and English stop words.
2. Score each node:
   - exact phrase in name: +20
   - term substring in name: +5
   - stem match (`authentication` ↔ `authenticate_user`): +2.5
   - byte fuzzy match (≤2 edits, ASCII only): +1.5
   - signature/doc hits: +2/+1 per term
3. Boosts: function/method ×1.2, class/struct ×1.1, public ×1.1,
   complexity>5 slight boost, coverage bonus up to ×2.
4. Sort desc, truncate to 20. File context is attached **after** releasing
   the graph lock (no I/O under lock).

## Evaluation set

`crates/neuracode-core/tests/test_retrieval_eval.rs` pins recall cases:

| Query | Must find |
|---|---|
| `authentication` | `authenticate_user`-family symbols |
| `CodeBrain` | exactly one `CodeBrain` struct node (no duplicates) |

Run: `cargo test -p neuracode-core --test test_retrieval_eval`.

## What this is not (yet)

- No embeddings, no cosine similarity, no vector DB in the Rust path.
- The Python `EmbeddingModel` (CodeBERT, optional) is independent and not
  consulted by the Rust search. A vector index may arrive behind a feature
  flag in v0.3; lexical recall will remain the default.
