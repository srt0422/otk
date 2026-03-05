# otk - OpenClaw Token Killer

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**High-performance CLI proxy to minimize LLM token consumption.**

[Website](https://www.openclaw.dev) | [GitHub](https://github.com/openclaw/otk) | [Install](INSTALL.md)

otk filters and compresses command outputs before they reach your LLM context, saving 60-90% of tokens on common operations.

## ⚠️ Important: Name Collision Warning

**There are TWO different projects named "otk":**

1. ✅ **This project (OpenClaw Token Killer)** - LLM token optimizer
   - Repos: `openclaw/otk`
   - Purpose: Reduce AI coding token consumption

2. ❌ **reachingforthejack/rtk** - Rust Type Kit (DIFFERENT PROJECT)
   - Purpose: Query Rust codebase and generate types
   - **DO NOT install this one if you want token optimization**

**How to verify you have the correct otk:**
```bash
otk --version   # Should show "otk 0.25.0"
otk gain        # Should show token savings stats
```

If `otk gain` doesn't exist, you installed the wrong package. See installation instructions below.

## Token Savings (30-min AI coding Session)

Typical session without otk: **~150,000 tokens**
With otk: **~45,000 tokens** → **70% reduction**

| Operation | Frequency | Standard | otk | Savings |
|-----------|-----------|----------|-----|---------|
| `ls` / `tree` | 10× | 2,000 | 400 | -80% |
| `cat` / `read` | 20× | 40,000 | 12,000 | -70% |
| `grep` / `rg` | 8× | 16,000 | 3,200 | -80% |
| `git status` | 10× | 3,000 | 600 | -80% |
| `git diff` | 5× | 10,000 | 2,500 | -75% |
| `git log` | 5× | 2,500 | 500 | -80% |
| `git add/commit/push` | 8× | 1,600 | 120 | -92% |
| `npm test` / `cargo test` | 5× | 25,000 | 2,500 | -90% |
| `ruff check` | 3× | 3,000 | 600 | -80% |
| `pytest` | 4× | 8,000 | 800 | -90% |
| `go test` | 3× | 6,000 | 600 | -90% |
| `docker ps` | 3× | 900 | 180 | -80% |
| **Total** | | **~118,000** | **~23,900** | **-80%** |

> Estimates based on medium-sized TypeScript/Rust projects. Actual savings vary by project size.

## Installation

### ⚠️ Pre-Installation Check (REQUIRED)

**ALWAYS verify if otk is already installed before installing:**

```bash
otk --version        # Check if installed
otk gain             # Verify it's the Token Killer (not Type Kit)
which otk            # Check installation path
```

If already installed and `otk gain` works, **DO NOT reinstall**. Skip to Quick Start.

### Homebrew (macOS/Linux)

```bash
brew install otk
```

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/openclaw/otk/refs/heads/master/install.sh | sh
```

> **Note**: otk installs to `~/.local/bin` by default. If this directory is not in your PATH, add it:
> ```bash
> echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
> ```

After installation, **verify you have the correct otk**:
```bash
otk gain  # Must show token savings stats (not "command not found")
```

### Alternative: Manual Installation

```bash
# From openclaw upstream (maintained by pszymkowiak)
cargo install --git https://github.com/openclaw/otk

# OR if published to crates.io
cargo install otk
```

⚠️ **WARNING**: `cargo install otk` from crates.io might install the wrong package (Type Kit instead of Token Killer). Always verify with `otk gain` after installation.

### Alternative: Pre-built Binaries

Download from [openclaw/otk/releases](https://github.com/openclaw/otk/releases):
- macOS: `otk-x86_64-apple-darwin.tar.gz` / `otk-aarch64-apple-darwin.tar.gz`
- Linux: `otk-x86_64-unknown-linux-gnu.tar.gz` / `otk-aarch64-unknown-linux-gnu.tar.gz`
- Windows: `otk-x86_64-pc-windows-msvc.zip`

## Quick Start

```bash
# 1. Verify installation
otk gain  # Must show token stats, not "command not found"

# 2. Initialize for AI coding (RECOMMENDED: hook-first mode)
otk init --global
# → Installs hook + creates slim OTK.md (10 lines, 99.5% token savings)
# → Follow printed instructions to add hook to ~/.ai-assistant/settings.json

# 3. Test it works
otk git status  # Should show ultra-compact output
otk init --show # Verify hook is installed and executable

# Alternative modes:
# otk init --global --ai-assistant-md  # Legacy: full injection (137 lines)
# otk init                       # Local project only (./AGENTS.md)
```

**New in v0.9.5**: Hook-first installation eliminates ~2000 tokens from AI assistant's context while maintaining full OTK functionality through transparent command rewriting.

## Global Flags

```bash
-u, --ultra-compact    # ASCII icons, inline format (extra token savings)
-v, --verbose          # Increase verbosity (-v, -vv, -vvv)
```

## Commands

### Files
```bash
otk ls .                        # Token-optimized directory tree
otk read file.rs                # Smart file reading
otk read file.rs -l aggressive  # Signatures only (strips bodies)
otk smart file.rs               # 2-line heuristic code summary
otk find "*.rs" .               # Compact find results
otk grep "pattern" .            # Grouped search results
```

### Git
```bash
otk git status                  # Compact status
otk git log -n 10               # One-line commits
otk git diff                    # Condensed diff
otk git add                     # → "ok ✓"
otk git commit -m "msg"         # → "ok ✓ abc1234"
otk git push                    # → "ok ✓ main"
otk git pull                    # → "ok ✓ 3 files +10 -2"
```

### Commands
```bash
otk test cargo test             # Show failures only (-90% tokens)
otk err npm run build           # Errors/warnings only
otk summary <long command>      # Heuristic summary
otk log app.log                 # Deduplicated logs
otk gh pr list                   # Compact PR listing
otk gh pr view 42                # PR details + checks summary
otk gh issue list                # Compact issue listing
otk gh run list                  # Workflow run status
otk wget https://example.com    # Download, strip progress bars
otk config                       # Show config (--create to generate)
otk ruff check                   # Python linting (JSON, 80% reduction)
otk pytest                       # Python tests (failures only, 90% reduction)
otk pip list                     # Python packages (auto-detect uv, 70% reduction)
otk go test                      # Go tests (NDJSON, 90% reduction)
otk golangci-lint run            # Go linting (JSON, 85% reduction)
```

### Data & Analytics
```bash
otk json config.json            # Structure without values
otk deps                        # Dependencies summary
otk env -f AWS                  # Filtered env vars

# Token Savings Analytics (includes execution time metrics)
otk gain                        # Summary stats with total exec time
otk gain --graph                # With ASCII graph of last 30 days
otk gain --history              # With recent command history (10)
otk gain --quota --tier 20x     # Monthly quota analysis (pro/5x/20x)

# Temporal Breakdowns (includes time metrics per period)
otk gain --daily                # Day-by-day with avg execution time
otk gain --weekly               # Week-by-week breakdown
otk gain --monthly              # Month-by-month breakdown
otk gain --all                  # All breakdowns combined

# Export Formats (includes total_time_ms and avg_time_ms fields)
otk gain --all --format json    # JSON export for APIs/dashboards
otk gain --all --format csv     # CSV export for Excel/analysis
```

> 📖 **API Documentation**: For programmatic access to tracking data (Rust library usage, CI/CD integration, custom dashboards), see [docs/tracking.md](docs/tracking.md).

### Discover — Find Missed Savings

Scans your AI coding session history to find commands where otk would have saved tokens. Use it to:
- **Measure what you're missing** — see exactly how many tokens you could save
- **Identify habits** — find which commands you keep running without otk
- **Spot new opportunities** — see unhandled commands that could become otk features

```bash
otk discover                    # Current project, last 30 days
otk discover --all              # All AI coding projects
otk discover --all --since 7    # Last 7 days across all projects
otk discover -p aristote        # Filter by project name (substring)
otk discover --format json      # Machine-readable output
```

Example output:
```
OTK Discover -- Savings Opportunities
====================================================
Scanned: 142 sessions (last 30 days), 1786 Bash commands
Already using OTK: 108 commands (6%)

MISSED SAVINGS -- Commands OTK already handles
----------------------------------------------------
Command              Count    OTK Equivalent        Est. Savings
git log                434    otk git               ~55.9K tokens
cargo test             203    otk cargo             ~49.9K tokens
ls -la                 107    otk ls                ~11.8K tokens
gh pr                   80    otk gh                ~10.4K tokens
----------------------------------------------------
Total: 986 commands -> ~143.9K tokens saveable

TOP UNHANDLED COMMANDS -- open an issue?
----------------------------------------------------
Command              Count    Example
git checkout            84    git checkout feature/my-branch
cargo run               32    cargo run -- gain --help
----------------------------------------------------
-> github.com/openclaw/otk/issues
```

### Containers
```bash
otk docker ps                   # Compact container list
otk docker images               # Compact image list
otk docker logs <container>     # Deduplicated logs
otk kubectl pods                # Compact pod list
otk kubectl logs <pod>          # Deduplicated logs
otk kubectl services             # Compact service list
```

### JavaScript / TypeScript Stack
```bash
otk lint                         # ESLint grouped by rule/file
otk lint biome                   # Supports other linters too
otk tsc                          # TypeScript errors grouped by file
otk next build                   # Next.js build compact output
otk prettier --check .           # Files needing formatting
otk vitest run                   # Test failures only
otk playwright test              # E2E results (failures only)
otk prisma generate              # Schema generation (no ASCII art)
otk prisma migrate dev --name x  # Migration summary
otk prisma db-push               # Schema push summary
```

### Python & Go Stack
```bash
# Python
otk ruff check                   # Ruff linter (JSON, 80% reduction)
otk ruff format                  # Ruff formatter (text filter)
otk pytest                       # Test failures with state machine parser (90% reduction)
otk pip list                     # Package list (auto-detect uv, 70% reduction)
otk pip install <package>        # Install with compact output
otk pip outdated                 # Outdated packages (85% reduction)

# Go
otk go test                      # NDJSON streaming parser (90% reduction)
otk go build                     # Build errors only (80% reduction)
otk go vet                       # Vet issues (75% reduction)
otk golangci-lint run            # JSON grouped by rule (85% reduction)
```

## Examples

### Standard vs otk

**Directory listing:**
```
# ls -la (45 lines, ~800 tokens)
drwxr-xr-x  15 user  staff    480 Jan 23 10:00 .
drwxr-xr-x   5 user  staff    160 Jan 23 09:00 ..
-rw-r--r--   1 user  staff   1234 Jan 23 10:00 Cargo.toml
...

# otk ls (12 lines, ~150 tokens)
📁 my-project/
├── src/ (8 files)
│   ├── main.rs
│   └── lib.rs
├── Cargo.toml
└── README.md
```

**Git operations:**
```
# git push (15 lines, ~200 tokens)
Enumerating objects: 5, done.
Counting objects: 100% (5/5), done.
Delta compression using up to 8 threads
...

# otk git push (1 line, ~10 tokens)
ok ✓ main
```

**Test output:**
```
# cargo test (200+ lines on failure)
running 15 tests
test utils::test_parse ... ok
test utils::test_format ... ok
...

# otk test cargo test (only failures, ~20 lines)
FAILED: 2/15 tests
  ✗ test_edge_case: assertion failed at src/lib.rs:42
  ✗ test_overflow: panic at src/utils.rs:18
```

## How It Works

```
  Without otk:

  ┌──────────┐  git status     ┌──────────┐  git status  ┌──────────┐
  │  AI assistant  │ ─────────────── │  shell   │ ──────────── │   git    │
  │   LLM    │                 │          │              │  (CLI)   │
  └──────────┘                 └──────────┘              └──────────┘
        ▲                                                      │
        │              ~2,000 tokens (raw output)              │
        └──────────────────────────────────────────────────────┘

  With otk:

  ┌──────────┐  git status     ┌──────────┐  git status  ┌──────────┐
  │  AI assistant  │ ─────────────── │   OTK    │ ──────────── │   git    │
  │   LLM    │                 │  (proxy) │              │  (CLI)   │
  └──────────┘                 └──────────┘              └──────────┘
        ▲                           │  ~2,000 tokens raw       │
        │                           └──────────────────────────┘
        │  ~200 tokens (filtered)   filter · group · dedup · truncate
        └───────────────────────────────────────────────────────
```

Four strategies applied per command type:

1. **Smart Filtering**: Removes noise (comments, whitespace, boilerplate)
2. **Grouping**: Aggregates similar items (files by directory, errors by type)
3. **Truncation**: Keeps relevant context, cuts redundancy
4. **Deduplication**: Collapses repeated log lines with counts

## Configuration

### Installation Modes

| Command | Scope | Hook | OTK.md | CLAUDE.md | Tokens in Context | Use Case |
|---------|-------|------|--------|-----------|-------------------|----------|
| `otk init -g` | Global | ✅ | ✅ (10 lines) | @OTK.md | ~10 | **Recommended**: All projects, automatic |
| `otk init -g --ai-assistant-md` | Global | ❌ | ❌ | Full (137 lines) | ~2000 | Legacy compatibility |
| `otk init -g --hook-only` | Global | ✅ | ❌ | Nothing | 0 | Minimal setup, hook-only |
| `otk init` | Local | ❌ | ❌ | Full (137 lines) | ~2000 | Single project, no hook |

```bash
otk init --show         # Show current configuration
otk init -g             # Install hook + OTK.md (recommended)
otk init -g --ai-assistant-md # Legacy: full injection into CLAUDE.md
otk init                # Local project: full injection into ./AGENTS.md
```

### Installation Flags

**Settings.json Control**:
```bash
otk init -g                 # Default: prompt to patch [y/N]
otk init -g --auto-patch    # Patch settings.json without prompting
otk init -g --no-patch      # Skip patching, show manual instructions
```

**Mode Control**:
```bash
otk init -g --ai-assistant-md     # Legacy: full 137-line injection (no hook)
otk init -g --hook-only     # Hook only, no OTK.md
```

**Uninstall**:
```bash
otk init -g --uninstall     # Remove all OTK artifacts
```

**What is settings.json?**
AI coding configuration file that registers the OTK hook. The hook transparently rewrites commands (e.g., `git status` → `otk git status`) before execution. Without this registration, AI assistant won't use the hook.

**Backup Behavior**:
OTK creates `~/.ai-assistant/settings.json.bak` before making changes. If something breaks, restore with:
```bash
cp ~/.ai-assistant/settings.json.bak ~/.ai-assistant/settings.json
```

**Migration**: If you previously used `otk init -g` with the old system (137-line injection), simply re-run `otk init -g` to automatically migrate to the new hook-first approach.

example of 3 days session:
```bash
📊 OTK Token Savings
════════════════════════════════════════

Total commands:    133
Input tokens:      30.5K
Output tokens:     10.7K
Tokens saved:      25.3K (83.0%)

By Command:
────────────────────────────────────────
Command               Count      Saved     Avg%
otk git status           41      17.4K    82.9%
otk git push             54       3.4K    91.6%
otk grep                 15       3.2K    26.5%
otk ls                   23       1.4K    37.2%

Daily Savings (last 30 days):
────────────────────────────────────────
01-23 │███████████████████                      6.4K
01-24 │██████████████████                       5.9K
01-25 │                                         18
01-26 │████████████████████████████████████████ 13.0K
```

### Custom Database Path

By default, OTK stores tracking data in `~/.local/share/otk/history.db`. You can override this:

**Environment variable** (highest priority):
```bash
export OTK_DB_PATH="/path/to/custom.db"
```

**Config file** (`~/.config/otk/config.toml`):
```toml
[tracking]
database_path = "/path/to/custom.db"
```

Priority: `OTK_DB_PATH` env var > `config.toml` > default location.

### Excluding Commands from Auto-Rewrite

By default, the hook rewrites all supported commands automatically. To exclude specific commands (e.g., keep raw `curl` output without schema extraction), add to your config:

**Config file** (`~/.config/otk/config.toml`, macOS: `~/Library/Application Support/otk/config.toml`):
```toml
[hooks]
exclude_commands = ["curl", "playwright"]
```

Excluded commands pass through the hook unchanged — no OTK filtering. This survives `otk init -g` re-runs since the config file is user-owned.

### Tee: Full Output Recovery

When OTK filters command output, LLM agents lose failure details (stack traces, assertion messages) and may re-run the same command 2-3 times. The **tee** feature saves raw output to a file so the agent can read it without re-executing.

**How it works**: On command failure, OTK writes the full unfiltered output to `~/.local/share/otk/tee/` and prints a one-line hint:
```
✓ cargo test: 15 passed (1 suite, 0.01s)
[full output: ~/.local/share/otk/tee/1707753600_cargo_test.log]
```

The agent reads the file instead of re-running the command — saving tokens.

**Default behavior**: Tee only on failures (exit code != 0), skip outputs < 500 chars.

**Config** (`~/.config/otk/config.toml`):
```toml
[tee]
enabled = true          # default: true
mode = "failures"       # "failures" (default), "always", or "never"
max_files = 20          # max files to keep (oldest rotated out)
max_file_size = 1048576 # 1MB per file max
# directory = "/custom/path"  # override default location
```

**Environment overrides**:
- `RTK_TEE=0` — disable tee entirely
- `RTK_TEE_DIR=/path` — override output directory

**Supported commands**: cargo (build/test/clippy/check/install/nextest), vitest, pytest, lint (eslint/biome/ruff/pylint/mypy), tsc, go (test/build/vet), err, test.

## Auto-Rewrite Hook (Recommended)

The most effective way to use otk is with the **auto-rewrite hook** for AI coding. Instead of relying on CLAUDE.md instructions (which subagents may ignore), this hook transparently intercepts Bash commands and rewrites them to their otk equivalents before execution.

**Result**: 100% otk adoption across all conversations and subagents, zero token overhead in AI assistant's context.

### What Are Hooks?

**For Beginners**:
AI coding hooks are scripts that run before/after AI assistant executes commands. OTK uses a **PreToolUse** hook that intercepts Bash commands and rewrites them (e.g., `git status` → `otk git status`) before execution. This is **transparent** - AI assistant never sees the rewrite, it just gets optimized output.

**Why settings.json?**
AI coding reads `~/.ai-assistant/settings.json` to find registered hooks. Without this file, AI assistant doesn't know the OTK hook exists. Think of it as the hook registry.

**Is it safe?**
Yes. OTK creates a backup (`settings.json.bak`) before changes. The hook is read-only (it only modifies command strings, never deletes files or accesses secrets). Review the hook script at `~/.ai-assistant/hooks/otk-rewrite.sh` anytime.

### How It Works

The hook runs as a AI coding [PreToolUse hook](https://docs.anthropic.com/en/docs/ai-assistant-code/hooks). When AI coding is about to execute a Bash command like `git status`, the hook rewrites it to `otk git status` before the command reaches the shell. AI coding never sees the rewrite — it's transparent.

```
  AI coding types:  git status
                           │
                    ┌──────▼──────────────────────┐
                    │  ~/.ai-assistant/settings.json     │
                    │  PreToolUse hook registered  │
                    └──────┬──────────────────────┘
                           │
                    ┌──────▼──────────────────────┐
                    │  otk-rewrite.sh              │
                    │  "git status"                │
                    │    →  "otk git status"       │  transparent rewrite
                    └──────┬──────────────────────┘
                           │
                    ┌──────▼──────────────────────┐
                    │  OTK (Rust binary)           │
                    │  executes real git status    │
                    │  filters output              │
                    └──────┬──────────────────────┘
                           │
  AI assistant receives:  "3 modified, 1 untracked ✓"
                    ↑ not 50 lines of raw git output
```

### Quick Install (Automated)

```bash
otk init -g
# → Installs hook to ~/.ai-assistant/hooks/otk-rewrite.sh (with executable permissions)
# → Creates ~/.ai-assistant/OTK.md (10 lines, minimal context footprint)
# → Adds @OTK.md reference to ~/.ai-assistant/CLAUDE.md
# → Prompts: "Patch settings.json? [y/N]"
# → If yes: creates backup (~/.ai-assistant/settings.json.bak), patches file

# Verify installation
otk init --show  # Shows hook status, settings.json registration
```

**Settings.json Patching Options**:
```bash
otk init -g                 # Default: prompts for consent [y/N]
otk init -g --auto-patch    # Patch immediately without prompting (CI/CD)
otk init -g --no-patch      # Skip patching, print manual JSON snippet
```

**What is settings.json?**
AI coding's configuration file that registers the OTK hook. Without this, AI assistant won't use the hook. OTK backs up the file before changes (`settings.json.bak`).

**Restart Required**: After installation, restart AI coding, then test with `git status`.

### Manual Install (Fallback)

If automatic patching fails or you prefer manual control:

```bash
# 1. Install hook and OTK.md
otk init -g --no-patch  # Prints JSON snippet

# 2. Manually edit ~/.ai-assistant/settings.json (add the printed snippet)

# 3. Restart AI coding
```

**Alternative: Full manual setup**

```bash
# 1. Copy the hook script
mkdir -p ~/.ai-assistant/hooks
cp .ai-assistant/hooks/otk-rewrite.sh ~/.ai-assistant/hooks/otk-rewrite.sh
chmod +x ~/.ai-assistant/hooks/otk-rewrite.sh

# 2. Add to ~/.ai-assistant/settings.json under hooks.PreToolUse:
```

Add this entry to the `PreToolUse` array in `~/.ai-assistant/settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "~/.ai-assistant/hooks/otk-rewrite.sh"
          }
        ]
      }
    ]
  }
}
```

### Per-Project Install

The hook is included in this repository at `.ai-assistant/hooks/otk-rewrite.sh`. To use it in another project, copy the hook and add the same settings.json entry using a relative path or project-level `.ai-assistant/settings.json`.

### Commands Rewritten

| Raw Command | Rewritten To |
|-------------|-------------|
| `git status/diff/log/add/commit/push/pull/branch/fetch/stash` | `otk git ...` |
| `gh pr/issue/run` | `otk gh ...` |
| `cargo test/build/clippy` | `otk cargo ...` |
| `cat <file>` | `otk read <file>` |
| `rg/grep <pattern>` | `otk grep <pattern>` |
| `ls` | `otk ls` |
| `vitest/pnpm test` | `otk vitest run` |
| `tsc/pnpm tsc` | `otk tsc` |
| `eslint/pnpm lint` | `otk lint` |
| `prettier` | `otk prettier` |
| `playwright` | `otk playwright` |
| `prisma` | `otk prisma` |
| `ruff check/format` | `otk ruff ...` |
| `pytest` | `otk pytest` |
| `pip list/install/outdated` | `otk pip ...` |
| `go test/build/vet` | `otk go ...` |
| `golangci-lint run` | `otk golangci-lint run` |
| `docker ps/images/logs` | `otk docker ...` |
| `kubectl get/logs` | `otk kubectl ...` |
| `curl` | `otk curl` |
| `pnpm list/ls/outdated` | `otk pnpm ...` |

Commands already using `otk`, heredocs (`<<`), and unrecognized commands pass through unchanged.

### Alternative: Suggest Hook (Non-Intrusive)

If you prefer AI coding to **suggest** otk usage rather than automatically rewriting commands, use the **suggest hook** pattern instead. This emits a system reminder when otk-compatible commands are detected, without modifying the command execution.

**Comparison**:

| Aspect | Auto-Rewrite Hook | Suggest Hook |
|--------|-------------------|--------------|
| **Strategy** | Intercepts and modifies command before execution | Emits system reminder when otk-compatible command detected |
| **Effect** | AI coding never sees the original command | AI coding receives hint to use otk, decides autonomously |
| **Adoption** | 100% (forced) | ~70-85% (depends on AI coding's adherence to instructions) |
| **Use Case** | Production workflows, guaranteed savings | Learning mode, auditing, user preference for explicit control |
| **Overhead** | Zero (transparent rewrite) | Minimal (reminder message in context) |

**When to use suggest over rewrite**:
- You want to audit which commands AI coding chooses to run
- You're learning otk patterns and want visibility into the rewrite logic
- You prefer AI coding to make explicit decisions rather than transparent rewrites
- You want to preserve exact command execution for debugging

#### Suggest Hook Setup

**1. Create the suggest hook script**

```bash
mkdir -p ~/.ai-assistant/hooks
cp .ai-assistant/hooks/otk-suggest.sh ~/.ai-assistant/hooks/otk-suggest.sh
chmod +x ~/.ai-assistant/hooks/otk-suggest.sh
```

**2. Add to `~/.ai-assistant/settings.json`**

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "~/.ai-assistant/hooks/otk-suggest.sh"
          }
        ]
      }
    ]
  }
}
```

The suggest hook detects the same commands as the rewrite hook but outputs a `systemMessage` instead of `updatedInput`, informing AI coding that an otk alternative exists.

## Uninstalling RTK

**Complete Removal (Global Only)**:
```bash
otk init -g --uninstall

# Removes:
#   - ~/.ai-assistant/hooks/otk-rewrite.sh
#   - ~/.ai-assistant/OTK.md
#   - @OTK.md reference from ~/.ai-assistant/CLAUDE.md
#   - OTK hook entry from ~/.ai-assistant/settings.json

# Restart AI coding after uninstall
```

**Restore from Backup** (if needed):
```bash
cp ~/.ai-assistant/settings.json.bak ~/.ai-assistant/settings.json
```

**Local Projects**: Manually remove OTK instructions from `./AGENTS.md`

**Binary Removal**:
```bash
# If installed via cargo
cargo uninstall otk

# If installed via package manager
brew uninstall otk          # macOS Homebrew
sudo apt remove otk         # Debian/Ubuntu
sudo dnf remove otk         # Fedora/RHEL
```

## Documentation

- **[TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)** - ⚠️ Fix common issues (wrong otk installed, missing commands, PATH issues)
- **[INSTALL.md](INSTALL.md)** - Detailed installation guide with verification steps
- **[AUDIT_GUIDE.md](docs/AUDIT_GUIDE.md)** - Complete guide to token savings analytics, temporal breakdowns, and data export
- **[CLAUDE.md](CLAUDE.md)** - AI coding integration instructions and project context
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Technical architecture and development guide
- **[SECURITY.md](SECURITY.md)** - Security policy, vulnerability reporting, and PR review process

## Troubleshooting

### Settings.json Patching Failed

**Problem**: `otk init -g` fails to patch settings.json

**Solutions**:
```bash
# Check if settings.json is valid JSON
cat ~/.ai-assistant/settings.json | python3 -m json.tool

# Use manual patching
otk init -g --no-patch  # Prints JSON snippet

# Restore from backup
cp ~/.ai-assistant/settings.json.bak ~/.ai-assistant/settings.json

# Check permissions
ls -la ~/.ai-assistant/settings.json
chmod 644 ~/.ai-assistant/settings.json
```

### Hook Not Working After Install

**Problem**: Commands still not using OTK after `otk init -g`

**Solutions**:
```bash
# Verify hook is registered
otk init --show

# Check settings.json manually
cat ~/.ai-assistant/settings.json | grep otk-rewrite

# Restart AI coding (critical step!)

# Test with a command
git status  # Should use otk automatically
```

### Uninstall Didn't Remove Everything

**Problem**: OTK traces remain after `otk init -g --uninstall`

**Manual Cleanup**:
```bash
# Remove hook
rm ~/.ai-assistant/hooks/otk-rewrite.sh

# Remove OTK.md
rm ~/.ai-assistant/OTK.md

# Remove @OTK.md reference
nano ~/.ai-assistant/CLAUDE.md  # Delete @OTK.md line

# Remove from settings.json
nano ~/.ai-assistant/settings.json  # Remove OTK hook entry

# Restore from backup
cp ~/.ai-assistant/settings.json.bak ~/.ai-assistant/settings.json
```

See **[TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)** for more issues and solutions.

## For Maintainers

### Security Review Workflow

OTK implements a comprehensive 3-layer security review process for external PRs:

#### Layer 1: Automated GitHub Action
Every PR triggers `.github/workflows/security-check.yml`:
- **Cargo audit**: CVE detection in dependencies
- **Critical files alert**: Flags modifications to high-risk files (runner.rs, tracking.rs, Cargo.toml, workflows)
- **Dangerous pattern scanning**: Shell injection, network operations, unsafe code, panic risks
- **Dependency auditing**: Supply chain verification for new crates
- **Clippy security lints**: Enforces Rust safety best practices

Results appear in the PR's GitHub Actions summary.

#### Layer 2: AI coding Skill
For comprehensive manual review, maintainers with [AI coding](https://ai-assistant.ai/code) can use:

```bash
/otk-pr-security <PR_NUMBER>
```

The skill performs:
- **Critical files analysis**: Detects modifications to shell execution, validation, or CI/CD files
- **Dangerous pattern detection**: Identifies shell injection, environment manipulation, exfiltration vectors
- **Supply chain audit**: Verifies new dependencies on crates.io (downloads, maintainer, license)
- **Semantic analysis**: Checks intent vs reality, logic bombs, code quality red flags
- **Structured report generation**: Produces security assessment with risk level and verdict

**Skill installation** (maintainers only):
```bash
# The skill is bundled in the otk-pr-security directory
# Copy to your AI assistant skills directory:
cp -r ~/.ai-assistant/skills/otk-pr-security ~/.ai-assistant/skills/
```

The skill includes:
- `SKILL.md` - Workflow automation and usage guide
- `critical-files.md` - RTK-specific file risk tiers with attack scenarios
- `dangerous-patterns.md` - Regex patterns with exploitation examples
- `checklist.md` - Manual review template

#### Layer 3: Manual Review
For PRs touching critical files or adding dependencies:
- **2 maintainers required** for Cargo.toml, workflows, or Tier 1 files
- **Isolated testing** recommended for high-risk changes
- Follow the checklist in SECURITY.md

See **[SECURITY.md](SECURITY.md)** for complete security policy and review guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please open an issue or PR on GitHub.

**For external contributors**: Your PR will undergo automated security review (see [SECURITY.md](SECURITY.md)). This protects RTK's shell execution capabilities against injection attacks and supply chain vulnerabilities.

## Contact

- Website: https://www.openclaw.dev
- Email: contact@openclaw.dev
- Issues: https://github.com/openclaw/otk/issues
