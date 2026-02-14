use anyhow::{anyhow, Result};
use std::env;
use std::path::PathBuf;

pub fn exe_dir() -> Result<PathBuf> {
    let exe = env::current_exe()
        .map_err(|e| anyhow!("Failed to get executable path: {}", e))?;

    exe.parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| anyhow!("Executable has no parent directory"))
}
