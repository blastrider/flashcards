#![forbid(unsafe_code)]
use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// Génère un PDF à partir du fichier HTML donné.
/// Ordre de tentative:
/// 1) wkhtmltopdf (préféré)
/// 2) pandoc (fallback, rendu CSS limité)
pub fn generate_pdf(html_path: &Path, pdf_path: &str) -> Result<()> {
    // wkhtmltopdf?
    match Command::new("wkhtmltopdf").arg("--version").status() {
        Ok(status) if status.success() => {
            println!("Utilisation de wkhtmltopdf pour générer le PDF...");
            let status = Command::new("wkhtmltopdf")
                .arg(html_path.as_os_str())
                .arg(pdf_path)
                .status()
                .context("failed to run wkhtmltopdf")?;
            if status.success() {
                println!("PDF généré (wkhtmltopdf): {}", pdf_path);
                return Ok(());
            } else {
                bail!("wkhtmltopdf échoué avec code: {}", status);
            }
        }
        _ => {
            // fallback pandoc
            println!("wkhtmltopdf non trouvé, tentative avec pandoc...");
            match Command::new("pandoc").arg("--version").status() {
                Ok(status) if status.success() => {
                    let status = Command::new("pandoc")
                        .arg(html_path.as_os_str())
                        .arg("-o")
                        .arg(pdf_path)
                        .status()
                        .context("failed to run pandoc")?;
                    if status.success() {
                        println!("PDF généré (pandoc): {}", pdf_path);
                        println!("[Note] pandoc peut altérer le rendu CSS. Pour un rendu fidèle, installez wkhtmltopdf.");
                        return Ok(());
                    } else {
                        bail!("pandoc échoué avec code: {}", status);
                    }
                }
                _ => {
                    bail!("Aucun convertisseur PDF trouvé: installez `wkhtmltopdf` (recommandé) ou `pandoc`.");
                }
            }
        }
    }
}
