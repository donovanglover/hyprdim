use crate::cli::Cli;
use clap::Parser;

pub fn clap() -> Cli {
    Cli::parse()
}
