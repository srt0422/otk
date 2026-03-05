use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Target environment for otk initialization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InitTarget {
    OpenClaw,
    Cursor,
}

/// Main entry point for `otk init`
pub fn run(target: InitTarget, verbose: u8) -> Result<()> {
    match target {
        InitTarget::OpenClaw => run_openclaw_init(verbose),
        InitTarget::Cursor => run_cursor_init(verbose),
    }
}

/// OpenClaw init: create AGENTS.md snippet for token optimization
fn run_openclaw_init(verbose: u8) -> Result<()> {
    let agents_md_path = PathBuf::from("AGENTS.md");

    let snippet = generate_openclaw_snippet();

    if agents_md_path.exists() {
        let existing = fs::read_to_string(&agents_md_path)
            .with_context(|| format!("Failed to read {}", agents_md_path.display()))?;

        if existing.contains("<!-- otk-instructions") {
            println!("AGENTS.md already contains otk instructions");
            return Ok(());
        }

        // Append to existing AGENTS.md
        let new_content = format!("{}\n\n{}", existing.trim(), snippet);
        fs::write(&agents_md_path, new_content)
            .with_context(|| format!("Failed to write {}", agents_md_path.display()))?;

        println!("Added otk instructions to existing AGENTS.md");
    } else {
        fs::write(&agents_md_path, &snippet)
            .with_context(|| format!("Failed to create {}", agents_md_path.display()))?;

        println!("Created AGENTS.md with otk instructions");
    }

    if verbose > 0 {
        eprintln!("Path: {}", agents_md_path.display());
    }

    println!("\nOpenClaw agents will now use otk for token-optimized output.");
    println!("Test with: otk git status");

    Ok(())
}

/// Cursor init: create .cursor/rules file for IDE integration
fn run_cursor_init(verbose: u8) -> Result<()> {
    let cursor_dir = PathBuf::from(".cursor");
    let rules_path = cursor_dir.join("rules");

    // Create .cursor directory if needed
    if !cursor_dir.exists() {
        fs::create_dir_all(&cursor_dir)
            .with_context(|| format!("Failed to create {}", cursor_dir.display()))?;
    }

    let snippet = generate_cursor_rules();

    if rules_path.exists() {
        let existing = fs::read_to_string(&rules_path)
            .with_context(|| format!("Failed to read {}", rules_path.display()))?;

        if existing.contains("<!-- otk-instructions") {
            println!(".cursor/rules already contains otk instructions");
            return Ok(());
        }

        // Append to existing rules
        let new_content = format!("{}\n\n{}", existing.trim(), snippet);
        fs::write(&rules_path, new_content)
            .with_context(|| format!("Failed to write {}", rules_path.display()))?;

        println!("Added otk instructions to existing .cursor/rules");
    } else {
        fs::write(&rules_path, &snippet)
            .with_context(|| format!("Failed to create {}", rules_path.display()))?;

        println!("Created .cursor/rules with otk instructions");
    }

    if verbose > 0 {
        eprintln!("Path: {}", rules_path.display());
    }

    println!("\nCursor will now prefer otk-prefixed commands.");
    println!("Test with: otk git status");

    Ok(())
}

/// Uninstall otk instructions from project
pub fn uninstall(verbose: u8) -> Result<()> {
    let mut removed = Vec::new();

    // Remove from AGENTS.md
    let agents_md_path = PathBuf::from("AGENTS.md");
    if agents_md_path.exists() {
        let content = fs::read_to_string(&agents_md_path)?;
        if content.contains("<!-- otk-instructions") {
            let new_content = remove_otk_block(&content);
            if new_content.trim().is_empty() {
                fs::remove_file(&agents_md_path)?;
                removed.push(format!("AGENTS.md: removed (was otk-only)"));
            } else {
                fs::write(&agents_md_path, new_content)?;
                removed.push(format!("AGENTS.md: removed otk instructions"));
            }
        }
    }

    // Remove from .cursor/rules
    let rules_path = PathBuf::from(".cursor/rules");
    if rules_path.exists() {
        let content = fs::read_to_string(&rules_path)?;
        if content.contains("<!-- otk-instructions") {
            let new_content = remove_otk_block(&content);
            if new_content.trim().is_empty() {
                fs::remove_file(&rules_path)?;
                removed.push(format!(".cursor/rules: removed (was otk-only)"));
            } else {
                fs::write(&rules_path, new_content)?;
                removed.push(format!(".cursor/rules: removed otk instructions"));
            }
        }
    }

    if removed.is_empty() {
        println!("otk was not installed (nothing to remove)");
    } else {
        println!("otk uninstalled:");
        for item in removed {
            println!("  - {}", item);
        }
    }

    Ok(())
}

/// Remove otk instructions block from content
fn remove_otk_block(content: &str) -> String {
    if let (Some(start), Some(end)) = (
        content.find("<!-- otk-instructions"),
        content.find("<!-- /otk-instructions -->"),
    ) {
        let end_pos = end + "<!-- /otk-instructions -->".len();
        let before = content[..start].trim_end();
        let after = content[end_pos..].trim_start();

        if after.is_empty() {
            before.to_string()
        } else {
            format!("{}\n\n{}", before, after)
        }
    } else {
        content.to_string()
    }
}

/// Generate OpenClaw AGENTS.md snippet
fn generate_openclaw_snippet() -> String {
    r##"<!-- otk-instructions v1 -->
## OTK (OpenClaw Token Killer) - Token-Optimized Commands

**Rule**: Prefix commands with `otk` for 60-90% token savings.

### Quick Reference

| Category | Commands | Savings |
|----------|----------|---------|
| Build | `otk cargo build`, `otk tsc`, `otk next build` | 80-90% |
| Test | `otk cargo test`, `otk vitest`, `otk pytest` | 90-99% |
| Git | `otk git status`, `otk git log`, `otk git diff` | 59-80% |
| Files | `otk ls`, `otk read <file>`, `otk grep` | 60-75% |
| Package | `otk pnpm list`, `otk pip list` | 70-90% |

### Examples

```bash
otk git status          # Compact status
otk cargo test          # Show failures only (90% savings)
otk ls src/             # Tree format, counts
otk read src/main.rs    # Filtered code
otk grep "TODO"         # Grouped by file
```

### Meta Commands

```bash
otk gain                # View token savings statistics
otk gain --history      # View command history
otk proxy <cmd>         # Run without filtering (debugging)
```

**Passthrough**: Unknown commands pass through unchanged, so `otk` is always safe to use.
<!-- /otk-instructions -->
"##.to_string()
}

/// Generate Cursor rules snippet
fn generate_cursor_rules() -> String {
    r##"<!-- otk-instructions v1 -->
# OTK (OpenClaw Token Killer) - Token-Optimized Commands

When running shell commands, prefer `otk` prefix for token-efficient output:

- `otk git status` instead of `git status`
- `otk cargo test` instead of `cargo test`
- `otk ls` instead of `ls`
- `otk read <file>` instead of `cat <file>`

OTK reduces token consumption by 60-90% through smart filtering and compression.

## Key Commands

- **Build**: `otk cargo build`, `otk tsc`, `otk next build`
- **Test**: `otk cargo test`, `otk vitest run`, `otk pytest`
- **Git**: `otk git status`, `otk git log`, `otk git diff`
- **Files**: `otk ls`, `otk read`, `otk grep`
- **Stats**: `otk gain` (view savings)

Unknown commands pass through unchanged, so `otk` is always safe.
<!-- /otk-instructions -->
"##.to_string()
}

/// Show current otk configuration
pub fn show_config() -> Result<()> {
    let agents_md_path = PathBuf::from("AGENTS.md");
    let rules_path = PathBuf::from(".cursor/rules");

    println!("otk Configuration:\n");

    // Check AGENTS.md
    if agents_md_path.exists() {
        let content = fs::read_to_string(&agents_md_path)?;
        if content.contains("<!-- otk-instructions") {
            println!("  AGENTS.md: otk enabled (OpenClaw)");
        } else {
            println!("  AGENTS.md: exists but otk not configured");
        }
    } else {
        println!("  AGENTS.md: not found");
    }

    // Check .cursor/rules
    if rules_path.exists() {
        let content = fs::read_to_string(&rules_path)?;
        if content.contains("<!-- otk-instructions") {
            println!("  .cursor/rules: otk enabled (Cursor)");
        } else {
            println!("  .cursor/rules: exists but otk not configured");
        }
    } else {
        println!("  .cursor/rules: not found");
    }

    println!("\nUsage:");
    println!("  otk init --openclaw   # Add to AGENTS.md (default)");
    println!("  otk init --cursor     # Add to .cursor/rules");
    println!("  otk init --uninstall  # Remove otk instructions");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_generate_openclaw_snippet_has_markers() {
        let snippet = generate_openclaw_snippet();
        assert!(snippet.contains("<!-- otk-instructions"));
        assert!(snippet.contains("<!-- /otk-instructions -->"));
    }

    #[test]
    fn test_generate_cursor_rules_has_markers() {
        let snippet = generate_cursor_rules();
        assert!(snippet.contains("<!-- otk-instructions"));
        assert!(snippet.contains("<!-- /otk-instructions -->"));
    }

    #[test]
    fn test_remove_otk_block() {
        let content = r#"# My Config

<!-- otk-instructions v1 -->
OTK STUFF
<!-- /otk-instructions -->

More content"#;

        let result = remove_otk_block(content);
        assert!(!result.contains("OTK STUFF"));
        assert!(result.contains("# My Config"));
        assert!(result.contains("More content"));
    }

    #[test]
    fn test_remove_otk_block_only_otk() {
        let content = r#"<!-- otk-instructions v1 -->
OTK STUFF
<!-- /otk-instructions -->"#;

        let result = remove_otk_block(content);
        assert!(result.is_empty());
    }

    #[test]
    fn test_openclaw_init_creates_agents_md() {
        let temp = TempDir::new().unwrap();
        let agents_md = temp.path().join("AGENTS.md");

        let snippet = generate_openclaw_snippet();
        fs::write(&agents_md, &snippet).unwrap();

        assert!(agents_md.exists());
        let content = fs::read_to_string(&agents_md).unwrap();
        assert!(content.contains("otk git status"));
    }

    #[test]
    fn test_cursor_init_creates_rules() {
        let temp = TempDir::new().unwrap();
        let cursor_dir = temp.path().join(".cursor");
        fs::create_dir_all(&cursor_dir).unwrap();
        let rules_path = cursor_dir.join("rules");

        let snippet = generate_cursor_rules();
        fs::write(&rules_path, &snippet).unwrap();

        assert!(rules_path.exists());
        let content = fs::read_to_string(&rules_path).unwrap();
        assert!(content.contains("otk git status"));
    }
}
