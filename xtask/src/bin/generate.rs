use std::path;
use std::process::{Command, Output};
use std::{error::Error, fs};

fn rfc9651_pdf() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rfc-editor.org/info/rfc9651.pdf".to_owned();
    let response = reqwest::blocking::get(&url)?;
    let data = response.bytes()?;
    fs::write("./rfc9651.pdf", &data)?;
    Ok(())
}

fn exit_ok(output: Output) -> Result<Output, Box<dyn Error>> {
    if output.status.success() {
        Ok(output)
    } else {
        Err(format!("{}", output.status).into())
    }
}

fn structured_field_tests() -> Result<(), Box<dyn Error>> {
    let root = path::absolute("./structured-field-tests/")?;

    let output = Command::new("git")
        .args(["fetch", "origin", "main"])
        .current_dir(&root)
        .output()?;
    exit_ok(output)?;

    let output = Command::new("git")
        .args(["reset", "--hard", "origin/main"])
        .current_dir(&root)
        .output()?;
    exit_ok(output)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Generating rfc9651.pdf...");
    rfc9651_pdf()?;
    eprintln!("Generated rfc9651.pdf");

    eprintln!("Generating structured-field-tests/...");
    structured_field_tests()?;
    eprintln!("Generated structured-field-tests/");

    Ok(())
}
