#![forbid(unsafe_code)]
use anyhow::{Context, Result};
use std::fs;
use crate::model::Card;

/// Écrit un Markdown simple et imprimable (utilisé comme source et backup).
pub fn write_md(md_path: &str, cards: &Vec<Card>) -> Result<()> {
    let mut md_s = String::new();
    md_s.push_str("# Flashcards\n\n");
    for c in cards {
        md_s.push_str(&format!("## {}\n\n", c.question));
        md_s.push_str(&format!("{}\n\n", c.answer));
        if let Some(cat) = &c.category {
            md_s.push_str(&format!("_cat: {}_\n\n", cat));
        }
        md_s.push_str("---\n\n");
    }
    fs::write(md_path, md_s).with_context(|| format!("write md {}", md_path))?;
    Ok(())
}
