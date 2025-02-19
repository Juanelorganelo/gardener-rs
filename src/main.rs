mod parser;
mod workflows;
mod vcs;
mod cli;
mod validation;

use cli::Cli;
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
}
