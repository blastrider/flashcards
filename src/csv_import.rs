#![forbid(unsafe_code)]
use anyhow::{Context, Result};
use csv::ReaderBuilder;
use std::fs::File;
use crate::model::Card;
use crate::storage::{load_store, save_store};

const MAX_FIELD_LEN: usize = 1000;

pub fn cmd_import(path: &str, strict: bool) -> Result<()> {
    let f = File::open(path).with_context(|| format!("open CSV {}", path))?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(f);

    // validate header
    let headers = rdr.headers().context("read headers")?.clone();
    if headers.len() < 2 {
        anyhow::bail!("CSV header must contain at least 'question,answer'");
    }
    if headers.get(0).map(|s| s.to_lowercase()) != Some("question".to_string())
        || headers.get(1).map(|s| s.to_lowercase()) != Some("answer".to_string())
    {
        anyhow::bail!("CSV header first columns must be: question,answer[,category]");
    }

    let mut rows = vec![];
    for (i, result) in rdr.records().enumerate() {
        let rec = result.with_context(|| format!("CSV parse error at record {}", i + 1))?;
        if rec.iter().all(|s| s.trim().is_empty()) {
            if strict {
                anyhow::bail!("empty line at {}", i + 1);
            } else {
                continue;
            }
        }
        let question = rec.get(0).map(|s| s.trim().to_string()).unwrap_or_default();
        let answer = rec.get(1).map(|s| s.trim().to_string()).unwrap_or_default();
        let category = rec.get(2).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());

        if question.is_empty() || answer.is_empty() {
            anyhow::bail!("empty question or answer at line {}", i + 1);
        }
        if question.len() > MAX_FIELD_LEN || answer.len() > MAX_FIELD_LEN {
            anyhow::bail!("field too long at line {}", i + 1);
        }

        rows.push((question, answer, category));
    }

    let mut store = load_store()?;
    for (q, a, c) in rows {
        let card = Card::new(q, a, c);
        store.cards.push(card);
    }
    save_store(&store)?;
    println!("Import OK — {} cartes ajoutées.", store.cards.len());
    Ok(())
}
