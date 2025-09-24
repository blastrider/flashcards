#![forbid(unsafe_code)]
use anyhow::Context;
use clap::Parser;


mod cli;
mod config;
mod csv_import;
mod export;
mod model;
mod quiz;
mod storage;

use cli::{Args, Command};

fn init_logging(debug: bool) {
    let filter = if debug {
        "flashcards_cli=debug,tokio=info"
    } else {
        "info"
    };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    init_logging(args.debug);

    match args.command {
        Command::Import { path, strict } => {
            csv_import::cmd_import(&path, strict).context("import failed")?;
        }
        Command::Quiz { count, category, mode } => {
            quiz::cmd_quiz(count, category.as_deref(), mode)?;
        }
        Command::Export { md, pdf } => {
            export::cmd_export(&md, pdf.as_deref())?;
        }
        Command::Stats => {
            storage::print_stats()?;
        }
        Command::Reset { all, category } => {
            storage::cmd_reset(all, category.as_deref())?;
        }
        Command::Config => {
            config::cmd_config()?;
        }
    }

    Ok(())
}
