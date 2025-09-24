#![forbid(unsafe_code)]
use anyhow::Result;
use crate::storage::data_dir;

pub fn cmd_config() -> Result<()> {
    let dir = data_dir()?;
    println!("Data directory: {}", dir.display());
    println!("Aucun paramètre éditable via CLI simple pour l'instant.");
    println!("Vous pouvez modifier config.toml dans le répertoire de données si nécessaire.");
    Ok(())
}
