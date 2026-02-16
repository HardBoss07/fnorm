use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use crate::fs;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub default_template: String,
    pub index_mode: String,
    pub cli_overrides_index_mode: bool,
    pub cleanup_separators: Vec<String>,
    pub index_padding: usize,
    pub sort_mode: String,
    pub include_extensions: Vec<String>,
    pub exclude_extensions: Vec<String>,
    pub force_lowercase_extension: bool,
    pub collision_pattern: String,
    pub profiles: HashMap<String, Profile>,
    pub image: Image,
    pub features: Features,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Profile {
    pub template: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Image {
    pub allow_non_images: bool,
    pub skip_if_no_dimensions: bool,
    pub metadata_only: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Features {
    pub skip_if_unchanged: bool,
    pub auto_collision_resolution: bool,
    pub start_index: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_template: "{parent}-{N}.{ext}".into(),
            index_mode: "per-directory".into(),
            cli_overrides_index_mode: true,
            cleanup_separators: vec!["_".into(), "-".into(), " ".into()],
            index_padding: 0,
            sort_mode: "name".into(),
            include_extensions: vec![],
            exclude_extensions: vec![],
            force_lowercase_extension: true,
            collision_pattern: "-{n}".into(),
            profiles: HashMap::new(),
            image: Image::default(),
            features: Features::default(),
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            template: "".into(),
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            allow_non_images: true,
            skip_if_no_dimensions: false,
            metadata_only: true,
        }
    }
}

impl Default for Features {
    fn default() -> Self {
        Self {
            skip_if_unchanged: true,
            auto_collision_resolution: true,
            start_index: 1,
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
