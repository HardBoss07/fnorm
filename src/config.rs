use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::path::Path;

use crate::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_template: String,

    #[serde(default = "default_index_mode")]
    pub index_mode: String,

    #[serde(default = "default_cleanup")]
    pub cleanup_separators: Vec<String>,
}

fn default_index_mode() -> String {
    "per-directory".into()
}

fn default_cleanup() -> Vec<String> {
    vec!["_".into(), "-".into(), " ".into()]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_template: "{parent}-{N}.{ext}".into(),
            index_mode: default_index_mode(),
            cleanup_separators: default_cleanup(),
        }
    }
}

impl Config {
    pub fn load(override_path: Option<&Path>) -> Result<Self> {
        if let Some(p) = override_path {
            return Self::parse(&fs::read_override(p)?);
        }

        if let Some(local) = fs::read_local()? {
            return Self::parse(&local);
        }

        if let Some(global) = fs::read_global()? {
            return Self::parse(&global);
        }

        Ok(Self::default())
    }

    fn parse(s: &str) -> Result<Self> {
        toml::from_str(s).map_err(|e| anyhow!("Config parse error: {}", e))
    }
}
