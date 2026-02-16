use crate::config::Config;
use crate::image_meta;
use anyhow::Result;
use std::path::Path;

/// Holds all data needed to render a template for a single file.
pub struct TemplateContext<'a> {
    pub path: &'a Path,
    pub config: &'a Config,
    pub index: usize,
    template_str: &'a str,
}

impl<'a> TemplateContext<'a> {
    pub fn new(path: &'a Path, config: &'a Config, index: usize, template_str: &'a str) -> Self {
        Self {
            path,
            config,
            index,
            template_str,
        }
    }
}

/// Renders the template string from the context.
pub fn render(context: &TemplateContext) -> Result<String> {
    let mut result = String::new();
    let mut chars = context.template_str.chars().peekable();

    // Lazily load image dimensions only if needed.
    let mut dims: Option<Option<(u32, u32)>> = None;

    while let Some(c) = chars.next() {
        if c == '{' {
            let mut key = String::new();
            while let Some(&next) = chars.peek() {
                chars.next();
                if next == '}' {
                    break;
                }
                key.push(next);
            }

            let replacement = match key.as_str() {
                "parent" => context
                    .path
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|s| s.to_string_lossy().to_lowercase())
                    .unwrap_or_default(),
                "PARENT" => context
                    .path
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|s| s.to_string_lossy().to_uppercase())
                    .unwrap_or_default(),
                "N" => {
                    let padding = context.config.index_padding;
                    if padding > 0 {
                        format!("{:0width$}", context.index, width = padding)
                    } else {
                        context.index.to_string()
                    }
                }
                "ext" => {
                    let ext = context
                        .path
                        .extension()
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default();
                    if context.config.force_lowercase_extension {
                        ext.to_lowercase()
                    } else {
                        ext.to_string()
                    }
                }
                "width" => {
                    if dims.is_none() {
                        dims = Some(image_meta::dimensions(context.path));
                    }
                    dims.flatten().map(|d| d.0.to_string()).unwrap_or_default()
                }
                "height" => {
                    if dims.is_none() {
                        dims = Some(image_meta::dimensions(context.path));
                    }
                    dims.flatten().map(|d| d.1.to_string()).unwrap_or_default()
                }
                _ => format!("{{{}}}", key),
            };

            result.push_str(&replacement);
        } else {
            result.push(c);
        }
    }

    Ok(result)
}
