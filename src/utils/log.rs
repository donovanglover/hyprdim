use crate::{cli::Cli, ui::clap};

/// A helper function to only print what's happening to users if they enable the verbose flag.
pub fn log(text: &str) {
    let Cli { verbose, .. } = clap();

    if verbose {
        println!("{text}")
    }
}
