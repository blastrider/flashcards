#![forbid(unsafe_code)]
pub mod md;
pub mod html;
pub mod pdf;

use anyhow::{Context, Result};
use std::path::Path;
use crate::storage::load_store;

/// API publique inchangée: génère Markdown + HTML, puis optionnellement PDF.
pub fn cmd_export(md: &str, pdf: Option<&str>) -> Result<()> {
    let store = load_store().context("load store for export")?;

    // 1) Markdown
    md::write_md(md, &store.cards).context("write markdown")?;
    println!("Markdown exporté: {}", md);

    // 2) HTML (same base name)
    let html_path = Path::new(md).with_extension("html");
    html::write_html(&html_path, &store.cards).with_context(|| format!("write html {}", html_path.display()))?;
    println!("HTML exporté (mise en page moderne): {}", html_path.display());

    // 3) PDF (optionnel)
    if let Some(pdf_path) = pdf {
        pdf::generate_pdf(&html_path, pdf_path)?;
    }

    Ok(())
}
