use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .args(["run", "--package", "xtask", "--bin", "generate-lib-rs"])
        .status()?;
    if !status.success() {
        return Err(format!("generate-lib-rs failed with status: {}", status).into());
    }

    let status = Command::new("cargo")
        .args(["run", "--package", "xtask", "--bin", "generate-rfc9651-html"])
        .status()?;
    if !status.success() {
        return Err(format!("generate-rfc9651-html failed with status: {}", status).into());
    }

    let status = Command::new("cargo")
        .args([
            "run",
            "--package",
            "xtask",
            "--bin",
            "generate-structured-field-tests",
        ])
        .status()?;
    if !status.success() {
        return Err(format!(
            "generate-structured-field-tests failed with status: {}",
            status
        )
        .into());
    }

    Ok(())
}
