use clap::Parser;
use std::env::current_dir;

use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLIArgs {
    /// The directory from which to serve files (relative to cwd or absolute)
    /// By default the current working directory is used.
    #[arg(short, long)]
    pub dir: Option<PathBuf>,
}

impl CLIArgs {
    pub fn get_default_serve_dir(self) -> PathBuf {
        self.dir
            .unwrap_or(current_dir().expect("Unable to find cwd!"))
    }
}

// TODO add the server address and possibly port to the CLI args
