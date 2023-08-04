use clap::Parser;
use cli::Cli;

mod cli;

pub fn log(text: &str) {
    let Cli { verbose, .. } = Cli::parse();

    if verbose {
        println!("{text}")
    }
}
