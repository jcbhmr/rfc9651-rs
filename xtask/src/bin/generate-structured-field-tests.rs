use std::{
    error::Error,
    path,
    process::{Command, Output},
};

fn exit_ok(output: Output) -> Result<Output, Box<dyn Error>> {
    if output.status.success() {
        Ok(output)
    } else {
        Err(format!("{}", output.status).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
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
