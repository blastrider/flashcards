#![forbid(unsafe_code)]
use anyhow::Context;
use clap::Parser;
use tracing_subscriber;
use flashcards::{cli, import, quiz_cmd, export_cmd, stats_cmd, reset_cmd, config_cmd};

fn init_logging(debug: bool) {
    let filter = if debug { "flashcards=debug" } else { "info" };
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    init_logging(args.debug);

    match args.command {
        cli::Command::Import { path, strict } => {
            import(&path, strict).context("import failed")?;
        }
        cli::Command::Quiz { count, category, mode } => {
            quiz_cmd(count, category.as_deref(), mode)?;
        }
        cli::Command::Export { md, pdf } => {
            export_cmd(&md, pdf.as_deref())?;
        }
        cli::Command::Stats => {
            stats_cmd()?;
        }
        cli::Command::Reset { all, category } => {
            reset_cmd(all, category.as_deref())?;
        }
        cli::Command::Config => {
            config_cmd()?;
        }
    }

    Ok(())
}
