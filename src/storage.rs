#![forbid(unsafe_code)]
use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{PathBuf, Path};
use crate::model::{Card, Progress};
use std::collections::HashMap;

const APP_QUALIFIER: &str = "com";
const APP_ORG: &str = "example";
const APP_NAME: &str = "flashcards-iced";

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub cards: Vec<Card>,
    pub progress: Vec<Progress>,
}

impl Default for Store {
    fn default() -> Self {
        Self { cards: vec![], progress: vec![] }
    }
}

/// Resolve data dir for the platform
pub(crate) fn data_dir() -> Result<PathBuf> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORG, APP_NAME)
        .map(|d| d.data_dir().to_path_buf())
        .context("unable to determine user data directory")
}

fn store_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("store.json"))
}

fn ensure_dir() -> Result<()> {
    let dir = data_dir()?;
    fs::create_dir_all(&dir).context("create data dir")?;
    Ok(())
}

fn atomic_write(path: &Path, content: &str) -> Result<()> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, content).context("write tmp file")?;
    fs::rename(&tmp, path).context("rename tmp -> path")?;
    Ok(())
}

/// Read store or default.
pub fn load_store() -> Result<Store> {
    ensure_dir()?;
    let path = store_path()?;
    if !path.exists() {
        return Ok(Store::default());
    }
    let s = fs::read_to_string(&path).context("read store file")?;
    let store: Store = serde_json::from_str(&s).context("parse store json")?;
    Ok(store)
}

pub fn save_store(store: &Store) -> Result<()> {
    ensure_dir()?;
    let path = store_path()?;
    let content = serde_json::to_string_pretty(store).context("serialize store")?;
    atomic_write(&path, &content)?;
    Ok(())
}

pub fn print_stats() -> Result<()> {
    let store = load_store()?;
    let total = store.cards.len();
    let mut by_box: HashMap<u8, usize> = HashMap::new();
    for p in store.progress.iter() {
        *by_box.entry(p.box_index).or_default() += 1;
    }
    println!("Cartes total: {}", total);
    for b in 1..=5 {
        println!("Boîte {}: {}", b, by_box.get(&b).unwrap_or(&0));
    }
    Ok(())
}

pub fn cmd_reset(all: bool, category: Option<&str>) -> Result<()> {
    let mut store = load_store()?;
    if all {
        store.progress.clear();
    } else if let Some(cat) = category {
        store.progress.retain(|p| {
            match store.cards.iter().find(|c| c.id == p.card_id) {
                Some(card) => card.category.as_deref() != Some(cat),
                None => true,
            }
        });
    } else {
        anyhow::bail!("specify --all or --category");
    }
    save_store(&store)?;
    println!("Progression réinitialisée.");
    Ok(())
}
