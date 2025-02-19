use std::{future::Future, io};

use clap::{builder::{IntoResettable, Resettable, OsStr}, Parser, ValueEnum};
use derive_more::Display;

use crate::vcs::{git::Git, Vcs};

const GIT_FLOW: &str = "git-flow";
const TRUNK: &str = "trunk";
const UNKNOWN: &str = "unknown";

#[derive(Clone, Debug, Display, PartialEq, Eq, ValueEnum)]
pub enum Preset {
    #[display("{TRUNK}")]
    Trunk,
    #[display("{GIT_FLOW}")]
    GitFlow,
    #[display("{UNKNOWN}")]
    Unknown,
}

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(value_enum, short, long, default_value_t = Preset::Trunk)]
    preset: Preset,
    #[arg(short, long, default_value_t = false)]
    ignore: bool
}


impl Cli {
    pub async fn run() -> Result<(), io::Error> {
        todo!();
    }
}
