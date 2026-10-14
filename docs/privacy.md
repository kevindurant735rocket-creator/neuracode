# Privacy & Security

## Local-first

NeuraCode analyzes code on your machine. Index databases live in the OS cache
directory (or `.neuracode/db/` for initialized projects). There is no account,
no telemetry, and no network upload in the default build.

## What gets read

- `neuracode index <path>` reads source files under that path (respecting
  `.gitignore` and `ignore_patterns`, which include `node_modules`, `.git`,
  `target`, `dist`, `build`, `__pycache__`, `.venv`).
- Secrets: `.env`, `.env.*`, `*.pem`, `*.key` are **never indexed** — they are
  in the default ignore list (see `NeuraCodeConfig::default`). Do not remove
  them unless you understand the consequence: file contents are stored in the
  local SQLite database.

## What gets written

- `~/.cache/neuracode/` (Linux), `~/Library/Caches/neuracode/` (macOS):
  `codebase.db`, `learning.db`.
- Target agent config files only gain a `neuracode` entry; existing user
  configuration is preserved (merge, never overwrite). Uninstall removes only
  the `neuracode` entry/section.

## Reporting

Security issues: open a GitHub issue with the `security` label; do not post
private code in the report.
