# Flashcards — CLI offline pour réviser (Leitner)

**But**
Outil CLI simple, offline, pour importer des fiches depuis un CSV, s’entraîner en mode quiz dans le terminal (méthode de Leitner), suivre la progression locale et exporter un support imprimable (Markdown → HTML → PDF).

Conception : cœur métier exposé sous forme de **bibliothèque** (`src/lib.rs`) + binaire léger `bin/flashcards-cli.rs`.
Contraintes : offline-only, pas de cloud, pas de tracking, `#![forbid(unsafe_code)]`, pas d`unwrap` en production.

---

## Fonctionnalités principales

* `import` : importe un CSV strict (`question,answer[,category]`) et valide les lignes.
* `quiz` : session interactive terminal (Entrée → afficher réponse → `Correct?`), mise à jour immédiate de la progression (Leitner 5 boîtes).
* `stats` : résumé des cartes et répartition par boîte.
* `export` : génère `flashcards.md` + `flashcards.html` (style moderne) ; option `--pdf` tente `wkhtmltopdf` (recommandé) puis `pandoc` en fallback.
* `reset` : réinitialise la progression (tout ou par catégorie).
* `config` : affiche le chemin du dossier de données.

---

## Build & installation

Prérequis : Rust toolchain stable.

```sh
# build (mode dev)
cargo build

# build release
cargo build --release

# installer localement dans cargo/bin
cargo install --path .
```

Binaire produit (déclaré dans `Cargo.toml`) : `flashcards-cli` (exécutable `target/release/flashcards-cli` ou via `cargo run --bin flashcards-cli -- ...`).

---

## Usage rapide (exemples)

### Importer des cartes

```sh
flashcards-cli import ./examples/cm2_cards.csv
# mode strict : refuse si en-tête invalide / lignes vides
flashcards-cli import ./examples/cm2_cards.csv --strict
```

### Lancer un quiz interactif

```sh
flashcards-cli quiz
flashcards-cli quiz --count 20 --category "Multiplications"
```

Workflow : la question s'affiche → appuie sur `Entrée` pour voir la réponse → répondre `y/n` à `Correct ?`.

### Exporter Markdown / HTML / PDF

```sh
# Génère markdown + html
flashcards-cli export --md /tmp/flashcards.md

# Génère markdown + html + tente de produire /tmp/flashcards.pdf
flashcards-cli export --md /tmp/flashcards.md --pdf /tmp/flashcards.pdf
```

* `wkhtmltopdf` est recommandé (respecte le CSS).
* Si absent, le binaire cherche `pandoc` en fallback (rendu CSS moins fidèle).
* Si aucun outil n’est disponible, la commande échoue avec un message explicite.

### Stats, reset, config

```sh
flashcards-cli stats
flashcards-cli reset --all
flashcards-cli reset --category "Géométrie"
flashcards-cli config   # affiche le dossier de stockage
```

---

## Format CSV (strict)

* En-tête **obligatoire** : `question,answer[,category]`
* Pas de lignes vides.
* Si un champ contient une virgule `,`, **entourer** le champ de doubles-quotes `"..."`.
* Longueur maximale d’un champ : 1000 caractères (configurable dans le code).
* Exemple :

```csv
question,answer,category
"Arrondis 7,241 au centième",3.68,Décimaux
Quelle est la capitale de la France?,Paris,Géographie
```

### Erreurs fréquentes

* `found record with 4 fields, but the previous record has 3 fields` → présence d’une virgule non-quotée ; corriger en entourant le champ concerné par `"` ou utiliser l’outil ci-dessous pour repérer les lignes fautives :

```sh
awk -F',' 'NF!=3 {print "Ligne:" NR " (champs=" NF "): " $0}' ./examples/cm2_cards.csv
```

---

## Emplacement des données

Dossier plateforme-approprié via `directories::ProjectDirs`, ex. sous Linux :

```
~/.local/share/flashcards-iced/
```

Fichiers : `store.json` (cartes + progression), `config.toml` (optionnel).

---

## Logs, debug & accessibilité

* Logs structurés via `tracing`.
* `--debug` active plus de logs ; `RUST_LOG` reconnu.
* Respect de `NO_COLOR` pour sorties sans couleur.
* Interface keyboard-first et compatible lecteurs d’écran (sorties textuelles).

---

## Tests & CI

* Tests inclus : `tests/test_csv.rs` (CSV valide / CSV malformé).
* Standards dev :

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

CI (GitHub Actions) fourni : format, clippy, tests, build.

---

## Développement & extension

* Architecture : **lib** (`src/lib.rs`) = logique métier testable ; **bin** (`bin/flashcards-cli.rs`) = wrapper CLI minimal.
* Pour ajouter une UI alternative (TUI/GUI), réutiliser simplement la lib.
* Export PDF : le code appelle des outils locaux (`wkhtmltopdf` / `pandoc`), pas de dépendance réseau.

---

## Dépannage rapide

* CSV refuse l’import → vérifier header et virgules non-quotées.
* `export --pdf` échoue → installer `wkhtmltopdf` (meilleur rendu) ou `pandoc`.
* Voir logs : `RUST_LOG=debug flashcards-cli ...` ou `flashcards-cli --debug ...`.

---

## Licence

Dual licence **MIT OR Apache-2.0** — voir `LICENSE*` dans le repo.

---

## Contribution

Prisent en charge : pull requests, issues pour bugs et demandes d’amélioration. Respecter la checklist CI (fmt, clippy, tests).