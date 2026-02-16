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

    let files = walker::collect_files(&cli.path, &config);

    let global_mode = if config.cli_overrides_index_mode && cli.global {
        true
    } else {
        config.index_mode == "global"
    };

    let mut indexer = Indexer::new(global_mode, config.features.start_index);

    let template_str = if let Some(profile_name) = &cli.profile {
        if let Some(profile) = config.profiles.get(profile_name) {
            &profile.template
        } else {
            // Potentially print a warning here in the future
            &config.default_template
        }
    } else {
        &config.default_template
    };

    for file in files {
        let parent = file.parent().unwrap();
        let n = indexer.next(parent);

        let context = template::TemplateContext::new(&file, &config, n, template_str);
        let rendered = template::render(&context)?;

        let cleaned = cleanup::cleanup(rendered, &config.cleanup_separators);

        let proposed_path = parent.join(cleaned);
        let final_path = renamer::resolve_collision(&proposed_path, &config);

        renamer::perform_rename(&file, &final_path, cli.dry_run(), &config)?;
    }

    Ok(())
}
