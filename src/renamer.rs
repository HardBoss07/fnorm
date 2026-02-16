use crate::config::Config;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Determines the final, safe path for a rename operation, handling collisions.
pub fn resolve_collision(proposed_path: &Path, config: &Config) -> PathBuf {
    if !config.features.auto_collision_resolution || !proposed_path.exists() {
        return proposed_path.to_path_buf();
    }

    let mut counter = 1;
    let mut target = proposed_path.to_path_buf();

    let stem = proposed_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();

    let ext = proposed_path
        .extension()
        .map(|s| format!(".{}", s.to_string_lossy()))
        .unwrap_or_default();

    while target.exists() {
        let suffix = config
            .collision_pattern
            .replace("{n}", &counter.to_string());
        let new_name = format!("{}{}{}", stem, suffix, ext);
        target = proposed_path.with_file_name(new_name);
        counter += 1;
    }

    target
}

/// Executes the rename operation.
pub fn perform_rename(old: &Path, new: &Path, dry_run: bool, config: &Config) -> Result<()> {
    if config.features.skip_if_unchanged && old == new {
        // In a future version, we might want a "verbose" mode
        // where we print skipped files. For now, just do nothing.
        return Ok(());
    }

    if !dry_run {
        // Ensure parent directory exists, especially for complex renames
        if let Some(parent) = new.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::rename(old, new)?;
    }

    println!("{} -> {}", old.display(), new.display());
    Ok(())
}
