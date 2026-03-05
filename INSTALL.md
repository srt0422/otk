# OTK Installation Guide - For AI Coding Assistants

## ⚠️ Name Collision Warning

**There are TWO completely different projects named "otk":**

1. ✅ **OpenClaw Token Killer** (this project) - LLM token optimizer
   - Repos: `openclaw/otk`
   - Has `otk gain` command for token savings stats

2. ❌ **Rust Type Kit** (reachingforthejack/rtk) - DIFFERENT PROJECT
   - Rust codebase query tool and type generator
   - **DO NOT install if you want token optimization**

## Pre-Installation Check (REQUIRED)

**AI assistants should ALWAYS verify if OTK is already installed before attempting installation.**

```bash
# Check if OTK is installed
otk --version

# CRITICAL: Verify it's the Token Killer (not Type Kit)
otk gain    # Should show token savings stats, NOT "command not found"

# Check installation path
which otk
```

If `otk gain` works, you have the **correct** OTK installed. **DO NOT reinstall**. Skip to "Project Initialization".

If `otk gain` fails but `otk --version` succeeds, you have the **wrong** OTK (Type Kit). Uninstall and reinstall the correct one (see below).

## Installation (only if OTK not available or wrong OTK installed)

### Step 0: Uninstall Wrong OTK (if needed)

If you accidentally installed Rust Type Kit:

```bash
cargo uninstall otk
```

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/openclaw/otk/master/install.sh | sh
```

After installation, **verify you have the correct otk**:
```bash
otk gain  # Must show token savings stats (not "command not found")
```

### Alternative: Manual Installation

```bash
# From openclaw repository (NOT reachingforthejack!)
cargo install --git https://github.com/openclaw/otk

# OR (if published and correct on crates.io)
cargo install otk

# ALWAYS VERIFY after installation
otk gain  # MUST show token savings, not "command not found"
```

⚠️ **WARNING**: `cargo install otk` from crates.io might install the wrong package. Always verify with `otk gain`.

## Project Initialization

### Which mode to choose?

```
  Do you want OTK active across ALL AI coding projects?
  │
  ├─ YES → otk init -g              (recommended)
  │         Hook + OTK.md (~10 tokens in context)
  │         Commands auto-rewritten transparently
  │
  ├─ YES, minimal → otk init -g --hook-only
  │         Hook only, nothing added to AGENTS.md
  │         Zero tokens in context
  │
  └─ NO, single project → otk init
            Local AGENTS.md only (137 lines)
            No hook, no global effect
```

### Recommended: Global Hook-First Setup

**Best for: All projects, automatic OTK usage**

```bash
otk init -g
# → Installs hook to ~/.ai-assistant/hooks/otk-rewrite.sh
# → Creates ~/.ai-assistant/OTK.md (10 lines, meta commands only)
# → Adds @OTK.md reference to ~/.ai-assistant/AGENTS.md
# → Prompts: "Patch settings.json? [y/N]"
# → If yes: patches + creates backup (~/.ai-assistant/settings.json.bak)

# Automated alternatives:
otk init -g --auto-patch    # Patch without prompting
otk init -g --no-patch      # Print manual instructions instead

# Verify installation
otk init --show  # Check hook is installed and executable
```

**Token savings**: ~99.5% reduction (2000 tokens → 10 tokens in context)

**What is settings.json?**
AI coding's hook registry. OTK adds a PreToolUse hook that rewrites commands transparently. Without this, AI assistant won't invoke the hook automatically.

```
  AI coding          settings.json        otk-rewrite.sh        OTK binary
       │                    │                     │                    │
       │  "git status"      │                     │                    │
       │ ──────────────────►│                     │                    │
       │                    │  PreToolUse trigger  │                    │
       │                    │ ───────────────────►│                    │
       │                    │                     │  rewrite command   │
       │                    │                     │  → otk git status  │
       │                    │◄────────────────────│                    │
       │                    │  updated command     │                    │
       │                    │                                          │
       │  execute: otk git status                                      │
       │ ─────────────────────────────────────────────────────────────►│
       │                                                               │  filter
       │  "3 modified, 1 untracked ✓"                                  │
       │◄──────────────────────────────────────────────────────────────│
```

**Backup Safety**:
OTK backs up existing settings.json before changes. Restore if needed:
```bash
cp ~/.ai-assistant/settings.json.bak ~/.ai-assistant/settings.json
```

### Alternative: Local Project Setup

**Best for: Single project without hook**

```bash
cd /path/to/your/project
otk init  # Creates ./AGENTS.md with full OTK instructions (137 lines)
```

**Token savings**: Instructions loaded only for this project

### Upgrading from Previous Version

#### From old 137-line AGENTS.md injection (pre-0.22)

```bash
otk init -g  # Automatically migrates to hook-first mode
# → Removes old 137-line block
# → Installs hook + OTK.md
# → Adds @OTK.md reference
```

#### From old hook with inline logic (pre-0.24) — ⚠️ Breaking Change

OTK 0.24.0 replaced the inline command-detection hook (~200 lines) with a **thin delegator** that calls `otk rewrite`. The binary now contains the rewrite logic, so adding new commands no longer requires a hook update.

The old hook still works but won't benefit from new rules added in future releases.

```bash
# Upgrade hook to thin delegator
otk init --global

# Verify the new hook is active
otk init --show
# Should show: ✅ Hook: ... (thin delegator, up to date)
```

## Common User Flows

### First-Time User (Recommended)
```bash
# 1. Install OTK
cargo install --git https://github.com/openclaw/otk
otk gain  # Verify (must show token stats)

# 2. Setup with prompts
otk init -g
# → Answer 'y' when prompted to patch settings.json
# → Creates backup automatically

# 3. Restart AI coding
# 4. Test: git status (should use otk)
```

### CI/CD or Automation
```bash
# Non-interactive setup (no prompts)
otk init -g --auto-patch

# Verify in scripts
otk init --show | grep "Hook:"
```

### Conservative User (Manual Control)
```bash
# Get manual instructions without patching
otk init -g --no-patch

# Review printed JSON snippet
# Manually edit ~/.ai-assistant/settings.json
# Restart AI coding
```

### Temporary Trial
```bash
# Install hook
otk init -g --auto-patch

# Later: remove everything
otk init -g --uninstall

# Restore backup if needed
cp ~/.ai-assistant/settings.json.bak ~/.ai-assistant/settings.json
```

## Installation Verification

```bash
# Basic test
otk ls .

# Test with git
otk git status

# Test with pnpm (fork only)
otk pnpm list

# Test with Vitest (feat/vitest-support branch only)
otk vitest run
```

## Uninstalling

### Complete Removal (Global Installations Only)

```bash
# Complete removal (global installations only)
otk init -g --uninstall

# What gets removed:
#   - Hook: ~/.ai-assistant/hooks/otk-rewrite.sh
#   - Context: ~/.ai-assistant/OTK.md
#   - Reference: @OTK.md line from ~/.ai-assistant/AGENTS.md
#   - Registration: OTK hook entry from settings.json

# Restart AI coding after uninstall
```

**For Local Projects**: Manually remove OTK block from `./AGENTS.md`

### Binary Removal

```bash
# If installed via cargo
cargo uninstall otk

# If installed via package manager
brew uninstall otk          # macOS Homebrew
sudo apt remove otk         # Debian/Ubuntu
sudo dnf remove otk         # Fedora/RHEL
```

### Restore from Backup (if needed)

```bash
cp ~/.ai-assistant/settings.json.bak ~/.ai-assistant/settings.json
```

## Essential Commands

### Files
```bash
otk ls .              # Compact tree view
otk read file.rs      # Optimized reading
otk grep "pattern" .  # Grouped search results
```

### Git
```bash
otk git status        # Compact status
otk git log -n 10     # Condensed logs
otk git diff          # Optimized diff
otk git add .         # → "ok ✓"
otk git commit -m "msg"  # → "ok ✓ abc1234"
otk git push          # → "ok ✓ main"
```

### Pnpm (fork only)
```bash
otk pnpm list         # Dependency tree (-70% tokens)
otk pnpm outdated     # Available updates (-80-90%)
otk pnpm install pkg  # Silent installation
```

### Tests
```bash
otk test cargo test   # Failures only (-90%)
otk vitest run        # Filtered Vitest output (-99.6%)
```

### Statistics
```bash
otk gain              # Token savings
otk gain --graph      # With ASCII graph
otk gain --history    # With command history
```

## Validated Token Savings

### Production T3 Stack Project
| Operation | Standard | OTK | Reduction |
|-----------|----------|-----|-----------|
| `vitest run` | 102,199 chars | 377 chars | **-99.6%** |
| `git status` | 529 chars | 217 chars | **-59%** |
| `pnpm list` | ~8,000 tokens | ~2,400 | **-70%** |
| `pnpm outdated` | ~12,000 tokens | ~1,200-2,400 | **-80-90%** |

### Typical AI coding Session (30 min)
- **Without OTK**: ~150,000 tokens
- **With OTK**: ~45,000 tokens
- **Savings**: **70% reduction**

## Troubleshooting

### OTK command not found after installation
```bash
# Check PATH
echo $PATH | grep -o '[^:]*\.cargo[^:]*'

# Add to PATH if needed (~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"

# Reload shell
source ~/.bashrc  # or source ~/.zshrc
```

### OTK command not available (e.g., vitest)
```bash
# Check branch
cd /path/to/otk
git branch

# Switch to feat/vitest-support if needed
git checkout feat/vitest-support

# Reinstall
cargo install --path . --force
```

### Compilation error
```bash
# Update Rust
rustup update stable

# Clean and recompile
cargo clean
cargo build --release
cargo install --path . --force
```

## Support and Contributing

- **Website**: https://www.openclaw.dev
- **Contact**: contact@openclaw.dev
- **Troubleshooting**: See [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) for common issues
- **GitHub issues**: https://github.com/openclaw/otk/issues
- **Pull Requests**: https://github.com/openclaw/otk/pulls

⚠️ **If you installed the wrong otk (Type Kit)**, see [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md#problem-otk-gain-command-not-found)

## AI Assistant Checklist

Before each session:

- [ ] Verify OTK is installed: `otk --version`
- [ ] If not installed → follow "Install from fork"
- [ ] If project not initialized → `otk init`
- [ ] Use `otk` for ALL git/pnpm/test/vitest commands
- [ ] Check savings: `otk gain`

**Golden Rule**: AI coding assistants should ALWAYS use `otk` as a proxy for shell commands that generate verbose output (git, pnpm, npm, cargo test, vitest, docker, kubectl).
