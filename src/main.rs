use std::path::PathBuf;

use clap::{arg, Parser, Subcommand};
use diag::du;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Du {
        #[arg(short, long)]
        reverse: bool,
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Du { reverse, path } => {
            let rootdir = std::fs::canonicalize(path)
                .unwrap_or_else(|_| panic!("canonicalizing path {} failed", path.display()));

            if !rootdir.is_dir() {
                panic!("Error: {} not a directory", rootdir.display());
            }

            du(&rootdir, *reverse).expect("du had a problem");
        }
    }
}
