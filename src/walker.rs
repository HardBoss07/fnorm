use crate::config::Config;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn collect_files(root: &Path, config: &Config) -> Vec<PathBuf> {
    let include_ext: Vec<String> = config
        .include_extensions
        .iter()
        .map(|s| s.to_lowercase())
        .collect();
    let exclude_ext: Vec<String> = config
        .exclude_extensions
        .iter()
        .map(|s| s.to_lowercase())
        .collect();

    let has_include_filter = !include_ext.is_empty();

    let mut files: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(move |entry| {
            if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                let lower_ext = ext.to_lowercase();

                if exclude_ext.contains(&lower_ext) {
                    return false;
                }

                if has_include_filter && !include_ext.contains(&lower_ext) {
                    return false;
                }

                true
            } else {
                has_include_filter == false
            }
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    // Sort the collected files based on the configuration.
    match config.sort_mode.as_str() {
        "name" => files.sort_by(|a, b| a.file_name().cmp(&b.file_name())),
        "extension" => files.sort_by(|a, b| {
            let a_ext = a.extension().unwrap_or_default();
            let b_ext = b.extension().unwrap_or_default();
            a_ext.cmp(b_ext)
        }),
        _ => {} // "none" or any other value means no sorting.
    }

    files
}
