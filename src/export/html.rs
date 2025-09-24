#![forbid(unsafe_code)]
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use crate::model::Card;

/// Génère un HTML autonome avec CSS intégré adapté à l'impression.
pub fn write_html(path: &Path, cards: &Vec<Card>) -> Result<()> {
    let html = build_html(cards);
    fs::write(path, html).with_context(|| format!("write html {}", path.display()))?;
    Ok(())
}

fn build_html(cards: &Vec<Card>) -> String {
    let css = r#"
:root{
  --bg: #f6f7fb;
  --card-bg: #ffffff;
  --accent: #0077cc;
  --muted: #6b7280;
  --radius: 10px;
  --shadow: 0 6px 18px rgba(15,23,42,0.08);
  --gap: 18px;
  --font-stack: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", "Liberation Sans", sans-serif;
  --hq: 18pt;
  --aq: 13pt;
}
* { box-sizing: border-box; }
html,body { margin:0; padding:0; background:var(--bg); font-family:var(--font-stack); color:#0f172a; }
.container { max-width:1100px; margin:28px auto; padding:18px; }
.header { display:flex; align-items:center; gap:12px; margin-bottom:18px; }
.brand { font-weight:700; font-size:20px; color:var(--accent); }
.subtitle { color:var(--muted); font-size:13px; margin-left:auto; }
.grid { display:grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap:var(--gap); }
.card { background: var(--card-bg); border-radius: var(--radius); box-shadow: var(--shadow); padding:18px; display:flex; flex-direction:column; gap:12px; break-inside: avoid; page-break-inside: avoid; }
.card .question { font-size: var(--hq); font-weight:700; color:#071033; }
.card .meta { color:var(--muted); font-size:12px; }
.card .answer { font-size: var(--aq); color:#12243a; white-space:pre-wrap; }
@media print {
  body { background: white; }
  .container { max-width:100%; margin:0; padding:0; }
  .grid { gap:10px; }
  .card { box-shadow:none; border:1px solid #e6e9ef; border-radius:6px; }
  .no-print { display:none; }
  .page-break { page-break-after: always; }
}
.page-break { height:1px; margin: 18px 0; }
"#;

    let mut html = String::new();
    html.push_str(&format!(
        r#"<!doctype html>
<html lang="fr">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width,initial-scale=1"/>
<title>Flashcards — Export</title>
<style>{}</style>
</head>
<body>
<div class="container">
  <div class="header">
    <div class="brand">Flashcards</div>
    <div class="subtitle">Export imprimable — Leitner</div>
  </div>
  <div class="grid">
"#,
        css
    ));

    for c in cards {
        let category_html = c
            .category
            .as_ref()
            .map(|s| format!(r#"<div class="meta">{}</div>"#, html_escape(s)))
            .unwrap_or_default();
        html.push_str(&format!(
            r#"<article class="card">
  <div class="question">{}</div>
  {}
  <div class="answer">{}</div>
</article>
"#,
            html_escape(&c.question),
            category_html,
            html_escape(&c.answer)
        ));
    }

    html.push_str(
        r#"
  </div>
  <div class="page-break no-print"></div>
</div>
</body>
</html>"#,
    );

    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}
