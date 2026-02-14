use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "fnorm - File Normalizor")]
pub struct Cli {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub config: Option<PathBuf>,

    #[arg(long)]
    pub global: bool,

    #[arg(long)]
    pub no_dry_run: bool,
}

impl Cli {
    pub fn dry_run(&self) -> bool {
        !self.no_dry_run
    }
}
