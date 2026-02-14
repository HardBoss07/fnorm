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

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::load(cli.config.as_deref())?;

    println!("{:#?}", config);

    Ok(())
}
