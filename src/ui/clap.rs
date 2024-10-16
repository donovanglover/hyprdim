use clap::Parser;
use crate::cli::Cli;

pub fn clap() -> Cli {
    Cli::parse()
}
