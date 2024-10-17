use crate::{cli::Options, ui::clap};

/// A helper function to only print what's happening to users if they enable the verbose flag.
pub fn log(text: &str) {
    let Options { verbose, .. } = clap();

    if verbose {
        println!("{text}")
    }
}
