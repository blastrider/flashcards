#![forbid(unsafe_code)]
// Ré-exporte les modules (assume que tu as ces fichiers sous src/)
pub mod cli;
pub mod config;
pub mod csv_import;
pub mod export;
pub mod model;
pub mod quiz;
pub mod storage;

use anyhow::Result;

/// API pratique appelée par le binaire.
/// Expose des wrappers simples vers les fonctions existantes.
pub fn import(path: &str, strict: bool) -> Result<()> {
    csv_import::cmd_import(path, strict)
}

pub fn quiz_cmd(count: Option<usize>, category: Option<&str>, mode: String) -> Result<()> {
    quiz::cmd_quiz(count, category, mode)
}

pub fn export_cmd(md: &str, pdf: Option<&str>) -> Result<()> {
    export::cmd_export(md, pdf)
}

pub fn stats_cmd() -> Result<()> {
    storage::print_stats()
}

pub fn reset_cmd(all: bool, category: Option<&str>) -> Result<()> {
    storage::cmd_reset(all, category)
}

pub fn config_cmd() -> Result<()> {
    config::cmd_config()
}
