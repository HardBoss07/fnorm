mod cli;
mod util;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Path: {:?}", cli.path);
    println!("Dry-run: {}", cli.dry_run());

    Ok(())
}
