use clap::{Parser, Subcommand};

/// Flashcards CLI — offline Leitner reviewer
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Debug (more logs)
    #[arg(long, short)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Import a strict CSV: header must be question,answer[,category]
    Import {
        /// Path to CSV
        path: String,
        /// Strict: refuse if any malformed rows
        #[arg(long)]
        strict: bool,
    },
    /// Start an interactive quiz
    Quiz {
        /// number of cards this session (default = all due)
        #[arg(long)]
        count: Option<usize>,
        /// filter by category
        #[arg(long)]
        category: Option<String>,
        /// mode: timed or self (currently behaves same)
        #[arg(long, default_value = "self")]
        mode: String,
    },
    /// Export markdown, optional pdf via local tool
    Export {
        /// Markdown output path
        #[arg(long)]
        md: String,
        /// PDF output path (requires external tool)
        #[arg(long)]
        pdf: Option<String>,
    },
    /// Show stats
    Stats,
    /// Reset progress
    Reset {
        /// reset all progress
        #[arg(long)]
        all: bool,
        /// reset one category
        #[arg(long)]
        category: Option<String>,
    },
    /// Show / edit config (simple)
    Config,
}
