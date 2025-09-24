#![forbid(unsafe_code)]
use anyhow::Result;
use inquire::{Text, Confirm};
use crate::storage::{load_store, save_store};
use crate::model::{Progress};

pub fn cmd_quiz(count: Option<usize>, category: Option<&str>, _mode: String) -> Result<()> {
    let mut store = load_store()?;
    if store.cards.is_empty() {
        println!("Aucune carte importée. Utilisez `flashcards import file.csv`.");
        return Ok(());
    }

    // Build due list: naive — all cards whose progress.next_due <= now
    let now = super::model::now_ts();
    let mut due: Vec<_> = store.cards.iter().filter(|c| {
        store.progress.iter().find(|p| p.card_id == c.id)
            .map(|p| p.next_due_ts <= now)
            .unwrap_or(true)
    }).cloned().collect();

    if let Some(cat) = category {
        due.retain(|c| c.category.as_deref() == Some(cat));
    }

    if due.is_empty() {
        println!("Aucune carte due pour l'instant.");
        return Ok(());
    }

    if let Some(n) = count {
        due.truncate(n);
    }

    for card in due.iter() {
        println!("Question: {}", card.question);
        let _ = Text::new("Appuyez Entrée pour afficher la réponse...").with_placeholder("").prompt();
        println!("Réponse:\n{}", card.answer);
        let correct = Confirm::new("Correct ?").with_default(false).prompt()?;
        // update progress
        let mut p = store.progress.iter_mut().find(|p| p.card_id == card.id).cloned();
        match p.as_mut() {
            None => {
                let mut np = Progress::new(&card.id);
                if correct { np.promote(); } else { /* stay or demote -> start at 1 */ }
                store.progress.push(np);
            }
            Some(_existing) => {
                let existing = store.progress.iter_mut().find(|pp| pp.card_id == card.id).unwrap();
                if correct { existing.promote(); } else { existing.demote(); }
            }
        }
        save_store(&store)?;
    }

    println!("Session terminée.");
    Ok(())
}
