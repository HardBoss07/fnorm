mod cli;
mod config;
mod fs;
mod util;

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
