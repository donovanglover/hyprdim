use clap::Parser;
use cli::Cli;

mod cli;

pub fn log(text: &str) {
    let cli = Cli::parse();

    if cli.verbose {
        println!("{text}")
    }
}
