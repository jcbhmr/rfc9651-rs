use base64::prelude::*;
use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // Examples:
    // - /*inline_data_url("./src/favicon.ico", "image/x-icon")*/"data:image/x-icon;base64,..."
    // - /*inline_data_url("./src/logo.png", "image/png")*/""
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
