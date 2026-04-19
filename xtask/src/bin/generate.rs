use base64::prelude::*;
use regex::Regex;
use std::path;
use std::process::{Command, Output};
use std::{error::Error, fs};

fn lib_rs() -> Result<(), Box<dyn Error>> {
    let regex = Regex::new(r#"/\*inline_data_url\("([^"]+)", "([^"]+)"\)\*/"([^"]*)""#)?;
    let rust = fs::read_to_string("./src/lib.rs")?;
    let mut error: Option<Box<dyn Error>> = None;
    let rust = regex.replace_all(&rust, |caps: &regex::Captures| {
        if error.is_some() {
            return "\0".to_owned();
        }
        let path = &caps[1];
        let content_type = &caps[2];
        let data = match fs::read(path) {
            Ok(data) => data,
            Err(e) => {
                error = Some(e.into());
                return "\0".to_owned();
            }
        };
        let base64 = BASE64_STANDARD.encode(&data);
        format!(
            r#"/*inline_data_url("{}", "{}")*/"data:{};base64,{}""#,
            &path, &content_type, &content_type, &base64
        )
    });
    if let Some(error) = error {
        return Err(error);
    }
    fs::write("./src/lib.rs", rust.as_ref())?;
    Ok(())
}

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
        .args(["fetch"])
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Generating src/lib.rs...");
    lib_rs()?;
    eprintln!("Generated src/lib.rs");

    eprintln!("Generating rfc9651.pdf...");
    rfc9651_pdf()?;
    eprintln!("Generated rfc9651.pdf");

    eprintln!("Generating structured-field-tests/...");
    structured_field_tests()?;
    eprintln!("Generated structured-field-tests/");

    Ok(())
}
