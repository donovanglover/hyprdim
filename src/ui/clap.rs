use crate::cli::Options;
use clap::Parser;

pub fn clap() -> Options {
    Options::parse()
}
