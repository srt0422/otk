---
name: otk
description: Use OTK (OpenClaw Token Killer) CLI proxy to minimize LLM token consumption. Prefer `otk <cmd>` over raw shell commands for ls, find, tree, git, grep, docker, kubectl, cargo, go, pnpm, pytest, and more. Saves 60-90% tokens on common operations.
---

# OTK — OpenClaw Token Killer

CLI proxy that filters, groups, deduplicates, and truncates shell output before it reaches your context. **60-90% token savings** on common commands.

## When to Use OTK

**Always prefer `otk <cmd>` over raw commands for:**
- File ops: `ls`, `find`, `tree`, `cat` (use `otk read`)
- Git: `status`, `diff`, `log`, `add`, `commit`, `push`, `pull`
- Search: `grep`/`rg`
- Testing: `cargo test`, `go test`, `pytest`, `vitest`
- Linting: `eslint`, `ruff`, `clippy`, `golangci-lint`, `tsc`, `prettier`
- Containers: `docker ps/images/logs`, `kubectl pods/services/logs`
- Package managers: `pnpm`, `pip`, `cargo`
- GitHub CLI: `gh pr`, `gh issue`, `gh run`
- Build tools: `cargo build`, `go build`, `next build`
- Other: `curl`, `wget`, `psql`, `aws`

**Use raw commands when:**
- You need exact unfiltered output for debugging
- The command isn't supported by OTK (it will passthrough anyway)
- You're piping output to another tool

## Setup

```bash
# For OpenClaw agents (appends instructions to ./AGENTS.md)
otk init --openclaw

# For Cursor (creates .cursor/rules)
otk init --cursor

# Global with auto-rewrite hook (recommended for Claude Code)
otk init -g              # hook + OTK.md
otk init -g --auto-patch # auto-patch settings.json
```

## Command Reference

### Files
```bash
otk ls .                          # Token-optimized directory tree
otk tree .                        # Compact tree output
otk read file.rs                  # Smart file reading
otk read file.rs -l aggressive    # Signatures only
otk find "*.rs" .                 # Compact find results
otk grep "pattern" .              # Grouped search results
otk grep "pattern" . -t py        # Filter by file type
```

### Git
```bash
otk git status          # Compact status
otk git log -n 10       # One-line commits
otk git diff            # Condensed diff
otk git add .           # → "ok ✓"
otk git commit -m "msg" # → "ok ✓ abc1234"
otk git push            # → "ok ✓ main"
otk git pull            # → "ok ✓ 3 files +10 -2"
```

### GitHub CLI
```bash
otk gh pr list          # Compact PR listing
otk gh pr view 42       # PR details + checks
otk gh issue list       # Compact issues
otk gh run list         # Workflow runs
```

### Testing (failures only, ~90% reduction)
```bash
otk test cargo test     # Generic test wrapper
otk cargo test          # Rust tests
otk cargo nextest run   # Nextest
otk go test ./...       # Go tests
otk pytest              # Python tests
otk vitest run          # Vitest
otk playwright test     # E2E tests
```

### Linting & Type-checking
```bash
otk lint                # ESLint (grouped by rule)
otk tsc                 # TypeScript errors (grouped by file)
otk ruff check          # Python linting (JSON, 80% reduction)
otk ruff format         # Python formatting
otk cargo clippy        # Rust clippy (grouped by lint)
otk golangci-lint run   # Go linting (JSON, 85% reduction)
otk prettier --check .  # Files needing formatting
```

### Build
```bash
otk cargo build         # Strip "Compiling" lines, keep errors
otk cargo check         # Strip "Checking" lines, keep errors
otk go build            # Errors only
otk next build          # Next.js compact output
otk err <any command>   # Show only errors/warnings from any command
```

### Containers
```bash
otk docker ps           # Compact container list
otk docker images       # Compact image list
otk docker logs <c>     # Deduplicated logs
otk docker compose logs # Compose logs
otk kubectl pods        # Compact pod list
otk kubectl services    # Compact service list
otk kubectl logs <pod>  # Deduplicated logs
```

### Package Managers
```bash
otk pnpm install        # Filter progress bars
otk pnpm list           # Compact dependency list
otk pip list            # Python packages (auto-detects uv)
otk pip install <pkg>   # Compact install output
otk pip outdated        # Outdated packages
otk cargo install <pkg> # Strip dep compilation noise
```

### Data & Utilities
```bash
otk json config.json        # JSON structure without values
otk deps                    # Dependencies summary
otk env -f AWS              # Filtered env vars (sensitive masked)
otk diff a.txt b.txt        # Ultra-condensed diff
otk log app.log             # Deduplicated log output
otk summary <long command>  # Heuristic summary
otk wget <url>              # Download, strip progress bars
otk curl <url>              # HTTP with schema extraction
otk aws s3 ls               # AWS CLI compact output
otk psql <args>             # PostgreSQL compact output
```

### Analytics
```bash
otk gain                    # Token savings summary
otk gain --graph            # With ASCII graph
otk gain --daily            # Day-by-day breakdown
otk gain --all --format json # Export as JSON
otk discover                # Find commands where OTK would save tokens
otk discover --all          # Across all projects
```

## Global Flags

```bash
-u, --ultra-compact    # Extra token savings (ASCII icons, inline format)
-v, --verbose          # Increase verbosity (-v, -vv, -vvv)
```

## Passthrough Mode

Unsupported subcommands pass through to the real tool:
```bash
otk git checkout feature    # Passes through to git checkout
otk docker build .          # Passes through to docker build
otk cargo run               # Passes through to cargo run
```

## Best Practices

1. **Default to OTK** — use `otk <cmd>` for every supported command
2. **Use `otk test`** for any test runner — biggest token savings (~90%)
3. **Use `otk err`** for build commands that produce verbose output
4. **Use `otk read`** instead of `cat` — intelligent filtering
5. **Use `otk grep`** instead of `rg`/`grep` — grouped, truncated output
6. **Check `otk gain`** periodically to verify savings
7. **Run `otk discover`** to find missed optimization opportunities
