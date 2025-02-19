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

// We need this to be able to do default_value = Preset::Trunk
impl IntoResettable<OsStr> for Preset {
    fn into_resettable(self) -> Resettable<OsStr> {
        match self {
            Preset::GitFlow => Resettable::Value(OsStr::from(GIT_FLOW)),
            Preset::Trunk => Resettable::Value(OsStr::from(TRUNK)),
            Preset::Unknown => Resettable::Reset
        }
    }
}

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(value_enum, short, long, default_value = Preset::Trunk)]
    preset: Preset,
}


impl Cli {
    pub async fn run() -> Result<(), io::Error> {
        todo!();
    }
}
