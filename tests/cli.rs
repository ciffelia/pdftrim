use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;

const EXAMPLE_PDF: &str = "tests/pdf/example.pdf";
const PDF20_PDF: &str = "tests/pdf/Simple PDF 2.0 file.pdf";

#[test]
fn help_succeeds() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:").and(predicate::str::contains("Arguments:")));

    Ok(())
}

#[test]
fn generate_completion_succeeds() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .args(["--generate-completion", "bash"])
        .assert()
        .success();

    Ok(())
}

#[test]
fn example_succeeds() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;

    let input_path = dir.path().join("example.pdf");
    std::fs::copy(EXAMPLE_PDF, &input_path)?;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .args([input_path])
        .assert()
        .success()
        .stdout(predicate::str::contains("==> 2 pages written on"));

    std::fs::metadata(dir.path().join("example-crop.pdf"))?;

    Ok(())
}

#[test]
fn example_without_extension_succeeds() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;

    let input_path = dir.path().join("example.pdf");
    std::fs::copy(EXAMPLE_PDF, &input_path)?;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .args([input_path.with_extension("")])
        .assert()
        .success()
        .stdout(predicate::str::contains("==> 2 pages written on"));

    std::fs::metadata(dir.path().join("example-crop.pdf"))?;

    Ok(())
}

#[test]
fn example_with_output_path_succeeds() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;

    let input_path = dir.path().join("example.pdf");
    let output_path = dir.path().join("output.pdf");
    std::fs::copy(EXAMPLE_PDF, &input_path)?;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .args([&input_path, &output_path])
        .assert()
        .success()
        .stdout(predicate::str::contains("==> 2 pages written on"));

    std::fs::metadata(output_path)?;

    Ok(())
}

#[test]
fn pdf20_succeeds() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;

    let input_path = dir.path().join("example.pdf");
    std::fs::copy(PDF20_PDF, &input_path)?;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .args([input_path])
        .assert()
        .success()
        .stdout(predicate::str::contains("==> 1 page written on"));

    std::fs::metadata(dir.path().join("example-crop.pdf"))?;

    Ok(())
}

#[test]
fn missing_input_fails() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: the following required arguments were not provided:",
        ));

    Ok(())
}

#[test]
fn missing_ghostscript_fails() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .args(["--gscmd", "nonexistent", EXAMPLE_PDF])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to execute Ghostscript"));

    Ok(())
}
