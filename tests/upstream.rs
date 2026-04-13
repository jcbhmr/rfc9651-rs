use derive_more::Display;
use sha2::{Digest, Sha256};
use std::{
    env,
    error::Error,
    fs,
    path::{self, Path},
    process::{Command, Output},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
enum GitRelativeStatus {
    #[display("up to date")]
    UpToDate,
    #[display("behind")]
    Behind,
    #[display("ahead")]
    Ahead,
    #[display("diverged")]
    Diverged,
}

trait TrimEndLineEnding {
    fn trim_end_line_ending(&self) -> &str;
}
impl TrimEndLineEnding for str {
    fn trim_end_line_ending(&self) -> &str {
        let mut result = self;
        if result.ends_with('\n') {
            result = &result[..result.len() - 1];
            if result.ends_with('\r') {
                result = &result[..result.len() - 1];
            }
        }
        result
    }
}

trait MyExitOk {
    fn my_exit_ok(self) -> Result<Output, Box<dyn Error>>;
}
impl MyExitOk for Output {
    fn my_exit_ok(self) -> Result<Self, Box<dyn Error>> {
        if self.status.success() {
            Ok(self)
        } else {
            Err(format!("{}", self.status).into())
        }
    }
}

fn get_git_relative_status(root: &Path) -> Result<GitRelativeStatus, Box<dyn Error>> {
    let local = {
        let output = Command::new("git")
            .args(["rev-parse", "@"])
            .current_dir(root)
            .output()?
            .my_exit_ok()?;
        String::from_utf8_lossy(&output.stdout)
            .trim_end_line_ending()
            .to_owned()
    };
    let remote = {
        let output = Command::new("git")
            .args(["rev-parse", "@{u}"])
            .current_dir(root)
            .output()?
            .my_exit_ok()?;
        String::from_utf8_lossy(&output.stdout)
            .trim_end_line_ending()
            .to_owned()
    };
    let base = {
        let output = Command::new("git")
            .args(["merge-base", "@", "@{u}"])
            .current_dir(root)
            .output()?
            .my_exit_ok()?;
        String::from_utf8_lossy(&output.stdout)
            .trim_end_line_ending()
            .to_owned()
    };
    if local == remote {
        Ok(GitRelativeStatus::UpToDate)
    } else if local == base {
        Ok(GitRelativeStatus::Behind)
    } else if remote == base {
        Ok(GitRelativeStatus::Ahead)
    } else {
        Ok(GitRelativeStatus::Diverged)
    }
}

#[test]
fn structured_field_tests() -> Result<(), Box<dyn Error>> {
    let root = path::absolute("./structured-field-tests/")?;
    let relative_status = get_git_relative_status(&root)?;
    let result = (relative_status == GitRelativeStatus::UpToDate)
        .then_some(())
        .ok_or_else(|| -> Box<dyn Error> {
            format!(
                "Git repository at {:?} is '{}' when compared to upstream\n{}",
                &root,
                relative_status,
                format!("try running `git pull` in {:?}", &root)
            )
            .into()
        });
    let is_ci = env::var("CI").is_ok();
    match (result, is_ci) {
        (Ok(()), _) => Ok(()),
        (Err(e), true) => Err(e),
        (Err(e), false) => {
            eprintln!("Warning: {}", e);
            Ok(())
        }
    }
}

#[test]
fn rfc9651_pdf() -> Result<(), Box<dyn Error>> {
    let local_path = path::absolute("./rfc9651.pdf")?;
    let local_hash = Sha256::digest(fs::read(&local_path)?).to_vec();
    let remote_url = "https://www.rfc-editor.org/rfc/rfc9651.pdf".to_owned();
    let remote_data = reqwest::blocking::get(&remote_url)?.bytes()?;
    let remote_hash = Sha256::digest(&remote_data).to_vec();
    let result = (local_hash == remote_hash)
        .then_some(())
        .ok_or_else(|| -> Box<dyn Error> {
            format!(
                "local {:?} does not match remote {:?}",
                &local_path, &remote_url
            )
            .into()
        });
    let is_ci = env::var("CI").is_ok();
    match (result, is_ci) {
        (Ok(()), _) => Ok(()),
        (Err(e), true) => Err(e),
        (Err(e), false) => {
            eprintln!("Warning: {}", e);
            fs::write(local_path.with_added_extension("new"), &remote_data)?;
            Ok(())
        }
    }
}
