use assert_cmd::Command;
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

#[test]
fn parse_valid_csv() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let path = dir.path().join("cards.csv");
    let mut f = File::create(&path)?;
    writeln!(f, "question,answer,category")?;
    writeln!(f, "Q1,A1,Math")?;
    drop(f);

    let mut cmd = Command::cargo_bin("flashcards-cli")?;
    cmd.arg("import").arg(path.to_str().unwrap());
    cmd.assert().success();
    Ok(())
}

#[test]
fn parse_malformed_csv() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let path = dir.path().join("cards.csv");
    let mut f = File::create(&path)?;
    writeln!(f, "wrong,header")?;
    writeln!(f, "Q1")?;
    drop(f);

    let mut cmd = Command::cargo_bin("flashcards-cli")?;
    cmd.arg("import").arg(path.to_str().unwrap()).arg("--strict");
    cmd.assert().failure();
    Ok(())
}
