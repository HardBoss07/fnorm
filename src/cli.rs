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

    #[arg(long, short = 'a')]
    pub apply: bool,

    #[arg(long)]
    pub profile: Option<String>,
}

impl Cli {
    pub fn dry_run(&self) -> bool {
        !self.apply
    }
}
