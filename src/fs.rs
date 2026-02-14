use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};

use crate::util::exe_dir;

const CONFIG_NAME: &str = ".fname.toml";

pub fn read_override(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(|e| anyhow!("Failed to read {}: {}", path.display(), e))
}

pub fn read_local() -> Result<Option<String>> {
    let path = PathBuf::from(CONFIG_NAME);
    if !path.exists() {
        return Ok(None);
    }

    Ok(Some(fs::read_to_string(&path).map_err(|e| {
        anyhow!("Failed to read {}: {}", path.display(), e)
    })?))
}

pub fn read_global() -> Result<Option<String>> {
    let path = exe_dir()?.join(CONFIG_NAME);
    if !path.exists() {
        return Ok(None);
    }

    Ok(Some(fs::read_to_string(&path).map_err(|e| {
        anyhow!("Failed to read {}: {}", path.display(), e)
    })?))
}
