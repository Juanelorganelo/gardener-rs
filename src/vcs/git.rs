use async_trait::async_trait;
use derive_more::From;
use std::{io, path::Path};
use tokio::process::Command;

use super::vcs::Vcs;

pub struct Git;

#[derive(From)]
pub enum Error {
    CommandFailed(io::Error),
}

#[async_trait]
impl Vcs for Git {
    type Error = Error;

    async fn detect() -> Option<Self> {
        if Path::new(".git").exists() {
            Some(Git)
        } else {
            None
        }
    }

    async fn get_current_branch(&self) -> Result<String, Self::Error> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .await?;
        Ok(String::from_utf8(output.stdout).expect("Failed to read git output"))
    }
}
