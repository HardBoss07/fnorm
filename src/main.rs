mod cleanup;
mod cli;
mod config;
mod fs;
mod image_meta;
mod index;
mod renamer;
mod template;
mod util;
mod walker;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::Config;
use index::Indexer;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load(cli.config.as_deref())?;

    let files = walker::collect_files(&cli.path);

    let global_mode = if cli.global {
        true
    } else {
        config.index_mode == "global"
    };

    let mut indexer = Indexer::new(global_mode);

    for file in files {
        let parent = file.parent().unwrap();
        let parent_name = parent.file_name().unwrap_or_default().to_string_lossy();

        let ext = file.extension().unwrap_or_default().to_string_lossy();

        let n = indexer.next(parent);

        let dims = image_meta::dimensions(&file);

        let rendered = template::render(&config.default_template, &parent_name, n, &ext, dims);

        let cleaned = cleanup::cleanup(rendered, &config.cleanup_separators);

        let new_path = parent.join(cleaned);

        renamer::rename(&file, &new_path, cli.dry_run())?;
    }

    Ok(())
}
