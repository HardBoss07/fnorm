use std::fs;
use std::path::{Path, PathBuf};

pub fn rename(old: &Path, new: &Path, dry: bool) -> anyhow::Result<()> {
    if dry {
        println!("{} -> {}", old.display(), new.display());
        return Ok(());
    }

    if new.exists() {
        anyhow::bail!("Collision: {}", new.display());
    }

    fs::rename(old, new)?;
    Ok(())
}
