use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rfc-editor.org/info/rfc9651".to_owned();
    let response = reqwest::blocking::get(&url)?;
    let data = response.bytes()?;
    fs::write("./rfc9651.html", &data)?;
    Ok(())
}
