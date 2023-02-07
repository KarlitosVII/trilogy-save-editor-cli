#![warn(clippy::all)]

#[macro_use]
extern crate derive_more;

mod cli;
mod save_data;
mod save_game;
mod unreal;

use anyhow::Result;
use clap::{Parser, Subcommand};
use indexmap::IndexMap as RealIndexMap;

use crate::cli::{ExportHeadMorph, ImportHeadMorph};

pub type IndexMap<K, V> = RealIndexMap<K, V, ahash::RandomState>;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Head Morph import
    ImportHeadMorph(ImportHeadMorph),
    /// Head Morph export
    ExportHeadMorph(ExportHeadMorph),
    /// Print license (CECILL-2.1)
    License,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ImportHeadMorph(import) => import.execute(),
        Commands::ExportHeadMorph(export) => export.execute(),
        Commands::License => {
            println!(include_str!("../LICENSE.txt"));
            Ok(())
        }
    }
}
